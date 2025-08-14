use std::net::TcpStream;

use serde::{Deserialize, de::Visitor};
use tungstenite::WebSocket;

use crate::{
    blocks::MinecraftBlock,
    result::{MinecraftError, MinecraftResult},
};

use super::{MCEnumAccessor, MCMapAccessor, MCSeqAccessor, MCStructAccessor};

pub struct MinecraftDeserializer {
    socket: WebSocket<TcpStream>,
}

impl<'a> MinecraftDeserializer {
    pub fn new(socket: WebSocket<TcpStream>) -> Self {
        MinecraftDeserializer { socket }
    }

    fn consume(&mut self) -> MinecraftResult<MinecraftBlock> {
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

    fn parse_number(&mut self, bits: u8) -> MinecraftResult<i64> {
        let n = bits as usize / 4; // Each hex char represents 4 bits

        let mut array = vec!['0'; n];
        for item in array.iter_mut() {
            let block = self.consume()?;
            let chr = block.to_digit()?;
            *item = chr;
        }

        let bits: Vec<u8> = array
            .chunks_exact(2)
            .map(|c| format!("{}{}", c[0], c[1]))
            .flat_map(|x| u8::from_str_radix(&x, 16))
            .collect();

        // Pad with zeros to make it 8 bytes for i64
        let mut padded_bits = [0u8; 8];
        for (i, &byte) in bits.iter().enumerate() {
            if i < 8 {
                padded_bits[i] = byte;
            }
        }

        let bits = i64::from_le_bytes(padded_bits);
        Ok(bits)
    }

    fn parse_f64(&mut self) -> MinecraftResult<f64> {
        let mut digits = Vec::new();

        loop {
            let block = self.consume()?;
            if block.is_log() {
                let chr = block.to_digit()?;
                digits.push(chr);
            } else {
                self.rewind()?;
                break;
            }
        }

        let uvs = digits.iter().rev().collect::<String>();
        let bits: u64 = uvs.parse()?;
        let value = f64::from_bits(bits);
        Ok(value)
    }

    fn parse_u8(&mut self) -> MinecraftResult<u8> {
        let mut src = String::with_capacity(2);
        src.push(self.consume()?.to_digit()?);
        src.push(self.consume()?.to_digit()?);
        Ok(u8::from_str_radix(&src, 16)?)
    }

    fn u8_from_blocks(block_1: MinecraftBlock, block_2: MinecraftBlock) -> MinecraftResult<u8> {
        let mut src = String::with_capacity(2);
        src.push(block_1.to_digit()?);
        src.push(block_2.to_digit()?);
        Ok(u8::from_str_radix(&src, 16)?)
    }

    // Consecutive blocks of wool
    fn parse_bytes(&mut self) -> MinecraftResult<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut mem = None;

        loop {
            let block = self.consume()?;
            match (block.is_wool(), mem) {
                (true, None) => {
                    mem = Some(block);
                }
                (true, Some(prev)) => {
                    let byte = Self::u8_from_blocks(prev, block)?;
                    bytes.push(byte);
                    mem = None;
                }
                (false, None) => {
                    // Rewind the non-wool block since we don't need it
                    self.rewind()?;
                    break;
                }
                (false, Some(_)) => {
                    return Err(MinecraftError::InvalidWoolSequence);
                }
            }
        }

        Ok(bytes)
    }

    fn parse_struct_sophisticated<V: Visitor<'a>>(
        &mut self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> MinecraftResult<<V as Visitor<'a>>::Value> {
        let name_actual = String::deserialize(&mut *self)?;
        if name_actual != name {
            return Err(MinecraftError::StructNameMismatch {
                expected: name.to_string(),
                found: name_actual,
            });
        }

        let len = usize::deserialize(&mut *self)?;
        visitor.visit_map(MCStructAccessor::new(self, len))
    }

    fn parse_struct<V: Visitor<'a>>(
        &mut self,
        visitor: V,
    ) -> MinecraftResult<<V as Visitor<'a>>::Value> {
        let _name = String::deserialize(&mut *self)?;
        let len = usize::deserialize(&mut *self)?;
        visitor.visit_map(MCStructAccessor::new(self, len))
    }

    fn parse_seq<V: Visitor<'a>>(
        &mut self,
        visitor: V,
    ) -> MinecraftResult<<V as Visitor<'a>>::Value> {
        let _enum_name = String::deserialize(&mut *self)?;
        let _variant_index = u32::deserialize(&mut *self)?;
        let variant_name = String::deserialize(&mut *self)?;
        visitor.visit_enum(MCEnumAccessor::new(self, variant_name))
    }

    fn handle_obsidian<V>(&mut self, visitor: V) -> MinecraftResult<<V as Visitor<'a>>::Value>
    where
        V: Visitor<'a>,
    {
        match self.consume()? {
            MinecraftBlock::Obsidian => visitor.visit_map(MCMapAccessor::new(self)),
            MinecraftBlock::QuartzBlock => self.parse_struct(visitor),
            MinecraftBlock::Cobblestone => self.parse_seq(visitor),
            _ => Err(MinecraftError::InvalidBlockSequence(
                "Obsidian sequence".to_string(),
            )),
        }
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut MinecraftDeserializer {
    type Error = MinecraftError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let block = self.consume()?;
        match block {
            MinecraftBlock::Glowstone | MinecraftBlock::RedstoneLamp => {
                self.rewind()?;
                self.deserialize_bool(visitor)
            }
            MinecraftBlock::Obsidian => self.handle_obsidian(visitor),
            b if b.is_log() => {
                self.rewind()?;
                visitor.visit_f64(self.parse_f64()?)
            }
            b if b.is_wool() => {
                self.rewind()?;
                self.deserialize_bytes(visitor)
            }
            _ => Err(MinecraftError::TypeMismatch {
                expected: "known type".to_string(),
                found: "unknown type".to_string(),
            }),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.consume()? {
            MinecraftBlock::Glowstone => visitor.visit_bool(true),
            MinecraftBlock::RedstoneLamp => visitor.visit_bool(false),
            other => Err(MinecraftError::UnexpectedBlock {
                expected: "Glowstone or RedstoneLamp".to_string(),
                found: other.to_string(),
            }),
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
            (other1, other2) => Err(MinecraftError::UnexpectedBlock {
                expected: "CoalBlock followed by Bricks".to_string(),
                found: format!("{}, {}", other1, other2),
            }),
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
            (other1, other2) => Err(MinecraftError::UnexpectedBlock {
                expected: "RawCopperBlock followed by Bricks".to_string(),
                found: format!("{}, {}", other1, other2),
            }),
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
            (other1, other2) => Err(MinecraftError::UnexpectedBlock {
                expected: "RawIronBlock followed by Bricks".to_string(),
                found: format!("{}, {}", other1, other2),
            }),
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
            (other1, other2) => Err(MinecraftError::UnexpectedBlock {
                expected: "RawGoldBlock followed by Bricks".to_string(),
                found: format!("{}, {}", other1, other2),
            }),
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
            other => Err(MinecraftError::UnexpectedBlock {
                expected: "CoalBlock".to_string(),
                found: other.to_string(),
            }),
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
            other => Err(MinecraftError::UnexpectedBlock {
                expected: "RawCopperBlock".to_string(),
                found: other.to_string(),
            }),
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
            other => Err(MinecraftError::UnexpectedBlock {
                expected: "RawIronBlock".to_string(),
                found: other.to_string(),
            }),
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
            other => Err(MinecraftError::UnexpectedBlock {
                expected: "RawGoldBlock".to_string(),
                found: other.to_string(),
            }),
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
            other => Err(MinecraftError::UnexpectedBlock {
                expected: "CryingObsidian".to_string(),
                found: other.to_string(),
            }),
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
            return Err(MinecraftError::UnexpectedBlock {
                expected: "EmeraldBlock".to_string(),
                found: "other block".to_string(),
            });
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
        match self.consume()? {
            MinecraftBlock::Bedrock => visitor.visit_none(),
            MinecraftBlock::RedstoneBlock => visitor.visit_some(self),
            other => Err(MinecraftError::UnexpectedBlock {
                expected: "Bedrock or RedstoneBlock".to_string(),
                found: other.to_string(),
            }),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
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
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::Bricks) => {
                visitor.visit_seq(MCSeqAccessor::new(self))
            }
            _ => Err(MinecraftError::Custom("Expected seq".to_string())),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::Glass) => {
                visitor.visit_seq(MCSeqAccessor::new(self))
            }
            (other1, other2) => Err(MinecraftError::UnexpectedBlock {
                expected: "Obsidian followed by Glass".to_string(),
                found: format!("{}, {}", other1, other2),
            }),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
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
            (other1, other2) => Err(MinecraftError::UnexpectedBlock {
                expected: "Obsidian followed by Obsidian".to_string(),
                found: format!("{}, {}", other1, other2),
            }),
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
                self.parse_struct_sophisticated(name, fields, visitor)
            }
            (other1, other2) => Err(MinecraftError::UnexpectedBlock {
                expected: "Obsidian followed by QuartzBlock".to_string(),
                found: format!("{}, {}", other1, other2),
            }),
        }
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
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::Cobblestone) => self.parse_seq(visitor),
            (other1, other2) => Err(MinecraftError::UnexpectedBlock {
                expected: "Obsidian followed by Cobblestone".to_string(),
                found: format!("{}, {}", other1, other2),
            }),
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
        self.deserialize_any(visitor)
    }
}
