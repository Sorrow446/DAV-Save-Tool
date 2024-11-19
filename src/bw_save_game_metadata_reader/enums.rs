use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub enum SaveType {
    #[default]
    Manual,
    Auto,
    Safety,
    Chapter,
    Milestone,
    Decision,
    PointOfNoReturn,
    CharGen,
    Count,
    Invalid,
}

impl SaveType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "BWSavegameType_Manual" => Some(Self::Manual),
            "BWSavegameType_Auto" => Some(Self::Auto),
            "BWSavegameType_Safety" => Some(Self::Safety),
            "BWSavegameType_Chapter" => Some(Self::Chapter),
            "BWSavegameType_Milestone" => Some(Self::Milestone),
            "BWSavegameType_Decision" => Some(Self::Decision),
            "BWSavegameType_PointOfNoReturn" => Some(Self::PointOfNoReturn),
            "BWSavegameType_CharGen" => Some(Self::CharGen),
            "BWSavegameType_Count" => Some(Self::Count),
            "BWSavegameType_Invalid" => Some(Self::Invalid),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum CharacterArchetype {
    #[default]
    Crow,
    Dalish,
    Desperado,
    FollowerBellara,
    FollowerDavrin,
    FollowerEmmrich,
    FollowerHarding,
    FollowerLucanis,
    FollowerNeve,
    FollowerSolas,
    FollowerSpite,
    FollowerTaash,
    FollowerVarric,
    Fortune,
    Mage,
    NullPlayer,
    PlayerRGZtest,
    Ranger01,
    Ranger02,
    Ranger03,
    Rogue,
    ShadowEvoker,
    Warden4,
    WardenArt,
    WardenChallenger,
    WardenCine,
    WardenEndurance,
    WardenPower,
    WardenStrategy,
    WardenTechnique,
    Warrior,
    Watcher,
}

impl CharacterArchetype {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            116806840 => Some(Self::FollowerBellara),
            1326121707 => Some(Self::FollowerHarding),
            1480587723 => Some(Self::Ranger02),
            1486725849 => Some(Self::WardenTechnique),
            1837455073 => Some(Self::ShadowEvoker),
            1887180846 => Some(Self::FollowerSpite),
            1902731980 => Some(Self::Rogue),
            1928218134 => Some(Self::FollowerNeve),
            2143795149 => Some(Self::FollowerLucanis),
            2257715964 => Some(Self::WardenStrategy),
            2325381541 => Some(Self::Watcher),
            2366407241 => Some(Self::Crow),
            240491018 => Some(Self::Mage),
            2602884150 => Some(Self::FollowerDavrin),
            267923513 => Some(Self::Warrior),
            2714609019 => Some(Self::Desperado),
            28757921 => Some(Self::WardenPower),
            2903517207 => Some(Self::Warden4),
            291152393 => Some(Self::Dalish),
            2930410500 => Some(Self::PlayerRGZtest),
            294481 => Some(Self::WardenCine),
            3417468734 => Some(Self::FollowerVarric),
            3509394015 => Some(Self::NullPlayer),
            3517341798 => Some(Self::Ranger01),
            3723887171 => Some(Self::WardenArt),
            3734548853 => Some(Self::FollowerEmmrich),
            3822852109 => Some(Self::Ranger03),
            394763556 => Some(Self::FollowerSolas),
            3998641339 => Some(Self::WardenChallenger),
            4003900063 => Some(Self::WardenEndurance),
            4131396826 => Some(Self::FollowerTaash),
            624386075 => Some(Self::Fortune),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum CharacterLineage {
    #[default]
    Elf,
    Dwarf,
    Human,
    Qunari,
}

impl CharacterLineage {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Human),
            1 => Some(Self::Dwarf),
            2 => Some(Self::Elf),
            3 => Some(Self::Qunari),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum CharacterFaction {
    #[default]
    AntivanCrows,
    GreyWardens,
    LordsOfFortune,
    ShadowDragons,
    TheMournWatch,
    VeilJumpers,
}

impl CharacterFaction {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::GreyWardens),
            1 => Some(Self::VeilJumpers),
            2 => Some(Self::ShadowDragons),
            3 => Some(Self::LordsOfFortune),
            4 => Some(Self::TheMournWatch),
            5 => Some(Self::AntivanCrows),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum CharacterGender {
    #[default]
    Male,
    Female,
}

impl CharacterGender {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Male),
            1 => Some(Self::Female),
            _ => None,
        }
    }
}


#[derive(Debug, Default, Serialize)]
pub enum CharacterPronouns {
    #[default]
    HeHim,
    SheHer,
    TheyThem,
}

impl CharacterPronouns {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::HeHim),
            1 => Some(Self::SheHer),
            2 => Some(Self::TheyThem),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum CharacterVoice {
    #[default]
    FeminineOne,
    FeminineTwo,
    MasculineOne,
    MasculineTwo,
}

impl CharacterVoice {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::FeminineTwo),
            1 => Some(Self::MasculineTwo),
            2 => Some(Self::FeminineOne),
            3 => Some(Self::MasculineOne),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum CharacterVoiceTone {
    #[default]
    Low,
    Medium,
}

impl CharacterVoiceTone {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Medium),
            1 => Some(Self::Low),
            _ => None,
        }
    }
}

// fem 1, med - voice 2, tone 0
// fem 2, med - voice 0, tone 0
// masc 1, med - voice 3, tone 0
// masc 2, low - voice 1, tone 1

// storyteller 2
// keeper 3
// adventurer 1
// underdog 4
// nightmare 5
// custom 6

#[derive(Debug, Default, Serialize)]
pub enum Difficulty {
    #[default]
    Adventurer,
    Custom,
    Keeper,
    Nightmare,
    Storyteller,
    Underdog,
}

impl Difficulty {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            1 => Some(Self::Adventurer),
            2 => Some(Self::Storyteller),
            3 => Some(Self::Keeper),
            4 => Some(Self::Underdog),
            5 => Some(Self::Nightmare),
            6 => Some(Self::Custom),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum KeyBindingProfile {
    #[default]
    Mage,
    Rogue,
    Warrior
}

impl KeyBindingProfile {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Warrior),
            1 => Some(Self::Rogue),
            2 => Some(Self::Mage),
            _ => None,
        }
    }
}