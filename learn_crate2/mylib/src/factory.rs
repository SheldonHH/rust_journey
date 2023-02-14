mod factory{
    // 控制私有性和作用域
    // mod produce_refrigerator { // 私有报错
    //     fn produce_re(){
    //         println!("produce refrigerator!");
    //     }
    // }

    pub mod produce_refrigerator { // 私有报错
        pub fn produce_re(){
            println!("produce refrigerator!");
        }
    }
    mod produce_washing_machine{
        fn produce_washing_machine(){
            println!("produce washing machine!");
        }
    }
}

