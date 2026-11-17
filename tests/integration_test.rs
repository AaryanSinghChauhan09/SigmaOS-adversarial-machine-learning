// Integration test for SigmaOS-adversarial-machine-learning
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_adversarial_machine_learning as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
