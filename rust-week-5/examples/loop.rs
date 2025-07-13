fn main() {
    let mut i = 0;
    loop {
        println!("loop..");
        i += 1;
        if i > 5 {
            break;
        }
    }

    // we can also return some value from loop ---

    let val = loop {
        println!("loop second time ..");
        i += 1;
        if i > 5 {
            break "loop ends here";
        }
    };

    println!("loop return value: {}", val);

    // 2nd way---

    let mut i = 0;

    while i <= 5 {
        println!("while loop...");
        i += 1;
    }

    for i in 0..6 {
        println!("for loop ...");
    }

    let arr = [1, 2, 3, 4, 5];
    let arrLen = arr.len();

    for i in 0..arrLen {
        println!("arr index:{}, value:{}", i, arr[i]);
    }
    //  for i in arr{
    //      println!("arr index:{}, value:{}",i,arr[i]);
    //  }

    let v = vec![1, 2, 3, 5];

    for i in v.iter() {
        println!("for loop for vector :{}", i);
    }
}
