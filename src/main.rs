extern crate rand;

use std::io;
// use std::cmp::Ordering;
// use rand::Rng;


fn main() {
    println!("Welcome to Knight Game!");
    println!("Would you like to go on an adventure? Y/N");
    
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)
        .expect("must be a string");
    println!("You chose: {}", answer);
    match answer.trim().to_uppercase().as_ref() {
        "Y" => play_game(),
        "N" => exit_game(),
        _ => bad_choice(),
    }
}

fn play_game() {
    println!("Playing game");

    struct Player {
        name: String,
        class: String,
        hp: u32,
        strenght: u32,
        speed: u32,
        dead: bool,
    };
    
}

fn exit_game() {
    println!("Exiting...");
}

fn bad_choice() {
    println!("You must choose Y or N");
    main();
}
