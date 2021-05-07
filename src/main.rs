use ron::{de::from_reader, Error};
use serde::{Deserialize, Serialize};
use std::fs;

fn main() {
    let helmets = get_armors_from_ron("helmets.ron").unwrap();
    save_armors_to_json(&helmets, "helmets.json");
}

pub fn get_armors_from_ron(path: &str) -> Result<Vec<Armor>, Error> {
    let armors: Vec<Armor> = from_reader(fs::File::open(path)?)?;
    Ok(armors)
}

pub fn save_armors_to_json(armors: &[Armor], path: &str) {
    let json = serde_json::to_string_pretty(armors).unwrap();

    fs::write(path, json).unwrap();
}

#[derive(Deserialize, Serialize)]
pub enum Gender {
    Female,
    Male,
    Neutral,
}

#[derive(Deserialize, Serialize)]
pub struct Armor {
    pub name: String,
    pub skills: Vec<(Skill, u8)>,
    pub slots: Vec<u8>,
    pub rare: u8,
    pub defense: u8,
    pub fire: i8,
    pub water: i8,
    pub thunder: i8,
    pub ice: i8,
    pub dragon: i8,
    pub gender: Gender,
}

#[derive(Deserialize, Serialize)]
pub enum Skill {
    Botanist,
    DefenseBoost,
    ItemProlonger,
    CriticalEye,
    Fortify,
    PoisonAttack,
    RecoilDown,
    QuickSheath,
    FireAttack,
    IceAttack,
    WaterAttack,
    ProtectivePolish,
    StaminaThief,
    Partbreaker,
    Mushroomancer,
    MaximumMight,
    MarathonRunner,
    PeakPerformance,
    AttackBoost,
    OffensiveGuard,
    Focus,
    RecoveryUp,
    NormalRapidUp,
    SpeedEating,
    Windproof,
    Bludgeoner,
    AffinitySliding,
    WideRange,
    StunResistance,
    LoadShells,
    ParalysisAttack,
    PierceUp,
    SleepAttack,
    BlightResistance,
    CriticalDraw,
    JumpMaster,
    Constitution,
    FreeMeal,
    GoodLuck,
    RazorSharp,
    SpareShot,
    WirebugWhisperer,
    Resentment,
    Handicraft,
    FlinchFree,
    RapidMorph,
    LatentPower,
    WeaknessExploit,
    Resuscitate,
    EvadeWindow,
    Slugger,
    SpecialAmmoBoost,
    Agitator,
    DivineBlessing,
    Geologist,
    HungerResistance,
    CriticalElement,
    EvadeExtender,
    DragonAttack,
    Heroics,
    SleepResistance,
    ParalysisResistance,
    PoisonResistance,
    WindAlignment,
    SpreadUp,
    ReloadSpeed,
    ThunderAlignment,
    Guard,
    StaminaSurge,
    Earplugs,
    BowChargePlus,
    BlastResistance,
    AmmoUp,
    LeapofFaith,
    DragonResistance,
    WaterResistance,
    RecoverySpeed,
    SpeedSharpening,
    MuckResistance,
    PowerProlonger,
    TremorResistance,
    HellfireCloak,
    BubblyDance,
    PunishingDraw,
    WallRunner,
    GuardUp,
    CriticalBoost,
    MindsEye,
    BlastAttack,
    MasterMounter,
    Counterstrike,
    ThunderAttack,
    Artillery,
    Bombardier,
    CaptureMaster,
    Diversion,
    FireResistance,
    HornMaestro,
    Ballistics,
    KushalaBlessing,
    ChameleosBlessing,
    TeostraBlessing,
    MastersTouch,
    RapidFireUp,
    CarvingPro,
    Steadiness,
    IceResistance,
    ThunderResistance,
    CarvingMaster,
}
