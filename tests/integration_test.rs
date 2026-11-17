// Integration test for SigmaOS-vit
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_vit as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
