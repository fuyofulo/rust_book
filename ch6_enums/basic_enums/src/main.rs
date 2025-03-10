#[derive(Debug)]
enum IpAddrKind {
    V4(u8),
    V6(u8)
}

fn main() {
    
    let addr1 = IpAddrKind::V4(12);
    let addr2 = IpAddrKind::V6(13);


    println!("{:?}", addr1);
    println!("{:?}", addr2);
}
