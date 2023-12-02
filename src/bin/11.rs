use std::fs;

use std::cmp;



fn main() {
    let contents = fs::read_to_string("./11.input")
        .expect("Should have been able to read the file");

    let map : Vec<Vec<char>> = { 
        let input = contents.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut map : Vec<Vec<char>> = Vec::new();

        for line in input {
            if line.iter().all(|c| *c != '#') {
                map.push(vec!['X';line.len()]);
            } else {
                map.push(line);
            }
        }
        map
    };

    let height = map.len();
    let width = map[0].len();

    let map_inverted : Vec<Vec<char>> = {
        let mut map_inverted = vec![Vec::new();map[0].len()];
        for w in 0..width {
            for h in 0..height {
                map_inverted[w].push(map[h][w]);
            }
        }

        let mut map : Vec<Vec<char>> = Vec::new();

        for line in map_inverted {
            if line.iter().all(|c| *c != '#') {
                map.push(vec!['X';line.len()]);
            } else {
                map.push(line);
            }
        }
        map
    };

    let mut galaxies : Vec<(i64,i64)> = Vec::new();

    for x in 0..map_inverted.len() {
        for y in 0..map_inverted[0].len() {
            if map_inverted[x][y] == '#' {
                galaxies.push((i64::try_from(x).unwrap(),i64::try_from(y).unwrap()));
            }
        }
    }

    println!("{:?}", galaxies);
    //let mut dists = Vec::new();
    let mut count = 0;

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let (g1x, g1y) = galaxies[i];
            let (g2x, g2y) = galaxies[j];
            let dist = (g1x-g2x).abs() + (g1y - g2y).abs();
            //println!("{:?} - {:?}", galaxies[i], galaxies[j]);
            count = count + dist;
            println!("count: {:?} {:?} - {:?}, {:?}", count, galaxies[i], galaxies[j], dist);
        }
    }
    println!("part 1{:?}", count);

    let mut count = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let (g1x, g1y) = galaxies[i];
            let (g2x, g2y) = galaxies[j];
            let mut dist = (g1x-g2x).abs() + (g1y - g2y).abs();
            for x in (cmp::min(g1x, g2x)+1)..(cmp::max(g1x, g2x)) {
                if map_inverted[usize::try_from(x).unwrap()][usize::try_from(g1y).unwrap()] == 'X' {
                    println!("1000000");
                    dist = dist + 999999;
                }
            }
            for y in (cmp::min(g1y, g2y) + 1)..(cmp::max(g2y, g1y)) {
                if map_inverted[usize::try_from(g1x).unwrap()][usize::try_from(y).unwrap()] == 'X' {
                    println!("1000000");
                    dist = dist + 999999;
                }

            }
            count = count + dist;
            println!("count: {:?} {:?} - {:?}, {:?}", count, galaxies[i], galaxies[j], dist);
        }
    }

    println!("part 2 {:?}", count);

    for l in &map {
        println!("{:?}", l);
    }

    println!("");

    for y in 0..map_inverted[0].len() {
        for x in 0..map_inverted.len() {
            print!("{}", map_inverted[x][y]);
        }
        print!("\n");
    }
}

