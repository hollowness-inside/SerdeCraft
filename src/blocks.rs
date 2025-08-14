use crate::result::MinecraftError;

macro_rules! block_enum {
    ({
        $($name:ident = $value:literal),*
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    WhiteTerracotta = "minecraft:white_glazed_terracotta",
    LightGrayTerracotta = "minecraft:light_gray_glazed_terracotta",
    GrayTerracotta = "minecraft:gray_glazed_terracotta",
    BlackTerracotta = "minecraft:black_glazed_terracotta",
    BrownTerracotta = "minecraft:brown_glazed_terracotta",
    RedTerracotta = "minecraft:red_glazed_terracotta",
    OrangeTerracotta = "minecraft:orange_glazed_terracotta",
    YellowTerracotta = "minecraft:yellow_glazed_terracotta",
    LimeTerracotta = "minecraft:lime_glazed_terracotta",
    GreenTerracotta = "minecraft:green_glazed_terracotta",
    CyanTerracotta = "minecraft:cyan_glazed_terracotta",
    LightBlueTerracotta = "minecraft:light_blue_glazed_terracotta",
    BlueTerracotta = "minecraft:blue_glazed_terracotta",
    PurpleTerracotta = "minecraft:purple_glazed_terracotta",
    MagentaTerracotta = "minecraft:magenta_glazed_terracotta",
    PinkTerracotta = "minecraft:pink_glazed_terracotta",

    CherryPlanks = "minecraft:cherry_planks",
    BambooPlanks = "minecraft:bamboo_planks",
    BirchPlanks = "minecraft:birch_planks",
    OakPlanks = "minecraft:oak_planks",
    JunglePlanks = "minecraft:jungle_planks",
    AcaciaPlanks = "minecraft:acacia_planks",
    SprucePlanks = "minecraft:spruce_planks",
    DarkOakPlanks = "minecraft:dark_oak_planks",
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
    CrimsonLog = "minecraft:crimson_stem",
    WarpedLog = "minecraft:warped_stem",

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
