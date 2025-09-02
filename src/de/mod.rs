mod main;
pub use main::MinecraftDeserializer;

mod r#enum;

mod map;
mod seq;

mod variant;
use variant::MCVariantAccessor;
