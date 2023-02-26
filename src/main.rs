// Using the Rng struct (or enum)
extern crate rand;
use rand::Rng;

fn main()
{
    // Creating the decider for the coin flip
    let mut decider = rand::thread_rng().gen_weighted_bool(2); // We put the .gen_weighted_bool so there is a 1/2 chance for heads (aka true)

    // If it flips on true its heads, and if it flips on false its tails
    if decider == true
    {
        println!("The coin flipped on: heads");
    }
    else 
    {
        println!("The coin flipped on: tails");    
    }
}