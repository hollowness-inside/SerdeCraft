mod main;
pub use main::MinecraftDeserializer;

mod r#enum;
use r#enum::MCEnumAccessor;

mod map;
use map::MCMapAccessor;

mod r#struct;
use r#struct::MCStructAccessor;

mod seq;
use seq::MCSeqAccessor;

mod variant;
use variant::MCVariantAccessor;
