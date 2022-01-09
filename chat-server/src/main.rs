use std::{
    io::{ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
    time::Duration,
};

const MSG_SIZE: usize = 1024;
const LOCAL_SERVER: &str = "127.0.0.1:8822";

fn main() {
    // 建立一个 tcp 服务
    let server = TcpListener::bind(LOCAL_SERVER).expect("listen failed");
    //
    server.set_nonblocking(true).expect("failed no nonblocking");
    // 建立一个 channel
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        // socket监听逻辑
        if let Ok((mut socket, addr)) = server.accept() {
            print!("{} connected", addr);
            clients.push(socket.try_clone().expect("failed to clone client"));
            let tx = tx.clone();
            // 开子线程
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        // 接收到消息，然后给所有的客户端发送
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg_string = String::from_utf8(msg).expect("Invalid utf8 message");
                        println!("{}: {:?}", addr, msg_string);
                        tx.send(msg_string).expect("Faild send message");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with {}", addr);
                        break;
                    }
                }
                // 睡一下
                thread::sleep(Duration::from_millis(100));
            });
        }
        // 拿到子线程的消息
        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    // 把buff重置一下
                    buff.resize(MSG_SIZE, 0);
                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }
        thread::sleep(Duration::from_millis(100));
    }
}
