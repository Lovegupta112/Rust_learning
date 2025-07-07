use chrono::{Utc,Local};
use dotenv::dotenv;
use std::env;

mod generics;
use generics::{Rect,print_variable,sum};

mod traits;

fn main() {

    let utc_date= Utc::now();
    let local_date= Local::now();
    println!("utc Date: {}, local Date: {}",utc_date,local_date);

    dotenv().ok();

    let redis_url=env::var("REDIS_ADDRESS");

    match redis_url {
        Ok(content)=>println!("Redis address: {} ",content),
        Err(e)=>println!("Error while reading redis address from env, Error: {}",e)
    }

    // unwrap is good if we sure there should be always value or will get result and 
    // if that doesn't exist app should not run( should crash) so at starting we are checking 

    let db_url=env::var("DATABASE_URL").unwrap();

    println!("DB url: {}",db_url);

    // generics & trait bound ---
    println!("sum of u32: {} and f32: {}",sum(20,40),sum(20.8, 40.2));
    print_variable(3.0);
    print_variable(50);
    print_variable(String::from("hello"));


    let rect1=Rect{
        width:12,
        height:13
    };

    println!("area of w: 12, h: 13 is: {} ",rect1.area());
    println!("area of w: 3.5, h: 5.6 is: {} ",Rect{width:3.5,height:5.6}.area());

    
    let rect2=traits::Rect{
        width:20.0,
        height:30.0
    };

    // println!("area of rect is {} ",rect2.area());

    let circle=traits::Circle{
     radius:10.0
    };

    // println!("area of circle is {} ",circle.area());

    traits::print_shape_area(rect2);
    traits::print_shape_area(circle);
    
}

