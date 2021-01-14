pub struct PrintDrop(pub &'static str);
impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Droping {}.", self.0);
    }
}
