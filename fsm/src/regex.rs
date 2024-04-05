use std::{ops::Range, process};

const COL_SIZE: usize = 130;
const EOE: usize = 129;

#[derive(Default, Copy, Clone)]
struct Action {
    next: usize,
    offset: i32,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        format!("({},{})", self.next, self.offset)
    }
}

struct State {
    ts: [Action; COL_SIZE],
}

impl State {
    fn new() -> Self {
        State {
            ts: [Action::default(); COL_SIZE],
        }
    }

    fn fill(&mut self, range: Range<usize>, state: usize) {
        for i in range {
            self.ts[i] = Action {
                next: state,
                offset: 1 as i32,
            };
        }
    }
}

#[derive(Default)]
pub struct Regex {
    cs: Vec<State>,
}

impl Regex {
    fn new() -> Self {
        Self { cs: Vec::new() }
    }

    fn push(&mut self, col: State) {
        self.cs.push(col);
    }

    fn push_eoe(&mut self) {
        // State EOF
        let mut col = State::new();
        col.ts[EOE] = Action {
            next: self.cs.len() + 1,
            offset: 1,
        };
        self.push(col);
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for symbol in 0..130 {
            print!("{:03} =>", symbol);
            for column in self.cs.iter() {
                print!("{} ", column.ts[symbol].to_string());
            }
            println!();
        }
    }

    fn matches(&self, input: &str) -> bool {
        let mut state: usize = 1;
        let mut head: usize = 0;
        let chars = input.chars().collect::<Vec<_>>();
        let n = chars.len();
        while 0 < state && state < self.cs.len() && head < n {
            let action = self.cs[state].ts[chars[head] as usize];
            state = action.next;
            head = (head as i32 + action.offset) as usize;
        }
        if state == 0 {
            return false;
        }
        if state < self.cs.len() {
            state = self.cs[state].ts[EOE].next;
        }
        state >= self.cs.len()
    }

    fn compile(src: &str) -> Self {
        let mut regex = Regex::new();

        // State 0 - Fail State
        regex.push(State::new());

        let mut inside_range = false;
        let mut ranges: Vec<usize> = Vec::new();

        for c in src.chars() {
            match c {
                '$' => {
                    // State EOF
                    regex.push_eoe();
                }
                '.' => {
                    let mut col = State::new();
                    col.fill(32..127, regex.cs.len() + 1);
                    regex.push(col);
                }
                '*' => {
                    let n = regex.cs.len();
                    let col = regex.cs.last_mut().unwrap();
                    for t in col.ts.iter_mut() {
                        if (*t).next == n {
                            (*t).next = n - 1;
                        } else if (*t).next == 0 {
                            (*t).next = n;
                            (*t).offset = 0;
                        } else {
                            unreachable!();
                        }
                    }
                }
                '[' => {
                    inside_range = true;
                }
                ']' => {
                    inside_range = false;
                    let mut col = State::new();
                    assert!(ranges.len() == 2, "Unbalanced range");

                    // checks overflow for last part of the range
                    let mut last = ranges[1] + 1;
                    if last >= COL_SIZE {
                        last = ranges[1];
                    }

                    col.fill(ranges[0]..last, regex.cs.len() + 1);
                    regex.push(col);
                    ranges.clear();
                }
                _ => {
                    if inside_range {
                        if c != '-' {
                            ranges.push(c as usize);
                        }
                    } else {
                        let mut col = State::new();
                        col.ts[c as usize] = Action {
                            next: regex.cs.len() + 1,
                            offset: 1,
                        };
                        regex.push(col);
                    }
                }
            };
        }

        if ranges.len() != 0 {
            eprintln!("ERROR: Can not compile `{}` Unbalanced range operation occured, review your expression", src);
            process::exit(1);
        }

        if !src.ends_with("$") {
            regex.push_eoe();
        }

        regex
    }

    fn simulate(debug: bool, src: &str, inputs: Vec<&str>) {
        let regex = Regex::compile(src);
        if debug {
            regex.dump();
        }
        for input in inputs {
            println!("{} matches with {} => {}", src, input, regex.matches(input));
        }
    }

    pub fn simulate_basic() {
        let src = "abc$";
        println!("\n----- Simulating Regex Basic Operation");
        Regex::simulate(false, src, vec!["hello, world", "abc", "dbc", "adb"]);
        println!("----------------------------------------");
    }

    pub fn simulate_dot() {
        let src = ".bc";
        println!("\n----- Simulating Regex Dot Operation");
        Regex::simulate(
            false,
            src,
            vec![
                "abc",
                "bbc",
                "cbd",
                "cbt",
                "cbc",
                "Hello World",
                "0bc",
                "1bc",
                "2bc",
                "3bc",
                "4bc",
            ],
        );
        println!("--------------------------------------");
    }

    pub fn simulate_range() {
        let src = "[0-3]bc";
        println!("\n----- Simulating Regex Range Operation");
        Regex::simulate(
            false,
            src,
            vec![
                "abc",
                "bbc",
                "cbd",
                "cbt",
                "cbc",
                "Hello World",
                "0bc",
                "1bc",
                "2bc",
                "3bc",
                "4bc",
            ],
        );
        println!("--------------------------------------");
    }

    pub fn simulate_star() {
        let src = ".*";
        Regex::simulate(
            false,
            src,
            vec![
                "abc",
                "bbc",
                "cbd",
                "cbt",
                "cbc",
                "Hello World",
                "0bc",
                "1bc",
                "2bc",
                "3bc",
                "4bc",
            ],
        );
        println!("--------------------");

        let src = "a*bc";
        Regex::simulate(
            false,
            src,
            vec![
                "abc",
                "bbc",
                "cbd",
                "cbt",
                "cbc",
                "Hello World",
                "0bc",
                "1bc",
                "2bc",
                "3bc",
                "4bc",
            ],
        );
    }
}
