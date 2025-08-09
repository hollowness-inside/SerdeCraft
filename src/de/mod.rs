mod main;
pub use main::MinecraftDeserializer;

mod r#enum;
mod r#map;
mod r#struct;
mod r#tuple;

mod r#variant;
pub(crate) use r#variant::MCVariantAccessor;
