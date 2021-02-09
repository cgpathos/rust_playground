extern crate rand;

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // 소유권
    {
        let s = "hello";
        println!("{}", s);
    }
    // println!("{}", s); // panic occur!! - out of scope

    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);


    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // error


    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}_~~~~~~~", s); // error

    let x = 5;
    makes_copy(x);
    println!("{}_~~~~", x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("no!!!!!!!!!!");
    let (s2, len) = calculate_length(s1);
    println!("s2 :: {}", s2);

    let len = calculate_length_reference(&s2);
    println!("s2 :: {}", s2);

    let mut s = String::from("hello~~~~~~~");
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    // let reference_to_nothing = dangle();


    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word -> {}", word);

    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// error
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world!!");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}