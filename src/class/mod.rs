pub mod fighter;
pub mod rogue;
pub mod wizard;

// Instead of glob exports, let's be specific about what we want to expose
pub use fighter::{base::get_base_fighter_properties, champion::ChampionFighter};
pub use rogue::{assassin::AssassinRogue, base::get_base_rogue_properties};
pub use wizard::{base::get_base_wizard_properties, evocation::EvocationWizard};
