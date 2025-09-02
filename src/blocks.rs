use std::hint::unreachable_unchecked;

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

block_enum!({
    Stone = "minecraft:stone",
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

    WhiteGlass = "minecraft:white_stained_glass",
    LightGrayGlass = "minecraft:light_gray_stained_glass",
    GrayGlass = "minecraft:gray_stained_glass",
    BlackGlass = "minecraft:black_stained_glass",
    BrownGlass = "minecraft:brown_stained_glass",
    RedGlass = "minecraft:red_stained_glass",
    OrangeGlass = "minecraft:orange_stained_glass",
    YellowGlass = "minecraft:yellow_stained_glass",
    LimeGlass = "minecraft:lime_stained_glass",
    GreenGlass = "minecraft:green_stained_glass",
    CyanGlass = "minecraft:cyan_stained_glass",
    LightBlueGlass = "minecraft:light_blue_stained_glass",
    BlueGlass = "minecraft:blue_stained_glass",
    PurpleGlass = "minecraft:purple_stained_glass",
    MagentaGlass = "minecraft:magenta_stained_glass",
    PinkGlass = "minecraft:pink_stained_glass",

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
            _ => unsafe { unreachable_unchecked() },
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

        Err(MinecraftError::Custom("Wrong block to bit".to_string()))
    }

    pub const fn is_glass(&self) -> bool {
        matches!(
            self,
            MinecraftBlock::WhiteGlass
                | MinecraftBlock::LightGrayGlass
                | MinecraftBlock::GrayGlass
                | MinecraftBlock::BlackGlass
                | MinecraftBlock::BrownGlass
                | MinecraftBlock::RedGlass
                | MinecraftBlock::OrangeGlass
                | MinecraftBlock::YellowGlass
                | MinecraftBlock::LimeGlass
                | MinecraftBlock::GreenGlass
                | MinecraftBlock::CyanGlass
                | MinecraftBlock::LightBlueGlass
                | MinecraftBlock::BlueGlass
                | MinecraftBlock::PurpleGlass
                | MinecraftBlock::MagentaGlass
                | MinecraftBlock::PinkGlass
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

    pub const fn to_digit(&self) -> MinecraftResult<u8> {
        let a = *self as u8;
        let b = if self.is_glass() {
            MinecraftBlock::WhiteGlass
        } else if self.is_log() {
            MinecraftBlock::CherryLog
        } else if self.is_terracotta() {
            MinecraftBlock::WhiteTerracotta
        } else if self.is_wool() {
            MinecraftBlock::WhiteWool
        } else if self.is_planks() {
            MinecraftBlock::CherryPlanks
        } else {
            return Err(MinecraftError::NotDigitBlock(*self));
        } as u8;

        Ok(a - b)
    }

    pub const fn dec_digit_to_log(decimal_digit: u8) -> MinecraftBlock {
        match decimal_digit {
            0 => MinecraftBlock::CherryLog,
            1 => MinecraftBlock::BambooLog,
            2 => MinecraftBlock::BirchLog,
            3 => MinecraftBlock::OakLog,
            4 => MinecraftBlock::JungleLog,
            5 => MinecraftBlock::AcaciaLog,
            6 => MinecraftBlock::SpruceLog,
            7 => MinecraftBlock::DarkOakLog,
            8 => MinecraftBlock::CrimsonStem,
            9 => MinecraftBlock::WarpedStem,
            // The caller `serialize_f64` provides the guarantee that the decimal digit is within range between 0 and 9
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    const fn hex_digit_to_terracotta(hex_digit: u8) -> MinecraftBlock {
        match hex_digit {
            0 => MinecraftBlock::WhiteTerracotta,
            1 => MinecraftBlock::LightGrayTerracotta,
            2 => MinecraftBlock::GrayTerracotta,
            3 => MinecraftBlock::BlackTerracotta,
            4 => MinecraftBlock::BrownTerracotta,
            5 => MinecraftBlock::RedTerracotta,
            6 => MinecraftBlock::OrangeTerracotta,
            7 => MinecraftBlock::YellowTerracotta,
            8 => MinecraftBlock::LimeTerracotta,
            9 => MinecraftBlock::GreenTerracotta,
            10 => MinecraftBlock::CyanTerracotta,
            11 => MinecraftBlock::LightBlueTerracotta,
            12 => MinecraftBlock::BlueTerracotta,
            13 => MinecraftBlock::PurpleTerracotta,
            14 => MinecraftBlock::MagentaTerracotta,
            15 => MinecraftBlock::PinkTerracotta,
            // Callers guarantee that the digit is between 0 and 15
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub const fn u8_to_terracotta(v: u8) -> [MinecraftBlock; 2] {
        let hi = (v >> 4) & 0x0F;
        let lo = v & 0x0F;

        let hi = MinecraftBlock::hex_digit_to_terracotta(hi);
        let lo = MinecraftBlock::hex_digit_to_terracotta(lo);

        [hi, lo]
    }

    const fn hex_digit_to_wool(hex_digit: u8) -> MinecraftBlock {
        match hex_digit {
            0 => MinecraftBlock::WhiteWool,
            1 => MinecraftBlock::LightGrayWool,
            2 => MinecraftBlock::GrayWool,
            3 => MinecraftBlock::BlackWool,
            4 => MinecraftBlock::BrownWool,
            5 => MinecraftBlock::RedWool,
            6 => MinecraftBlock::OrangeWool,
            7 => MinecraftBlock::YellowWool,
            8 => MinecraftBlock::LimeWool,
            9 => MinecraftBlock::GreenWool,
            10 => MinecraftBlock::CyanWool,
            11 => MinecraftBlock::LightBlueWool,
            12 => MinecraftBlock::BlueWool,
            13 => MinecraftBlock::PurpleWool,
            14 => MinecraftBlock::MagentaWool,
            15 => MinecraftBlock::PinkWool,
            // Callers guarantee that the digit is between 0 and 15
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub const fn u8_to_wool(v: u8) -> [MinecraftBlock; 2] {
        let hi = (v >> 4) & 0x0F;
        let lo = v & 0x0F;

        let hi = MinecraftBlock::hex_digit_to_wool(hi);
        let lo = MinecraftBlock::hex_digit_to_wool(lo);

        [hi, lo]
    }
}

#[test]
fn block_to_bit_test() {
    for i in 0..75 {
        let block = MinecraftBlock::bit_to_block(i).unwrap();
        let bit = block.block_to_bit().unwrap();
        assert_eq!(bit, i);
    }
}
