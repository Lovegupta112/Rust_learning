use std::string;

use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug)]
struct Person{
    #[serde()]
    username:String,
    password:String
}

fn main() {

    let user2 = Person{
        username: String::from("itzlg"),
        password: String::from("1234")
    };

    let serialized_string=serde_json::to_string(&user2);

    let mut serialized_string_data=String::from("");

    match serialized_string  {
        Ok(content)=>{
             println!("{}",content);
             serialized_string_data= content;
        },
        Err(e)=> println!("something went wrong...")
    }

    println!("serialized_string_data: {}",serialized_string_data);
        let deserialized_string: Result<Person, serde_json::Error>=serde_json::from_str(&serialized_string_data);

         match deserialized_string {
             Ok(user_res)=>println!("{:?}",user_res),
             Err(e)=>println!("something went wrong...")
         }
    
    //Assignment:--
    //1- Serialize to Yaml
     let serialized_yml=serde_yml::to_string(&user2).unwrap();
     println!("serialized_yml: {}",serialized_yml);

     let deserialized_yml:Person=serde_yml::from_str(&serialized_yml).unwrap();
    println!("deserialized_yml: {:?}",deserialized_yml);


    let str1=String::from("hello");
    let ans;

    {
        let str2=String::from("world!");
        ans=longest_string(&str1,&str2);
    }

    println!("res: {}",ans);


}

fn longest_string<'a,'b>(s1:&'a String,s2:&'b String) -> &'a String{
    return s1;
}
