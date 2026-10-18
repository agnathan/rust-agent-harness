use harness_core::stream::demo_reply;

fn main() {
    let (reply, started) = demo_reply();
    println!("reply: {reply}");
    println!("started: {}", started.join(","));
}
