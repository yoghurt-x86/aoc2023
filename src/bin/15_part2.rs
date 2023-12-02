use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

use std::cmp;


extern crate combine;
use combine::sep_end_by1;
use combine::{many1, Parser, sep_by, choice, many};
use combine::parser::range::{take_while};
use combine::parser::char::{letter, newline, spaces, char, digit, alpha_num};

#[derive(Debug)]
pub enum Op {
    Add(String, i32),
    Remove(String),
}


fn hash(h: &str) -> u32 {
    let mut hash : u32 = 0;
    for c in h.chars() {
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
}

fn main() {
    let integer = || many1(digit()).map(|s : String| s.parse::<i32>().unwrap());
    let word = || many1(letter());
    let item = || 
        word().and(
            choice((
                char('=').with(integer()).map(Option::from),
                char('-').map(|_| None)
            ))
        ).map(|(w, o)| match o {
                Some(i) => Op::Add(w, i),
                None => Op::Remove(w),
            }
        );
    let content = || sep_by(item(), char(','))
        .map(| l : Vec<_>| l);
    let contents = fs::read_to_string("./15.input")
        .expect("Should have been able to read the file");

    let blah = content().parse(contents.as_str()).unwrap().0;

    let mut blocks : HashMap<u32, Vec<(String, i32)>> = HashMap::new();
    for i in 0..256 {
        blocks.insert(i, Vec::new());
    }

    let mut c = 0;
    for op in blah {
        c = c + 1;
        match &op {
            Op::Add(w, lens) => {
                let hash = hash(&w);
                match blocks.get_mut(&hash) { 
                    Some(vec) => {
                        let idx = vec.iter().position(|(v,_)| v == w);
                        match idx {
                            Some(i) => { vec[i] = (w.clone(),*lens)},
                            None => {vec.push((w.clone(),*lens));},
                        };
                    }, 
                    _ => {},
                };
            },
            Op::Remove(w) => {
                let hash = hash(&w);
                match blocks.get_mut(&hash) { 
                    Some(vec) => {
                        let idx = vec.iter().position(|(v,_)| v == w);
                        match idx {
                            Some(i) => {vec.remove(i);},
                            _ => {},
                        };
                    }, 
                    _ => {},
                };
            },
        };
        println!("{}: {:?}", c, &op);
        for (k,v) in &blocks {
            if !v.is_empty() {
                println!("{}:{:?}", k, v);
            }
        }
    }


    let mut sum = 0;
    for (k,v) in &blocks {
        for (slot, (w,i)) in v.iter().enumerate() {
            let a = 1 + k;
            let b = u32::try_from(slot + 1).unwrap();
            let c = u32::try_from(*i).unwrap();
            let product = a * b * c;
            println!("{}:{} = {:?}", w, i, product );
            sum = sum + product;
        }
    }


    //println!("contents {:?}", blah);
    //println!("contents {:?}", hashes);
    println!("{:?}", sum);



}

