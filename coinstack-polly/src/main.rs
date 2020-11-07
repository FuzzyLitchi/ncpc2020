use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd, PartialEq};

#[derive(Debug)]
struct CoinStack {
    location: usize,
    height: u32,
}

// impl CoinStack {
//     fn dec(self) -> Option<Self> {
//         if self.height == 1 {
//             return None;
//         }
//         Some(Self {
//             location: self.location,
//             height: self.height - 1,
//         })
//     }
// }

impl PartialEq for CoinStack {
    fn eq(&self, other: &CoinStack) -> bool {
        self.height == other.height
    }
}

impl PartialOrd for CoinStack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.height.cmp(&other.height).reverse())
    }
}

impl Eq for CoinStack {}

impl Ord for CoinStack {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height).reverse()
    }
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let _n = lines
        .next().unwrap();


    let mut coin_stacks: Vec<CoinStack> = lines.next().unwrap().split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .enumerate()
        .map(|(l, h)| CoinStack {
            location: l+1,
            height: h,
        })
        .collect();
    
    coin_stacks.sort();
    // let mut heap = BinaryHeap::new();
    // for stack in coin_stacks {
    //     heap.push(stack)
    // }

    let mut moves: Vec<(usize, usize)> = Vec::new();
    // println!("{:?}", coin_stacks);
    
    loop {
        fn find(list: &[CoinStack]) -> Option<usize> {
            list.iter().position(|s| s.height >= 1)
        }

        let first = find(&coin_stacks);
        match first {
            None => {
                println!("yes");
                for m in moves {
                    println!("{} {}", m.0, m.1);
                }
                break;
            },
            Some(first) => {
                // dbg!(first, &coin_stacks[first+1..]);
                let second = find(&coin_stacks[first+1..]).map(|i| i+1+first);
                match second {
                    None => {
                        println!("no");
                        break;
                    },
                    Some(second) => {
                        // dbg!(first, second, &coin_stacks);
                        moves.push((coin_stacks[first].location, coin_stacks[second].location));
                        coin_stacks[first].height -= 1;
                        coin_stacks[second].height -= 1;
                    }
                }
            }
        }
    }

    Ok(())
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
