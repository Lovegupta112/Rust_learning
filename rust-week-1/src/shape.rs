use std::f32::consts::PI;

pub enum Shape{
    Square(f32),
    Circle(f32),
    Rectangle(f32,f32)
}


// pub fn get_shape_area(shape:Shape){

    // match shape{
    //     Shape::Square(side)=>println!("area of sq is {}",side*side),
    //     Shape::Circle(radius)=>println!("area of circle is {}",PI*radius*radius),
    //     Shape::Rectangle(width,height )=>println!("area of rectangle is {}",width*height),
    // }

// }

// or can implement function in enum 

 impl Shape{

  pub fn get_shape_area(&self)->f32{
       return match self{
        Shape::Square(side)=>side*side,
        Shape::Circle(radius)=>PI*radius*radius,
        Shape::Rectangle(width,height )=>width*height,
    }
    }
}