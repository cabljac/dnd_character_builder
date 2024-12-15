use inquire::{Select, Text};
pub mod class;
pub mod types;
use types::*;

fn display_features(features: &[&ClassFeature]) {
    for feature in features {
        println!("\n  {}: (Level {})", feature.name, feature.level);
        match &feature.feature_type {
            FeatureType::Passive { description } => {
                println!("    {}", description);
            }
            FeatureType::Choice {
                description,
                options,
                num_to_choose,
            } => {
                println!("    {}", description);
                println!("    Choose {} from:", num_to_choose);
                for option in options {
                    println!("    - {}: {}", option.name, option.description);
                }
            }
            FeatureType::Resource {
                description,
                uses,
                recharge,
            } => {
                println!("    {}", description);
                println!("    Uses: {}, Recharges on: {:?}", uses, recharge);
            }
        }
    }
}

fn main() {
    println!("D&D Character Builder");
    println!("--------------------");

    let races = vec![
        Race::Human,
        Race::Elf,
        Race::Dwarf,
        Race::Halfling,
        Race::Dragonborn,
        Race::Gnome,
        Race::HalfElf,
        Race::HalfOrc,
        Race::Tiefling,
    ];

    let classes = vec![
        Class::fighter(),
        Class::wizard(),
        Class::rogue(),
        class::fighter::champion::ChampionFighter::create(),
        class::wizard::evocation::EvocationWizard::create(),
        class::rogue::assassin::AssassinRogue::create(),
    ];

    let name = Text::new("What is your character's name?")
        .prompt()
        .expect("Failed to get character name");

    let race = Select::new("Choose your race:", races)
        .prompt()
        .expect("Failed to select race");

    let class = Select::new("Choose your class:", classes)
        .prompt()
        .expect("Failed to select class");

    let character = Character {
        name,
        race,
        class,
        level: 1,
    };

    println!("\nCharacter Created!");
    println!("==================");
    println!("Name: {}", character.name);
    println!("Race: {}", character.race);
    println!("Class: {}", character.class);
    println!("Level: {}", character.level);

    match &character.class {
        Class::Fighter(props) | Class::Wizard(props) | Class::Rogue(props) => {
            println!("\nClass Properties");
            println!("===============");
            println!("Hit Die: {:?}", props.hit_die);
            println!("Primary Abilities: {:?}", props.primary_ability);
            println!(
                "Saving Throw Proficiencies: {:?}",
                props.saving_throw_proficiencies
            );
            println!("Armor Proficiencies: {:?}", props.armor_proficiencies);
            println!("Weapon Proficiencies: {:?}", props.weapon_proficiencies);

            println!("\nClass Features");
            println!("==============");

            let available_features = props
                .features
                .iter()
                .filter(|f| f.level <= character.level)
                .collect::<Vec<_>>();
            display_features(&available_features);

            if let Some(subclass) = &props.subclass {
                println!("\nSubclass: {}", subclass.name);
                println!("======================");
                println!("{}", subclass.description);

                let subclass_features = subclass
                    .features
                    .iter()
                    .filter(|f| f.level <= character.level)
                    .collect::<Vec<_>>();

                if !subclass_features.is_empty() {
                    println!("\nSubclass Features");
                    println!("================");
                    display_features(&subclass_features);
                }
            }
        }
    }
}
