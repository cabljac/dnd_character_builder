use crate::types::*;

pub fn get_base_fighter_properties() -> ClassProperties {
    ClassProperties {
        hit_die: HitDie::D10,
        primary_ability: vec![Ability::Strength, Ability::Dexterity],
        saving_throw_proficiencies: vec![Ability::Strength, Ability::Constitution],
        armor_proficiencies: vec![
            ArmorType::LightArmor,
            ArmorType::MediumArmor,
            ArmorType::HeavyArmor,
            ArmorType::Shields,
        ],
        weapon_proficiencies: vec![WeaponType::SimpleWeapons, WeaponType::MartialWeapons],
        features: vec![
            ClassFeature {
                name: String::from("Fighting Style"),
                level: 1,
                feature_type: FeatureType::Choice {
                    description: String::from(
                        "You adopt a particular style of fighting as your specialty.",
                    ),
                    options: vec![
                        FeatureOption {
                            name: String::from("Archery"),
                            description: String::from(
                                "+2 bonus to attack rolls with ranged weapons",
                            ),
                        },
                        FeatureOption {
                            name: String::from("Defense"),
                            description: String::from("+1 to AC when wearing armor"),
                        },
                        FeatureOption {
                            name: String::from("Dueling"),
                            description: String::from(
                                "+2 to damage when wielding a single one-handed weapon",
                            ),
                        },
                        FeatureOption {
                            name: String::from("Great Weapon Fighting"),
                            description: String::from(
                                "Reroll 1s and 2s on damage with two-handed weapons",
                            ),
                        },
                    ],
                    num_to_choose: 1,
                },
            },
            ClassFeature {
                name: String::from("Second Wind"),
                level: 1,
                feature_type: FeatureType::Resource {
                    description: String::from(
                        "You can use a bonus action to regain 1d10 + fighter level HP.",
                    ),
                    uses: 1,
                    recharge: RechargeType::ShortRest,
                },
            },
            ClassFeature {
                name: String::from("Action Surge"),
                level: 2,
                feature_type: FeatureType::Resource {
                    description: String::from("You can take one additional action on your turn."),
                    uses: 1,
                    recharge: RechargeType::ShortRest,
                },
            },
        ],
        subclass: None,
    }
}

pub trait FighterSubclass {
    fn get_subclass_properties() -> SubclassProperties;
}
