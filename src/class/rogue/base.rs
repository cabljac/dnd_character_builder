use crate::types::*;

pub fn get_base_rogue_properties() -> ClassProperties {
    ClassProperties {
        hit_die: HitDie::D8,
        primary_ability: vec![Ability::Dexterity],
        saving_throw_proficiencies: vec![Ability::Dexterity, Ability::Intelligence],
        armor_proficiencies: vec![ArmorType::LightArmor],
        weapon_proficiencies: vec![WeaponType::SimpleWeapons],
        features: vec![
            ClassFeature {
                name: String::from("Expertise"),
                level: 1,
                feature_type: FeatureType::Choice {
                    description: String::from("Choose two of your skill proficiencies. Your proficiency bonus is doubled for these skills."),
                    options: vec![], // Would be populated based on character's skills
                    num_to_choose: 2,
                },
            },
            ClassFeature {
                name: String::from("Sneak Attack"),
                level: 1,
                feature_type: FeatureType::Passive {
                    description: String::from(
                        "Once per turn, you can deal extra damage when you hit with a finesse or ranged weapon, \
                        if you have advantage or another enemy of your target is within 5 feet of it. \
                        The damage is 1d6 at 1st level, and increases as you gain levels."
                    ),
                },
            },
            ClassFeature {
                name: String::from("Thieves' Cant"),
                level: 1,
                feature_type: FeatureType::Passive {
                    description: String::from(
                        "You have learned thieves' cant, a secret mix of dialect, jargon, and code."
                    ),
                },
            },
            ClassFeature {
                name: String::from("Cunning Action"),
                level: 2,
                feature_type: FeatureType::Passive {
                    description: String::from(
                        "You can take a bonus action on each of your turns to Dash, Disengage, or Hide."
                    ),
                },
            },
        ],
        subclass: None,
    }
}

pub trait RogueSubclass {
    fn get_subclass_properties() -> SubclassProperties;
}
