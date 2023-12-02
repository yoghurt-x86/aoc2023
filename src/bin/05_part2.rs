use std::fs;

extern crate combine;
use combine::{satisfy, many1, Parser, sep_by, sep_end_by};
use combine::parser::char::{space, spaces, string, char, digit, newline};
use std::cmp;


#[derive(Debug, PartialEq, Clone, Copy)]
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
    //assumes rows are sorted by source index
    fn map(&self, (start, width): (i64, i64)) -> Vec<(i64, i64)> {
        let mut result : Vec<(i64, i64)> = Vec::new();
        let mut final_result : Vec<(i64, i64)> = Vec::new();
        result.push((start,width));

        for r in &self.rows {
            let mut buff : Vec<(i64, i64)> = Vec::new();
            for (s, w) in result {
                if r.start < (s + w) && (r.start + r.width) > s {
                    if r.start > s {
                        buff.push((s, r.start - s));
                    }
                    if (r.start + r.width) < (s + w) {
                        buff.push((r.start+r.width, (s + w) - (r.start + r.width) ));
                    }
                    let st = cmp::max(r.start, s);
                    let wi = cmp::min(r.start + r.width, s + w) - st;
                    final_result.push((st + (r.end - r.start), wi));
                } else {
                    buff.push((s, w));
                }
            }
            result = buff;
        }
        final_result
    }
}

fn main() {
    let contents = fs::read_to_string("./05.input")
        .expect("Should have been able to read the file");

    let integer = || many1(digit()).map(|s : String| s.parse::<i64>().unwrap());
    let word = || many1(satisfy(|c: char| c.is_alphanumeric() || c == '-')).map(|s : String| s);

    let seed_pair = || (integer().skip(space()), integer());
    let seeds = || (string("seeds:").with(spaces()), sep_end_by(seed_pair(), spaces()))
            .map(|(_,seeds) : (_, Vec<(i64, i64)>)| seeds);
    let map_name = || word().skip(space()).skip(string("map:")).skip(newline());
    let map_row = || (integer().skip(space()), integer().skip(space()), integer().skip(newline()))
            .map(|(a,b,c) : (i64, i64,i64)| 
                Row { start : b, end: a, width : c}
            );
    let map = || (map_name(), many1(map_row()))
            .map(|(name, rows) : (String, Vec<Row> )| {
                let mut mut_rows : Vec<Row> = (&rows).to_vec();
                mut_rows.sort_by(|a, b| a.start.cmp(&b.start));
                Map {name: name, rows: mut_rows }
            });
    let file = || (seeds(), sep_by(map(), spaces()))
            .map(|(seeds, maps): (Vec<(i64, i64)>, Vec<Map>)| (seeds, maps));


    let res = file().parse(contents.as_str()).unwrap().0;

    fn map_all(maps: &Vec<Map>, num : (i64,i64)) -> Vec<(i64, i64)> {
        maps.iter().fold(vec![num], |ranges, m| {
            ranges.iter().flat_map(|r| m.map(*r)).collect::<Vec<_>>()
            }
        )
    }

    let something : Vec<_> = res.0.iter().flat_map(|seed_range| map_all(&res.1, *seed_range)).collect();

    println!("part 2: {:?}", something.iter().min_by(|a,b| a.0.cmp(&b.0)).unwrap().0);

}

