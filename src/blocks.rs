use crate::{MinecraftResult, result::MinecraftError};

macro_rules! block_enum {
    ({
        $($name:ident = $value:literal),*
    }) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, num_enum::TryFromPrimitive)]
        pub enum MinecraftBlock {
            $($name,)*
        }

        impl std::fmt::Display for MinecraftBlock {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(MinecraftBlock::$name => $value,)*
                })
            }
        }

        impl TryFrom<&str> for MinecraftBlock {
            type Error = MinecraftError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Ok(match value {
                    $($value => MinecraftBlock::$name,)*
                    _ => return Err(MinecraftError::UnknownBlockType(value.to_string()))
                })
            }
        }
    };
}

pub const BASE: usize = 91;

block_enum!({
    Stone = "minecraft:stone",
    EndStone = "minecraft:end_stone",
    Cobblestone = "minecraft:cobblestone",
    QuartzBlock = "minecraft:quartz_block",
    Obsidian = "minecraft:obsidian",
    Shroomlight = "minecraft:shroomlight",
    ChiseledDeepslate = "minecraft:chiseled_deepslate",
    Blackstone = "minecraft:blackstone",
    GildedBlackstone = "minecraft:gilded_blackstone",
    Prismarine = "minecraft:prismarine",
    DarkPrismarine = "minecraft:dark_prismarine",
    AmethystBlock = "minecraft:amethyst_block",
    PurpurBlock = "minecraft:purpur_block",
    PurpurPillar = "minecraft:purpur_pillar",

    OchreFroglight = "minecraft:ochre_froglight",
    VerdantFroglight = "minecraft:verdant_froglight",
    PearlescentFroglight = "minecraft:pearlescent_froglight",
    SeaLantern = "minecraft:sea_lantern",

    WhiteWool = "minecraft:white_wool",
    LightGrayWool = "minecraft:light_gray_wool",
    GrayWool = "minecraft:gray_wool",
    BlackWool = "minecraft:black_wool",
    BrownWool = "minecraft:brown_wool",
    RedWool = "minecraft:red_wool",
    OrangeWool = "minecraft:orange_wool",
    YellowWool = "minecraft:yellow_wool",
    LimeWool = "minecraft:lime_wool",
    GreenWool = "minecraft:green_wool",
    CyanWool = "minecraft:cyan_wool",
    LightBlueWool = "minecraft:light_blue_wool",
    BlueWool = "minecraft:blue_wool",
    PurpleWool = "minecraft:purple_wool",
    MagentaWool = "minecraft:magenta_wool",
    PinkWool = "minecraft:pink_wool",

    WhiteStainedGlass = "minecraft:white_stained_glass",
    LightGrayStainedGlass = "minecraft:light_gray_stained_glass",
    GrayStainedGlass = "minecraft:gray_stained_glass",
    BlackStainedGlass = "minecraft:black_stained_glass",
    BrownStainedGlass = "minecraft:brown_stained_glass",
    RedStainedGlass = "minecraft:red_stained_glass",
    OrangeStainedGlass = "minecraft:orange_stained_glass",
    YellowStainedGlass = "minecraft:yellow_stained_glass",
    LimeStainedGlass = "minecraft:lime_stained_glass",
    GreenStainedGlass = "minecraft:green_stained_glass",
    CyanStainedGlass = "minecraft:cyan_stained_glass",
    LightBlueStainedGlass = "minecraft:light_blue_stained_glass",
    BlueStainedGlass = "minecraft:blue_stained_glass",
    PurpleStainedGlass = "minecraft:purple_stained_glass",
    MagentaStainedGlass = "minecraft:magenta_stained_glass",
    PinkStainedGlass = "minecraft:pink_stained_glass",

    WhiteConcrete = "minecraft:white_concrete",
    LightGrayConcrete = "minecraft:light_gray_concrete",
    GrayConcrete = "minecraft:gray_concrete",
    BlackConcrete = "minecraft:black_concrete",
    BrownConcrete = "minecraft:brown_concrete",
    RedConcrete = "minecraft:red_concrete",
    OrangeConcrete = "minecraft:orange_concrete",
    YellowConcrete = "minecraft:yellow_concrete",
    LimeConcrete = "minecraft:lime_concrete",
    GreenConcrete = "minecraft:green_concrete",
    CyanConcrete = "minecraft:cyan_concrete",
    LightBlueConcrete = "minecraft:light_blue_concrete",
    BlueConcrete = "minecraft:blue_concrete",
    PurpleConcrete = "minecraft:purple_concrete",
    MagentaConcrete = "minecraft:magenta_concrete",
    PinkConcrete = "minecraft:pink_concrete",

    WhiteGlazedTerracotta = "minecraft:white_glazed_terracotta",
    LightGrayGlazedTerracotta = "minecraft:light_gray_glazed_terracotta",
    GrayGlazedTerracotta = "minecraft:gray_glazed_terracotta",
    BlackGlazedTerracotta = "minecraft:black_glazed_terracotta",
    BrownGlazedTerracotta = "minecraft:brown_glazed_terracotta",
    RedGlazedTerracotta = "minecraft:red_glazed_terracotta",
    OrangeGlazedTerracotta = "minecraft:orange_glazed_terracotta",
    YellowGlazedTerracotta = "minecraft:yellow_glazed_terracotta",
    LimeGlazedTerracotta = "minecraft:lime_glazed_terracotta",
    GreenGlazedTerracotta = "minecraft:green_glazed_terracotta",
    CyanGlazedTerracotta = "minecraft:cyan_glazed_terracotta",
    LightBlueGlazedTerracotta = "minecraft:light_blue_glazed_terracotta",
    BlueGlazedTerracotta = "minecraft:blue_glazed_terracotta",
    PurpleGlazedTerracotta = "minecraft:purple_glazed_terracotta",
    MagentaGlazedTerracotta = "minecraft:magenta_glazed_terracotta",
    PinkGlazedTerracotta = "minecraft:pink_glazed_terracotta",

    WhiteTerracotta = "minecraft:white_terracotta",
    LightGrayTerracotta = "minecraft:light_gray_terracotta",
    GrayTerracotta = "minecraft:gray_terracotta",
    BlackTerracotta = "minecraft:black_terracotta",
    BrownTerracotta = "minecraft:brown_terracotta",
    RedTerracotta = "minecraft:red_terracotta",
    OrangeTerracotta = "minecraft:orange_terracotta",
    YellowTerracotta = "minecraft:yellow_terracotta",
    LimeTerracotta = "minecraft:lime_terracotta",
    GreenTerracotta = "minecraft:green_terracotta",
    CyanTerracotta = "minecraft:cyan_terracotta",
    LightBlueTerracotta = "minecraft:light_blue_terracotta",
    BlueTerracotta = "minecraft:blue_terracotta",
    PurpleTerracotta = "minecraft:purple_terracotta",
    MagentaTerracotta = "minecraft:magenta_terracotta",
    PinkTerracotta = "minecraft:pink_terracotta",

    OakPlanks = "minecraft:oak_planks",
    SprucePlanks = "minecraft:spruce_planks",
    BirchPlanks = "minecraft:birch_planks",
    JunglePlanks = "minecraft:jungle_planks",
    AcaciaPlanks = "minecraft:acacia_planks",
    DarkOakPlanks = "minecraft:dark_oak_planks",
    MangrovePlanks = "minecraft:mangrove_planks",
    CherryPlanks = "minecraft:cherry_planks",
    BambooPlanks = "minecraft:bamboo_planks",
    CrimsonPlanks = "minecraft:crimson_planks",
    WarpedPlanks = "minecraft:warped_planks",

    CoalBlock = "minecraft:coal_block",
    RedstoneBlock = "minecraft:redstone_block",

    CherryLog = "minecraft:cherry_log",
    BambooLog = "minecraft:bamboo_block",
    BirchLog = "minecraft:birch_log",
    OakLog = "minecraft:oak_log",
    JungleLog = "minecraft:jungle_log",
    AcaciaLog = "minecraft:acacia_log",
    SpruceLog = "minecraft:spruce_log",
    DarkOakLog = "minecraft:dark_oak_log",
    CrimsonStem = "minecraft:crimson_stem",
    WarpedStem = "minecraft:warped_stem",

    DarkOakFence = "minecraft:dark_oak_fence",
    CherryFence = "minecraft:cherry_fence",
    CrimsonFence = "minecraft:crimson_fence",
    WarpedFence = "minecraft:warped_fence",

    GoldBlock = "minecraft:gold_block",
    IronBlock = "minecraft:iron_block",
    LapisBlock = "minecraft:lapis_block",
    EmeraldBlock = "minecraft:emerald_block",
    DiamondBlock = "minecraft:diamond_block",

    RawCopperBlock = "minecraft:raw_copper_block",
    RawIronBlock = "minecraft:raw_iron_block",
    RawGoldBlock = "minecraft:raw_gold_block",

    Bricks = "minecraft:bricks",
    Glass = "minecraft:glass",
    CryingObsidian = "minecraft:crying_obsidian",
    BeeNest = "minecraft:bee_nest",
    Bedrock = "minecraft:bedrock",

    RedstoneLamp = "minecraft:redstone_lamp",
    Glowstone = "minecraft:glowstone",

    NetherQuartzBlock = "minecraft:nether_quartz_block",
    CrimsonNylium = "minecraft:crimson_nylium",
    WarpedNylium = "minecraft:warped_nylium"
});

impl MinecraftBlock {
    pub fn bit_to_block(mut bit: u8) -> MinecraftResult<Self> {
        let start = match bit {
            0..16 => MinecraftBlock::WhiteWool as u8,
            16..32 => {
                bit -= 16;
                MinecraftBlock::WhiteConcrete as u8
            }
            32..48 => {
                bit -= 32;
                MinecraftBlock::WhiteTerracotta as u8
            }
            48..64 => {
                bit -= 48;
                MinecraftBlock::WhiteGlazedTerracotta as u8
            }
            64..75 => {
                bit -= 64;
                MinecraftBlock::OakPlanks as u8
            }
            75..91 => {
                bit -= 75;
                MinecraftBlock::WhiteStainedGlass as u8
            }
            _ => unimplemented!(),
        };

        let offset = start + bit;
        offset
            .try_into()
            .map_err(|_| MinecraftError::Custom("Could not convert bit to block".to_string()))
    }

    pub fn block_to_bit(&self) -> MinecraftResult<u8> {
        if self.is_wool() {
            let start = MinecraftBlock::WhiteWool as u8;
            return Ok(*self as u8 - start);
        }

        if self.is_concrete() {
            let start = MinecraftBlock::WhiteConcrete as u8;
            return Ok(*self as u8 - start + 16);
        }

        if self.is_terracotta() {
            let start = MinecraftBlock::WhiteTerracotta as u8;
            return Ok(*self as u8 - start + 32);
        }

        if self.is_glazed_terracotta() {
            let start = MinecraftBlock::WhiteGlazedTerracotta as u8;
            return Ok(*self as u8 - start + 48);
        }

        if self.is_planks() {
            let start = MinecraftBlock::OakPlanks as u8;
            return Ok(*self as u8 - start + 64);
        }

        if self.is_glass() {
            let start = MinecraftBlock::WhiteStainedGlass as u8;
            return Ok(*self as u8 - start + 75);
        }

        Err(MinecraftError::Custom(format!(
            "Wrong block to bit: {}",
            self
        )))
    }

    pub const fn is_glass(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::WhiteStainedGlass
                | MinecraftBlock::LightGrayStainedGlass
                | MinecraftBlock::GrayStainedGlass
                | MinecraftBlock::BlackStainedGlass
                | MinecraftBlock::BrownStainedGlass
                | MinecraftBlock::RedStainedGlass
                | MinecraftBlock::OrangeStainedGlass
                | MinecraftBlock::YellowStainedGlass
                | MinecraftBlock::LimeStainedGlass
                | MinecraftBlock::GreenStainedGlass
                | MinecraftBlock::CyanStainedGlass
                | MinecraftBlock::LightBlueStainedGlass
                | MinecraftBlock::BlueStainedGlass
                | MinecraftBlock::PurpleStainedGlass
                | MinecraftBlock::MagentaStainedGlass
                | MinecraftBlock::PinkStainedGlass
        )
    }

    pub const fn is_wool(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::WhiteWool
                | MinecraftBlock::LightGrayWool
                | MinecraftBlock::GrayWool
                | MinecraftBlock::BlackWool
                | MinecraftBlock::BrownWool
                | MinecraftBlock::RedWool
                | MinecraftBlock::OrangeWool
                | MinecraftBlock::YellowWool
                | MinecraftBlock::LimeWool
                | MinecraftBlock::GreenWool
                | MinecraftBlock::CyanWool
                | MinecraftBlock::LightBlueWool
                | MinecraftBlock::BlueWool
                | MinecraftBlock::PurpleWool
                | MinecraftBlock::MagentaWool
                | MinecraftBlock::PinkWool
        )
    }

    pub const fn is_log(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::CherryLog
                | MinecraftBlock::BambooLog
                | MinecraftBlock::BirchLog
                | MinecraftBlock::OakLog
                | MinecraftBlock::JungleLog
                | MinecraftBlock::AcaciaLog
                | MinecraftBlock::SpruceLog
                | MinecraftBlock::DarkOakLog
                | MinecraftBlock::CrimsonStem
                | MinecraftBlock::WarpedStem
        )
    }

    pub const fn is_terracotta(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::WhiteTerracotta
                | MinecraftBlock::LightGrayTerracotta
                | MinecraftBlock::GrayTerracotta
                | MinecraftBlock::BlackTerracotta
                | MinecraftBlock::BrownTerracotta
                | MinecraftBlock::RedTerracotta
                | MinecraftBlock::OrangeTerracotta
                | MinecraftBlock::YellowTerracotta
                | MinecraftBlock::LimeTerracotta
                | MinecraftBlock::GreenTerracotta
                | MinecraftBlock::CyanTerracotta
                | MinecraftBlock::LightBlueTerracotta
                | MinecraftBlock::BlueTerracotta
                | MinecraftBlock::PurpleTerracotta
                | MinecraftBlock::MagentaTerracotta
                | MinecraftBlock::PinkTerracotta
        )
    }

    pub const fn is_planks(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::CherryPlanks
                | MinecraftBlock::BambooPlanks
                | MinecraftBlock::BirchPlanks
                | MinecraftBlock::OakPlanks
                | MinecraftBlock::JunglePlanks
                | MinecraftBlock::MangrovePlanks
                | MinecraftBlock::AcaciaPlanks
                | MinecraftBlock::SprucePlanks
                | MinecraftBlock::DarkOakPlanks
                | MinecraftBlock::CrimsonPlanks
                | MinecraftBlock::WarpedPlanks
        )
    }

    pub const fn is_concrete(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::WhiteConcrete
                | MinecraftBlock::LightGrayConcrete
                | MinecraftBlock::GrayConcrete
                | MinecraftBlock::BlackConcrete
                | MinecraftBlock::BrownConcrete
                | MinecraftBlock::RedConcrete
                | MinecraftBlock::OrangeConcrete
                | MinecraftBlock::YellowConcrete
                | MinecraftBlock::LimeConcrete
                | MinecraftBlock::GreenConcrete
                | MinecraftBlock::CyanConcrete
                | MinecraftBlock::LightBlueConcrete
                | MinecraftBlock::BlueConcrete
                | MinecraftBlock::PurpleConcrete
                | MinecraftBlock::MagentaConcrete
                | MinecraftBlock::PinkConcrete
        )
    }

    pub const fn is_glazed_terracotta(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::WhiteGlazedTerracotta
                | MinecraftBlock::LightGrayGlazedTerracotta
                | MinecraftBlock::GrayGlazedTerracotta
                | MinecraftBlock::BlackGlazedTerracotta
                | MinecraftBlock::BrownGlazedTerracotta
                | MinecraftBlock::RedGlazedTerracotta
                | MinecraftBlock::OrangeGlazedTerracotta
                | MinecraftBlock::YellowGlazedTerracotta
                | MinecraftBlock::LimeGlazedTerracotta
                | MinecraftBlock::GreenGlazedTerracotta
                | MinecraftBlock::CyanGlazedTerracotta
                | MinecraftBlock::LightBlueGlazedTerracotta
                | MinecraftBlock::BlueGlazedTerracotta
                | MinecraftBlock::PurpleGlazedTerracotta
                | MinecraftBlock::MagentaGlazedTerracotta
                | MinecraftBlock::PinkGlazedTerracotta
        )
    }

    pub const fn is_light(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::OchreFroglight
                | MinecraftBlock::VerdantFroglight
                | MinecraftBlock::PearlescentFroglight
                | MinecraftBlock::Shroomlight
                | MinecraftBlock::Glowstone
        )
    }
}

#[test]
fn block_to_bit_test() {
    for i in 0..BASE as u8 {
        let block = MinecraftBlock::bit_to_block(i).unwrap();
        let bit = block.block_to_bit().unwrap();
        assert_eq!(bit, i);
    }
}
