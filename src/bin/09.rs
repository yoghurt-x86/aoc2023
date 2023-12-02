use std::fs;
use itertools::Itertools;

extern crate combine;
use combine::{many1, Parser, sep_by, choice, many};
use combine::parser::char::{newline, char, digit};


fn main() {
    let integer = || many1(digit()).map(|s : String| s.parse::<i32>().unwrap());
    let signed_int = || choice((
        integer(), 
        char('-').with(integer()).map(|i| -i )
    ));
    
    let row = || sep_by(signed_int(), char(' ')).skip(newline());

    let file = || many(row());

    let contents = fs::read_to_string("./09.input.backup")
        .expect("Should have been able to read the file");

    let res : Vec<Vec<i32>> = file().parse(contents.as_str()).unwrap().0;

    let sum : i32 = res.iter().map(| row | {
        let mut pyramid : Vec<Vec<i32>>= Vec::new();
        let mut waw : Vec<i32> = row.clone();
        loop  {
            waw = waw.iter().tuple_windows().map(|(i1,i2)| i2 - i1).collect::<Vec<i32>>();
            if waw.iter().all(|num| *num == 0) {
                break;
            }
            pyramid.push(waw.clone());
        }
        pyramid.iter().for_each(|line| println!("{:?}", line));

        let next = pyramid.iter().map(|r| r.last().unwrap()).fold(0, |acc, i| acc + i) + row.last().unwrap(); 
        let prev = row.first().unwrap() - pyramid.iter().rev().map(|r| r.first().unwrap()).fold(0, |acc, i| { println!("{}", i); i - acc} );
        println!("{}", prev);
        prev
    }).sum();

    println!("part 1 {:?}", sum);
}

