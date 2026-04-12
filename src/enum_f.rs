enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
        
    }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
        fn call(&self) {
            // 메서드 본문이 여기 정의될 것입니다
            match self {
                Message::Write(text) => print!("{}", text),
                _ => {}
            }
        }
    }

struct IpAddrs {
    kind : IpAddrKind,
    address : String,
}

pub fn enum_file() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    let home = IpAddrs{
        kind : IpAddrKind::V4,
        address : String::from("127.0.0.1"),
    };
    

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    print!("{:?}",loopback);
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}