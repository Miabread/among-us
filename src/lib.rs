use std::{
    io::{stdin, stdout, Read, Write},
    str::FromStr,
};

use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Evidence {
    Sus,
    Vented,
    Sussy,
    Electrical,
    Who,
    Where,
    Voted,
    Red,
    Blue,
    Purple,
    Green,
    Yellow,
    Cyan,
    Black,
    White,
    Brown,
    Lime,
    Pink,
    Orange,
}

impl FromStr for Evidence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Evidence::*;
        Ok(match s {
            "SUS" => Sus,
            "VENTED" => Vented,
            "SUSSY" => Sussy,
            "ELECTRICAL" => Electrical,
            "WHO?" => Who,
            "WHERE?" => Where,
            "VOTED" => Voted,
            "RED" => Red,
            "BLUE" => Blue,
            "PURPLE" => Purple,
            "GREEN" => Green,
            "YELLOW" => Yellow,
            "CYAN" => Cyan,
            "BLACK" => Black,
            "WHITE" => White,
            "BROWN" => Brown,
            "LIME" => Lime,
            "PINK" => Pink,
            "ORANGE" => Orange,
            _ => panic!("SUSSY BAKA"),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct EmergencyMeeting {
    general: u8,
    control: u8,
    stack: Vec<u8>,
    accused: Option<Evidence>,
    evidence: Vec<Evidence>,
    current: usize,
}

impl EmergencyMeeting {
    pub fn from_evidence(evidence: &str) -> EmergencyMeeting {
        let evidence = evidence
            .split_whitespace()
            .map(|token| token.parse().unwrap())
            .collect();

        EmergencyMeeting {
            evidence,
            ..Default::default()
        }
    }

    pub fn execute(&mut self) {
        while self.execute_control() {
            dbg!(&self);
        }
    }

    fn execute_control(&mut self) -> bool {
        match self.evidence[self.current] {
            Evidence::Sus => self.execute_color(),
            Evidence::Vented => self.control = self.control.wrapping_add(10),
            Evidence::Sussy => self.control = self.control.wrapping_sub(1),
            Evidence::Electrical => self.control = 0,
            Evidence::Who => {
                if *self.stack.last().unwrap() == self.control {
                    while self.evidence[self.current] != Evidence::Where {
                        self.current += 1
                    }
                    self.current += 1
                }
            }
            Evidence::Where => {
                if *self.stack.last().unwrap() != self.control {
                    while self.evidence[self.current] != Evidence::Who {
                        self.current -= 1
                    }
                }
            }
            Evidence::Voted => return false,
            token => self.accused = Some(token),
        }
        true
    }

    fn execute_color(&mut self) {
        match self.accused.unwrap() {
            Evidence::Red => self.general = self.general.wrapping_add(1),
            Evidence::Blue => self.stack.push(self.general),
            Evidence::Purple => {
                self.stack.pop();
            }
            Evidence::Green => {
                print!("{}", char::from(*self.stack.last().unwrap()));
                stdout().flush().unwrap();
            }
            Evidence::Yellow => {
                let mut input = [0u8];
                stdin().read_exact(&mut input).unwrap();
                self.stack.push(input[0]);
            }
            Evidence::Cyan => {
                for _ in 0..=thread_rng().gen_range(0..=self.general) {
                    let _ = self.stack.pop();
                }
            }
            Evidence::Black => print!("{}", self.stack.last().unwrap()),
            Evidence::White => self.general = self.general.wrapping_sub(1),
            Evidence::Brown => self.general = *self.stack.last().unwrap(),
            Evidence::Lime => {
                *self.stack.last_mut().unwrap() = self.stack.last().unwrap().wrapping_mul(2)
            }
            Evidence::Pink => self.control = 0,
            Evidence::Orange => self.control = self.control.wrapping_add(10),
            _ => panic!("BAKA SUSSY"),
        }
    }
}
