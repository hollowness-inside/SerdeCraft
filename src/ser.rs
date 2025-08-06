use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use websocket::{Message, sync::Client};

use crate::{blocks::MinecraftBlock, result::MinecraftError};

pub struct MinecraftSerializer(Arc<Mutex<Client<TcpStream>>>);

impl MinecraftSerializer {
    pub fn new(client: Arc<Mutex<Client<TcpStream>>>) -> Self {
        MinecraftSerializer(client)
    }

    fn place_block(&mut self, block: MinecraftBlock) -> Result<(), MinecraftError> {
        let block_name = block.to_string();
        let message = Message::text(block_name);
        self.0.try_lock().unwrap().send_message(&message).unwrap();
        Ok(())
    }

    fn place_blocks(&mut self, blocks: &[MinecraftBlock]) -> Result<(), MinecraftError> {
        for &block in blocks {
            self.place_block(block)?;
        }
        Ok(())
    }

    fn u64_to_blocks(v: u64) -> Vec<MinecraftBlock> {
        v.to_string()
            .chars()
            .map(|digit| match digit {
                '0' => MinecraftBlock::CherryLog,
                '1' => MinecraftBlock::BambooLog,
                '2' => MinecraftBlock::BirchLog,
                '3' => MinecraftBlock::OakLog,
                '4' => MinecraftBlock::JungleLog,
                '5' => MinecraftBlock::AcaciaLog,
                '6' => MinecraftBlock::SpruceLog,
                '7' => MinecraftBlock::DarkOakLog,
                '8' => MinecraftBlock::CrimsonLog,
                '9' => MinecraftBlock::WarpedLog,
                _ => panic!("Invalid digit in u64_to_blocks"),
            })
            .collect()
    }

    fn bytes_to_blocks(v: &[u8]) -> Vec<MinecraftBlock> {
        let mut blocks = Vec::new();

        let len = v.len() as u64;
        blocks.extend(Self::u64_to_blocks(len));
        blocks.push(MinecraftBlock::SpruceLog);

        for &c in v {
            let b = Self::u8_to_blocks(c);
            blocks.extend(b);
        }

        blocks
    }

    fn u8_to_blocks(c: u8) -> Vec<MinecraftBlock> {
        let hex = format!("{:02X}", c);
        let mut blocks = Vec::new();
        for c in hex.chars() {
            let block = match c {
                '0' => MinecraftBlock::WhiteWool,
                '1' => MinecraftBlock::LightGrayWool,
                '2' => MinecraftBlock::GrayWool,
                '3' => MinecraftBlock::BlackWool,
                '4' => MinecraftBlock::BrownWool,
                '5' => MinecraftBlock::RedWool,
                '6' => MinecraftBlock::OrangeWool,
                '7' => MinecraftBlock::YellowWool,
                '8' => MinecraftBlock::LimeWool,
                '9' => MinecraftBlock::GreenWool,
                'A' => MinecraftBlock::CyanWool,
                'B' => MinecraftBlock::LightBlueWool,
                'C' => MinecraftBlock::BlueWool,
                'D' => MinecraftBlock::PurpleWool,
                'E' => MinecraftBlock::MagentaWool,
                'F' => MinecraftBlock::PinkWool,
                _ => panic!("Invalid byte in u8_to_block: {}", hex),
            };

            blocks.push(block);
        }

        blocks
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

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        match v {
            true => self.place_block(MinecraftBlock::RedstoneBlock),
            false => self.place_block(MinecraftBlock::CoalBlock),
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        if v < 0 {
            self.place_block(MinecraftBlock::CherryFence)?;
        }

        let b = MinecraftSerializer::u8_to_blocks(v as u8);
        self.place_blocks(&b)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        if v < 0 {
            self.place_block(MinecraftBlock::DarkOakFence)?;
        }
        self.serialize_u64(v as u64)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let b = MinecraftSerializer::u8_to_blocks(v);
        self.place_blocks(&b)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::RawIronBlock)?;
        for block in MinecraftSerializer::u64_to_blocks(v) {
            self.place_block(block)?;
        }
        self.place_block(MinecraftBlock::RawGoldBlock)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::IronBlock)?;
        let hex = format!("{:X}", v.to_bits());
        for digit in hex.chars() {
            let block = match digit {
                '0' => MinecraftBlock::WhiteWool,
                '1' => MinecraftBlock::LightGrayWool,
                '2' => MinecraftBlock::GrayWool,
                '3' => MinecraftBlock::BlackWool,
                '4' => MinecraftBlock::BrownWool,
                '5' => MinecraftBlock::RedWool,
                '6' => MinecraftBlock::OrangeWool,
                '7' => MinecraftBlock::YellowWool,
                '8' => MinecraftBlock::LimeWool,
                '9' => MinecraftBlock::GreenWool,
                'A' => MinecraftBlock::CyanWool,
                'B' => MinecraftBlock::LightBlueWool,
                'C' => MinecraftBlock::BlueWool,
                'D' => MinecraftBlock::PurpleWool,
                'E' => MinecraftBlock::MagentaWool,
                'F' => MinecraftBlock::PinkWool,
                _ => return Err(MinecraftError::InvalidData("serialize_f32".to_string())),
            };
            self.place_block(block)?;
        }

        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::GoldBlock)?;
        let hex = format!("{:X}", v.to_bits());
        for digit in hex.chars() {
            let block = match digit {
                '0' => MinecraftBlock::WhiteWool,
                '1' => MinecraftBlock::LightGrayWool,
                '2' => MinecraftBlock::GrayWool,
                '3' => MinecraftBlock::BlackWool,
                '4' => MinecraftBlock::BrownWool,
                '5' => MinecraftBlock::RedWool,
                '6' => MinecraftBlock::OrangeWool,
                '7' => MinecraftBlock::YellowWool,
                '8' => MinecraftBlock::LimeWool,
                '9' => MinecraftBlock::GreenWool,
                'A' => MinecraftBlock::CyanWool,
                'B' => MinecraftBlock::LightBlueWool,
                'C' => MinecraftBlock::BlueWool,
                'D' => MinecraftBlock::PurpleWool,
                'E' => MinecraftBlock::MagentaWool,
                'F' => MinecraftBlock::PinkWool,
                _ => return Err(MinecraftError::InvalidData("serialize_f64".to_string())),
            };
            self.place_block(block)?;
        }

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::LapisBlock)?;
        let hex = format!("{:X}", v as u8);
        for digit in hex.chars() {
            let block = match digit {
                '0' => MinecraftBlock::WhiteWool,
                '1' => MinecraftBlock::LightGrayWool,
                '2' => MinecraftBlock::GrayWool,
                '3' => MinecraftBlock::BlackWool,
                '4' => MinecraftBlock::BrownWool,
                '5' => MinecraftBlock::RedWool,
                '6' => MinecraftBlock::OrangeWool,
                '7' => MinecraftBlock::YellowWool,
                '8' => MinecraftBlock::LimeWool,
                '9' => MinecraftBlock::GreenWool,
                'A' => MinecraftBlock::CyanWool,
                'B' => MinecraftBlock::LightBlueWool,
                'C' => MinecraftBlock::BlueWool,
                'D' => MinecraftBlock::PurpleWool,
                'E' => MinecraftBlock::MagentaWool,
                'F' => MinecraftBlock::PinkWool,
                _ => return Err(MinecraftError::InvalidData("serialize_char".to_string())),
            };
            self.place_block(block)?;
        }
        self.place_block(MinecraftBlock::LapisBlock)?;

        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::CrimsonNylium)?;
        self.serialize_u64(v.len() as u64)?;

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::WarpedNylium)?;
        let blocks = MinecraftSerializer::bytes_to_blocks(v);
        self.place_blocks(&blocks)?;

        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.place_block(MinecraftBlock::Cobblestone)?;
        self.serialize_str(name)?;
        self.serialize_u32(variant_index)?;
        self.serialize_str(variant)?;
        serde::Serialize::serialize(value, self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.place_block(MinecraftBlock::QuartzBlock)?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.place_block(MinecraftBlock::NetherQuartzBlock)?;
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl serde::ser::SerializeSeq for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut m = MinecraftSerializer::new(self.0.clone());
        value.serialize(&mut m)?;
        self.place_block(MinecraftBlock::RawIronBlock)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::QuartzBlock)
    }
}

impl serde::ser::SerializeTuple for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl serde::ser::SerializeTupleStruct for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
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

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
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

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut m = MinecraftSerializer::new(self.0.clone());
        key.serialize(&mut m)?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.place_block(MinecraftBlock::RawGoldBlock)?;
        let mut m = MinecraftSerializer::new(self.0.clone());
        value.serialize(&mut m)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.place_block(MinecraftBlock::NetherQuartzBlock)?;
        Ok(())
    }
}

impl serde::ser::SerializeStruct for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl serde::ser::SerializeStructVariant for &mut MinecraftSerializer {
    type Ok = ();
    type Error = MinecraftError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
