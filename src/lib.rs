/// Advent of Code 2023
///
/// Part 1: quick and dirty, basic string tokenizer
///
/// Part 2: using Option with a reduce operation
///
///
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

/// Reads lines from a file and provides iterators
///
#[derive(Debug)]
pub struct InfiniteLinesReader {
    /// The actual lines read from the file
    lines: Vec<String>,
}

impl InfiniteLinesReader {
    /// Init with lines from a file
    pub fn init(fname: &str) -> Result<Self, Error> {
        // open given file name and read all lines in it
        let f = File::open(fname)?;

        let mut reader = BufReader::new(f);

        let mut lines: Vec<String> = Vec::new();

        let mut buffer = String::new();

        let mut eof = false;
        while !eof {
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    eof = true;
                }
                Ok(_) => {
                    // keep a copy of each line
                    lines.push(buffer.trim_end_matches("\n").to_string());
                    buffer.clear();
                }
                Err(_error) => {
                    return Err(_error);
                }
            }
        }

        Ok(InfiniteLinesReader { lines })
    }

    /// Endlessly provide input lines
    pub fn cycle(&self) -> impl Iterator<Item = &String> {
        self.lines.iter().cycle()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.lines.iter()
    }
    
    pub fn length(&self) -> usize {
        self.lines.len()
    }
}

pub struct PagedIterator<I> {
    page_length: usize,
    page_number: usize,
    line_number: usize,
    iter: I,
}

impl<I> PagedIterator<I> {
    pub fn init(iter: I, page_length: usize) -> PagedIterator<I> {
        PagedIterator { page_length, page_number: 1, line_number: 0, iter }
    }
}

impl<I> Iterator for PagedIterator<I> where I: Iterator {
    type Item = (usize, usize, <I as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            y => {
                self.line_number += 1;
                if self.line_number > self.page_length {
                    self.page_number += 1;
                    self.line_number = 1;
                }
                Some((self.page_number, self.line_number, y?))
            }
        }
    }
}

pub fn solve(fname: &str) -> Result<usize, Error> {

    let reader = InfiniteLinesReader::init(fname)?;
    let mut lines = PagedIterator::init(reader.iter(), reader.length());

    let mut rx = 0usize;

    while let Some((p, n, cv)) = lines.next() {
        println!("# processing input line {}::{} {}", p, n, cv);

        let tokens: Vec<&str> = cv.split(&[' ', ',', ':', ';'][..]).map(|t| t.trim()).filter(|t| t.len() > 0).collect();

        assert_eq!("Game", tokens[0]);

        let id = tokens[1].parse::<usize>().expect("failed to parse Game ID");
        
        let mut valid = true;

        let mut r = 2usize;
        while r < tokens.len() {
            // value
            let value = tokens[r].parse::<usize>().expect("failed to parse colour value");
            // colour
            let colour = tokens[r + 1];

            println!("# colour: {:?} with count: {:?}", colour, value);

            //  check
            match colour {
                "red" => if value > 12 { valid = false; break; }
                "green" => if value > 13 { valid = false; break; }
                "blue" => if value > 14 { valid = false; break; }
                _ => { panic!("failed to match colour name"); }
            }

            r += 2;
        };

        if valid {
            println!("# game {:?} is valid", id);
            rx += id;
        } else {
            println!("# game {:?} is not valid", id);
        }
    }

    println!("# result {:?}", rx);

    Ok(rx)
}

pub fn reduce<T, F>(a: Option<T>, b: Option<T>, f: F) -> Option<T>
where
    F: FnOnce(T, T) -> T,
{
    match (a, b) {
        (Some(l), Some(r)) => Some(f(l, r)),
        (x @ Some(_), None) | (None, x @ Some(_)) => x,
        (None, None) => None,
    }
}

pub trait OptionExt {
    type T;
    fn reduce<F>(self, other: Option<Self::T>, f: F) -> Option<Self::T> where F: FnOnce(Self::T, Self::T) -> Self::T;
}

impl<T> OptionExt for Option<T> {
    type T = T;
    fn reduce<F>(self, other: Option<Self::T>, f: F) -> Option<Self::T> where F: FnOnce(Self::T, Self::T) -> Self::T
    {
        reduce(self, other, f)
    }
}

pub fn ext_solve(fname: &str) -> Result<usize, Error> {

    let reader = InfiniteLinesReader::init(fname)?;
    let mut lines = PagedIterator::init(reader.iter(), reader.length());

    let mut rx = 0usize;

    while let Some((p, n, cv)) = lines.next() {
        println!("# processing input line {}::{} {}", p, n, cv);

        let tokens: Vec<&str> = cv.split(&[' ', ',', ':', ';'][..]).map(|t| t.trim()).filter(|t| t.len() > 0).collect();

        assert_eq!("Game", tokens[0]);

        let mut r = 2usize;

        let mut mvalues: [Option<usize>; 3] = [None, None, None];

        while r < tokens.len() {
            // value
            let value = tokens[r].parse::<usize>().expect("failed to parse colour value");
            // colour
            let colour = tokens[r + 1];

            println!("# colour: {:?} with count: {:?}", colour, value);

            //  check
            //
            let index = match colour { "red" => 0, "green" => 1, "blue" => 2, _ => panic!("failed to match colour name") };

            mvalues[index] = mvalues[index].reduce(Some(value), usize::max);
            println!("# mvalues: {:?}", mvalues);

            r += 2;
        };

        let mut game_power: usize = 1;
        for c in mvalues {
            game_power *= c.unwrap_or(0);
        }

        println!("# game power for min values: {:?} is {:?}", mvalues, game_power);
        rx += game_power;
    }

    println!("# result {:?}", rx);
    Ok(rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let rx = solve("data/sample.txt").expect("failed to solve input puzzle");
    }
    
    #[test]
    fn puzzle() {
        let _rx = solve("data/input.txt").expect("failed to solve input puzzle");
    }

    #[test]
    fn ext_sample() {
        let rx = ext_solve("data/sample.txt").expect("failed to solve input puzzle");
    }

    #[test]
    fn ext_puzzle() {
        let rx = ext_solve("data/input.txt").expect("failed to solve input puzzle");
    }
}
