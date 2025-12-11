/* #[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr{
    address: String,
    kind: IpAddrKind,
}

impl IpAddr{
    fn new(address: String, kind: IpAddrKind) -> Self{
        Self{
            address,
            kind
        }
    }
}
fn main() {
   /*  route(IpAddr{
        address: String::from("127.0.0.1"),
        kind: IpAddrKind::V4,
    }); */


    let google_address= IpAddr::new(String::from("1.2.3.4.5"), IpAddrKind::V4);
    println!("Google Address: {:?}", google_address);
    let new_address= IpAddr::new(String::from("2001:db8::68"), IpAddrKind::V6);
    println!("New Address: {:?}", new_address);
}

fn route(ip: IpAddr){
    println!("Rounting {} to IP {:?}", ip.address, ip.kind);
}
 */



/* #[derive(Debug)]

enum IpAddrKind{
    V4(String),
    V6(u8, u8, u8),
}

fn main(){
    let home= IpAddrKind::V4(String::from("1,2,3.3.3"));
    let loopback= IpAddrKind::V6(12,34,56);
    route(home);
    route(loopback);
}

fn route(ip: IpAddrKind) {
    println!("Routing {:?} to IpAddrKind", ip);
} */


/* #[derive(Debug)]
struct IpAdd(String);
#[derive(Debug)]
struct IpV6(u8, u8, u8);

#[derive(Debug)]
enum IpAddr{
    V4(IpAdd),
    V6(IpV6),
}

fn main(){
    let check1= IpAddr::V4(IpAdd(String::from("Hello")));
    println!("{:?} ", check1);

    route(IpAddr::V6(IpV6(12,34,56)));
}


fn route(ip: IpAddr){
    println!("Routing {:?} to IP", ip);
} */


fn main(){
    let check= Some(1);
    println!("{:?}", check);

    let sum= check.unwrap() +1;
    println!("Sum: {}", sum);
}