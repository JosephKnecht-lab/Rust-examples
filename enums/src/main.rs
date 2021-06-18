#[derive(Debug)]



enum IpAddrKind {
    V4,
    V6,
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Enums value: {:?}{:?}", four, six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

}

fn route(ip_kind: IpAddrKind) {}



