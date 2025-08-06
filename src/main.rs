use rust_learning::async_await;
use rust_learning::closures;
use rust_learning::dicts;
use rust_learning::error_handling;
use rust_learning::generics;
use rust_learning::iterators;
use rust_learning::lifetimes;
use rust_learning::oop;
use rust_learning::reference_cycles;
use rust_learning::smart_pointers;
use rust_learning::threads;
use rust_learning::traits;
use rust_learning::vectors;
use trpl;

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
    threads::test();
    trpl::run(async_await::test());
    oop::test();
}
