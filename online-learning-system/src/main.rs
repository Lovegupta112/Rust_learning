trait Course {
    fn get_overview(&self)->String;
}
pub struct Workshop {
   pub title:String,
   pub instructor:String,
   pub duration:u16
}
pub struct Seminar{
   pub title:String,
    speaker:String,
    location:String
}
impl Course for Workshop{
  fn get_overview(&self)->String{
//    self.title+self.instructor+self.duration
   format!("title: {}, instructor: {}, duration: {}",self.title,self.instructor,self.duration)
}
}
impl Course for Seminar{
  fn get_overview(&self)->String{
    // self.title+self.speaker+self.location 
     format!("title: {}, speaker: {}, location: {}",self.title,self.speaker,self.location)
  }
}
fn print_course_overview<T : Course>(course:T){
    let res= course.get_overview();
     println!("{res}");
}
fn main() {
      let workshop1=Workshop{title:"hi".to_string(),instructor:"raj".to_string(),duration:12};
      let s1=Seminar{title:"s1".to_string(),speaker:"raj" .to_string(),location:"bangalore".to_string()};

     print_course_overview(workshop1);
     print_course_overview(s1); 
     
    
 }


