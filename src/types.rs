use crate::class;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub level: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Dragonborn,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HitDie {
    D6,
    D8,
    D10,
    D12,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ArmorType {
    LightArmor,
    MediumArmor,
    HeavyArmor,
    Shields,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WeaponType {
    SimpleWeapons,
    MartialWeapons,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RechargeType {
    ShortRest,
    LongRest,
    Daily,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureOption {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FeatureType {
    Passive {
        description: String,
    },
    Choice {
        description: String,
        options: Vec<FeatureOption>,
        num_to_choose: u8,
    },
    Resource {
        description: String,
        uses: u8,
        recharge: RechargeType,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassFeature {
    pub name: String,
    pub level: u8,
    pub feature_type: FeatureType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassProperties {
    pub hit_die: HitDie,
    pub primary_ability: Vec<Ability>,
    pub saving_throw_proficiencies: Vec<Ability>,
    pub armor_proficiencies: Vec<ArmorType>,
    pub weapon_proficiencies: Vec<WeaponType>,
    pub features: Vec<ClassFeature>,
    pub subclass: Option<SubclassProperties>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Class {
    Fighter(ClassProperties),
    Wizard(ClassProperties),
    Rogue(ClassProperties),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubclassProperties {
    pub name: String,
    pub description: String,
    pub features: Vec<ClassFeature>,
}

// Display implementations
impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Race::Human => write!(f, "Human"),
            Race::Elf => write!(f, "Elf"),
            Race::Dwarf => write!(f, "Dwarf"),
            Race::Halfling => write!(f, "Halfling"),
            Race::Dragonborn => write!(f, "Dragonborn"),
            Race::Gnome => write!(f, "Gnome"),
            Race::HalfElf => write!(f, "Half-Elf"),
            Race::HalfOrc => write!(f, "Half-Orc"),
            Race::Tiefling => write!(f, "Tiefling"),
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Class::Fighter(_) => write!(f, "Fighter"),
            Class::Wizard(_) => write!(f, "Wizard"),
            Class::Rogue(_) => write!(f, "Rogue"),
        }
    }
}

// Just keep the base class constructors here
impl Class {
    pub fn fighter() -> Self {
        Class::Fighter(class::fighter::base::get_base_fighter_properties())
    }

    pub fn wizard() -> Self {
        Class::Wizard(class::wizard::base::get_base_wizard_properties())
    }

    pub fn rogue() -> Self {
        Class::Rogue(class::rogue::base::get_base_rogue_properties())
    }
}
