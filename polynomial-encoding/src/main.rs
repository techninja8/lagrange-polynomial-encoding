#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod polynomial;
mod gf256;

use crate::polynomial::Polynomial;
use crate::gf256::GF256;
// use crate::gf256::*;
use std::io;


fn main() {
    let mut input = String::new();

    println!("Enter two hex values (e.g. 57 83): ");
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<u8> = input.trim()
        .split_whitespace()
        .filter_map(|s| u8::from_str_radix(s, 16).ok())
        .collect();

    if nums.len() == 2 {
        let a = GF256(nums[0]);
        let b = GF256(nums[1]);

        println!("Addition: {}", a.add(b));
        println!("Multiplication: {}", a.mul(b));
        println!("Inverse of first number: {}", a.inverse());
    } else {
        println!("Invalid Input!");
    }
}

/*

fn main() {
    // Test Polynomial and GF256 module

    let a = GF256(7);
    let b = GF256(3);

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a.add(b));
    println!("a * b = {}", a.mul(b));
    println!("a⁻¹ = {}", a.inverse());
    println!("b⁻¹ = {}", b.inverse());

    let p1 = Polynomial::new(vec![GF256(3), GF256(5), GF256(7)]); // should return 3 + 5x + 7x²
    let p2 = Polynomial::new(vec![GF256(1), GF256(2)]); // should return 1 + 2x

    println!("p1 = {}", p1);
    println!("p2 = {}", p2);
    println!("p1 + p2 = {}", p1.add(&p2));
    println!("p1 * p2 = {}", p1.mul(&p2));
    println!("p1(2) = {}", p1.evaluate(GF256(2)));

    let xs = vec![GF256(1), GF256(2), GF256(3)];
    let ys = vec![GF256(5), GF256(9), GF256(17)];

    let poly = Polynomial::lagrange_interpolation(&xs, &ys);
    println!("Interpolation Polynomials: {}", poly);

    for i in 0..xs.len() {
        assert_eq!(poly.evaluate(xs[i]), ys[i]);
    }

    println!("Interpolation Verified!");
    
    // Uncomment section to test the modules 
    
    

}*/