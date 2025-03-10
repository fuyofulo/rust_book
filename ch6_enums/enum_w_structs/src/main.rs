use std::fmt::Debug;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}


fn main() {
    
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let away = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("1.2.3.4.5"),
    };

    
    println!("home: {:?}", home);
    println!("away: {:?}", away);

}
