#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn rust_add(a: i32, b: i32) -> i32 {
    let result = unsafe{add(a,b)};
    return result;
}

fn rust_sub(a: i32, b: i32) -> i32 {
    let result = unsafe{subtract(a,b)};
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_it() {
        let result = rust_add(12, 3);
        println!("{}", result);
        assert_eq!(result, 15);
    }
    #[test]
    fn test_it_2() {
        let result = rust_sub(12, 3);
        println!("{}", result);
        assert_eq!(result, 9);
    }
}
