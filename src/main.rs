extern crate rand;

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    //
    let space = "                  ";
    let space = space.len();


    let _guess: u32 = "42".parse().expect("Not a number");
    let hangeul = '야';

    let tup: (i32, f64, u8) = (500, 6.34, 1);

    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let array = [1, 2, 3, 4, 5, ];

    let first = array[0];
    let second = array[1];
    let last = array[4];

    println!("array last : {}", last);


    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is : {}", y);


    println!("f is {}", test());

    println!("plus_one is {}", plus_one(4));

    // 한줄 주석
    /*
    여러줄 주석

    잇힝
     */


    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by 4, 3 or 2");
    }


    let condition = true;
    let number = if condition {
        5
    } else {
        6 // "six" lead to error.
    };
    println!("The value of number is : {}", number);


    // Clion에서 그냥 돌렸더니 컴터 죽을라고 함
    // loop {
    //     println!("again!");
    // }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50, ];

    // while
    let mut index = 0;
    while index < 5 {
        println!("the value is : {}", a[index]);
        index = index + 1;
    }

    // for
    for element in a.iter() {
        println!("the value is : {}", element);
    }


    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");

    for n in 0..10 {
        println!("{} -> {}", n, fibo(n))
    }
}

fn fibo(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn another_function(x: i32, y: i32) {
    println!("The value if x is : {}", x);
    println!("The value if y is : {}", y);
}

fn test() -> u32 {
    let f = 32;
    f
}

fn plus_one(x: i32) -> i32 {
    x + 1
}