use serde::{
    Deserializer,
    de::{EnumAccess, VariantAccess, Visitor},
};

use crate::{MinecraftBlock, MinecraftError};

use super::MinecraftDeserializer;

pub struct MCEnumAccessor<'a> {
    de: &'a mut MinecraftDeserializer,
}

impl<'a> MCEnumAccessor<'a> {
    pub fn new(de: &'a mut MinecraftDeserializer) -> Self {
        MCEnumAccessor { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for MCEnumAccessor<'a> {
    type Error = MinecraftError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let variant_index = seed.deserialize(&mut *self.de)?;
        Ok((variant_index, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for MCEnumAccessor<'a> {
    type Error = MinecraftError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        struct LengthVisitor;
        impl<'de> Visitor<'de> for LengthVisitor {
            type Value = u32;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("tuple variant length")
            }
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> {
                Ok(v)
            }
        }

        let _serialized_len = self.de.deserialize_u32(LengthVisitor)?;

        let result = visitor.visit_seq(&mut *self.de)?;

        let end_block = self.de.consume()?;
        if end_block != MinecraftBlock::DarkPrismarine {
            return Err(MinecraftError::UnexpectedBlock {
                expected: MinecraftBlock::DarkPrismarine,
                found: end_block,
            });
        }

        Ok(result)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        struct LengthVisitor;
        impl<'de> Visitor<'de> for LengthVisitor {
            type Value = u32;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct variant length")
            }
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> {
                Ok(v)
            }
        }

        let _serialized_len = self.de.deserialize_u32(LengthVisitor)?;

        let result = visitor.visit_map(&mut crate::de::map::MCMapAccess::new(
            self.de,
            MinecraftBlock::EmeraldBlock,
        ))?;

        Ok(result)
    }
}
