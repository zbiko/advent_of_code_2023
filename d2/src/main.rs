use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

struct Game {
    iter: u32,
    hands: Vec<RGB>,
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Game {
            iter: 1,
            hands: Vec::new(),
        };

        lazy_static! {
            static ref RE: regex::Regex = Regex::new(r"[^;:]+[;:]?").unwrap();
            static ref NAME_RE: regex::Regex = Regex::new(r"Game (?<iter>\d+)").unwrap();
            static ref RED_RE: regex::Regex = Regex::new(r"(\d+) red").unwrap();
            static ref GREEN_RE: regex::Regex = Regex::new(r"(\d+) green").unwrap();
            static ref BLUE_RE: regex::Regex = Regex::new(r"(\d+) blue").unwrap();
        }
        let it = RE.captures_iter(s);

        for e in it {
            let s = e.get(0).unwrap().as_str();
            if s.ends_with(":") {
                // Iteration
                match NAME_RE.captures(s) {
                    Some(parsed) => out.iter = parsed.get(1).unwrap().as_str().parse().unwrap(),
                    None => {}
                }
            } else {
                let mut rgb = RGB { r: 0, g: 0, b: 0 };
                match RED_RE.captures(s) {
                    Some(parsed) => rgb.r = parsed.get(1).unwrap().as_str().parse().unwrap(),
                    None => {}
                }
                match GREEN_RE.captures(s) {
                    Some(parsed) => rgb.g = parsed.get(1).unwrap().as_str().parse().unwrap(),
                    None => {}
                }
                match BLUE_RE.captures(s) {
                    Some(parsed) => rgb.b = parsed.get(1).unwrap().as_str().parse().unwrap(),
                    None => {}
                }
                out.hands.push(rgb);
            }
        }

        Ok(out)
    }
}

impl Game {
    fn print(&self) {
        print!("{} :", self.iter);
        for h in &self.hands {
            println!("\t {} {} {}", h.r, h.g, h.b);
        }
        println!();
    }

    fn get_maximums(&self) -> RGB {
        let mut max = RGB { r: 0, g: 0, b: 0 };
        self.hands.iter().for_each(|v| {
            if v.r > max.r {
                max.r = v.r;
            };
            if v.g > max.g {
                max.g = v.g;
            };
            if v.b > max.b {
                max.b = v.b;
            };
        });
        max
    }
}

fn main() {
    let contents = fs::read_to_string("input_day_2.txt").expect("should read the file");
    let mut sum = 0;
    let mut power_sum: u128 = 0;
    contents.lines().for_each(|l| {
        let game = Game::from_str(l).unwrap();
        let mut possible = true;
        for el in &game.hands {
            if el.r > 12 || el.g > 13 || el.b > 14 {
                possible = false;
                break;
            }
        }
        if possible {
            //game.print();
            sum += game.iter;
        }
        let maximums: RGB = game.get_maximums();
        power_sum += u128::from(maximums.r) * u128::from(maximums.g) * u128::from(maximums.b);
    });
    println!("SUM : {}", sum);
    println!("POWER SUM : {}", power_sum);
}
