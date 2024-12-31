use std::fs::File;
use std::io::{self, Write, Read};
use std::time::Instant;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Kitty {
    name: String,
    happiness: i32,
    health: i32,
    energy: i32,
    age: u32,
    playtime: u32,
    gone_to_vet: bool,
    gone_to_beach: bool,
    gone_on_walk: bool,
    gone_to_park: bool,
}

impl Kitty {
    fn new(name: String) -> Self {
        Kitty {
            name,
            happiness: 0,
            health: 100,
            energy: 100,
            age: 1,
            playtime: 0,
            gone_to_vet: false,
            gone_to_beach: false,
            gone_on_walk: false,
            gone_to_park: false,
        }
    }

    fn save(&self) {
        let data = serde_json::to_string(self).expect("Could not serialize kitty data");
        let mut file = File::create("kitty_save.json").expect("Could not create save file");
        file.write_all(data.as_bytes()).expect("Could not write to save file");
    }

    fn load() -> Option<Self> {
        let mut file = File::open("kitty_save.json").ok()?;
        let mut data = String::new();
        file.read_to_string(&mut data).ok()?;
        serde_json::from_str(&data).ok()
    }
}

fn main() {
    let mut start = String::new();
    println!("Welcome! This program will give you your own virtual kitty over the command line! Would you like to get started?");
    println!("y: Yeah!!Lesgoo!");
    println!("h: help me out");
    println!("q: quit");

    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    match io::stdin().read_line(&mut start) {
        Ok(_) => (),
        Err(err) => println!("Could not parse input: {}", err)
    }

    if start.trim().to_lowercase() == "y" {
        // proceed to naming the kitty
    } else if start.trim().to_lowercase() == "h" {
        println!("This program will give you your own virtual kitty to take care of. You will be given options to take care of your kitty. You can give your kitty water, food, play with it, or snuggle with it. You can also choose to ignore your kitty (please don't!). Your kitty will have a happiness level that will change based on your actions. If your kitty's happiness level reaches below 0, your kitty will run away. Have fun!");
        println!("\n");
    } else if start.trim().to_lowercase() == "q" {
        std::process::exit(0);
    } else {
        println!("Invalid action");
    }

    let mut kitty = if let Some(kitty) = Kitty::load() {
        println!("Loaded saved kitty: {}", kitty.name);
        kitty
    } else {
        let mut name = String::new();
        println!("Name your kitty: ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        io::stdin().read_line(&mut name).expect("Could not parse input");
        Kitty::new(name.trim().to_string())
    };

    loop {
        let start = Instant::now();

        println!(r"  ╱|、");
        println!(r" (˚ˎ 。7 ");
        println!(r" |、˜ 〵   ");
        println!(r" じしˍ,)ノ");

        while start.elapsed().as_secs() < 5 {}

        let to_run = rand::thread_rng().gen_range(1..=6);
        match to_run {
            1 => thirsty(&mut kitty),
            2 => hungry(&mut kitty),
            3 => playful(&mut kitty),
            4 => snuggles(&mut kitty),
            5 => random_event(&mut kitty),
            6 => shop(&mut kitty),
            _ => (),
        }

        println!("\n");
        println!("Happiness: {}", kitty.happiness);
        println!("Health: {}", kitty.health);
        println!("Energy: {}", kitty.energy);
        println!("Age: {}", kitty.age);
        println!("Playtime: {}", kitty.playtime);
        println!("\n");

        if kitty.happiness < 0 {
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);
        }

        if kitty.health <= 0 {
            println!("{}'s health reached 0. Game over!", kitty.name);
            std::process::exit(0);
        }

        if kitty.energy <= 0 {
            println!("{} is too tired. Game over!", kitty.name);
            std::process::exit(0);
        }

        if kitty.age == 2 && !kitty.gone_to_vet {
            vet(&kitty.name);
            kitty.gone_to_vet = true;
        }

        if kitty.age == 3 && !kitty.gone_to_beach {
            beach(&kitty.name);
            kitty.gone_to_beach = true;
        }

        if kitty.age == 4 && !kitty.gone_on_walk {
            walk(&kitty.name);
            kitty.gone_on_walk = true;
        }

        if kitty.age == 5 && !kitty.gone_to_park {
            park(&kitty.name);
            kitty.gone_to_park = true;
        }

        kitty.age += 1;
        kitty.save();
    }
}

fn thirsty(kitty: &mut Kitty) {
    loop {
        let mut action = String::new();

        let thirsty = format!(r#"
{} is thirsty! What do you do?
        
        a: give {} water
        b: give {} food
        c: play with {}
        d: snuggle with {}
        e: ignore {}"#, kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim());

        println!("{}", thirsty);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let water = format!("
{} drinks water.
{}'s happiness went up by 3", kitty.name.trim(), kitty.name.trim());

            println!("{}", water);
            kitty.happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "b" {
            let food = format!("
{} doesn't want food.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", food);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            let play = format!("
{} doesn't want to play.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", play);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            let snuggle = format!("
{} doesn't want to snuggle.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", snuggle);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            let sad = format!("
{} is sad.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", sad);
            kitty.happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn hungry(kitty: &mut Kitty) {
    loop {
        let mut action = String::new();

        let hungry = format!(r#"
{} is hungry! What do you do?
        
        a: give {} water
        b: give {} food
        c: play with {}
        d: snuggle with {}
        e: ignore {}"#, kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim());

        println!("{}", hungry);

        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let water = format!("
{} doesn't want water.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", water);
            kitty.happiness -= 2;

            break;
        } else if action.trim().to_lowercase() == "b" {
            let food = format!("
{} eats.
{}'s happiness went up by 3", kitty.name.trim(), kitty.name.trim());

            println!("{}", food);
            kitty.happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "c" {
            let play = format!("
{} doesn't want to play.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", play);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            let snuggle = format!("
{} doesn't want to snuggle.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", snuggle);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            let sad = format!("
{} is sad.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", sad);
            kitty.happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn playful(kitty: &mut Kitty) {
    loop {
        let mut action = String::new();

        let playful = format!(r#"
{} wants to play! What do you do?
        
        a: give {} water
        b: give {} food
        c: play with {}
        d: snuggle with {}
        e: ignore {}"#, kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim());

        println!("{}", playful);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let water = format!("
{} doesn't want water.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", water);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "b" {
            let food = format!("
{} doesn't want food.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", food);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            let play = format!("
{} plays.
{}'s happiness went up by 3", kitty.name.trim(), kitty.name.trim());

            println!("{}", play);
            kitty.happiness += 3;
            kitty.playtime += 1;
            break;
        } else if action.trim().to_lowercase() == "d" {
            let snuggle = format!("
{} doesn't want to snuggle.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", snuggle);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            let sad = format!("
{} is sad.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", sad);
            kitty.happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn snuggles(kitty: &mut Kitty) {
    loop {
        let mut action = String::new();

        let snuggles = format!(r#"
{} wants to snuggle! What do you do?
        
        a: give {} water
        b: give {} food
        c: play with {}
        d: snuggle with {}
        e: ignore {}"#, kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim(), kitty.name.trim());

        println!("{}", snuggles);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let water = format!("
{} doesn't want water.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", water);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "b" {
            let food = format!("
{} doesn't want food.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", food);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            let play = format!("
{} doesn't want to play.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", play);
            kitty.happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            let snuggle = format!("
{} snuggles.
{}'s happiness went up by 3", kitty.name.trim(), kitty.name.trim());

            println!("{}", snuggle);
            kitty.happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "e" {
            let sad = format!("
{} is sad.
{}'s happiness went down by 2", kitty.name.trim(), kitty.name.trim());

            println!("{}", sad);
            kitty.happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn vet(name: &String) {
    loop {
        let mut action = String::new();

        let vet = format!(r#"
{} needs to go to the vet! What do you do?
        
        a: take {} to the vet
        b: take {} to the park
        c: take {} on a walk
        d: take {} to the beach
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", vet);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let tovet = format!("
{} goes to the vet.
{} is perfectly healthy.", name.trim(), name.trim());

            println!("{}", tovet);

            break;
        } else if action.trim().to_lowercase() == "b" {
            let ignore = format!("You ignored your {}'s health.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "c" {
            let ignore = format!("You ignored your {}'s health.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "d" {
            let ignore = format!("You ignored your {}'s health.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "e" {
            let ignore = format!("You ignored your {}'s health.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn beach(name: &String) {
    loop {
        let mut action = String::new();

        let beach = format!(r#"
{} needs to go to the beach! What do you do?
        
        a: take {} to the vet
        b: take {} to the park
        c: take {} on a walk
        d: take {} to the beach
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", beach);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "b" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "c" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "d" {
            let tobeach = format!("
{} goes to the beach.
{} is perfectly happy.", name.trim(), name.trim());

            println!("{}", tobeach);

            break;

        } else if action.trim().to_lowercase() == "e" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn walk(name: &String) {
    loop {
        let mut action = String::new();

        let walk = format!(r#"
{} needs to go on a walk! What do you do?
        
        a: take {} to the vet
        b: take {} to the park
        c: take {} on a walk
        d: take {} to the beach
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", walk);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "b" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "c" {
            let onwalk = format!("
{} goes on a walk.
{} is perfectly happy.", name.trim(), name.trim());

            println!("{}", onwalk);

            break;
        } else if action.trim().to_lowercase() == "d" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "e" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn park(name: &String) {
    loop {
        let mut action = String::new();

        let park = format!(r#"
{} needs to go to the park! What do you do?
        
        a: take {} to the vet
        b: take {} to the park
        c: take {} on a walk
        d: take {} to the beach
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", park);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "b" {
            let topark = format!("
{} goes to the park.
{} is perfectly happy.", name.trim(), name.trim());

            println!("{}", topark);

            break;

        } else if action.trim().to_lowercase() == "c" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);
        } else if action.trim().to_lowercase() == "d" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "e" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn random_event(kitty: &mut Kitty) {
    let event = rand::thread_rng().gen_range(1..=3);
    match event {
        1 => {
            println!("{} found a toy! Happiness increased by 5.", kitty.name);
            kitty.happiness += 5;
        }
        2 => {
            println!("{} got sick! Health decreased by 10.", kitty.name);
            kitty.health -= 10;
        }
        3 => {
            println!("{} had a great nap! Happiness increased by 3.", kitty.name);
            kitty.happiness += 3;
        }
        _ => (),
    }
    if kitty.health <= 0 {
        println!("{}'s health reached 0. Game over!", kitty.name);
        std::process::exit(0);
    }
}

fn shop(kitty: &mut Kitty) {
    loop {
        let mut action = String::new();

        let shop = format!(r#"
Welcome to the shop! What would you like to buy?
        
        a: Health potion (+20 health) - 10 playtime points
        b: Energy drink (+20 energy) - 10 playtime points
        c: Happiness treat (+10 happiness) - 5 playtime points
        d: Exit shop"#);

        println!("{}", shop);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        match action.trim().to_lowercase().as_str() {
            "a" => {
                if kitty.playtime >= 10 {
                    println!("You bought a health potion!");
                    kitty.health += 20;
                    kitty.playtime -= 10;
                } else {
                    println!("Not enough playtime points!");
                }
            }
            "b" => {
                if kitty.playtime >= 10 {
                    println!("You bought an energy drink!");
                    kitty.energy += 20;
                    kitty.playtime -= 10;
                } else {
                    println!("Not enough playtime points!");
                }
            }
            "c" => {
                if kitty.playtime >= 5 {
                    println!("You bought a happiness treat!");
                    kitty.happiness += 10;
                    kitty.playtime -= 5;
                } else {
                    println!("Not enough playtime points!");
                }
            }
            "d" => break,
            _ => println!("Invalid action"),
        }
    }
}