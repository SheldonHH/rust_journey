pub mod dog{
    pub fn hello(){
        println!("wang wang");
    }

    pub fn is_dog() -> bool{
        true
    }
}

pub mod cat{
    pub fn hello(){
        println!("miaomiao")
    }

    pub fn is_cat() -> bool{
        true
    }
}


//  cargo test