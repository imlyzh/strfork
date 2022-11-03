#![allow(non_upper_case_globals)]
pub mod ir;
pub mod bytecodes;
pub mod ir2bc;
pub mod direct_interpreter;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
