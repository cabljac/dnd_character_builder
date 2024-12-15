use super::base::RogueSubclass;
use crate::types::*;

pub struct AssassinRogue;

impl RogueSubclass for AssassinRogue {
    fn get_subclass_properties() -> SubclassProperties {
        SubclassProperties {
            name: String::from("Assassin"),
            description: String::from(
                "You focus your training on the grim art of death, becoming a deadly killer."
            ),
            features: vec![
                ClassFeature {
                    name: String::from("Bonus Proficiencies"),
                    level: 3,
                    feature_type: FeatureType::Passive {
                        description: String::from(
                            "You gain proficiency with the disguise kit and the poisoner's kit."
                        ),
                    },
                },
                ClassFeature {
                    name: String::from("Assassinate"),
                    level: 3,
                    feature_type: FeatureType::Passive {
                        description: String::from(
                            "You have advantage on attack rolls against any creature that hasn't taken a turn in combat yet. \
                            In addition, any hit you score against a creature that is surprised is a critical hit."
                        ),
                    },
                },
                ClassFeature {
                    name: String::from("Infiltration Expertise"),
                    level: 9,
                    feature_type: FeatureType::Passive {
                        description: String::from(
                            "You can unfailingly create false identities for yourself. You must spend seven days and 25gp to \
                            establish the history, profession, and affiliations for an identity. You can't establish an identity \
                            that belongs to someone else. Thereafter, if you adopt the new identity as a disguise, other creatures \
                            believe you to be that person until given an obvious reason not to."
                        ),
                    },
                },
                ClassFeature {
                    name: String::from("Impostor"),
                    level: 13,
                    feature_type: FeatureType::Passive {
                        description: String::from(
                            "You gain the ability to unerringly mimic another person's speech, writing, and behavior. \
                            You must spend at least three hours studying these three components of the person's behavior, \
                            listening to speech, examining handwriting, and observing mannerisms. Your ruse is indiscernible \
                            to the casual observer. If a wary creature suspects something is amiss, you have advantage on any \
                            Charisma (Deception) check you make to avoid detection."
                        ),
                    },
                },
            ],
        }
    }
}

impl AssassinRogue {
    pub fn create() -> Class {
        let mut props = super::base::get_base_rogue_properties();
        props.subclass = Some(Self::get_subclass_properties());
        Class::Rogue(props)
    }
}
