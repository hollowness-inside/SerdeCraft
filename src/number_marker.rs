use crate::MinecraftBlock;

pub(crate) struct NumberMarker {
    pub(crate) marker: MinecraftBlock,
    pub(crate) signed: Option<MinecraftBlock>,
}

impl NumberMarker {
    pub(crate) const I8: Self = Self {
        marker: MinecraftBlock::EndStone,
        signed: Some(MinecraftBlock::OchreFroglight),
    };

    pub(crate) const I16: Self = Self {
        marker: MinecraftBlock::RawIronBlock,
        signed: Some(MinecraftBlock::VerdantFroglight),
    };

    pub(crate) const I32: Self = Self {
        marker: MinecraftBlock::RawCopperBlock,
        signed: Some(MinecraftBlock::PearlescentFroglight),
    };

    pub(crate) const I64: Self = Self {
        marker: MinecraftBlock::RawGoldBlock,
        signed: Some(MinecraftBlock::SeaLantern),
    };

    pub(crate) const U8: Self = Self {
        marker: MinecraftBlock::EndStone,
        signed: None,
    };

    pub(crate) const U16: Self = Self {
        marker: MinecraftBlock::RawIronBlock,
        signed: None,
    };

    pub(crate) const U32: Self = Self {
        marker: MinecraftBlock::RawCopperBlock,
        signed: None,
    };

    pub(crate) const U64: Self = Self {
        marker: MinecraftBlock::RawGoldBlock,
        signed: None,
    };

    pub(crate) const F32: Self = Self {
        marker: MinecraftBlock::Shroomlight,
        signed: None,
    };

    pub(crate) const F64: Self = Self {
        marker: MinecraftBlock::Glowstone,
        signed: None,
    };

    pub(crate) const CHAR: Self = Self {
        marker: MinecraftBlock::ChiseledDeepslate,
        signed: None,
    };

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
}
