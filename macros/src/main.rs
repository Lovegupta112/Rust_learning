
trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

struct Swap {
    qty_1:u32,
    qty_2:u32

}

impl Serialize for Swap{
    fn serialize(&self) -> Vec<u8> {
        let  mut v=vec![];

        // v.append(self.qty_1.to_ne_bytes());
        // v.append(self.qty_2.to_be_bytes());
        return v;
    }
}
fn main() {
    // println!("Hello, world!");
     let s=Swap{
        qty_1:11,
        qty_2:12
     };
    let res= s.serialize();
    println!("v: {:?}",res)
}
