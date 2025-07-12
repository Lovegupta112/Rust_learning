fn main() {
    // initializing a array with fixed size of 3 --
    let arr: [i32; 3] = [1, 2, -3];

    println!("arr[0]={}", arr[0]);

    // mutating a array -----

    let mut arr: [i32; 3] = [20, 50, -30];

    arr[0] = 40;
    println!("arr={:?}", arr);

    // initializing all elements to 0

    let arr: [i32; 10] = [0; 10];
    println!("arr={:?},length={}", arr, arr.len());

    let nums: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Slice - size not known at compile time

    // first 3
    let s: &[i32] = &nums[0..3]; //or &nums[...3];
    println!("first s={:?},length={}", s, s.len());

    // last 3
    // let s:&[i32]= &nums[7..10]; //or
    let s: &[i32] = &nums[nums.len() - 3..nums.len()];
    println!("last s={:?},length={}", s, s.len());

    //  medium 3
    let s: &[i32] = &nums[4..7];
    println!("medium s={:?},length={}", s, s.len());

    let s: &[i32] = &nums[..];
    println!("all s={:?},length={}", s, s.len());
}
