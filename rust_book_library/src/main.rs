
#[derive(Debug)]
 struct Book{
     title:String,
     author:String,
     available:bool,
 }

 impl Book {
   
   pub fn new(title:String,author:String)->Self{
     Book{
      title,
      author,
      available:true
     }
   }

   pub fn borrow(&mut self){
    self.available=false;
   }

   pub fn return_book(&mut self){
     self.available=true;
   }
   pub fn is_available(&self)->bool{
     self.available
   }



 }
fn main() {

  let mut book1=Book::new( String::from("Science"),
     String::from("ananoymous"));

  let is_available=book1.is_available();
  println!("{} is available: {}",book1.title,is_available);
  
  book1.borrow();
  let is_available=book1.is_available();
  println!("{} is available: {}",book1.title,is_available);

  book1.return_book();
  let is_available=book1.is_available();
  println!("{} is available: {}",book1.title,is_available);


}
