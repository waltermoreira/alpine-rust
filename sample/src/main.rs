fn main() {
    let ctx = zmq::Context::new();
    let sock = ctx.socket(zmq::REP).unwrap();
    sock.bind("tcp://*:8989").unwrap();
    println!("Send a message to port 8989 to get a response");
    let s = sock.recv_string(0).unwrap().unwrap();
    sock.send(&format!("Hello, {s}"), 0).unwrap();
    println!("Check your socket for the response");
}
