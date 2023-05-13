fn print_array() -> String {
   "Hello, world!".to_string()
}

#[cfg(test)]
mod test{
    use super::print_array;
    #[test]
    fn valid_array_test(){
        let result = print_array();
        assert_eq!(result, "Hello, world!".to_string());
    }
}