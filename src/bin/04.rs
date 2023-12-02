use std::fs;

extern crate combine;
use combine::{sep_end_by, many1, Parser};
use combine::parser::char::{spaces, string, char, digit};
use std::collections::BTreeSet;
use std::collections::HashMap;

fn main() {
    let lex_char = |c| char(c);

    let integer = || many1(digit()).map(|s : String| s.parse::<i32>().unwrap());

    let set = || spaces().with(sep_end_by(integer(), spaces())
        .map(|list : Vec<i32>| BTreeSet::from_iter(list.iter().cloned())));
        
    let game = || string("Card").with(spaces()).with(integer());
    let line_parser = || (game(), lex_char(':'), set(), lex_char('|').with(spaces()), set())
        .map(|(id, _ , win, _, ynumb) : (i32,_, BTreeSet<i32>, _ , BTreeSet<i32>)| (id, win, ynumb));

    let contents = fs::read_to_string("./04.input")
        .expect("Should have been able to read the file");

    let lines1 = contents.lines().clone();
    let lines2 = contents.lines().clone();

    let res :i32 = lines1.map( |l| {
        let (_id, win, ynumb) = line_parser().parse(l).expect("line could not be parsed").0;
        let matches = win.intersection(&ynumb);

        let count = matches.count();
        if count == 0 {
            0
        } else {
            i32::pow(2, u32::try_from(count - 1).unwrap())
        }
    }).sum();
    println!("part 1 {:?}", res);

    let mut h = HashMap::new();

    lines2.for_each( |l| {
        let (id, win, ynumb) = line_parser().parse(l).expect("line could not be parsed").0;
        let matches = win.intersection(&ynumb);
        let count = matches.count();
        h.insert(id, i32::try_from(count).unwrap());
    });

    let mut sum = 0;

    for (key, value) in &h {
        fn count_copies(hash : &HashMap<i32, i32>, k : i32, v : i32) -> i32 {
            let mut sum2 = 0;
            for i in 0..v {
                let k2 : i32 = k + i + 1;
                let v2 = hash.get(&k2).unwrap().clone();
                sum2 = sum2 + count_copies(hash, k2, v2) + 1;
            };
            sum2
        };
        let cardpoints = 1 + count_copies(&h, *key, *value);
        sum = sum + cardpoints;
    }

    println!("part 2 {:?}", sum);
}

