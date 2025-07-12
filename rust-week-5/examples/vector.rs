fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("vector v: {:?}", v);

    println!("while initialization setting value in vector : -----");

    let v = vec![1, 2, 3];
    println!("vector v: {:?}", v);

    let v: Vec<i8> = vec![1, 2, 3];
    println!("vector v: {:?}", v);

    let v = vec![1u8, 2, 3];
    println!("vector v: {:?}", v);

    println!("setting all equal value 0 in vector : -----");

    let v = vec![0i8; 20];
    println!("vector v: {:?}", v);

    //  operations --------

    //   get---
    println!("v19: {}", v[19]);
    // println!("v21: {}",v[21]); //throw error and crashed  bcz 21 index not present ---

    // better function for handling error and getting value---
    let res = v.get(21);

    println!("v21: {:?}", res);

    //update -------
    let mut v: Vec<i8> = vec![1, 3, 5];
    v[1] = 7;

    //pop (remove last elm) -----
    let mut v: Vec<i8> = vec![1, 3, 5];
    let res = v.pop();
    println!("pop: {:?}, updated vector v: {:?}", res, v);

    // slice: -----

    let s = &v[0..2];
    println!("slice: {:?}", s);
}
