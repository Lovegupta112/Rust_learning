
fn main() {
     let add_one=|x:i32|x+1;

     let res=add_one(3);
     println!("7..{}",res);
//    ----------------------------------------      
     let mut counter=0;

     let mut increase_counter=|| {
        counter=counter+1;
     };

     increase_counter();
      increase_counter();
      increase_counter();
     increase_counter();
     println!("counter: {counter}");


    //  ------------------------------------

    let x=String::from("hello world");

    let consume_and_return_x=||x;

    let y=consume_and_return_x();

    // println!("x: {x}"); //show error as y is owner 
    println!("y: {y}"); 

    // closures with vector --------

    let vec=vec![1,2,3];
    
    let double_vec:Vec<i32>=vec.into_iter().map(|x|x*2).collect();

    print!("double_vec: {:?}",double_vec);

    let vec1=vec![1,2,3,4];
    let filter_even:Vec<&i32>=vec1.iter().filter(|x|*x%2==0).collect();
    let vec_sum=vec1.iter().copied().reduce(|acc,c|acc+c).unwrap();
    println!("filter even vec: {:?}, sum of vec: {}",filter_even,vec_sum)
}