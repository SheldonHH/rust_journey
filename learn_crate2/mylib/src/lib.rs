pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 模块声明
pub mod factory;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
