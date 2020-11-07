use std::io::{self, BufRead};

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let _n = lines
        .next().unwrap();

    let mut list: Vec<u64> = lines.next().unwrap().split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    for i in 0..list.len() { // O(n)
        let original = list[i];
        for alt in modified(original) { //O(d) where d is digits, so like 15 max
            list[i] = alt;
            if !is_sorted(&list) { //O(n)
                let mut h = list.iter();
                print!("{}", h.next().unwrap());
                for h in h {
                    print!(" {}", h);
                }
                println!();
                return Ok(());
            }
        }
        list[i] = original;
    }
    println!("impossible");

    Ok(())
}

fn modified(n: u64) -> Vec<u64> {
    let mut numbers = Vec::new();

    for power in 1..=log10(n) {
        let one = 10_u64.pow(power as u32-1);
        let ten = 10_u64.pow(power as u32);
        let striped_n = n-(n%ten - n%one);
        // dbg!(one, ten, striped_n);
        for digit in 1..=9 {
            let value = striped_n + one*digit;
            if value == n || value > 10_u64.pow(15) {
                continue;
            }
            numbers.push(value);
        }
    }

    numbers
}

// Not really log10
fn log10(n: u64) -> u64 {
    if n <= 9 {
        1
    } else {
        log10(n/10) + 1
    }
}

fn is_sorted(list: &[u64]) -> bool {
    for ugh in list.windows(2) {
        if ugh[0] > ugh[1] {
            return false;
        }
    }

    true
}

struct Error {
    inner: Box<dyn error::Error>
}

use std::{fmt, error};
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> From<T> for Error
where
    T: error::Error + Send + Sync + 'static
{
    fn from(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}
