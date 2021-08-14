#![feature(test)]

mod naive;
mod compile;
use std::{thread::sleep, time::Duration};

pub trait PassPosition {}

pub struct Beginning;
impl PassPosition for Beginning {}

pub struct Middle;
impl PassPosition for Middle {}

pub struct End;
impl PassPosition for End {}

pub struct Singular;
impl PassPosition for Singular {}

pub trait DrawPass<P: PassPosition> {
    // Simplified
    fn render(&self);
}

struct One;
impl<P: PassPosition> DrawPass<P> for One {
    fn render(&self) {
        println!("1 {}", std::any::type_name::<P>());
    }
}
struct Two;
impl<P: PassPosition> DrawPass<P> for Two {
    fn render(&self) {
        println!("2 {}", std::any::type_name::<P>());
    }
}
struct Three;
impl<P: PassPosition> DrawPass<P> for Three {
    fn render(&self) {
        println!("3 {}", std::any::type_name::<P>());
    }
}
struct Four;
impl<P: PassPosition> DrawPass<P> for Four {
    fn render(&self) {
        println!("4 {}", std::any::type_name::<P>());
    }
}