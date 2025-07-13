fn main() {
    let x = 2;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("four"),
    }
    match x {
        1 | 2 | 3 => println!("one or two or three "),
        _ => println!("four"),
    }

    let x = 10;
    match x {
        //    1..10 => println!("one to ten.. "), //excluded 10
        //    1..=10 => println!("one to ten.. "), //included 10
        i @ 1..=10 => println!("one to ten.. {i} matched "), //if match occur asign that value in i
        _ => println!("above  than 10 or below than 1 "),
    }

    let res: Result<i32, String> = Ok(231);
    // let res:Result<i32,String>=Err(String::from("error coming.."));

    match res {
        Ok(val) => println!("value is {val}"),
        Err(e) => println!("something went wrong, {e}"),
    }

    let x: Option<i32> = Some(5);
    let x: Option<i32> = None;

    match x {
        Some(val) => println!("value is {val}"),
        None => println!("something went wrong"),
    }

    // we can also return value -------

    let val = match x {
        Some(data) => data,
        None => 0,
    };

    println!("return value  is {val}");
}
