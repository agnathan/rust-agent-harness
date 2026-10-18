use harness_core::{run_saved_evals, run_stress_evals};

#[tokio::main]
async fn main() {
    for score in run_saved_evals().await {
        println!(
            "eval: {} {}",
            score.name,
            if score.passed { "pass" } else { "fail" }
        );
    }
    for score in run_stress_evals().await {
        println!(
            "eval: {} {}",
            score.name,
            if score.passed { "pass" } else { "fail" }
        );
    }
}
