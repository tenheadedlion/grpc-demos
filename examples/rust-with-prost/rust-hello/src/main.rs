pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

pub fn create_hello_req(name: String) -> hello::HelloRequest {
    hello::HelloRequest {
        name: name
    }
}

fn main() {
    let req = create_hello_req("you".to_string());
    dbg!(req);
}