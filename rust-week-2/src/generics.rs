use std::ops::{Add,Mul};
pub struct Rect<T>{
   pub width: T,
   pub height: T
}

impl<T:Mul<Output = T> + Copy> Rect<T>{
 pub fn area(&self)->T{
     self.width * self.height
  }
}

// generics & trait bound ---

pub fn sum<T:Add<Output = T>>(a:T,b:T)->T{
    a+b
}

pub fn print_variable<T:std::fmt::Display>(var:T){
  println!("{}",var);
}