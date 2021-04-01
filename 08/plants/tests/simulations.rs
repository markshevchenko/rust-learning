extern crate plants;

use plants::{Fern, run_simulation};

#[test]
fn run_simulation_with_1_day() {
    let mut fern = Fern { size: 1.0, growth_rate: 0.1 };

    run_simulation(&mut fern, 1);

    assert_eq!(1.1, fern.size);
}