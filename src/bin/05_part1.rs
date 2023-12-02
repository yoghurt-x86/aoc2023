use std::fs;

extern crate combine;
use combine::{satisfy, many1, Parser, sep_by, sep_end_by};
use combine::parser::char::{space, spaces, string, char, digit, newline};
use std::cmp;


#[derive(Debug, PartialEq)]
pub struct Row {
    start : i64, 
    end : i64, 
    width : i64, 
}

#[derive(Debug, PartialEq)]
pub struct Map {
    name : String,
    rows : Vec<Row>, 
}

impl Map {
    fn map(&self, num: i64) -> i64 {
        for r in &self.rows {
            if num >= r.start && num < (r.start + r.width) {
                return num + (r.end - r.start)
            }
        }
        return num
    }
}

fn main() {
    let contents = fs::read_to_string("./05.input")
        .expect("Should have been able to read the file");

    //let lex_char = |c| char(c).skip(spaces());
    let integer = || many1(digit()).map(|s : String| s.parse::<i64>().unwrap());
    let word = || many1(satisfy(|c: char| c.is_alphanumeric() || c == '-')).map(|s : String| s);

    let seeds = || (string("seeds:").with(spaces()), sep_end_by(integer(), spaces()))
            .map(|(_,seeds) : (_, Vec<i64>)| seeds);
    let map_name = || word().skip(space()).skip(string("map:")).skip(newline());
    let map_row = || (integer().skip(space()), integer().skip(space()), integer().skip(newline()))
            .map(|(a,b,c) : (i64, i64,i64)| 
                Row { start : b, end: a, width : c}
            );
    let map = || (map_name(), many1(map_row()))
            .map(|(name, rows) : (String, Vec<Row> )| 
                Map {name: name, rows: rows}
            );
    let file = || (seeds(), sep_by(map(), spaces()))
            .map(|(seeds, maps): (Vec<i64>, Vec<Map>)| (seeds, maps));


    let res = file().parse(contents.as_str()).unwrap().0;

    fn map_all(maps: &Vec<Map>, num : i64) -> i64 {
        maps.iter().fold(num, |n, m| m.map(n))
    }

    for lol in &res.1 {
        println!("{}", lol.name);
    }

    let locations : Vec<i64> = res.0.iter().map(|seed| map_all(&res.1, *seed)).collect();

    println!("part1 {:?}", locations.iter().min());
}

