use std::net::TcpStream;

use serde::Deserialize;
use tungstenite::WebSocket;

use crate::{
    blocks::MinecraftBlock,
    de::{
        r#enum::MCEnumAccessor, map::MCMapAccessor, r#struct::MCStructAccessor,
        tuple::MCTupleAccessor,
    },
    result::{MinecraftError, MinecraftResult},
};

pub struct MinecraftDeserializer {
    socket: WebSocket<TcpStream>,
}

impl MinecraftDeserializer {
    pub fn new(socket: WebSocket<TcpStream>) -> Self {
        MinecraftDeserializer { socket }
    }

    fn peek(&mut self) -> MinecraftResult<MinecraftBlock> {
        self.socket
            .write(tungstenite::Message::Text("peek".into()))?;
        self.socket.flush()?;

        let response = self.socket.read()?;
        let text = response.to_text()?;
        text.try_into()
    }

    fn consume(&mut self) -> MinecraftResult<MinecraftBlock> {
        self.socket
            .write(tungstenite::Message::Text("consume".into()))?;
        self.socket.flush()?;

        let response = self.socket.read()?;
        let text = response.to_text()?;

        text.try_into()
    }

    fn parse_number(&mut self, bits: u8) -> MinecraftResult<i64> {
        let n = bits as usize / 4;

        let mut array = ['0'; 16];
        for item in array.iter_mut() {
            let block = self.consume()?;
            let chr = block.to_char().ok_or(MinecraftError::Char)?;
            *item = chr;
        }

        let bits: Vec<u8> = array
            .chunks_exact(2)
            .map(|c| format!("{}{}", c[0], c[1]))
            .flat_map(|x| u8::from_str_radix(&x, 16))
            .collect();

        let bits: [u8; 8] = bits
            .try_into()
            .map_err(|_| MinecraftError::Custom("Invalid number of bits".to_string()))?;

        let bits = i64::from_le_bytes(bits);
        Ok(bits)
    }

    fn parse_f64(&mut self) -> MinecraftResult<f64> {
        let mut chars = Vec::new();

        loop {
            let block = self.peek()?;
            if block.is_glass() {
                self.consume()?;
                let chr = block.to_char().ok_or(MinecraftError::Char)?;
                chars.push(chr);
            } else {
                break;
            }
        }

        let bits = chars
            .chunks_exact(2)
            .map(|c| format!("{}{}", c[0], c[1]))
            .flat_map(|x| u8::from_str_radix(&x, 16))
            .collect::<Vec<_>>();

        let value = f64::from_le_bytes(
            bits.try_into()
                .map_err(|_| MinecraftError::Custom("Invalid f64".to_string()))?,
        );

        Ok(value)
    }

    fn parse_u8(&mut self) -> MinecraftResult<u8> {
        let mut src = String::with_capacity(2);
        src.push(self.consume()?.to_char().ok_or(MinecraftError::Char)?);
        src.push(self.consume()?.to_char().ok_or(MinecraftError::Char)?);
        Ok(u8::from_str_radix(&src, 16)?)
    }

    fn u8_from_blocks(&mut self, blocks: &[MinecraftBlock]) -> MinecraftResult<u8> {
        let mut src = String::with_capacity(2);
        src.push(blocks[0].to_char().ok_or(MinecraftError::Char)?);
        src.push(blocks[1].to_char().ok_or(MinecraftError::Char)?);
        u8::from_str_radix(&src, 16).map_err(|_| MinecraftError::Custom("Invalid u8".to_string()))
    }

    // Consecutive blocks of wool
    fn parse_bytes(&mut self) -> MinecraftResult<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut mem = None;

        loop {
            let block = self.peek()?;
            match (block.is_wool(), mem) {
                (true, None) => {
                    self.consume()?;
                    mem = Some(block);
                }
                (true, Some(prev)) => {
                    self.consume()?;
                    let byte = self.u8_from_blocks(&[prev, block])?;
                    bytes.push(byte);
                    mem = None;
                }
                (false, None) => break,
                (false, Some(_)) => {
                    return Err(MinecraftError::Custom("Invalid wool sequence".to_string()));
                }
            }
        }

        Ok(bytes)
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut MinecraftDeserializer {
    type Error = MinecraftError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek()? {
            MinecraftBlock::Glowstone | MinecraftBlock::RedstoneLamp => {
                self.deserialize_bool(visitor)
            }
            _ => Err(MinecraftError::Custom("Unknown type".to_string())),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.consume()? {
            MinecraftBlock::Glowstone => visitor.visit_bool(true),
            MinecraftBlock::RedstoneLamp => visitor.visit_bool(false),
            _ => Err(MinecraftError::Custom("Expected a boolean".to_string())),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let first = self.consume()?;
        let second = self.consume()?;
        match (first, second) {
            (MinecraftBlock::CoalBlock, MinecraftBlock::Bricks) => {
                let n = self.parse_number(8)? as i8;
                visitor.visit_i8(n)
            }
            _ => Err(MinecraftError::Custom("Expected i8".to_string())),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let first = self.consume()?;
        let second = self.consume()?;
        match (first, second) {
            (MinecraftBlock::RawCopperBlock, MinecraftBlock::Bricks) => {
                let n = self.parse_number(16)? as i16;
                visitor.visit_i16(n)
            }
            _ => Err(MinecraftError::Custom("Expected i16".to_string())),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let first = self.consume()?;
        let second = self.consume()?;
        match (first, second) {
            (MinecraftBlock::RawIronBlock, MinecraftBlock::Bricks) => {
                let n = self.parse_number(32)? as i32;
                visitor.visit_i32(n)
            }
            _ => Err(MinecraftError::Custom("Expected i32".to_string())),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let first = self.consume()?;
        let second = self.consume()?;
        match (first, second) {
            (MinecraftBlock::RawGoldBlock, MinecraftBlock::Bricks) => {
                let n = self.parse_number(64)?;
                visitor.visit_i64(n)
            }
            _ => Err(MinecraftError::Custom("Expected i64".to_string())),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let first = self.consume()?;
        match first {
            MinecraftBlock::CoalBlock => {
                let n = self.parse_number(8)? as u8;
                visitor.visit_u8(n)
            }
            _ => Err(MinecraftError::Custom("Expected u8".to_string())),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.consume()? {
            MinecraftBlock::RawCopperBlock => {
                let n = self.parse_number(16)? as u16;
                visitor.visit_u16(n)
            }
            _ => Err(MinecraftError::Custom("Expected u16".to_string())),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.consume()? {
            MinecraftBlock::RawIronBlock => {
                let n = self.parse_number(32)? as u32;
                visitor.visit_u32(n)
            }
            _ => Err(MinecraftError::Custom("Expected u32".to_string())),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.consume()? {
            MinecraftBlock::RawGoldBlock => {
                let n = self.parse_number(64)? as u64;
                visitor.visit_u64(n)
            }
            _ => Err(MinecraftError::Custom("Expected u64".to_string())),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = self.parse_f64()?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.consume()? {
            MinecraftBlock::CryingObsidian => {
                let value = self.parse_u8()?;
                visitor.visit_char(value as char)
            }
            _ => Err(MinecraftError::Custom("Expected char".to_string())),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = self.parse_bytes()?;
        if self.consume()? != MinecraftBlock::EmeraldBlock {
            return Err(MinecraftError::Custom("Expected string".to_string()));
        }

        let value = String::from_utf8(value)?;
        visitor.visit_string(value)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = self.parse_bytes()?;
        visitor.visit_bytes(&value)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::Glass) => {
                visitor.visit_seq(MCTupleAccessor::new(self))
            }
            _ => Err(MinecraftError::Custom("Expected tuple".to_string())),
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
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::Obsidian) => {
                visitor.visit_map(MCMapAccessor::new(self))
            }
            _ => Err(MinecraftError::Custom("Expected struct".to_string())),
        }
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
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::QuartzBlock) => {
                let name_actual = String::deserialize(&mut *self)?;
                if name_actual != name {
                    return Err(MinecraftError::NotMatching(format!(
                        "Expected struct name '{}', found '{}'",
                        name, name_actual
                    )));
                }

                let len = usize::deserialize(&mut *self)?;
                visitor.visit_map(MCStructAccessor::new(self, len))
            }
            _ => Err(MinecraftError::Custom("Expected struct".to_string())),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::Cobblestone) => {
                let _enum_name = String::deserialize(&mut *self)?;
                let _variant_index = u32::deserialize(&mut *self)?;
                let variant_name = String::deserialize(&mut *self)?;
                visitor.visit_enum(MCEnumAccessor::new(self, variant_name))
            }
            _ => Err(MinecraftError::Custom("Expected enum".to_string())),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
