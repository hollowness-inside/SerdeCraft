mod main;
pub use main::MinecraftDeserializer;

mod r#enum;
pub(self) use r#enum::MCEnumAccessor;

mod map;
pub(self) use map::MCMapAccessor;

mod r#struct;
pub(self) use r#struct::MCStructAccessor;

mod seq;
pub(self) use seq::MCSeqAccessor;

mod variant;
pub(self) use variant::MCVariantAccessor;
