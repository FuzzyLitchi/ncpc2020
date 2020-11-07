use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Dam {
    downstream: usize,
    capacity: u32,
    current: u32,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let (n_dams, water_to_destroy_camp): (usize, u32) = {
        let line = lines.next().unwrap();
        let mut s = line.trim().split_whitespace();
        (s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap())
    };

    let mut dams = Vec::with_capacity(n_dams);

    for _ in 0..n_dams {
        let line = lines.next().unwrap();
        let mut s = line.trim().split_whitespace();
        let downstream = s.next().unwrap().parse().unwrap();
        let capacity = s.next().unwrap().parse().unwrap();
        let current = s.next().unwrap().parse().unwrap();

        dams.push(Dam { downstream, capacity, current });
    }

    let mut dam_resolver = DamResolver {
        dams: &dams,
        water_to_destroy_camp,
        cache: HashMap::new(),
    };

    let min = (0..n_dams).map(|dam| dam_resolver.resolve_leaf(dam))
        .chain(Some(water_to_destroy_camp)).min().unwrap();

    println!("{}", min);
}

struct DamResolver<'a> {
    dams: &'a [Dam],
    water_to_destroy_camp: u32,
    cache: HashMap<usize, u32>,
}

impl DamResolver<'_> {
    #[inline]
    fn resolve_leaf(&mut self, i: usize) -> u32 {
        if let Some(&water_needed) = self.cache.get(&i) {
            water_needed
        } else {
            let water_needed = self.resolve(self.dams[i]);
            self.cache.insert(i, water_needed);
            water_needed
        }
    }
    fn resolve(&mut self, dam: Dam) -> u32 {
        let Dam {
            downstream,
            capacity,
            current
        } = dam;
        let extra_water_needed = capacity - current;
        let required_water_downstream = if downstream != 0 {
            self.resolve_leaf(downstream-1)
        } else {
            self.water_to_destroy_camp
        };
        extra_water_needed + required_water_downstream.saturating_sub(capacity)
    }
}
