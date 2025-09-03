use serde::{
    Deserializer,
    de::{EnumAccess, VariantAccess},
};

use crate::{MinecraftBlock, MinecraftError};

use super::MinecraftDeserializer;

pub struct MCEnumAccessor<'a, 'de: 'a> {
    de: &'a mut MinecraftDeserializer,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'a, 'de> MCEnumAccessor<'a, 'de> {
    pub fn new(de: &'a mut MinecraftDeserializer) -> Self {
        MCEnumAccessor {
            de,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, 'a> EnumAccess<'de> for MCEnumAccessor<'a, 'de> {
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

impl<'de, 'a> VariantAccess<'de> for MCEnumAccessor<'a, 'de> {
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
        self.de.parse_number(MinecraftBlock::RawCopperBlock, None)?;
        self.de.deserialize_seq(visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.parse_number(MinecraftBlock::RawCopperBlock, None)?;
        self.de
            .deserialize_struct("struct_variant", fields, visitor)
    }
}
