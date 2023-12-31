
# Advent of Code 2023
![Tests](https://github.com/pedrofgodinho/advent_of_code23/actions/workflows/rust.yml/badge.svg)

Solutions to [Advent of Code 2023](https://adventofcode.com/2023) in Rust 🎄


| Day                          | Part 1 | Part 2 | Thoughts
|------------------------------|--------|--------|----------
| [1](src/solutions/day1.rs)   |   ⭐   |   ⭐   | As with most early advent of code challenges, it's mostly parsing strings. 
| [2](src/solutions/day2.rs)   |   ⭐   |   ⭐   | Again, 90% of the work is parsing strings. While doing part 1 I parsed them to a nice data structure that ended up being overkill for part 2, sadly. 
| [3](src/solutions/day3.rs)   |   ⭐   |   ⭐   | A difficulty spike from day 2. First challenge where the algorithm is actually harder than the input parsing. 
| [4](src/solutions/day4.rs)   |   ⭐   |   ⭐   | Much easier than day 3. Very fun challenge overall. 
| [5](src/solutions/day5.rs)   |   ⭐   |   ⭐   | Pretty fun challenge. Part 1 was fairly easy, part 2 took some debugging to get right. Curiously, I later tried a brute-force solution, and it "only" takes 18 seconds on my cpu. 
| [6](src/solutions/day6.rs)   |   ⭐   |   ⭐   | Simple and elegant problem, with a simple and elegant solution.
| [7](src/solutions/day7.rs)   |   ⭐   |   ⭐   | A bit boring, in that the challenge is simple but requires quite a bit of writing to solve. Not too hard otherwise. 
| [8](src/solutions/day8.rs)   |   ⭐   |   ⭐   | Very interesting problem, very poorly stated. Part 1 is great. Part 2, however, requires you making a some assumptions that are not at all stated. 
| [9](src/solutions/day9.rs)   |   ⭐   |   ⭐   | Found this one to be very easy, part 1 and 2 being basically the same. Fun challenge nonetheless.
| [10](src/solutions/day10.rs) |   ⭐   |   ⭐   | Very fun, but slightly tricky challenge. Second part can be solved in tons of different ways. Left some prints commented out instead of removing them because they're just so satifying.
| [11](src/solutions/day11.rs) |   ⭐   |   ⭐   | Not much to say, it's a fairly easy challenge. Not super interesting, but not a bad challenge either. 
| [12](src/solutions/day12.rs) |   ⭐   |   ⭐   | Maybe it's just because I'm not used to dynamic programming, but I found this challenge *very* hard. Pretty fun challenge though. My implementation still takes 50ms to run, could probably optimize it further.
| [13](src/solutions/day13.rs) |   ⭐   |   ⭐   | Compared to previous days, it was a very easy challenge. Took it as an opportunity to learn more about ndarray. Enjoyable problem to solve overall
| [14](src/solutions/day14.rs) |   ⭐   |   ⭐   | Not a very hard problem. Took me a little bit because I tried to do part 1 without mutating, which meant I had to rewrite most code for part 2. Overall, fairly easy challenge.
| [15](src/solutions/day15.rs) |   ⭐   |   ⭐   | Had to read part two 4 or 5 times to understand the requirements, but otherwise, fairly easy challenge. Kinda hard to optimize in rust, since ideally we'd use a self referential struct. 
| [16](src/solutions/day16.rs) |   ⭐   |   ⭐   | Fairly standard search algorithm implementation. Used a vec as a stack to avoid recursion. Took me a while to debug dumb mistakes in part 1, but otherwise pretty standard
