use std::io::{self, BufRead};

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let _songs = lines
        .next().unwrap()
        .parse::<u32>()?;

    let songs: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    
    const MODULO: u64 = 1000000007;
    let mut count_3 = 0; // Amount of 3's after where we are
    let mut perms_2 = 0; // Amount of /2+3/
    // let mut perms_1 = 0; // 
    let mut perms = 0; // total 1 2* 3 perms
    for &song in songs.iter().rev() {
        match song {
            3 => count_3 += 1,
            2 => perms_2 = (perms_2 + count_3 + perms_2)%MODULO,
                    // We have 23 for each 3 +count_3
                    // We have 22+3 for each 2_perm +perms_2
            1 => perms = (perms_2 + perms)%MODULO,
            _ => unreachable!(),
        }
    }
    println!("{}", perms);

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
