use serde::de::{EnumAccess, IntoDeserializer, value::StringDeserializer};

use super::{MCVariantAccessor, MinecraftDeserializer};
use crate::result::MinecraftError;

pub(crate) struct MCEnumAccessor<'de> {
    deserializer: &'de mut MinecraftDeserializer,
    variant_name: String,
}

impl<'de> MCEnumAccessor<'de> {
    pub fn new(deserializer: &'de mut MinecraftDeserializer, variant_name: String) -> Self {
        MCEnumAccessor {
            deserializer,
            variant_name,
        }
    }
}

impl<'a, 'de> EnumAccess<'de> for MCEnumAccessor<'a> {
    type Error = MinecraftError;
    type Variant = MCVariantAccessor<'a>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let variant_name = self.variant_name.clone();
        let name_deserializer: StringDeserializer<MinecraftError> =
            variant_name.into_deserializer();
        let val = seed.deserialize(name_deserializer)?;
        Ok((val, MCVariantAccessor::new(self.deserializer)))
    }
}
