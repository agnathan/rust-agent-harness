use providers::{run_turn, FakeModel, Pool};

fn main() {
    let mut pool = Pool::new(vec!["sk-one".into(), "sk-two".into()]);
    let _ = pool.mark_exhausted_and_rotate();
    let (reply, _route) = run_turn(
        &[
            FakeModel {
                name: "primary".into(),
                reply: "no".into(),
            },
            FakeModel {
                name: "fallback".into(),
                reply: "pong".into(),
            },
        ],
        true,
    );
    println!("reply: {reply}");
    println!("client: {}", pool.client_id());
}
