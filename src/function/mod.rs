pub mod functions;
pub mod higherorderfunction;
pub mod closure;

#[cfg(test)]
mod function_test{
    use super::closure;
    #[test]
    fn closure() {
        closure::closure();
    }
}