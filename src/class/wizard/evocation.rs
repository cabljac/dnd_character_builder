use super::base::WizardSubclass;
use crate::types::*;

pub struct EvocationWizard;

impl WizardSubclass for EvocationWizard {
    fn get_subclass_properties() -> SubclassProperties {
        SubclassProperties {
            name: String::from("School of Evocation"),
            description: String::from("You focus your study on magic that creates powerful elemental effects such as bitter cold, searing flame, rolling thunder, crackling lightning, and burning acid."),
            features: vec![
                ClassFeature {
                    name: String::from("Evocation Savant"),
                    level: 2,
                    feature_type: FeatureType::Passive {
                        description: String::from(
                            "The gold and time you must spend to copy an evocation spell into your spellbook is halved."
                        ),
                    },
                },
                ClassFeature {
                    name: String::from("Sculpt Spells"),
                    level: 2,
                    feature_type: FeatureType::Passive {
                        description: String::from(
                            "You can create pockets of relative safety within the effects of your evocation spells. \
                            When you cast an evocation spell that affects other creatures that you can see, \
                            you can choose a number of them equal to 1 + the spell's level. \
                            The chosen creatures automatically succeed on their saving throws against the spell, \
                            and they take no damage if they would normally take half damage on a successful save."
                        ),
                    },
                },
            ],
        }
    }
}

// Constructor for evocation wizard
impl EvocationWizard {
    pub fn create() -> Class {
        let mut props = super::base::get_base_wizard_properties();
        props.subclass = Some(Self::get_subclass_properties());
        Class::Wizard(props)
    }
}
