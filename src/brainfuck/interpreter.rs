use std::io::Write;

#[derive(Debug)]
pub struct BFInterpreter {
    tape: String,
    memory: Vec<u8>,
    tape_index: usize,
    memory_index: usize,
}
impl BFInterpreter {
    pub fn new(tape: &str) -> BFInterpreter {
        // create new BFInterpreter Instance
        let instance = BFInterpreter {
            tape: tape.to_string(),
            memory: vec![0u8],
            tape_index: 0,
            memory_index: 0,
        };
        return instance;
    }
    pub fn run(&mut self) {
        loop {
            self.step_execute();
            if self.tape_index >= self.tape.len() {
                return;
            }
        }
    }
    fn step_execute(&mut self) {
        let current_char = self.tape.chars().nth(self.tape_index).unwrap();
        match current_char {
            '>' => self.langle(),
            '<' => self.rangle(),
            '+' => self.plus(),
            '-' => self.minus(),
            '.' => self.period(),
            ',' => self.comma(),
            '[' => self.lbracket(),
            ']' => self.rbracket(),
            _ => {
                self.tape_index += 1;
            }
        };
    }
    fn langle(&mut self) {
        // processing '>'
        self.memory_index += 1;
        if self.memory_index >= self.memory.len() {
            self.memory.push(0);
        }
        self.tape_index += 1;
    }
    fn rangle(&mut self) {
        // processing '<'
        self.memory_index -= 1;
        self.tape_index += 1;
    }
    fn plus(&mut self) {
        // processing '+'
        let index = self.memory_index;
        self.memory[index] += 1;
        self.tape_index += 1;
    }
    fn minus(&mut self) {
        // processing '-'
        let index = self.memory_index;
        self.memory[index] -= 1;
        self.tape_index += 1;
    }
    fn period(&mut self) {
        // processing '.'
        let index = self.memory_index;
        let c = self.memory[index] as char;
        print!("{}", c);
        std::io::stdout().flush().unwrap();
        self.tape_index += 1;
    }
    fn comma(&mut self) {
        // processing ','
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let c = input.bytes().nth(0).expect("Failed to read input");
        let index = self.memory_index;
        self.memory[index] = c;
        self.tape_index += 1;
    }
    fn lbracket(&mut self) {
        // processing '['
        let index = self.memory_index;
        if self.memory[index] != 0 {
            self.tape_index += 1;
            return;
        }
        let paired_index = self
            .paired_bracket(self.tape_index, BracketPair::LEFT)
            .unwrap();
        self.tape_index = paired_index + 1;
    }
    fn rbracket(&mut self) {
        // processing ']'
        let index = self.memory_index;
        if self.memory[index] == 0 {
            self.tape_index += 1;
            return;
        }
        let paired_index = self
            .paired_bracket(self.tape_index, BracketPair::RIGHT)
            .unwrap();
        self.tape_index = paired_index + 1;
    }
    fn paired_bracket(&self, bracket_index: usize, bracket_type: BracketPair) -> Option<usize> {
        // return paired brakcet index of `bracket_index`
        let mut index_stack = Vec::<usize>::new();
        for index in 0..self.tape.len() {
            let index_char = self.tape.chars().nth(index).unwrap();

            if index_char != '[' && index_char != ']' {
                continue;
            }
            if index_char == '[' {
                index_stack.push(index);
                continue;
            }
            let popped = index_stack.pop().unwrap();
            match bracket_type {
                BracketPair::LEFT => {
                    if popped == bracket_index {
                        return Some(index);
                    }
                }
                BracketPair::RIGHT => {
                    if index == bracket_index {
                        return Some(popped);
                    }
                }
            };
        }
        return None;
    }
}
enum BracketPair {
    LEFT,
    RIGHT,
}
#[test]
fn test_paired_bracket() {
    let instance = BFInterpreter::new("[[[][]]>>>]");
    assert_eq!(instance.paired_bracket(0, BracketPair::LEFT).unwrap(), 10);
    assert_eq!(instance.paired_bracket(1, BracketPair::LEFT).unwrap(), 6);
    assert_eq!(instance.paired_bracket(3, BracketPair::RIGHT).unwrap(), 2);
    assert_eq!(instance.paired_bracket(10, BracketPair::RIGHT).unwrap(), 0);
}
#[test]
fn test_operation() {
    let mut instance = BFInterpreter::new("++>+++++[<+>-]");
    instance.run();
    println!("{:?}", instance.memory);
    assert_eq!(instance.memory[0], 7);
    assert_eq!(instance.memory[1], 0);
}
#[test]
fn test_product() {
    let tape = "++>+++++[<+>-]++++ ++++ [ < +++ +++ > -] <";
    let mut instance = BFInterpreter::new(tape);
    instance.run();
    assert_eq!(instance.memory[0] as char, '7');
}
