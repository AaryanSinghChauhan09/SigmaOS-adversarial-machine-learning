//! SigmaOS-adversarial-machine-learning — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_adversarial_machine_learning_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-adversarial-machine-learning: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-adversarial-machine-learning: ok");
}
