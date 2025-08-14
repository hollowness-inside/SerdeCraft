use std::net::TcpStream;

use serde::Serialize;
use tungstenite::{Message, WebSocket};

use crate::{blocks::MinecraftBlock, result::MinecraftError};

pub struct MinecraftSerializer {
    socket: WebSocket<TcpStream>,
}

impl MinecraftSerializer {
    pub fn new(socket: WebSocket<TcpStream>) -> Self {
        MinecraftSerializer { socket }
    }

    fn place_block(&mut self, block: MinecraftBlock) -> Result<(), MinecraftError> {
        let block_name = block.to_string();
        let message = Message::Text(block_name.into());
        self.socket
            .send(message)
            .map_err(|e| MinecraftError::WebSocketSend {
                message: "Failed to send block placement message".to_string(),
                source: Box::new(e),
            })?;
        self.socket
            .read()
            .map_err(|e| MinecraftError::WebSocketReceive {
                message: "Failed to receive block placement confirmation".to_string(),
                source: Box::new(e),
            })?;
        Ok(())
    }

    fn place_blocks(&mut self, blocks: &[MinecraftBlock]) -> Result<(), MinecraftError> {
        for &block in blocks {
            self.place_block(block)?;
        }
        Ok(())
    }

    fn serialize_number(
        &mut self,
        v: u64,
        bits: usize,
        signed: bool,
    ) -> Result<(), MinecraftError> {
        match bits {
            8 => self.place_block(MinecraftBlock::CoalBlock)?,
            16 => self.place_block(MinecraftBlock::RawCopperBlock)?,
            32 => self.place_block(MinecraftBlock::RawIronBlock)?,
            64 => self.place_block(MinecraftBlock::RawGoldBlock)?,
            _ => panic!("Unsupported bit size for serialization: {}", bits),
        }

        if signed {
            self.place_block(MinecraftBlock::Bricks)?
        }

        v.to_le_bytes()
            .into_iter()
            .take(bits / 8)
            .flat_map(MinecraftBlock::u8_to_terracotta)
            .map(|block| self.place_block(block))
            .collect()
    }
}

impl serde::ser::Serializer for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline(always)]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let block = match v {
            true => MinecraftBlock::Glowstone,
            false => MinecraftBlock::RedstoneLamp,
        };

        self.place_block(block)
    }

    #[inline(always)]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u64, 8, true)
    }

    #[inline(always)]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u64, 16, true)
    }

    #[inline(always)]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u64, 32, true)
    }

    #[inline(always)]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u64, 64, true)
    }

    #[inline(always)]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u64, 8, false)
    }

    #[inline(always)]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u64, 16, false)
    }

    #[inline(always)]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u64, 32, false)
    }

    #[inline(always)]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v, 64, false)
    }

    #[inline(always)]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let mut bits = v.to_bits();
        while bits > 0 {
            let rem = bits % 10;
            let block = MinecraftBlock::dec_digit_to_log(rem as u8);
            self.place_block(block)?;
            bits /= 10;
        }

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::CryingObsidian)?;
        let block = MinecraftBlock::u8_to_wool(v as u8);
        self.place_blocks(&block)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())?;
        self.place_block(MinecraftBlock::EmeraldBlock)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let blocks: Vec<_> = v
            .iter()
            .copied()
            .flat_map(MinecraftBlock::u8_to_wool)
            .collect();

        self.place_blocks(&blocks)
    }

    #[inline(always)]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::Bedrock)
    }

    #[inline(always)]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.place_block(MinecraftBlock::RedstoneBlock)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::Obsidian)?;
        self.place_block(MinecraftBlock::Cobblestone)?;
        name.serialize(&mut *self)?;
        variant_index.serialize(&mut *self)?;
        variant.serialize(&mut *self)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    #[inline(always)]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.place_block(MinecraftBlock::Obsidian)?;
        self.place_block(MinecraftBlock::Bricks)?;
        Ok(self)
    }

    #[inline(always)]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.place_block(MinecraftBlock::Obsidian)?;
        self.place_block(MinecraftBlock::Glass)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    #[inline(always)]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.place_block(MinecraftBlock::Obsidian)?;
        self.place_block(MinecraftBlock::Obsidian)?;
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.place_block(MinecraftBlock::Obsidian)?;
        self.place_block(MinecraftBlock::QuartzBlock)?;
        name.serialize(&mut *self)?;
        len.serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl serde::ser::SerializeSeq for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    #[inline(always)]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline(always)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeTuple for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    #[inline(always)]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline(always)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeTupleStruct for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl serde::ser::SerializeTupleVariant for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl serde::ser::SerializeMap for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    #[inline(always)]
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        key.serialize(&mut **self)
    }

    #[inline(always)]
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline(always)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeStruct for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    #[inline(always)]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    #[inline(always)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeStructVariant for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
