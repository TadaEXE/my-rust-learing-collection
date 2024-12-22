use std::{io, isize};

fn main() {
    loop {
        println!("Input a number");

        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Unable to read input");

        let index: isize = match index.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        println!(
            "Recursive: {} Itterative: {}",
            recursive_fib(index),
            itterative_fib(index)
        );
    }
}

fn recursive_fib(i: isize) -> isize {
    if i == 0 {
        return 0;
    }
    else if i == 1 || i == 2 {
        return 1;
    }
    if i > 0 {
        recursive_fib(i - 1) + recursive_fib(i - 2)
    } else {
        isize::pow(-1, u32::try_from(i.abs()).expect("Critical wtf moment") + 1)
            * recursive_fib(i.abs())
    }
}

fn itterative_fib(i: isize) -> isize {
    if i == 0 {
        return 0;
    }
    else if i == 1 || i == 2 {
        return 1;
    }

    let mut i_1: isize = 1;
    let mut i_2: isize = 1;

    for _ in 3..=i.abs() {
        let tmp = i_1 + i_2;
        i_2 = i_1;
        i_1 = tmp;
    }

    i_1 * if i < 0 {
        isize::pow(-1, u32::try_from(i.abs()).expect("Critical wtf moment") + 1)
    } else {
        1
    }
}
