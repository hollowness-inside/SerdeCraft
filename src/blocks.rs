use crate::result::MinecraftError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MinecraftBlock {
    Stone,
    Cobblestone,
    QuartzBlock,
    Obsidian,

    WhiteWool,
    LightGrayWool,
    GrayWool,
    BlackWool,
    BrownWool,
    RedWool,
    OrangeWool,
    YellowWool,
    LimeWool,
    GreenWool,
    CyanWool,
    LightBlueWool,
    BlueWool,
    PurpleWool,
    MagentaWool,
    PinkWool,

    WhiteGlass,
    LightGrayGlass,
    GrayGlass,
    BlackGlass,
    BrownGlass,
    RedGlass,
    OrangeGlass,
    YellowGlass,
    LimeGlass,
    GreenGlass,
    CyanGlass,
    LightBlueGlass,
    BlueGlass,
    PurpleGlass,
    MagentaGlass,
    PinkGlass,

    WhiteTerracotta,
    LightGrayTerracotta,
    GrayTerracotta,
    BlackTerracotta,
    BrownTerracotta,
    RedTerracotta,
    OrangeTerracotta,
    YellowTerracotta,
    LimeTerracotta,
    GreenTerracotta,
    CyanTerracotta,
    LightBlueTerracotta,
    BlueTerracotta,
    PurpleTerracotta,
    MagentaTerracotta,
    PinkTerracotta,

    CherryPlanks,
    BambooPlanks,
    BirchPlanks,
    OakPlanks,
    JunglePlanks,
    AcaciaPlanks,
    SprucePlanks,
    DarkOakPlanks,
    CrimsonPlanks,
    WarpedPlanks,

    CoalBlock,
    RedstoneBlock,

    CherryLog,
    BambooLog,
    BirchLog,
    OakLog,
    JungleLog,
    AcaciaLog,
    SpruceLog,
    DarkOakLog,
    CrimsonLog,
    WarpedLog,

    DarkOakFence,
    CherryFence,
    CrimsonFence,
    WarpedFence,

    GoldBlock,
    IronBlock,
    LapisBlock,
    EmeraldBlock,
    DiamondBlock,

    RawCopperBlock,
    RawIronBlock,
    RawGoldBlock,

    RedstoneLamp,
    Glowstone,
    CryingObsidian,
    BeeNest,
    Bedrock,

    Bricks,
    Glass,

    NetherQuartzBlock,
    CrimsonNylium,
    WarpedNylium,
}

impl std::fmt::Display for MinecraftBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            MinecraftBlock::Stone => "minecraft:stone",
            MinecraftBlock::Cobblestone => "minecraft:cobblestone",
            MinecraftBlock::QuartzBlock => "minecraft:quartz_block",
            MinecraftBlock::Obsidian => "minecraft:obsidian",

            MinecraftBlock::WhiteWool => "minecraft:white_wool",
            MinecraftBlock::LightGrayWool => "minecraft:light_gray_wool",
            MinecraftBlock::GrayWool => "minecraft:gray_wool",
            MinecraftBlock::BlackWool => "minecraft:black_wool",
            MinecraftBlock::BrownWool => "minecraft:brown_wool",
            MinecraftBlock::RedWool => "minecraft:red_wool",
            MinecraftBlock::OrangeWool => "minecraft:orange_wool",
            MinecraftBlock::YellowWool => "minecraft:yellow_wool",
            MinecraftBlock::LimeWool => "minecraft:lime_wool",
            MinecraftBlock::GreenWool => "minecraft:green_wool",
            MinecraftBlock::CyanWool => "minecraft:cyan_wool",
            MinecraftBlock::LightBlueWool => "minecraft:light_blue_wool",
            MinecraftBlock::BlueWool => "minecraft:blue_wool",
            MinecraftBlock::PurpleWool => "minecraft:purple_wool",
            MinecraftBlock::MagentaWool => "minecraft:magenta_wool",
            MinecraftBlock::PinkWool => "minecraft:pink_wool",

            MinecraftBlock::WhiteGlass => "minecraft:white_stained_glass",
            MinecraftBlock::LightGrayGlass => "minecraft:light_gray_stained_glass",
            MinecraftBlock::GrayGlass => "minecraft:gray_stained_glass",
            MinecraftBlock::BlackGlass => "minecraft:black_stained_glass",
            MinecraftBlock::BrownGlass => "minecraft:brown_stained_glass",
            MinecraftBlock::RedGlass => "minecraft:red_stained_glass",
            MinecraftBlock::OrangeGlass => "minecraft:orange_stained_glass",
            MinecraftBlock::YellowGlass => "minecraft:yellow_stained_glass",
            MinecraftBlock::LimeGlass => "minecraft:lime_stained_glass",
            MinecraftBlock::GreenGlass => "minecraft:green_stained_glass",
            MinecraftBlock::CyanGlass => "minecraft:cyan_stained_glass",
            MinecraftBlock::LightBlueGlass => "minecraft:light_blue_stained_glass",
            MinecraftBlock::BlueGlass => "minecraft:blue_stained_glass",
            MinecraftBlock::PurpleGlass => "minecraft:purple_stained_glass",
            MinecraftBlock::MagentaGlass => "minecraft:magenta_stained_glass",
            MinecraftBlock::PinkGlass => "minecraft:pink_stained_glass",

            MinecraftBlock::WhiteTerracotta => "minecraft:white_glazed_terracotta",
            MinecraftBlock::LightGrayTerracotta => "minecraft:light_gray_glazed_terracotta",
            MinecraftBlock::GrayTerracotta => "minecraft:gray_glazed_terracotta",
            MinecraftBlock::BlackTerracotta => "minecraft:black_glazed_terracotta",
            MinecraftBlock::BrownTerracotta => "minecraft:brown_glazed_terracotta",
            MinecraftBlock::RedTerracotta => "minecraft:red_glazed_terracotta",
            MinecraftBlock::OrangeTerracotta => "minecraft:orange_glazed_terracotta",
            MinecraftBlock::YellowTerracotta => "minecraft:yellow_glazed_terracotta",
            MinecraftBlock::LimeTerracotta => "minecraft:lime_glazed_terracotta",
            MinecraftBlock::GreenTerracotta => "minecraft:green_glazed_terracotta",
            MinecraftBlock::CyanTerracotta => "minecraft:cyan_glazed_terracotta",
            MinecraftBlock::LightBlueTerracotta => "minecraft:light_blue_glazed_terracotta",
            MinecraftBlock::BlueTerracotta => "minecraft:blue_glazed_terracotta",
            MinecraftBlock::PurpleTerracotta => "minecraft:purple_glazed_terracotta",
            MinecraftBlock::MagentaTerracotta => "minecraft:magenta_glazed_terracotta",
            MinecraftBlock::PinkTerracotta => "minecraft:pink_glazed_terracotta",

            MinecraftBlock::CherryPlanks => "minecraft:cherry_planks",
            MinecraftBlock::BambooPlanks => "minecraft:bamboo_planks",
            MinecraftBlock::BirchPlanks => "minecraft:birch_planks",
            MinecraftBlock::OakPlanks => "minecraft:oak_planks",
            MinecraftBlock::JunglePlanks => "minecraft:jungle_planks",
            MinecraftBlock::AcaciaPlanks => "minecraft:acacia_planks",
            MinecraftBlock::SprucePlanks => "minecraft:spruce_planks",
            MinecraftBlock::DarkOakPlanks => "minecraft:dark_oak_planks",
            MinecraftBlock::CrimsonPlanks => "minecraft:crimson_planks",
            MinecraftBlock::WarpedPlanks => "minecraft:warped_planks",

            MinecraftBlock::CoalBlock => "minecraft:coal_block",
            MinecraftBlock::RedstoneBlock => "minecraft:redstone_block",

            MinecraftBlock::CherryLog => "minecraft:cherry_log",
            MinecraftBlock::BambooLog => "minecraft:bamboo_block",
            MinecraftBlock::BirchLog => "minecraft:birch_log",
            MinecraftBlock::OakLog => "minecraft:oak_log",
            MinecraftBlock::JungleLog => "minecraft:jungle_log",
            MinecraftBlock::AcaciaLog => "minecraft:acacia_log",
            MinecraftBlock::SpruceLog => "minecraft:spruce_log",
            MinecraftBlock::DarkOakLog => "minecraft:dark_oak_log",
            MinecraftBlock::CrimsonLog => "minecraft:crimson_stem",
            MinecraftBlock::WarpedLog => "minecraft:warped_stem",

            MinecraftBlock::DarkOakFence => "minecraft:dark_oak_fence",
            MinecraftBlock::CherryFence => "minecraft:cherry_fence",
            MinecraftBlock::CrimsonFence => "minecraft:crimson_fence",
            MinecraftBlock::WarpedFence => "minecraft:warped_fence",

            MinecraftBlock::GoldBlock => "minecraft:gold_block",
            MinecraftBlock::IronBlock => "minecraft:iron_block",
            MinecraftBlock::LapisBlock => "minecraft:lapis_block",
            MinecraftBlock::EmeraldBlock => "minecraft:emerald_block",
            MinecraftBlock::DiamondBlock => "minecraft:diamond_block",

            MinecraftBlock::RawCopperBlock => "minecraft:raw_copper_block",
            MinecraftBlock::RawIronBlock => "minecraft:raw_iron_block",
            MinecraftBlock::RawGoldBlock => "minecraft:raw_gold_block",

            MinecraftBlock::Bricks => "minecraft:bricks",
            MinecraftBlock::Glass => "minecraft:glass",
            MinecraftBlock::CryingObsidian => "minecraft:crying_obsidian",
            MinecraftBlock::BeeNest => "minecraft:bee_nest",
            MinecraftBlock::Bedrock => "minecraft:bedrock",

            MinecraftBlock::RedstoneLamp => "minecraft:redstone_lamp",
            MinecraftBlock::Glowstone => "minecraft:glowstone",

            MinecraftBlock::NetherQuartzBlock => "minecraft:nether_quartz_block",
            MinecraftBlock::CrimsonNylium => "minecraft:crimson_nylium",
            MinecraftBlock::WarpedNylium => "minecraft:warped_nylium",
        };

        write!(f, "{w}")
    }
}

impl TryFrom<&str> for MinecraftBlock {
    type Error = MinecraftError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let b = match value {
            "minecraft:stone" => MinecraftBlock::Stone,
            "minecraft:cobblestone" => MinecraftBlock::Cobblestone,
            "minecraft:quartz_block" => MinecraftBlock::QuartzBlock,
            "minecraft:obsidian" => MinecraftBlock::Obsidian,

            "minecraft:white_wool" => MinecraftBlock::WhiteWool,
            "minecraft:light_gray_wool" => MinecraftBlock::LightGrayWool,
            "minecraft:gray_wool" => MinecraftBlock::GrayWool,
            "minecraft:black_wool" => MinecraftBlock::BlackWool,
            "minecraft:brown_wool" => MinecraftBlock::BrownWool,
            "minecraft:red_wool" => MinecraftBlock::RedWool,
            "minecraft:orange_wool" => MinecraftBlock::OrangeWool,
            "minecraft:yellow_wool" => MinecraftBlock::YellowWool,
            "minecraft:lime_wool" => MinecraftBlock::LimeWool,
            "minecraft:green_wool" => MinecraftBlock::GreenWool,
            "minecraft:cyan_wool" => MinecraftBlock::CyanWool,
            "minecraft:light_blue_wool" => MinecraftBlock::LightBlueWool,
            "minecraft:blue_wool" => MinecraftBlock::BlueWool,
            "minecraft:purple_wool" => MinecraftBlock::PurpleWool,
            "minecraft:magenta_wool" => MinecraftBlock::MagentaWool,
            "minecraft:pink_wool" => MinecraftBlock::PinkWool,

            "minecraft:white_stained_glass" => MinecraftBlock::WhiteGlass,
            "minecraft:light_gray_stained_glass" => MinecraftBlock::LightGrayGlass,
            "minecraft:gray_stained_glass" => MinecraftBlock::GrayGlass,
            "minecraft:black_stained_glass" => MinecraftBlock::BlackGlass,
            "minecraft:brown_stained_glass" => MinecraftBlock::BrownGlass,
            "minecraft:red_stained_glass" => MinecraftBlock::RedGlass,
            "minecraft:orange_stained_glass" => MinecraftBlock::OrangeGlass,
            "minecraft:yellow_stained_glass" => MinecraftBlock::YellowGlass,
            "minecraft:lime_stained_glass" => MinecraftBlock::LimeGlass,
            "minecraft:green_stained_glass" => MinecraftBlock::GreenGlass,
            "minecraft:cyan_stained_glass" => MinecraftBlock::CyanGlass,
            "minecraft:light_blue_stained_glass" => MinecraftBlock::LightBlueGlass,
            "minecraft:blue_stained_glass" => MinecraftBlock::BlueGlass,
            "minecraft:purple_stained_glass" => MinecraftBlock::PurpleGlass,
            "minecraft:magenta_stained_glass" => MinecraftBlock::MagentaGlass,
            "minecraft:pink_stained_glass" => MinecraftBlock::PinkGlass,

            "minecraft:white_glazed_terracotta" => MinecraftBlock::WhiteTerracotta,
            "minecraft:light_gray_glazed_terracotta" => MinecraftBlock::LightGrayTerracotta,
            "minecraft:gray_glazed_terracotta" => MinecraftBlock::GrayTerracotta,
            "minecraft:black_glazed_terracotta" => MinecraftBlock::BlackTerracotta,
            "minecraft:brown_glazed_terracotta" => MinecraftBlock::BrownTerracotta,
            "minecraft:red_glazed_terracotta" => MinecraftBlock::RedTerracotta,
            "minecraft:orange_glazed_terracotta" => MinecraftBlock::OrangeTerracotta,
            "minecraft:yellow_glazed_terracotta" => MinecraftBlock::YellowTerracotta,
            "minecraft:lime_glazed_terracotta" => MinecraftBlock::LimeTerracotta,
            "minecraft:green_glazed_terracotta" => MinecraftBlock::GreenTerracotta,
            "minecraft:cyan_glazed_terracotta" => MinecraftBlock::CyanTerracotta,
            "minecraft:light_blue_glazed_terracotta" => MinecraftBlock::LightBlueTerracotta,
            "minecraft:blue_glazed_terracotta" => MinecraftBlock::BlueTerracotta,
            "minecraft:purple_glazed_terracotta" => MinecraftBlock::PurpleTerracotta,
            "minecraft:magenta_glazed_terracotta" => MinecraftBlock::MagentaTerracotta,
            "minecraft:pink_glazed_terracotta" => MinecraftBlock::PinkTerracotta,

            "minecraft:cherry_planks" => MinecraftBlock::CherryPlanks,
            "minecraft:bamboo_planks" => MinecraftBlock::BambooPlanks,
            "minecraft:birch_planks" => MinecraftBlock::BirchPlanks,
            "minecraft:oak_planks" => MinecraftBlock::OakPlanks,
            "minecraft:jungle_planks" => MinecraftBlock::JunglePlanks,
            "minecraft:acacia_planks" => MinecraftBlock::AcaciaPlanks,
            "minecraft:spruce_planks" => MinecraftBlock::SprucePlanks,
            "minecraft:dark_oak_planks" => MinecraftBlock::DarkOakPlanks,
            "minecraft:crimson_planks" => MinecraftBlock::CrimsonPlanks,
            "minecraft:warped_planks" => MinecraftBlock::WarpedPlanks,

            "minecraft:coal_block" => MinecraftBlock::CoalBlock,
            "minecraft:redstone_block" => MinecraftBlock::RedstoneBlock,

            "minecraft:cherry_log" => MinecraftBlock::CherryLog,
            "minecraft:bamboo_block" => MinecraftBlock::BambooLog,
            "minecraft:birch_log" => MinecraftBlock::BirchLog,
            "minecraft:oak_log" => MinecraftBlock::OakLog,
            "minecraft:jungle_log" => MinecraftBlock::JungleLog,
            "minecraft:acacia_log" => MinecraftBlock::AcaciaLog,
            "minecraft:spruce_log" => MinecraftBlock::SpruceLog,
            "minecraft:dark_oak_log" => MinecraftBlock::DarkOakLog,
            "minecraft:crimson_stem" => MinecraftBlock::CrimsonLog,
            "minecraft:warped_stem" => MinecraftBlock::WarpedLog,

            "minecraft:dark_oak_fence" => MinecraftBlock::DarkOakFence,
            "minecraft:cherry_fence" => MinecraftBlock::CherryFence,
            "minecraft:crimson_fence" => MinecraftBlock::CrimsonFence,
            "minecraft:warped_fence" => MinecraftBlock::WarpedFence,

            "minecraft:gold_block" => MinecraftBlock::GoldBlock,
            "minecraft:iron_block" => MinecraftBlock::IronBlock,
            "minecraft:lapis_block" => MinecraftBlock::LapisBlock,
            "minecraft:emerald_block" => MinecraftBlock::EmeraldBlock,
            "minecraft:diamond_block" => MinecraftBlock::DiamondBlock,

            "minecraft:raw_copper_block" => MinecraftBlock::RawCopperBlock,
            "minecraft:raw_iron_block" => MinecraftBlock::RawIronBlock,
            "minecraft:raw_gold_block" => MinecraftBlock::RawGoldBlock,

            "minecraft:bricks" => MinecraftBlock::Bricks,
            "minecraft:glass" => MinecraftBlock::Glass,
            "minecraft:crying_obsidian" => MinecraftBlock::CryingObsidian,
            "minecraft:bee_nest" => MinecraftBlock::BeeNest,
            "minecraft:bedrock" => MinecraftBlock::Bedrock,

            "minecraft:redstone_lamp" => MinecraftBlock::RedstoneLamp,
            "minecraft:glowstone" => MinecraftBlock::Glowstone,

            "minecraft:nether_quartz_block" => MinecraftBlock::NetherQuartzBlock,
            "minecraft:crimson_nylium" => MinecraftBlock::CrimsonNylium,
            "minecraft:warped_nylium" => MinecraftBlock::WarpedNylium,
            _ => return Err(MinecraftError::InvalidBlockType(value.to_string())),
        };

        Ok(b)
    }
}

impl MinecraftBlock {
    pub fn is_glass(&self) -> bool {
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

    pub fn is_wool(&self) -> bool {
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

    pub fn is_log(&self) -> bool {
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
                | MinecraftBlock::CrimsonLog
                | MinecraftBlock::WarpedLog
        )
    }

    pub fn to_digit(&self) -> Option<char> {
        let c = match self {
            MinecraftBlock::CherryLog => '0',
            MinecraftBlock::BambooLog => '1',
            MinecraftBlock::BirchLog => '2',
            MinecraftBlock::OakLog => '3',
            MinecraftBlock::JungleLog => '4',
            MinecraftBlock::AcaciaLog => '5',
            MinecraftBlock::SpruceLog => '6',
            MinecraftBlock::DarkOakLog => '7',
            MinecraftBlock::CrimsonLog => '8',
            MinecraftBlock::WarpedLog => '9',

            MinecraftBlock::WhiteTerracotta => '0',
            MinecraftBlock::LightGrayTerracotta => '1',
            MinecraftBlock::GrayTerracotta => '2',
            MinecraftBlock::BlackTerracotta => '3',
            MinecraftBlock::BrownTerracotta => '4',
            MinecraftBlock::RedTerracotta => '5',
            MinecraftBlock::OrangeTerracotta => '6',
            MinecraftBlock::YellowTerracotta => '7',
            MinecraftBlock::LimeTerracotta => '8',
            MinecraftBlock::GreenTerracotta => '9',
            MinecraftBlock::CyanTerracotta => 'A',
            MinecraftBlock::LightBlueTerracotta => 'B',
            MinecraftBlock::BlueTerracotta => 'C',
            MinecraftBlock::PurpleTerracotta => 'D',
            MinecraftBlock::MagentaTerracotta => 'E',
            MinecraftBlock::PinkTerracotta => 'F',

            MinecraftBlock::WhiteGlass => '0',
            MinecraftBlock::LightGrayGlass => '1',
            MinecraftBlock::GrayGlass => '2',
            MinecraftBlock::BlackGlass => '3',
            MinecraftBlock::BrownGlass => '4',
            MinecraftBlock::RedGlass => '5',
            MinecraftBlock::OrangeGlass => '6',
            MinecraftBlock::YellowGlass => '7',
            MinecraftBlock::LimeGlass => '8',
            MinecraftBlock::GreenGlass => '9',
            MinecraftBlock::CyanGlass => 'A',
            MinecraftBlock::LightBlueGlass => 'B',
            MinecraftBlock::BlueGlass => 'C',
            MinecraftBlock::PurpleGlass => 'D',
            MinecraftBlock::MagentaGlass => 'E',
            MinecraftBlock::PinkGlass => 'F',

            MinecraftBlock::WhiteWool => '0',
            MinecraftBlock::LightGrayWool => '1',
            MinecraftBlock::GrayWool => '2',
            MinecraftBlock::BlackWool => '3',
            MinecraftBlock::BrownWool => '4',
            MinecraftBlock::RedWool => '5',
            MinecraftBlock::OrangeWool => '6',
            MinecraftBlock::YellowWool => '7',
            MinecraftBlock::LimeWool => '8',
            MinecraftBlock::GreenWool => '9',
            MinecraftBlock::CyanWool => 'A',
            MinecraftBlock::LightBlueWool => 'B',
            MinecraftBlock::BlueWool => 'C',
            MinecraftBlock::PurpleWool => 'D',
            MinecraftBlock::MagentaWool => 'E',
            MinecraftBlock::PinkWool => 'F',

            MinecraftBlock::CherryPlanks => '0',
            MinecraftBlock::BambooPlanks => '1',
            MinecraftBlock::BirchPlanks => '2',
            MinecraftBlock::OakPlanks => '3',
            MinecraftBlock::JunglePlanks => '4',
            MinecraftBlock::AcaciaPlanks => '5',
            MinecraftBlock::SprucePlanks => '6',
            MinecraftBlock::DarkOakPlanks => '7',
            MinecraftBlock::CrimsonPlanks => '8',
            MinecraftBlock::WarpedPlanks => '9',

            _ => return None,
        };

        Some(c)
    }
}
