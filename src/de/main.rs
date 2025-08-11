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

    pub(super) fn rewind(&mut self) -> MinecraftResult<()> {
        self.socket
            .write(tungstenite::Message::Text("rewind".into()))?;
        self.socket.flush()?;

        let response = self.socket.read()?;
        let text = response.to_text()?;
        match text == "done" {
            true => Ok(()),
            false => Err(MinecraftError::Custom("Rewind failed".to_string())),
        }
    }

    fn parse_number(&mut self, bits: u8) -> MinecraftResult<i64> {
        let n = bits as usize / 4; // Each hex char represents 4 bits

        let mut array = vec!['0'; n];
        for item in array.iter_mut() {
            let block = self.consume()?;
            let chr = block.to_digit().ok_or(MinecraftError::Char)?;
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
                let chr = block.to_digit().ok_or(MinecraftError::Char)?;
                digits.push(chr);
            } else {
                self.rewind()?;
                break;
            }
        }

        let uvs = digits.iter().collect::<String>();
        let bits: u64 = uvs.parse()?;
        let value = f64::from_bits(bits);
        Ok(value)
    }

    fn parse_u8(&mut self) -> MinecraftResult<u8> {
        let mut src = String::with_capacity(2);
        src.push(self.consume()?.to_digit().ok_or(MinecraftError::Char)?);
        src.push(self.consume()?.to_digit().ok_or(MinecraftError::Char)?);
        Ok(u8::from_str_radix(&src, 16)?)
    }

    fn u8_from_blocks(&mut self, blocks: &[MinecraftBlock]) -> MinecraftResult<u8> {
        let mut src = String::with_capacity(2);
        src.push(blocks[0].to_digit().ok_or(MinecraftError::Char)?);
        src.push(blocks[1].to_digit().ok_or(MinecraftError::Char)?);
        u8::from_str_radix(&src, 16).map_err(|_| MinecraftError::Custom("Invalid u8".to_string()))
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
                    let byte = self.u8_from_blocks(&[prev, block])?;
                    bytes.push(byte);
                    mem = None;
                }
                (false, None) => {
                    // Rewind the non-wool block since we don't need it
                    self.rewind()?;
                    break;
                }
                (false, Some(_)) => {
                    return Err(MinecraftError::Custom("Invalid wool sequence".to_string()));
                }
            }
        }

        Ok(bytes)
    }

    fn parse_struct_sophisticated<V: Visitor<'a>>(
        &mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> MinecraftResult<<V as Visitor<'a>>::Value> {
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
            _ => Err(MinecraftError::Custom(
                "Unexpected Obsidian Sequence".to_string(),
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
        match self.consume()? {
            MinecraftBlock::Bedrock => visitor.visit_none(),
            MinecraftBlock::RedstoneBlock => visitor.visit_some(self),
            _ => Err(MinecraftError::Custom("Expected option".to_string())),
        }
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
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::Bricks) => {
                visitor.visit_seq(MCSeqAccessor::new(self))
            }
            _ => Err(MinecraftError::Custom("Expected seq".to_string())),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b1 = self.consume()?;
        let b2 = self.consume()?;
        match (b1, b2) {
            (MinecraftBlock::Obsidian, MinecraftBlock::Glass) => {
                visitor.visit_seq(MCSeqAccessor::new(self))
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
                self.parse_struct_sophisticated(name, fields, visitor)
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
            (MinecraftBlock::Obsidian, MinecraftBlock::Cobblestone) => self.parse_seq(visitor),
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
        self.deserialize_any(visitor)
    }
}
