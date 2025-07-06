pub fn find_first_char(str:&String,ch:char)->Option<u32>{
   
   let mut index=0;
   
    for c in str.chars() {
        if c==ch {
            index=index+1;
            return Some(index);
        }
    }
    
    None
}

