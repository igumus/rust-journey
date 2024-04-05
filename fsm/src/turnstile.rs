use std::io::{self, BufRead, Write};

#[derive(Copy, Clone)]
pub enum State {
    Locked,
    Unlocked,
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            Self::Locked => String::from("Locked"),
            Self::Unlocked => String::from("Unlocked"),
        }
    }
}

pub enum Event {
    Push,
    Coin,
}

pub struct Turnstile {
    state: State,
}

impl Turnstile {
    pub fn new() -> Self {
        Self {
            state: State::Locked,
        }
    }

    fn next(&mut self, event: Event) {
        match (self.state, event) {
            (State::Locked, Event::Push) => self.state = State::Locked,
            (State::Locked, Event::Coin) => self.state = State::Unlocked,
            (State::Unlocked, Event::Push) => self.state = State::Locked,
            (State::Unlocked, Event::Coin) => self.state = State::Unlocked,
        }
    }

    pub fn simulate(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        print!("{} > ", self.state.to_string());
        stdout.flush().unwrap();
        for line in stdin.lock().lines() {
            match line.unwrap().as_str() {
                "coin" => self.next(Event::Coin),
                "push" => self.next(Event::Push),
                "quit" => return,
                unknown => {
                    eprintln!("Unknown Event: {}", unknown);
                    eprintln!("USAGE: coin,push to change current state");
                }
            }
            print!("{} > ", self.state.to_string());
            stdout.flush().unwrap();
        }
    }
}
