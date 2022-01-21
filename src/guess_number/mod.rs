pub mod guess;

#[cfg(test)]
mod tests {
    use super::guess;
    #[test]
    fn guess_test() {
        guess::guss();
    }
}
