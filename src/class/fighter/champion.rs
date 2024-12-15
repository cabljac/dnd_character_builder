use super::base::FighterSubclass;
use crate::types::*;

pub struct ChampionFighter;

impl FighterSubclass for ChampionFighter {
    fn get_subclass_properties() -> SubclassProperties {
        SubclassProperties {
            name: String::from("Champion"),
            description: String::from("The Champion focuses on the development of raw physical power honed to deadly perfection."),
            features: vec![
                ClassFeature {
                    name: String::from("Improved Critical"),
                    level: 3,
                    feature_type: FeatureType::Passive {
                        description: String::from(
                            "Your weapon attacks score a critical hit on a roll of 19 or 20."
                        ),
                    },
                },
                ClassFeature {
                    name: String::from("Additional Fighting Style"),
                    level: 10,
                    feature_type: FeatureType::Choice {
                        description: String::from("You can choose a second option from the Fighting Style class feature."),
                        options: vec![
                            FeatureOption {
                                name: String::from("Archery"),
                                description: String::from("+2 bonus to attack rolls with ranged weapons"),
                            },
                            FeatureOption {
                                name: String::from("Defense"),
                                description: String::from("+1 to AC when wearing armor"),
                            },
                            FeatureOption {
                                name: String::from("Dueling"),
                                description: String::from("+2 to damage when wielding a single one-handed weapon"),
                            },
                            FeatureOption {
                                name: String::from("Great Weapon Fighting"),
                                description: String::from("Reroll 1s and 2s on damage with two-handed weapons"),
                            },
                        ],
                        num_to_choose: 1,
                    },
                },
                ClassFeature {
                    name: String::from("Superior Critical"),
                    level: 15,
                    feature_type: FeatureType::Passive {
                        description: String::from(
                            "Your weapon attacks score a critical hit on a roll of 18-20."
                        ),
                    },
                },
            ],
        }
    }
}

impl ChampionFighter {
    pub fn create() -> Class {
        let mut props = super::base::get_base_fighter_properties();
        props.subclass = Some(Self::get_subclass_properties());
        Class::Fighter(props)
    }
}
