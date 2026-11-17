// Integration test for SigmaOS-arch-optimization
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_optimization as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
