use crate::MinecraftBlock;

macro_rules! impl_num_const {
    ($($ufn:ident: $base:ident$( - $sfn:ident: $bl:ident)?;)*) => {
        $(
            impl_num_const!($ufn: $base $(- $sfn: $bl)*);
        )*
    };

    ($ufn:ident: $base:ident - $sfn:ident: $bl:ident) => {
        impl_num_const!($ufn: $base);

        pub(crate) const $sfn: Self = Self {
            marker: MinecraftBlock::$base,
            signed: Some(MinecraftBlock::$bl),
        };
    };

    ($name:ident: $base:ident) => {
        pub(crate) const $name: Self = Self {
            marker: MinecraftBlock::$base,
            signed: None,
        };
    }
}

/// Helper struct for associating number types with Minecraft blocks.
pub(crate) struct NumberMarker {
    /// The marker block representing the number type.
    pub(crate) marker: MinecraftBlock,

    /// An optional block indicating if the number is signed (not necessarily negative).
    pub(crate) signed: Option<MinecraftBlock>,
}

impl NumberMarker {
    impl_num_const!(U8:EndStone-I8:OchreFroglight);
    impl_num_const!(U16:RawIronBlock-I16:VerdantFroglight);
    impl_num_const!(U32:RawCopperBlock-I32:PearlescentFroglight);
    impl_num_const!(U64:RawGoldBlock-I64:SeaLantern);
    impl_num_const!(F32:Shroomlight);
    impl_num_const!(F64:Glowstone);
    impl_num_const!(CHAR:ChiseledDeepslate);

    /// Check if a block is a valid number marker.
    pub(crate) fn is_marker(block: &MinecraftBlock) -> bool {
        matches!(
            block,
            MinecraftBlock::EndStone
                | MinecraftBlock::RawIronBlock
                | MinecraftBlock::RawCopperBlock
                | MinecraftBlock::RawGoldBlock
                | MinecraftBlock::Shroomlight
                | MinecraftBlock::Glowstone
                | MinecraftBlock::ChiseledDeepslate
        )
    }

    /// Check if a block is a valid sign marker.
    pub(crate) fn is_sign_marker(block: &MinecraftBlock) -> bool {
        matches!(
            block,
            MinecraftBlock::OchreFroglight
                | MinecraftBlock::VerdantFroglight
                | MinecraftBlock::PearlescentFroglight
                | MinecraftBlock::SeaLantern
        )
    }
}
