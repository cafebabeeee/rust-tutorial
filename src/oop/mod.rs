 pub mod color;
 pub mod drop_order;
/// algebraic data type
/// struct enum
/// ```
/// #[derive(Debug, Clone, Copy)]
/// struct Book {
///     // this field does not implement `Copy`rustcE0204
///     name: String,
///     isbn :i32,
///     version: i32,
/// }
/// ```

#[derive(Debug, Clone, Copy)]
struct Book<'a> {
    name: &'a str,
    isbn :i32,
    version: i32,
}

 #[cfg(test)]
 mod tests {
    use crate::oop::Book;
    use crate::oop::drop_order::PrintDrop;
    use crate::oop::color::Colorized;
    
    #[test]
    fn struct_update() {
        let book = Book {
            name: "Rust program",
            isbn: 123456,
            version: 1,
        };
        let book1 = Book {
            version: 2,
            .. book
        };
        println!("{:?}", book);
        println!("{:?}", book1);
    }

    #[test]
    fn color_console() {
        let hi = "Hello".yellow().on_blue();
        println!("{}", hi);
    }   
    
    #[test]
    fn drop() {
        // console will print:
        //  Droping y.
        //  Droping x.
        let x = PrintDrop("x");
        let y = PrintDrop("y");
        // Droping b.
        // Droping a.
        // Droping x.
        // Droping y.
        // Droping z.
        let t1= (PrintDrop("x"), PrintDrop("y"), PrintDrop("z"));
        let t2= (PrintDrop("a"), PrintDrop("b"), panic!());
    }
}