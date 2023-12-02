use std::fs;
use itertools::Itertools;

use std::cmp;


extern crate combine;
use combine::sep_end_by1;
use combine::{many1, Parser, sep_by, choice, many};
use combine::parser::range::{take_while};
use combine::parser::char::{newline, spaces, char, digit};

fn main() {
    let item = || take_while(|c: char| !c.is_whitespace() && c != ',');
    let content = || sep_by(item(), char(','))
        .map(| l : Vec<_>| l);
    let contents = fs::read_to_string("./15.input")
        .expect("Should have been able to read the file");

    let blah : Vec<&str>= content().parse(contents.as_str()).unwrap().0;

    let hashes = blah.iter().map(|b| {
        let mut hash : u32 = 0;
        for c in b.chars() {
            let m = { 
                if c.is_ascii() {
                    c as u8
                } else {
                    panic!("Not ascii");
                }
            };

            hash = hash + (u32::from(m));
            hash = hash * 17;
            hash = hash % 256;
        }
        hash
    }).collect::<Vec<_>>();


    let sum : u32 = hashes.iter().sum();
    //println!("contents {:?}", blah);
    //println!("contents {:?}", hashes);
    //
    //
    println!("{}", sum);



}

