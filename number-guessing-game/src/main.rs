use std::io::stdin;
use rand::{random, random_range, rng, Rng};


fn main() {
    
    // let val1:u32=random();
    // let val2:u32=rng().random();
     let mut inputStr=String::new();
     println!("Enter any random number from 1 to 100..");
     let val3=random_range(1..100);
      let input=stdin().read_line(&mut inputStr);

      match input{
        Ok(data)=>println!("val3: {}, input:{} ",val3,data),
        Err(error) => println!("error: {error}"),
      }

    // print!("val1: {},\n val2: {},\n val3: {}",val1,val2,val3);


    
}
