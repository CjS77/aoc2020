use std::fs;

pub fn day8a() -> String {
    let stack = read_data();
    let mut vm = VM::new();
    vm.load_instructions(stack);
    match vm.run() {
        Ok(v) => format!("Ok - {}", v),
        Err(v) => format!("Infinite loop - {}", v),
    }
}

pub fn day8b() -> String {
    let stack = read_data();
    let mut vm = VM::new();
    vm.load_instructions(stack);
    vm.self_fix()
}

#[derive(Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let ins = &s[0..3];
        let val = s[4..].parse::<i32>().unwrap();
        match ins {
            "acc" => Instruction::Acc(val),
            "jmp" => Instruction::Jmp(val),
            "nop" => Instruction::Nop(val),
            _ => panic!("Unknown instruction"),
        }
    }
}

struct VM {
    stack: Vec<Instruction>,
    pointer: usize,
    accumulator: i32,
    marker: Vec<bool>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            accumulator: 0,
            pointer: 0,
            marker: Vec::new(),
        }
    }

    pub fn load_instructions(&mut self, stack: Vec<String>) {
        self.stack = stack.iter()
            .map(|s| Instruction::from_str(s.as_str()))
            .collect();
    }

    pub fn run(&mut self) -> Result<i32, i32> {
        self.accumulator = 0;
        self.pointer = 0;
        self.marker = vec![false; self.stack.len()];
        loop {
            match self.get_next() {
                Ok(Some(next)) => self.execute(next),
                Ok(None) => { println!("Ok"); break; },
                Err(_) => return Err(self.accumulator),
            }
        }
        Ok(self.accumulator)
    }

    pub fn self_fix(&mut self) -> String {
        for (i, ins) in self.stack.iter().enumerate() {
            let mut new_stack = self.stack.clone();
            match ins {
                Instruction::Acc(_) => continue,
                Instruction::Jmp(val) => { new_stack[i] = Instruction::Nop(*val); },
                Instruction::Nop(val) => { new_stack[i] = Instruction::Jmp(*val); },
            }
            let mut vm = VM::new();
            vm.stack = new_stack;
            if let Ok(val) = vm.run() {
                return format!("Fixed - {}", val);
            }
        }
        "No solution".to_string()
    }

    fn get_next(&mut self) -> Result<Option<Instruction>, ()> {
        let s = match self.stack.get(self.pointer) {
            Some(s) => s.clone(),
            None => return Ok(None),
        };
        if self.marker[self.pointer] {
            return Err(());
        }
        Ok(Some(s))
    }

    fn execute(&mut self, inst: Instruction) {
        self.marker[self.pointer] = true;
        match inst {
            Instruction::Acc(val) => {
                self.accumulator += val;
                self.pointer += 1
            }
            Instruction::Jmp(val) => self.pointer = (self.pointer as i32 + val) as usize,
            Instruction::Nop(_) => self.pointer += 1,
        }
    }
}

fn read_data() -> Vec<String> {
    fs::read_to_string("assets/day8.txt")
        .expect("Could not read file")
        .lines()
        .filter(|&s| !s.is_empty())
        .map(String::from)
        .collect()
}