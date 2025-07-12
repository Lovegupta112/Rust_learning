// Declerative macros --------
macro_rules! say_hello {
  ()=>{
    println!("Hello, world!");
  }
}

// procedural macros -------

#[derive(Debug)]
struct User{
    username:String,
    password:String,
    age:u32
}


#[derive(Debug,Clone)]
struct Person{
  is_male:bool,
  age:u32,
  name:String
}


fn main() {
    say_hello!();


   let u=User{
       username:String::from("itzlg"),
       password:String::from("lg1234"),
       age:23
   };

   let a=Person{
    is_male:true,
    age:23,
    name:String::from("itzlg")
   };

   let b=&a;

   
   println!("{:?},{:?},{:?}",u,a,b);
}
