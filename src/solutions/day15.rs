use super::Solution;

pub struct Day15 {
    input: String,
    map: [Vec<(String, u32)>; 256],
}

impl Solution for Day15 {
    fn part1(&mut self) -> String {
        self.input
            .trim()
            .split(',')
            .map(|s| dhash(s) as usize)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&mut self) -> String {
        for step in self.input.clone().trim().split(',') {
            if let Some(label) = step.strip_suffix('-') {
                self.execute_minus(label);
            } else {
                let label = &step[..step.len() - 2];
                let num = step.chars().last().unwrap().to_digit(10).unwrap();
                self.execute_equal(label, num);
            }
        }

        let mut total = 0;
        for idx in 0..self.map.len() {
            for (lens_idx, lens) in self.map[idx].iter().enumerate() {
                total += (idx + 1) * (lens_idx + 1) * lens.1 as usize;
            }
        }
        total.to_string()
    }

    fn parse(input: String) -> Box<dyn Solution> {
        Box::new(Self {
            input,
            map: std::array::from_fn(|_| Vec::new()),
        })
    }
}

impl Day15 {
    fn execute_minus(&mut self, label: &str) {
        let box_idx = dhash(label) as usize;
        for idx in 0..self.map[box_idx].len() {
            let b = &self.map[box_idx][idx];
            if b.0 == label {
                self.map[box_idx].remove(idx);
                break;
            }
        }
    }

    fn execute_equal(&mut self, label: &str, num: u32) {
        let box_idx = dhash(label) as usize;
        for idx in 0..self.map[box_idx].len() {
            let b = &mut self.map[box_idx][idx];
            if b.0 == label {
                b.1 = num;
                return;
            }
        }
        self.map[box_idx].push((label.to_owned(), num));
    }
}

fn dhash(str: &str) -> u8 {
    let mut hash = 0usize;
    for chr in str.bytes() {
        hash += chr as usize;
        hash *= 17;
        hash %= 256;
    }
    hash as u8
}
