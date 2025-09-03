use std::net::TcpStream;

use tungstenite::WebSocket;

use crate::{
    blocks::MinecraftBlock,
    de::map::MCMapAccess,
    result::{MinecraftError, MinecraftResult},
};

use super::r#enum::MCEnumAccessor;

pub struct MinecraftDeserializer {
    socket: WebSocket<TcpStream>,
}

impl<'a> MinecraftDeserializer {
    pub fn new(socket: WebSocket<TcpStream>) -> Self {
        MinecraftDeserializer { socket }
    }

    pub(super) fn peek(&mut self) -> MinecraftResult<MinecraftBlock> {
        self.socket
            .write(tungstenite::Message::Text("peek".into()))?;
        self.socket.flush()?;

        let response = self.socket.read()?;
        let text = response.to_text()?;

        text.try_into()
    }

    pub(super) fn consume(&mut self) -> MinecraftResult<MinecraftBlock> {
        self.socket
            .write(tungstenite::Message::Text("consume".into()))?;
        self.socket.flush()?;

        let response = self.socket.read()?;
        let text = response.to_text()?;

        text.try_into()
    }

    pub(super) fn rewind(&mut self) -> MinecraftResult<()> {
        self.socket
            .write(tungstenite::Message::Text("rewind".into()))?;
        self.socket.flush()?;

        let response = self.socket.read()?;
        let text = response.to_text()?;
        match text == "done" {
            true => Ok(()),
            false => Err(MinecraftError::RewindFailed),
        }
    }

    fn parse_number(
        &mut self,
        marker_block: MinecraftBlock,
        signed: Option<MinecraftBlock>,
    ) -> MinecraftResult<u128> {
        let b = self.consume()?;
        if b != marker_block {
            return Err(MinecraftError::UnexpectedBlock {
                expected: marker_block.to_string(),
                found: b.to_string(),
            });
        }

        if let Some(sign_block) = signed {
            let b = self.consume()?;
            if b != sign_block {
                return Err(MinecraftError::UnexpectedBlock {
                    expected: sign_block.to_string(),
                    found: b.to_string(),
                });
            }
        }

        let mut result = 0;
        loop {
            let block = self.consume()?;
            if block == marker_block {
                break;
            }

            let bit = block.block_to_bit()? as u128;
            result *= 75;
            result += bit;
        }

        Ok(result)
    }

    fn parse_a_number(&mut self) -> Result<u128, MinecraftError> {
        let marker_block = self.consume()?;
        if !matches!(
            marker_block,
            MinecraftBlock::EndStone
                | MinecraftBlock::RawIronBlock
                | MinecraftBlock::RawCopperBlock
                | MinecraftBlock::RawGoldBlock
                | MinecraftBlock::Shroomlight // f32
                | MinecraftBlock::Glowstone // f64
        ) {
            return Err(MinecraftError::Custom(
                "This is not a number prefix".to_string(),
            ));
        }

        let mut result = 0;

        let signed_block = self.consume()?;
        if !signed_block.is_light() {
            result += signed_block.block_to_bit()? as u128;
        }

        loop {
            let block = self.consume()?;
            if block == marker_block {
                break;
            }

            let bit = block.block_to_bit()? as u128;
            result *= 75;
            result += bit;
        }

        Ok(result)
    }

    fn parse_bytes(&mut self, marker_block: MinecraftBlock) -> Result<Vec<u8>, MinecraftError> {
        let b = self.consume()?;
        if b != marker_block {
            return Err(MinecraftError::UnexpectedBlock {
                expected: marker_block.to_string(),
                found: b.to_string(),
            });
        }

        let mut bytes = Vec::new();
        loop {
            let b1 = self.consume()?;
            if b1 == MinecraftBlock::Prismarine {
                break;
            }

            let b2 = self.consume()?;
            if b2 == MinecraftBlock::Prismarine {
                return Err(MinecraftError::UnexpectedBlock {
                    expected: marker_block.to_string(),
                    found: b2.to_string(),
                });
            }

            let v = b1.block_to_bit()? as u32 * 75 + b2.block_to_bit()? as u32;
            let v = v as u8;
            bytes.push(v);
        }

        Ok(bytes)
    }

    fn parse_string(&mut self) -> Result<String, MinecraftError> {
        let bytes = self.parse_bytes(MinecraftBlock::GildedBlackstone)?;
        let string = String::from_utf8(bytes)?;
        Ok(string)
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut MinecraftDeserializer {
    type Error = MinecraftError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.consume()? {
            MinecraftBlock::RedstoneBlock => visitor.visit_bool(true),
            MinecraftBlock::RedstoneLamp => visitor.visit_bool(false),
            _ => Err(MinecraftError::Custom("Wrong boolean block".to_string())),
        }
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let number = self.parse_number(
            MinecraftBlock::EndStone,
            Some(MinecraftBlock::OchreFroglight),
        )?;
        visitor.visit_i8(number as i8)
    }

    #[inline]
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let number = self.parse_number(
            MinecraftBlock::RawIronBlock,
            Some(MinecraftBlock::VerdantFroglight),
        )?;
        visitor.visit_i16(number as i16)
    }

    #[inline]
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let number = self.parse_number(
            MinecraftBlock::RawCopperBlock,
            Some(MinecraftBlock::PearlescentFroglight),
        )?;
        visitor.visit_i32(number as i32)
    }

    #[inline]
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let number = self.parse_number(
            MinecraftBlock::RawGoldBlock,
            Some(MinecraftBlock::SeaLantern),
        )?;
        visitor.visit_i64(number as i64)
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let number = self.parse_number(MinecraftBlock::EndStone, None)?;
        visitor.visit_u8(number as u8)
    }

    #[inline]
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let number = self.parse_number(MinecraftBlock::RawIronBlock, None)?;
        visitor.visit_u16(number as u16)
    }

    #[inline]
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let number = self.parse_number(MinecraftBlock::RawCopperBlock, None)?;
        visitor.visit_u32(number as u32)
    }

    #[inline]
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let number = self.parse_number(MinecraftBlock::RawGoldBlock, None)?;
        visitor.visit_u64(number as u64)
    }

    #[inline]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bits = self.parse_number(MinecraftBlock::Shroomlight, None)? as u32;
        let v = f32::from_bits(bits);
        visitor.visit_f32(v)
    }

    #[inline]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bits = self.parse_number(MinecraftBlock::Glowstone, None)? as u64;
        let v = f64::from_bits(bits);
        visitor.visit_f64(v)
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let raw = self.parse_number(MinecraftBlock::ChiseledDeepslate, None)? as u32;
        let c = char::from_u32(raw)
            .ok_or_else(|| MinecraftError::Custom("Could not convert u32 to char".to_string()))?;
        visitor.visit_char(c)
    }

    #[inline(always)]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let string = self.parse_string()?;
        visitor.visit_string(string)
    }

    #[inline(always)]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bytes = self.parse_bytes(MinecraftBlock::Blackstone)?;
        visitor.visit_byte_buf(bytes)
    }

    #[inline(always)]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.consume()?;
        if b != MinecraftBlock::Bedrock {
            return Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::Bedrock.to_string(),
                found: b.to_string(),
            });
        }

        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.consume()?;
        if b != MinecraftBlock::SpruceLog {
            return Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::SpruceLog.to_string(),
                found: b.to_string(),
            });
        }

        let actual_name = self.parse_string()?;
        assert_eq!(actual_name, name);

        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.consume()?;
        if b != MinecraftBlock::CherryLog {
            return Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::CherryLog.to_string(),
                found: b.to_string(),
            });
        }

        let seq = visitor.visit_seq(&mut *self);

        let b = self.consume()?;
        match b {
            MinecraftBlock::DarkPrismarine => seq,
            _ => Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::DarkPrismarine.to_string(),
                found: b.to_string(),
            }),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.consume()?;
        if b != MinecraftBlock::CrimsonStem {
            return Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::CrimsonStem.to_string(),
                found: b.to_string(),
            });
        }

        let tuple = visitor.visit_seq(&mut *self);

        let b = self.consume()?;
        match b {
            MinecraftBlock::DarkPrismarine => tuple,
            _ => Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::DarkPrismarine.to_string(),
                found: b.to_string(),
            }),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.consume()?;
        if b != MinecraftBlock::WarpedStem {
            return Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::WarpedStem.to_string(),
                found: b.to_string(),
            });
        }

        let actual_name = self.parse_string()?;
        assert_eq!(name, actual_name);

        let actual_len = self.parse_a_number()? as usize;
        assert_eq!(actual_len, len);

        let tuple_struct = visitor.visit_seq(&mut *self);

        let b = self.consume()?;
        match b {
            MinecraftBlock::DarkPrismarine => tuple_struct,
            _ => Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::DarkPrismarine.to_string(),
                found: b.to_string(),
            }),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.consume()?;
        if b != MinecraftBlock::PurpurPillar {
            return Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::PurpurPillar.to_string(),
                found: b.to_string(),
            });
        }

        let access = MCMapAccess::new(self, MinecraftBlock::AmethystBlock);
        visitor.visit_map(access)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.consume()?;
        if b != MinecraftBlock::GoldBlock {
            return Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::GoldBlock.to_string(),
                found: b.to_string(),
            });
        }

        let actual_name = self.parse_string()?;
        assert_eq!(name, actual_name);

        let actual_len = self.parse_a_number()? as usize;
        assert_eq!(actual_len, fields.len());

        let access = MCMapAccess::new(self, MinecraftBlock::EmeraldBlock);
        visitor.visit_map(access)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(MCEnumAccessor::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek()? {
            MinecraftBlock::GildedBlackstone => self.deserialize_string(visitor),
            MinecraftBlock::OakLog => {
                self.consume()?;
                let number = self.parse_a_number()? as u32;
                visitor.visit_u32(number)
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
