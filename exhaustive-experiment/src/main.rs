use std::io::{stdin, BufRead};
use std::collections::BTreeSet;

#[derive(Debug, Copy, Clone)]
enum TestResult {
    Positive,
    Negative,
    None
}

#[derive(Debug, Copy, Clone)]
struct Component {
    x: i64,
    y: i64,
    test: TestResult,
}

impl Component {
    #[inline]
    fn below(&self, other: &Self) -> bool {
        let y_diff = other.y - self.y;
        2 * (other.x - self.x).abs() <= y_diff
    }
}

fn main() {
    use self::TestResult::*;

    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut components = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut s = line.trim().split_whitespace();
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let test = match s.next().unwrap() {
            "P" => Positive,
            "N" => Negative,
            "-" => None,
            _ => unimplemented!(),
        };
        components.push(Component {x, y, test});
    }

    let mut counted = BTreeSet::new();
    let mut to_remove = Vec::with_capacity(n);

    components.sort_by_key(|c| c.y);

    for (i, component) in components.iter().enumerate() {
        match component.test {
            Positive => {
                counted.insert(i);
                for &j in counted.range(..i) {
                    if component.below(&components[j]) {
                        to_remove.push(j);
                    }
                }
                for j in &to_remove {
                    counted.remove(j);
                }
                to_remove.clear();
            }
            Negative => {
                for &j in counted.range(..i) {
                    if component.below(&components[j]) {
                        println!("impossible");
                        return
                    }
                }
            }
            None => (),
        }
    }

    println!("{}", counted.len());
}
