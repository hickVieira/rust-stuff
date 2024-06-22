enum Species {
    Human,
    Crab,
    Octopus,
    Fish,
    Clam,
}

enum PoisonType {
    Acid,
    Poison,
    Panic,
}

enum Size {
    Small,
    Medium,
    Big,
}

enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

struct SeaCreature {
    species: Species,
    names: String,
    arms: u32,
    legs: u32,
    weapon: Weapon,
}

pub fn run() {
    let Ferris = SeaCreature {
        species: Species::Crab,
        names: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(32, Size::Small),
    };

    match Ferris.species {
        Species::Crab => {
            println!("{} is a crab", Ferris.names);
            match Ferris.weapon {
                Weapon::Claw(num_claws, size_claws) => {
                    let size_desc = match size_claws {
                        Size::Small => "small",
                        Size::Medium => "medium",
                        Size::Big => "big",
                    };
                }
                _ => {}
            }
        }
        Species::Octopus => println!("{} is an octopus", Ferris.names),
        Species::Fish => println!("{} is a fish", Ferris.names),
        Species::Clam => println!("{} is a clam", Ferris.names),
        _ => println!("{} is a human", Ferris.names),
    }
}