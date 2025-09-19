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

pub(crate) struct NumberMarker {
    pub(crate) marker: MinecraftBlock,
    pub(crate) signed: Option<MinecraftBlock>,
}

impl NumberMarker {
    impl_num_const! {
        U8: EndStone - I8: OchreFroglight;
        U16: RawIronBlock - I16: VerdantFroglight;
        U32: RawCopperBlock - I32: PearlescentFroglight;
        U64: RawGoldBlock - I64: SeaLantern;
        F32: Shroomlight;
        F64: Glowstone;
        CHAR: ChiseledDeepslate;
    }

    pub(super) fn is_marker(block: &MinecraftBlock) -> bool {
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

    pub(super) fn is_sign_marker(block: &MinecraftBlock) -> bool {
        matches!(
            block,
            MinecraftBlock::OchreFroglight
                | MinecraftBlock::VerdantFroglight
                | MinecraftBlock::PearlescentFroglight
                | MinecraftBlock::SeaLantern
        )
    }
}
