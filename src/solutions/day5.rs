use super::Solution;

pub struct Day5 {
    seeds: Vec<isize>,
    mappings: Vec<Mapping>,
    seeds2: Vec<SRange>,
}

impl Solution for Day5 {
    fn part1(&mut self) -> String {
        self.seeds
            .iter()
            .map(|&seed| {
                self.mappings
                    .iter()
                    .fold(seed, |accum, mapping| mapping.apply(accum))
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(&mut self) -> String {
        self.seeds2
            .iter()
            .map(|srange| {
                let mut range = vec![*srange];
                for mapping in self.mappings.iter() {
                    mapping.apply_range(&mut range);
                }
                range.iter().min_by_key(|range| range.start).unwrap().start
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn parse(input: String) -> Box<dyn Solution> {
        let mut groups = input.split("\n\n");
        let seeds = groups
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .map(|seed| seed.parse().unwrap())
            .collect::<Vec<_>>();
        let mappings = groups.map(Mapping::parse).collect();
        let seeds2 = seeds
            .chunks_exact(2)
            .map(|chunk| SRange {
                start: chunk[0],
                end: chunk[0] + chunk[1] - 1,
            })
            .collect::<Vec<_>>();
        Box::new(Self {
            seeds,
            mappings,
            seeds2,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct SRange {
    start: isize,
    end: isize,
}

#[derive(Debug)]
struct MRange {
    destination: isize,
    source: isize,
    length: isize,
}

impl MRange {
    fn apply(&self, source: isize) -> Option<isize> {
        if (self.source..(self.source + self.length)).contains(&source) {
            Some(source - self.source + self.destination)
        } else {
            None
        }
    }

    fn apply_range(&self, other: &SRange) -> [Option<SRange>; 3] {
        let mut before = None;
        let mut inside = None;
        let mut after = None;

        let self_end = self.source + self.length;

        if other.start < self.source {
            before = Some(SRange {
                start: other.start.min(self.source),
                end: other.end.min(self.source - 1),
            });
        }

        if !(other.end < self.source || other.start >= self_end) {
            inside = Some(SRange {
                start: (other.start - self.source).max(0) + self.destination,
                end: self.destination + (other.end - self.source).min(self.length - 1),
            });
        }

        if other.end >= self_end {
            after = Some(SRange {
                start: self_end.max(other.start),
                end: other.end,
            })
        }

        [before, inside, after]
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MRange>,
}

impl Mapping {
    fn parse(input: &str) -> Self {
        let ranges = input
            .lines()
            .skip(1)
            .map(|line| {
                let mut numbers = line.split(' ');
                MRange {
                    destination: numbers.next().unwrap().parse().unwrap(),
                    source: numbers.next().unwrap().parse().unwrap(),
                    length: numbers.next().unwrap().parse().unwrap(),
                }
            })
            .collect();
        Self { ranges }
    }

    fn apply(&self, source: isize) -> isize {
        for range in self.ranges.iter() {
            if let Some(destination) = range.apply(source) {
                return destination;
            }
        }
        source
    }

    fn apply_range(&self, ranges: &mut Vec<SRange>) {
        let mut mapped = Vec::new();
        let mut results = Vec::new();
        for mapping in self.ranges.iter() {
            while let Some(range) = ranges.pop() {
                let applied = mapping.apply_range(&range);
                if let Some(mapped_range) = applied[1] {
                    mapped.push(mapped_range);
                }
                if let Some(before) = applied[0] {
                    results.push(before);
                }
                if let Some(after) = applied[2] {
                    results.push(after);
                }
            }
            std::mem::swap(ranges, &mut results);
        }
        ranges.extend(mapped);
    }
}
