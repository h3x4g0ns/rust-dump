#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {}

fn birthday() {
    let age: u32 = 6;
    let voting_age: u32 = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Too young to vote"),
        Ordering::Greater => println!("Old enough to vote"),
        Ordering::Equal => println!("Old enough to vote"),
    }
}

fn age() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "20";
    let mut age: u32 = age.trim().parse().expect("age was not a number");
    age += 1;
    println!("i am {} and i want {} dollary doos", age, ONE_MIL);
}

fn greeting() {
    println!("ello, what is your name!");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("HHello {} {}", name.trim_end(), greeting);
}
