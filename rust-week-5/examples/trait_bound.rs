trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for i32 {}

fn a<T: A>(val: T) {}
fn ab<T: A+B>(val: T) {}
fn main() {
    let val1: u32 = 1;
    let val2: i32 = 1;

    // a(val1);
}
