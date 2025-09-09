use std::{i32, u32};

#[derive(Clone)]
struct User{
    first:String,
    second:String,
    age:u32
}
impl  User{
   pub fn hello(&self)->String{
    let val=&self.first;
      self.first.to_string()
   }
}
fn main() {
    let i: i32 = -1;
    let u: u32 = i as u32;

    println!("i32: {i} to u32: {u}");

    let iMax = i32::MAX;
    let uMin = u32::MIN;

    println!("iMax: {iMax} , uMin: {uMin}");


    let us=User{
        first:String::from("itz"),
        second:String::from("lg"),
        age:21
    };

    let us2=us.clone();
    print!("hii,,{},us2: {:?}",us.hello(),us2.hello());
    
}
