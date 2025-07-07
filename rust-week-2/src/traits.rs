use std::f32::consts::PI;

pub trait  Shape {
  fn area(&self) -> f32;
}
pub struct Rect{
  pub width:f32,
  pub height:f32
}

impl Shape for Rect{

  fn area(&self)->f32{
       self.width * self.height
    }
}

pub struct Circle{
    pub radius:f32
}

impl Shape for Circle{

    fn area(&self)->f32{
        PI*self.radius*self.radius
     }
}

pub fn print_shape_area<T:Shape>(s:T){
  println!("{}",s.area())
}