use mio::net::{TcpListener, TcpStream};
use mio::{Events, Poll, Interest, Token};
use std::{io::Result as ioResult, net::SocketAddr};
use std::env;

const SERVER:Token = Token(0);
const CLIENT:Token = Token(1);
fn main() -> ioResult<()>{
    let net_interface = env::args().nth(0);
    //assert_eq!(net_interface, None);
    println!("os net interface  {:?}", net_interface.unwrap());
    let mut poll:Poll = Poll::new()?;
    let mut event: Events = Events::with_capacity(128);
    let addr: SocketAddr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let mut server: TcpListener = TcpListener::bind(addr)?;
    poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

    let mut client = TcpStream::connect(addr)?;

    poll.registry().register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    loop {
        poll.poll(&mut event, None)?;

        for e in event.iter() {
            match e.token() {
                SERVER => {
                    let connection = server.accept()?;
                    println!("SERVER recive a connection.");
                    drop(connection);
                }
                CLIENT => {
                    if e.is_readable() {
                        println!("CLIENT read Event");
                    }
                    if e.is_writable() {
                        println!("CLIENT write Event");     
                    }
                    return  Ok(())
                }
                _ => unreachable!()
            }
        }
    }
}
