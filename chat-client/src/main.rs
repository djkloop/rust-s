use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::{self, TryRecvError},
    thread,
    time::Duration,
};

const MSG_SIZE: usize = 1024;
const LOCAL_SERVER: &str = "127.0.0.1:8822";

fn main() {
    let mut client = TcpStream::connect(LOCAL_SERVER).expect("connect failed");
    client
        .set_nonblocking(true)
        .expect("failed to non-blocking");
    let (tx, rx) = mpsc::channel::<String>();
    // 开子线程
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg_string = String::from_utf8(msg.clone()).unwrap();
                println!("recv: {}", msg_string);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("server down");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("msg send -> {:?}", msg);
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {}
        }
        thread::sleep(Duration::from_millis(100));
    });
    println!("msg:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == "quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("offline");
}
