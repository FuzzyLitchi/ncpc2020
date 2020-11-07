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
        match coin_stacks.len() {
            0 => {
                println!("yes");
                println!("{} bytes", moves.len()*std::mem::size_of::<(usize,usize)>());
                for r#move in moves {
                    println!("{} {}", r#move.0, r#move.1);
                }
                break;
            },
            1 => {
                println!("no");
                break;
            } 
            _ => {
                let a_h;
                let b_h;
                {
                    let a = &coin_stacks[0];
                    let b = &coin_stacks[1];
                    // dbg!(a, b);
                    moves.push((a.location,b.location));
                    a_h = a.height;
                    b_h = b.height;
                }

                match b_h - 1 {
                    0 => {coin_stacks.remove(1);},
                    v => coin_stacks[1].height = v,
                }
                match a_h - 1 {
                    0 => {coin_stacks.remove(0);},
                    v => coin_stacks[0].height = v,
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
