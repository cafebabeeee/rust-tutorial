fn main() {
    // let four = IPAddrKind :: V4;
    // let six = IPAddrKind :: V6;

    let localhost = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{:?}", localhost);
}

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,                       // has no data associated with it at all
    Move { x: i32, y: i32 },    // includes an anonymous struct inside it
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

struct QuitMesseage;
struct MoveMesseage {
    x: i32,
    y: i32,
}
struct WriteMesseage(String);
struct ChangeColorMesseage(i32, i32, i32);
