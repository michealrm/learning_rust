use std::{cmp::min, cmp::*};
use crate::modtest; // globs, either * or function/module, not both

fn func() -> i32 {
    {5} // blocks are expressions
}

fn expr() -> bool {
    let feeling_lucky = true;
    let guess = 5;
    // if/else blocks are expressions
    if feeling_lucky {
        // match/case is an expression as well
        match guess {
            5 => true,
            _ => false,
        }
    }
    else {
        false
    }
}

// i.e. strings have their own namespace that you can perform ops with
fn types_are_namespaces_too() {
    let x = "amos".len();
    let x = str::len("amos");
    let v: Vec<f64> = Vec::new(); // because every module has use std::prelude::v1::*;
    let v: Vec<f64> = std::vec::Vec::new(); // this is full namespace
}

struct Vec2 {
    x: f64, // 64-bit floating point, aka "double precision"
    y: f64,
}

fn struct_ops() {
    let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { y: 2.0, x: 4.0 };
    let v3 = Vec2 {
        x: 14.0,
        ..v2 // inits rest of fields - has to be in last position
    };
    let v4 = Vec2 { ..v3 }; // or all of fields
}

fn deconstr() {
    let v = Vec2 { x: 3.0, y: 6.0 };
    let Vec2 { x, y } = v;
    // `x` is now 3.0, `y` is now `6.0`
    let Vec2 { x, .. } = v;
    // this throws away `v.y`
}

fn if_let() {
    let vec: Vec2 = Vec2 {x: 4.0, y: 1.0};
    if let Vec2 {x: 4.0, y} = vec {
        // x must be 4, y can be anything
        // we get both variables
        print!("{}{}", vec.x, y);
    }
}

struct Vec2Test {
    a: f64,
    b: i32,
}

impl Vec2Test {
    fn is_something(&self) -> bool {
        return  self.a == 1.0 && self.b == 4;
    }
}

pub(crate) fn pattern_test() {
    let vec: Vec2Test = Vec2Test {b: 1, a: 1.0 };
    if let Vec2Test {b: 4, a} = vec {
        // b must be 4, a can be anything
        print!("{}", a);
    }
}

fn mut_example() {
    let mut a = 5;
    let mut b = Vec2 { x: 0.0, y: 0.0 };
    a = 10;
    b.x = 1.0;
    b.y = 2.0;
}