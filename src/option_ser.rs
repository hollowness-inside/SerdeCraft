use serde::Serialize;

use crate::{MinecraftBlock, MinecraftError, MinecraftSerializer, websocket::MCWebSocket};

macro_rules! serialize {
    ($index:literal = $method:ident$(<$T:tt>)?($($v:ident: $arg:ty),*) => $R:ident) => {
        fn $method$(<$T: ?Sized + Serialize>)?(self, $($v: $arg),*) -> Result<Self::$R, Self::Error> {
            self.serializer
                .socket
                .send_block(MinecraftBlock::bit_to_block($index)?)?;
            self.serializer.$method($($v),*)
        }
    };

    ($index:literal = $method:ident$(<$T:tt>)?($($v:ident: $arg:ty),*)) => {
        fn $method$(<$T: ?Sized + Serialize>)?(self, $($v: $arg),*) -> Result<Self::Ok, Self::Error> {
            self.serializer
                .socket
                .send_block(MinecraftBlock::bit_to_block($index)?)?;
            self.serializer.$method($($v),*)
        }
    };

    {$($index:literal = $method:ident$(<$T:tt>)?($($v:ident: $arg:ty),*) $(=> $R:ident)?),*} => {
        $(
            serialize!($index = $method$(<$T>)?($($v: $arg),*) $(=> $R)?);
        )*
    };
}

pub(super) struct OptionSerializer<'a, S> {
    serializer: &'a mut MinecraftSerializer<S>,
}

impl<'a, S> OptionSerializer<'a, S> {
    pub fn new(serializer: &'a mut MinecraftSerializer<S>) -> Self {
        Self { serializer }
    }
}

impl<'a, S: MCWebSocket> serde::ser::Serializer for OptionSerializer<'a, S> {
    type Ok = ();
    type Error = MinecraftError;

    type SerializeSeq = &'a mut MinecraftSerializer<S>;
    type SerializeTuple = &'a mut MinecraftSerializer<S>;
    type SerializeTupleStruct = &'a mut MinecraftSerializer<S>;
    type SerializeTupleVariant = &'a mut MinecraftSerializer<S>;
    type SerializeMap = &'a mut MinecraftSerializer<S>;
    type SerializeStruct = &'a mut MinecraftSerializer<S>;
    type SerializeStructVariant = &'a mut MinecraftSerializer<S>;

    serialize! {
        0 = serialize_bool(v: bool),
        1 = serialize_i8(v: i8),
        2 = serialize_i16(v: i16),
        3 = serialize_i32(v: i32),
        4 = serialize_i64(v: i64),
        5 = serialize_u8(v: u8),
        6 = serialize_u16(v: u16),
        7 = serialize_u32(v: u32),
        8 = serialize_u64(v: u64),
        9 = serialize_f32(v: f32),
        10 = serialize_f64(v: f64),
        11 = serialize_char(v: char),
        12 = serialize_str(v: &str),
        13 = serialize_bytes(v: &[u8]),
        14 = serialize_none(),
        15 = serialize_unit(),
        16 = serialize_unit_struct(v: &'static str),
        17 = serialize_unit_variant(name: &'static str, variant_index: u32, variant: &'static str),
        18 = serialize_newtype_struct<T>(name: &'static str,value: &T),
        19 = serialize_newtype_variant<T>(name: &'static str, variant_index: u32, variant: &'static str, value: &T),
        20 = serialize_seq(len: Option<usize>) => SerializeSeq,
        21 = serialize_tuple(len: usize) => SerializeTuple,
        22 = serialize_tuple_struct(name: &'static str, len: usize) => SerializeTupleStruct,
        23 = serialize_tuple_variant(name: &'static str, variant_index: u32, variant: &'static str, len: usize) => SerializeTupleVariant,
        24 = serialize_map(len: Option<usize>) => SerializeMap,
        25 = serialize_struct(name: &'static str, len: usize) => SerializeStruct,
        26 = serialize_struct_variant(name: &'static str, variant_index: u32, variant: &'static str, len: usize) => SerializeStructVariant
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.serialize_some(value)
    }
}
