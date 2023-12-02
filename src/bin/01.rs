use std::fs;

fn main() {
    //Part 1
    let contents = fs::read_to_string("./01.input")
        .expect("Should have been able to read the file");

    let lines = contents.lines();

    let num = lines.map(|l| {
        let chars : Vec<char> = l.chars().filter(|c| c.is_numeric()).collect();
        let c1 = chars.first().unwrap();
        let c2 = chars.last().unwrap();

        let num = String::from_iter([c1, c2]); 
        let int = num.parse::<i32>().unwrap(); 
        int
        }
    ).sum::<i32>();

    println!("{}", num);

    //Part 2
    let contents = fs::read_to_string("./01.input")
        .expect("Should have been able to read the file");

    let lines = contents.lines();
    let num = lines.map(|l| {
        fn start(l: &str) -> char{
            match l {
                s if s.starts_with("one") => return '1',
                s if s.starts_with("two") => return '2',
                s if s.starts_with("three") => return '3',
                s if s.starts_with("four") => return '4',
                s if s.starts_with("five") => return '5',
                s if s.starts_with("six") => return '6',
                s if s.starts_with("seven") => return '7',
                s if s.starts_with("eight") => return '8',
                s if s.starts_with("nine") => return '9',
                s if s.starts_with(|c: char| c.is_numeric()) => return l.chars().nth(0).unwrap(),
                _ => start(&l[1..]),
            }
        }

        fn end(l: &str) -> char {
            let len = l.len();
            match l {
                s if s.ends_with("one") => return '1',
                s if s.ends_with("two") => return '2',
                s if s.ends_with("three") => return '3',
                s if s.ends_with("four") => return '4',
                s if s.ends_with("five") => return '5',
                s if s.ends_with("six") => return '6',
                s if s.ends_with("seven") => return '7',
                s if s.ends_with("eight") => return '8',
                s if s.ends_with("nine") => return '9',
                s if s.ends_with(|c: char| c.is_numeric()) => l.chars().nth(len-1).unwrap(),//return l[(len-1)..len].parse::<i32>().unwrap(),
                _ => end(&l[..len-1]),
            }
        }

        let c1 = start(l);
        let c2 = end(l);
        let num = String::from_iter([c1, c2]); 
        let int = num.parse::<i32>().unwrap(); 

        int
    }).sum::<i32>();

    println!("{}", num);
}

