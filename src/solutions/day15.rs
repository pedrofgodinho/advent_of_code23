use super::Solution;

pub struct Day15 {
    input: String,
    steps: Vec<Step>,
    map: [Vec<(String, u8)>; 256],
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
        for idx in 0..self.steps.len() {
            self.execute_step(idx);
        }
        let mut total = 0;
        for idx in 0..self.map.len() {
            // total += idx + 1
            for (lens_idx, lens) in self.map[idx].iter().enumerate() {
                total += (idx + 1) * (lens_idx + 1) * lens.1 as usize;
            }
        }
        total.to_string()
    }

    fn parse(input: String) -> Box<dyn Solution> {
        let steps = input.trim().split(',').map(Step::from_str).collect();
        Box::new(Self {
            input,
            steps,
            map: std::array::from_fn(|_| Vec::new()),
        })
    }
}

impl Day15 {
    fn execute_step(&mut self, step_idx: usize) {
        let step = &self.steps[step_idx];
        let box_idx = step.get_box() as usize;
        match step.op {
            Op::Minus => {
                for idx in 0..self.map[box_idx].len() {
                    let b = &self.map[box_idx][idx];
                    if b.0 == step.label {
                        self.map[box_idx].remove(idx);
                        break;
                    }
                }
            }
            Op::Equals(number) => {
                for idx in 0..self.map[box_idx].len() {
                    let b = &mut self.map[box_idx][idx];
                    if b.0 == step.label {
                        b.1 = number;
                        return;
                    }
                }
                self.map[box_idx].push((step.label.clone(), number));
            }
        }
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

#[derive(Debug)]
enum Op {
    Minus,
    Equals(u8),
}

#[derive(Debug)]
struct Step {
    label: String,
    op: Op,
}

impl Step {
    fn from_str(str: &str) -> Self {
        if let Some(label) = str.strip_suffix('-') {
            Self {
                label: label.to_owned(),
                op: Op::Minus,
            }
        } else {
            let mut split = str.split('=');
            Self {
                label: split.next().unwrap().to_owned(),
                op: Op::Equals(split.next().unwrap().parse().unwrap()),
            }
        }
    }

    fn get_box(&self) -> u8 {
        dhash(&self.label)
    }
}
