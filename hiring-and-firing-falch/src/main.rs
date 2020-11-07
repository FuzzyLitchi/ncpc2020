use std::io::{self, BufRead};
use std::ops::Range;

#[inline]
fn is_empty(a: &Range<u32>) -> bool {
    !(a.start < a.end)
}

#[inline]
fn range_overlap(a: &Range<u32>, b: &Range<u32>) -> bool {
    if is_empty(a) || is_empty(b) {
        false
    } else {
        (a.end > b.start && a.start < b.end) ||
        (b.end > a.start && b.start < a.end)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let n_days: usize = lines.next().unwrap().parse().unwrap();

    let mut hr_workers: Vec<Vec<Range<u32>>> = Vec::new();

    let mut worker_i: u32 = 0;
    let hr_worker_each_day = (0..n_days).map(|_| {
        let line = lines.next().unwrap();
        let mut s = line.trim().split_whitespace();
        let fires: u32 = s.next().unwrap().parse().unwrap();
        let hires: u32 = s.next().unwrap().parse().unwrap();

        (fires, hires)
    }).map(|(fires, hires)| {
        let range = worker_i - fires .. worker_i + hires;
        let i = if is_empty(&range) {
            0
        } else {
            let (i, hr) = if let Some((i, hr)) = hr_workers
                .iter_mut()
                .enumerate()
                .find(|(_, hr_worker)| {
                    hr_worker.iter().all(|r| !range_overlap(&range, &r))
                }) {
                (i, hr)
            } else {
                let i = hr_workers.len();
                hr_workers.push(Vec::new());
                (i, &mut hr_workers[i])
            };
            hr.push(range);
            i
        };

        worker_i -= fires;

        for (j, hr_worker) in hr_workers.iter_mut().enumerate() {
            let slice = if i == j {
                let len = hr_worker.len();
                &mut hr_worker[..len - 1]
            } else {
                &mut hr_worker[..]
            };
            for range in slice {
                range.start = range.start.min(worker_i);
                range.end = range.end.min(worker_i);
            }
        }

        worker_i += hires;

        format!("{}", i+1)
    }).collect::<Vec<_>>();
    println!("{}", hr_workers.len());
    println!("{}", hr_worker_each_day.join(" "));
}
