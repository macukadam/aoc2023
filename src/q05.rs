use std::{
    fs::File,
    io::{BufReader, Lines},
};

#[derive(Debug)]
struct Seed {
    val: u32,
}

impl Seed {
    fn new(val: u32) -> Self {
        Self { val }
    }
}

#[derive(Debug)]
struct SeedMapping {
    destination: usize,
    source: usize,
    steps: usize,
}

impl SeedMapping {
    fn new(destination: usize, source: usize, steps: usize) -> Self {
        SeedMapping {
            destination,
            source,
            steps,
        }
    }

    fn path(&self, seed: usize) -> usize {
        if self.source <= seed && self.source + self.steps > seed {
            let ret = (self.destination as isize - self.source as isize + seed as isize) as usize;
            return ret;
        }
        seed
    }
}

fn load_seeds(lines: Lines<BufReader<File>>) -> (Vec<Seed>, Vec<Vec<SeedMapping>>) {
    let mut maps: Vec<Vec<SeedMapping>> = Vec::new();
    let mut seeds: Vec<Seed> = Vec::new();

    let mut index = 0;
    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        if i == 0 {
            let (_, second_half) = line.split_once(':').unwrap();

            seeds = second_half
                .split_whitespace()
                .map(|x| Seed::new(x.parse::<u32>().unwrap()))
                .collect();
            continue;
        }

        if line.is_empty() {
            continue;
        }

        if line.contains(':') {
            index += 1;
            maps.push(Vec::new());
            continue;
        }

        let v: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        maps[index - 1].push(SeedMapping::new(v[0], v[1], v[2]));
    }

    (seeds, maps)
}


pub fn part2(lines: Lines<BufReader<File>>) {
    println!("part2");
    let mut min: usize = 999999999999999;
    let (seeds, maps) = load_seeds(lines);

    for i in (0..seeds.len()).step_by(2) {
        println!("i: {}", i);
        let seed_range = seeds[i].val as usize..=seeds[i].val as usize + seeds[i + 1].val as usize;
        println!("seed_range: {:?}", seed_range);

        for seed in seed_range {
            let mut val = seed;

            for innermaps in &maps {
                for map in innermaps {
                    let old_val = val;
                    val = map.path(val);
                    if val != old_val {
                        break;
                    }
                }
            }

            if val < min {
                min = val;
            }
        }
    }

    println!("min: {}", min);
}

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut values: Vec<usize> = Vec::new();
    let (seeds, maps) = load_seeds(lines);

    for seed in seeds.iter() {
        let mut val = seed.val as usize;

        for innermaps in &maps {
            for map in innermaps {
                let old_val = val;
                val = map.path(val);
                if val != old_val {
                    break;
                }
            }
        }

        values.push(val);
    }

    let min = values.iter().min().unwrap();
    println!("min: {}", min);
}
