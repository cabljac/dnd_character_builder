use crate::types::*;

pub fn get_base_wizard_properties() -> ClassProperties {
    ClassProperties {
        hit_die: HitDie::D6,
        primary_ability: vec![Ability::Intelligence],
        saving_throw_proficiencies: vec![Ability::Intelligence, Ability::Wisdom],
        armor_proficiencies: vec![],
        weapon_proficiencies: vec![WeaponType::SimpleWeapons],
        features: vec![
            ClassFeature {
                name: String::from("Spellcasting"),
                level: 1,
                feature_type: FeatureType::Passive {
                    description: String::from(
                        "As a student of arcane magic, you have a spellbook containing spells. \
                        At 1st level, you have a spellbook containing six 1st-level wizard spells of your choice."
                    ),
                },
            },
            ClassFeature {
                name: String::from("Arcane Recovery"),
                level: 1,
                feature_type: FeatureType::Resource {
                    description: String::from(
                        "Once per day when you finish a short rest, you can recover spell slots."
                    ),
                    uses: 1,
                    recharge: RechargeType::Daily,
                },
            },
        ],
        subclass: None,  // Base properties have no subclass
    }
}

// Common wizard methods could go here
pub trait WizardSubclass {
    fn get_subclass_properties() -> SubclassProperties;
}
