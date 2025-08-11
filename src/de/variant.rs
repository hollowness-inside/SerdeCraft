use serde::de::VariantAccess;

use super::MinecraftDeserializer;
use crate::result::MinecraftError;

pub(super) struct MCVariantAccessor<'a> {
    deserializer: &'a mut MinecraftDeserializer,
}

impl<'a> MCVariantAccessor<'a> {
    pub fn new(deserializer: &'a mut MinecraftDeserializer) -> Self {
        MCVariantAccessor { deserializer }
    }
}

impl<'a, 'de> VariantAccess<'de> for MCVariantAccessor<'a> {
    type Error = MinecraftError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.deserializer)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self.deserializer, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_struct(self.deserializer, "", fields, visitor)
    }
}
