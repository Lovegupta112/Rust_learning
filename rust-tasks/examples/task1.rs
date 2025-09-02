#![allow(unexpected_cfgs)]

pub fn find_largest(slice:&[i32])->Option<i32>{

    // solution 1 ---
    let mut max_value=i32::MIN;
     
    for &val in slice{

        if max_value < val {
          max_value=val;
        }
    }

    if slice.len() <=0 {
        None
    }
    else {
       Some(max_value) 
    }

    // ------------------------------------------------------  

    // solution 2 ----
    
    // slice.iter().max().copied()


}

fn main(){
    
    let numbers=[1,3,7,3,5];
    let res=find_largest(&numbers);

    assert_eq!(res,Some(7),"Couldn't find the largest!");
    println!("Successfully found the largest!")

}