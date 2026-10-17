use gw_learned::demo;

fn main() {
    let (offered, body) = demo();
    println!("offered: {offered}");
    println!("code: {body}");
}
