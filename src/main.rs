mod thmins;
mod modtest;
mod callme_modtest;

use crate::modtest::gcd::fakeMain as gcdMain;

fn main() {
    let random_number_from_crate: f64 = rand::random();
    println!(":o T/generic resolved to f64 :3 {}", random_number_from_crate);
    thmins::pattern_test();
    modtest::gcd::gcd(5, 2);
    gcdMain();
}
