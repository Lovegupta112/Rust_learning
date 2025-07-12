use std::{i32, u32};

fn main() {
    let i: i32 = -1;
    let u: u32 = i as u32;

    println!("i32: {i} to u32: {u}");

    let iMax = i32::MAX;
    let uMin = u32::MIN;

    println!("iMax: {iMax} , uMin: {uMin}");
}
