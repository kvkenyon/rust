use rust_learning::closures;
use rust_learning::dicts;
use rust_learning::error_handling;
use rust_learning::generics;
use rust_learning::iterators;
use rust_learning::lifetimes;
use rust_learning::reference_cycles;
use rust_learning::smart_pointers;
use rust_learning::traits;
use rust_learning::vectors;

fn main() {
    vectors::test();
    dicts::test();
    error_handling::test();
    generics::test();
    traits::test();
    lifetimes::test();
    closures::test();
    iterators::test();
    smart_pointers::test();
    reference_cycles::test();
}
