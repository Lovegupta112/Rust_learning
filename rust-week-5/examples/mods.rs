pub mod b {
    pub fn foo() {
        println!("This is module b..");
    }
}

pub mod a {

    use super::b;

    pub fn print() {
        println!("This is module a..");
    }

    pub fn print_foo() {
        b::foo();
    }
}

fn main() {
    a::print();
    a::print_foo();
}
