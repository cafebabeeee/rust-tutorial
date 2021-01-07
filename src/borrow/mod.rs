pub mod borrow;
#[cfg(test)]
mod tests{
    use super::borrow;
    #[test]
    fn compute() {
        let a = 15;
        let mut b = 3;
        borrow::compute(&a, &mut b);
        println!(" b value is {}", b)
    }

    #[test]
    fn borrow_fun() {
        let string = String::from("hello,rust");

        let len = borrow::calculate_length(&string);

        println!("The length is {}.", len);

        //borrow::borrow::change(&mut string);
    }

}