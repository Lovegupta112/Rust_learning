fn return_many() -> (u32, String) {
    (240, String::from("hello world"))
}

fn main() {
    let tuple1 = ("ritik", 123, true);
    println!("tuple1: {:?}", tuple1);
    let tuple2 = (("ritik", 123, true), ("zane", 32), "hello world", ());
    println!(
        "nested.0.1: {} ,nested.1.1: {}, nested.3: {:?}",
        tuple2.0.1, tuple2.1.1, tuple2.3
    );

    // destructuring a tuple
    let (username, num, isValid) = tuple1;

    println!("{username},{num},{isValid}");

    // Partial destructuring (ignore first and last values)

    let (_, b, _) = tuple1;

    let tuple1 = ();

    println!("b: {b}, tuple1: {:?}", tuple1);

    let (a, b) = return_many();

    println!("a: {a}, b: {b}");
}
