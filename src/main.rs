pub mod dicts;
pub mod error_handling;
pub mod generics;
pub mod lifetimes;
pub mod spreadsheet_cell;
pub mod testing;
pub mod traits;
pub mod vectors;

fn main() {
    vectors::test();
    dicts::test();
    error_handling::test();
    generics::test();
    traits::test();
    lifetimes::test();
}
