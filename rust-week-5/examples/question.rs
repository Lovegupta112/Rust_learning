fn f1() -> Result<u32, String> {
    Ok(5)
}
fn f2() -> Result<u32, String> {
    Ok(6)
}

fn sum_f1_f2() -> Result<u32, String> {
    let v1 = f1()?;
    let v2 = f2()?;

    Ok(v1 + v2)
}

fn main() {
    let res = sum_f1_f2();
    println!("res sum: {:?}", res);
}
