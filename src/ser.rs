use std::net::TcpStream;

use serde::Serialize;
use tungstenite::{Message, WebSocket};

use crate::{
    blocks::{BASE, MinecraftBlock},
};

macro_rules! number_to_bits {
    ($value:tt) => {{
        if $value == 0 {
            vec![MinecraftBlock::bit_to_block(0)?]
        } else {
            let mut value = $value;
            let mut bits: Vec<MinecraftBlock> = Vec::new();
            while value != 0 {
            let bit = value.rem_euclid(BASE as u128);
            value /= BASE as u128;

                let block = MinecraftBlock::bit_to_block(bit as u8)?;
                bits.push(block);
            }
            bits.reverse();
            bits
        }
    }};
}

pub struct MinecraftSerializer {
    socket: WebSocket<TcpStream>,
}

impl MinecraftSerializer {
    pub fn new(socket: WebSocket<TcpStream>) -> Self {
        MinecraftSerializer { socket }
    }

    pub(super) fn place_block(&mut self, block: MinecraftBlock) -> Result<(), MinecraftError> {
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

    #[inline(always)]
    fn place_blocks(&mut self, blocks: &[MinecraftBlock]) -> Result<(), MinecraftError> {
        blocks.iter().try_for_each(|&block| self.place_block(block))
    }

    fn serialize_number<T: Into<u128>>(
        &mut self,
        v: T,
        marker_block: MinecraftBlock,
        signed: Option<MinecraftBlock>,
    ) -> Result<(), MinecraftError> {
        self.place_block(marker_block)?;

        if let Some(block) = signed {
            self.place_block(block)?;
        }

        let v = v.into();
        self.place_blocks(&number_to_bits!(v))?;
        self.place_block(marker_block)
    }

    fn write_bytes(&mut self, v: &[u8]) -> MinecraftResult<()> {
        let blocks: Vec<MinecraftBlock> = v
            .iter()
            .map(|&x| {
                let bits = number_to_bits!(x);
                let zero = MinecraftBlock::bit_to_block(0)?;
                let mut padded = vec![zero; 2 - bits.len()];
                padded.extend(bits);
                Ok::<Vec<MinecraftBlock>, MinecraftError>(padded)
            })
            .collect::<MinecraftResult<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        self.place_blocks(&blocks)
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

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        match v {
            true => self.place_block(MinecraftBlock::RedstoneBlock),
            false => self.place_block(MinecraftBlock::RedstoneLamp),
        }
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(
            v as u8,
            MinecraftBlock::EndStone,
            Some(MinecraftBlock::OchreFroglight),
        )
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(
            v as u16,
            MinecraftBlock::RawIronBlock,
            Some(MinecraftBlock::VerdantFroglight),
        )
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(
            v as u32,
            MinecraftBlock::RawCopperBlock,
            Some(MinecraftBlock::PearlescentFroglight),
        )
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(
            v as u64,
            MinecraftBlock::RawGoldBlock,
            Some(MinecraftBlock::SeaLantern),
        )
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v, MinecraftBlock::EndStone, None)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v, MinecraftBlock::RawIronBlock, None)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v, MinecraftBlock::RawCopperBlock, None)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v, MinecraftBlock::RawGoldBlock, None)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let bits = v.to_bits();
        self.serialize_number(bits, MinecraftBlock::Shroomlight, None)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let bits = v.to_bits();
        self.serialize_number(bits, MinecraftBlock::Glowstone, None)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u32, MinecraftBlock::ChiseledDeepslate, None)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::GildedBlackstone)?;
        self.write_bytes(v.as_bytes())?;
        self.place_block(MinecraftBlock::Prismarine)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::Blackstone)?;
        self.write_bytes(v)?;
        self.place_block(MinecraftBlock::Prismarine)
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::CoalBlock)?;
        self.place_block(MinecraftBlock::CoalBlock)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.place_block(MinecraftBlock::CoalBlock)?;
        value.serialize(OptionSerializer::new(self))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::Bedrock)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::OakLog)?;
        variant_index.serialize(&mut *self)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.place_block(MinecraftBlock::SpruceLog)?;
        name.serialize(&mut *self)?;
        value.serialize(&mut *self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.place_block(MinecraftBlock::DarkOakLog)?;
        variant_index.serialize(&mut *self)?;
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.place_block(MinecraftBlock::CherryLog)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.place_block(MinecraftBlock::CrimsonStem)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.place_block(MinecraftBlock::WarpedStem)?;
        name.serialize(&mut *self)?;
        (len as u32).serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.place_block(MinecraftBlock::PurpurBlock)?;
        self.serialize_u32(variant_index)?;
        self.serialize_u32(len as u32)?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.place_block(MinecraftBlock::PurpurPillar)?;
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.place_block(MinecraftBlock::GoldBlock)?;
        name.serialize(&mut *self)?;
        (len as u32).serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.place_block(MinecraftBlock::DiamondBlock)?;
        variant_index.serialize(&mut *self)?;
        (len as u32).serialize(&mut *self)?;
        Ok(self)
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
        self.place_block(MinecraftBlock::DarkPrismarine)
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
        <Self as serde::ser::SerializeSeq>::serialize_element(self, value)
    }

    #[inline(always)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as serde::ser::SerializeSeq>::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        <Self as serde::ser::SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as serde::ser::SerializeSeq>::end(self)
    }
}

impl serde::ser::SerializeTupleVariant for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        <Self as serde::ser::SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as serde::ser::SerializeSeq>::end(self)
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
        self.place_block(MinecraftBlock::AmethystBlock)
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
        <Self as serde::ser::SerializeMap>::serialize_key(self, key)?;
        <Self as serde::ser::SerializeMap>::serialize_value(self, value)
    }

    #[inline(always)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::EmeraldBlock)
    }
}

impl serde::ser::SerializeStructVariant for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        <Self as serde::ser::SerializeStruct>::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as serde::ser::SerializeStruct>::end(self)
    }
}
