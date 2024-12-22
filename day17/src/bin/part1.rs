fn main() {
    let mut m = Machine {
        A: 63281501,
        B: 0,
        C: 0,
        IP: 0,
    };
    let program: Vec<u8> = vec![2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0];
    let out = m.run(&program);
    let mut out_string = String::new();
    for x in out {
        let next = format!("{x},");
        out_string.push_str(&next);
    }
    println!("{}", out_string);
}

#[derive(Debug, Eq, PartialEq)]
struct Machine<T> {
    #[allow(non_snake_case)]
    pub A: T,
    #[allow(non_snake_case)]
    pub B: T,
    #[allow(non_snake_case)]
    pub C: T,
    // Instruction pointer.
    IP: usize,
}

#[derive(Debug)]
enum Instr {
    // Division using A, store A.
    Adv = 0,
    //Bitwise xor
    Bxl = 1,
    // Keep bottom 3 bits ( operand %8)
    // store B
    Bst = 2,
    Jnz = 3,
    // B XOR C store in B
    Bxc = 4,
    Out = 5,
    // Division with A store B.
    Bdv = 6,
    // Division with A store C.
    Cdv = 7,
}

impl TryFrom<u8> for Instr {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let i = match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => return Err(String::from("Could not decode Intr")),
        };
        Ok(i)
    }
}

impl Machine<u32> {
    fn combo_value(&self, operand: u8) -> u32 {
        match operand {
            4 => self.A,
            5 => self.B,
            6 => self.C,
            7 => {
                panic!("invalid operand")
            }
            d => {
                // 0..3
                d.into()
            }
        }
    }
    fn run(&mut self, prog: &[u8]) -> Vec<u32> {
        let mut out = vec![];
        'fetch_execute: loop {
            let opcode_byte = match prog.get(self.IP) {
                Some(next) => *next,
                // Halt
                None => break 'fetch_execute,
            };
            let opcode = Instr::try_from(opcode_byte).expect("Could not decode");
            println!("opcode {:#?}", opcode);
            let literal_operand = prog[self.IP + 1usize];

            match opcode {
                Instr::Adv => {
                    let combo = self.combo_value(literal_operand);
                    let demoninator = 2u32.pow(combo);
                    println!("Adv :  Anext = {} / {}", self.A, demoninator);
                    self.A /= demoninator;
                }
                Instr::Bst => {
                    let combo = self.combo_value(literal_operand);
                    self.B = combo % 8;
                }
                Instr::Bxl => {
                    self.B ^= literal_operand as u32;
                }
                Instr::Jnz => {
                    println!("jnz {}", self.A);
                    if self.A == 0 {
                        break 'fetch_execute;
                    } else {
                        self.IP = literal_operand as usize;
                        //... skip auto increment.
                        continue;
                    }
                }
                Instr::Bxc => self.B ^= self.C,
                Instr::Out => {
                    let combo = self.combo_value(literal_operand);
                    out.push(combo % 8);
                }
                Instr::Bdv => {
                    let combo = self.combo_value(literal_operand);
                    let demoninator = 2u32.pow(combo);
                    self.B = self.A / demoninator;
                }
                Instr::Cdv => {
                    let combo = self.combo_value(literal_operand);
                    let demoninator = 2u32.pow(combo);
                    self.C = self.A / demoninator;
                }
            }
            self.IP += 2;
        }

        out
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut m = Machine {
            A: 0,
            B: 0,
            C: 9,
            IP: 0,
        };
        m.run(&vec![2, 6]);
        assert_eq!(m.B, 1);

        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut m = Machine {
            A: 10,
            B: 0,
            C: 9,
            IP: 0,
        };
        assert_eq!(m.run(&vec![5, 0, 5, 1, 5, 4]), vec![0, 1, 2]);

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut m = Machine {
            A: 2024,
            B: 0,
            C: 9,
            IP: 0,
        };
        assert_eq!(
            m.run(&vec![0, 1, 5, 4, 3, 0]),
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(m.A, 0);

        // If register B contains 29, the program 1,7 would set register B to 26.
        let mut m = Machine {
            A: 0,
            B: 29,
            C: 0,
            IP: 0,
        };
        let prog = vec![1, 7];
        m.run(&prog);
        assert_eq!(m.B, 26);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        let mut m = Machine {
            A: 0,
            B: 2024,
            C: 43690,
            IP: 0,
        };
        m.run(&vec![4, 0]);
        assert_eq!(m.B, 44354);
    }
}
