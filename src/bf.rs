use std::io::{self, Read};

/// The BrainFrick struct represents a BrainF*ck interpreter
///
/// mem: A vector of bytes that represents a 'tape' or an array of 0s
/// d: Our data pointer that is used to access `mem`
/// prog: A space in memory for our program that we will read
/// pc: The program counter, used for reading from `prog`
/// exit: When our pc has reached the end of our program we'll mark that we can finish executing
pub struct BrainFrick {
    /// vector of bytes that represents a 'tape' or an array of 0s
    mem: Vec<u8>,
    /// Our data pointer that is used to access `mem`
    d: usize,
    /// A space in memory for our program that we will read
    prog: Vec<u8>,
    /// The program counter, used for reading from `prog`
    pc: usize,
    /// When our pc has reached the end of our program we'll mark that we can finish executing
    pub exit: bool,
}

impl BrainFrick {
    /// Return a new instance of our interpreter struct with our defaults
    pub fn new() -> BrainFrick {
        BrainFrick {
            mem: vec![0; 30_000],
            d: 0,
            prog: vec![0],
            pc: 0,
            exit: false,
        }
    }

    /// Load the contents of a string into our interpreter as a program in memory
    pub fn load(&mut self, program: String) {
        for b in program.bytes() {
            self.prog.push(b);
        }
    }

    /// Evaluates whichever instruction is currently pointed to by our program counter
    /// then increment our program counter to read the next instruction
    pub fn eval(&mut self) {
        match self.prog[self.pc] as char {
            '>' => self.inc_pos(),
            '<' => self.dec_pos(),
            '+' => self.inc_data(),
            '-' => self.dec_data(),
            '.' => self.out_char(),
            ',' => self.in_char(),
            '[' => self.open(),
            ']' => self.close(),
            _ => (),
        }
        self.pc += 1;

        // If we're at the end of our program we can finish
        if self.pc >= self.prog.len() {
            self.exit = true;
        }
    }

    /// Increment the data pointer (position on tape)
    fn inc_pos(&mut self) {
        let before = self.d;
        let result = before.wrapping_add(1);
        self.d = result;
    }

    /// Decrement the data pointer (position on tape)
    fn dec_pos(&mut self) {
        let before = self.d;
        let result = before.wrapping_sub(1);
        self.d = result;
    }

    /// Increment the data found at the data pointer
    fn inc_data(&mut self) {
        let before = self.mem[self.d];
        let result = before.wrapping_add(1);
        self.mem[self.d] = result;
    }

    /// Decrement the data found at the data pointer
    fn dec_data(&mut self) {
        let before = self.mem[self.d];
        let result = before.wrapping_sub(1);
        self.mem[self.d] = result;
    }

    /// Print out the value at the current location of the data pointer
    fn out_char(&self) {
        print!("{}", self.mem[self.d] as char)
    }

    /// Read a character to the current location of the data pointer
    fn in_char(&mut self) {
        let input = io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8);

        match input {
            Some(value) => self.mem[self.d] = value,
            None => (),
        }
    }

    /// Handle an open bracket
    fn open(&mut self) {
        let mut balance = 1;
        if self.mem[self.d] == 0 {
            while balance != 0 {
                self.pc += 1;
                if self.prog[self.pc] == '[' as u8 {
                    balance += 1;
                } else if self.prog[self.pc] == ']' as u8 {
                    balance -= 1;
                }
            }
        }
    }

    /// Handle a closed bracket
    fn close(&mut self) {
        let mut balance = 0;
        loop {
            if self.prog[self.pc] == '[' as u8 {
                balance += 1;
            } else if self.prog[self.pc] == ']' as u8 {
                balance -= 1;
            }
            self.pc -= 1;
            if balance == 0 {
                break;
            }
        }
    }
}
