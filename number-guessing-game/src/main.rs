use rand::{Rng, random, random_range, rng};
use std::io::stdin;

fn generate_rand_number() -> u32 {
    let val = random_range(1..100);
    val
}

fn compare_num(random_number: u32, user_option: u32) {
    if random_number == user_option {
        println!("You win!");
        println!(
            "System Number: {}, Your number:{} ",
            random_number, user_option
        );
    } else if random_number > user_option {
        println!("Too Low!");
        println!(
            "System Number: {}, Your number:{} ",
            random_number, user_option
        );
    } else {
        println!("Too High!");
        println!(
            "System Number: {}, Your number:{} ",
            random_number, user_option
        );
    }
}
fn main() {
    // let val1:u32=random();
    // let val2:u32=rng().random();
    // let val3=random_range(1..100);

    // print!("val1: {},\n val2: {},\n val3: {}",val1,val2,val3);

    let random_number = generate_rand_number();
    let mut input_str = String::from("");
    println!("Enter any random number from 1 to 100..");
    stdin()
        .read_line(&mut input_str)
        .expect("failed to read input");

    let user_option: u32 = input_str
        .trim()
        .parse()
        .expect("Please type a number from 1 - 100.");

    compare_num(random_number, user_option);

}
