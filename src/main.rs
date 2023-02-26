// Using the Rng struct (or enum)
extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::process::Command;

enum Side
{
    Heads = 1,
    Tails = 0
}

fn main()
{
    // Getting the ASCII art for heads
    let mut heads_file = File::open("heads.txt").expect("Cannot open heads.txt!");
    let mut heads_art = String::new();
    heads_file.read_to_string(&mut heads_art).expect("Cannot read from heads.txt");

    // Getting the ASCII art for tails
    let mut tails_file = File::open("tails.txt").expect("Cannot open tails.txt!");
    let mut tails_art = String::new();
    tails_file.read_to_string(&mut tails_art).expect("Cannot read from tails.txt");

    // Creating the decider for the coin flip
    let decider = rand::thread_rng().gen_weighted_bool(2); // We put the 2 in the .gen_weighted_bool function so there is a 1/2 chance for heads (aka true)

    let heads = Side::Heads;
    let tails = Side::Tails;

    // Putting the animation on the screen
    clear();
    animation(&heads_art, &tails_art);

    // If it flips on true its heads, and if it flips on false its tails
    if decider // == true
    {
        println!("{}\n\n", heads_art);
        println!("The coin flipped on: heads");
    }
    else // decider == false
    {
        println!("{}\n\n", tails_art);
        println!("The coin flipped on: tails");    
    }
}

fn animation(art1:&String, art2:&String)
{
    for i in 1..11
    {
        println!("{}", art1);
        thread::sleep(Duration::from_secs_f32(0.10));
        clear();
        println!("{}", art2);
        thread::sleep(Duration::from_secs_f32(0.10));
        clear();
    }
}

fn clear()
{
    // Clear the console screen
    if cfg!(windows) {
        let _ = Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status();
    } else {
        let _ = Command::new("clear")
            .status();
    }
}