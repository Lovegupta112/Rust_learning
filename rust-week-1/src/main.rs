// use std::collections::HashMap;

// fn sum(a:u32,b:u32)->u32{
//     return a+b;
// }
// fn is_even(a:u32)->bool{
//     return a%2==0;
// }


// mod shape;
// use shape::Shape;
// struct  Rect{
//     width:u32,
//     height:u32
// }

// impl Rect{

//     fn area(&self)->u32{
//       return self.width * self.height;
//     }

//     fn print(a:u32){
//       println!("hello: {} ",a);
//     }
// }

// enum  Direction {
//     North,
//     South,
//     East,
//     West
// }

// fn find_direction(dir:Direction){
//     match dir {
//         Direction::East => println!("You moved to east!"),
//         Direction::North => println!("You moved to north!"),
//         Direction::West => println!("You moved to west!"),
//         _=>println!("You moved to south!")
//     }
// }

// use std::fs;

mod options;
use options::find_first_char;

fn main() {
    // println!("Hello, world!");
    // println!("{}",sum(2,3));
    // println!("{}",is_even(4));
    // let name= String::from("love");
    // println!("my first name is {}",name);

    // let mut a=5; //default immutable, manually specify mutable
    //  println!("a: {}",a);
    //  a=6;
    //  println!("a: {}",a);

    // ----------------------------------------------------------
    // Hasmap -------------
    // let mut student_grades=HashMap::new();

    // student_grades.insert("alice", 100);

    // let alice_balance= student_grades["alice"];

    // println!("{:?}",student_grades);
    // println!("{:?}",alice_balance);

    // let v=student_grades.entry("Trinity").or_insert(200);
    // println!("{:?}",v);
    // assert_eq!(*v,200);    

    //     let names = [String::from("Alice"),String::from("Bob")];

    //     for name in &names  {
    //                println!("{:?}", name);

    //     }

    //         println!("{:?}", names);

    // ----------------------------------------------------------------
    // struct ------
    //  let r=Rect{
    //     width:30,
    //     height:50
    //  };

    //  println!("{}",r.area());
    //  Rect::print(56);

    // --------------------------------------------
    // enum and pattern matching -----

    // let my_direction=Direction::South;
    
    // find_direction(my_direction);

    // let shape1=Shape::Circle(10.0);

    // // get_shape_area(shape1);
    //  println!("area of circle: {} ", shape1.get_shape_area());

    // ----------------------------------------------------------
    // error handling --------

    // let contents=fs::read_to_string("a.txt");

    // match contents {
    //     Ok(contents)=>println!("{}",contents),
    //     Err(e)=>println!("error while reading file")
    // }

    let ch='a';
    let str=String::from("hello");
    let res=find_first_char(&str,ch);
    
     match res {
        Some(index)=>println!("index of {} in {} is: {}",ch,str,index),
        None=>println!("{} not found in {}",ch,str)
     }

}



