use std::io::{self, BufRead};

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let x = lines
        .next().unwrap()
        .chars()
        .filter(|&s| s=='S')
        .count();

    let y = lines
        .next().unwrap()
        .chars()
        .filter(|&s| s=='S')
        .count();

    let c = x*y;
    println!("{}0{}", "S(".repeat(c), ")".repeat(c));

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
