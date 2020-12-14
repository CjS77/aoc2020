use std::fs;
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

pub fn day14a() -> String {
    let lines = read_data();
    let mut machine = Machine::read_program(lines);
    machine.exec();
    let res = machine.mem.values().into_iter().sum::<usize>();
    format!("{}", res)
}

pub fn day14b() -> String {
    let lines = read_data();
    let mut machine = Machine::read_program(lines);
    machine.exec2();
    let res = machine.mem.values().into_iter().sum::<usize>();
    format!("{}", res)
}

struct Machine {
    instructions: Vec<Instruction>,
    mem: HashMap<usize, usize>,
}

impl Machine {
    pub fn read_program(lines: Vec<String>) -> Self {
        let mut instructions = Vec::new();
        let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        for s in lines {
            if s.starts_with("mask") {
                let map = BitMap::from_str(&s[7..]);
                instructions.push(Instruction::Mask(map));
            } else {
                let cap = re.captures(&s).unwrap();
                let index = cap[1].parse::<usize>().unwrap();
                let val = cap[2].parse::<usize>().unwrap();
                instructions.push(Instruction::Mem((index, val)));
            }
        }
        Self {
            instructions,
            mem: HashMap::new(),
        }
    }

    pub fn exec(&mut self) {
        self.mem = HashMap::new();
        let mut mask = [BitMap::Nop; 36];
        for ins in &self.instructions {
            match ins {
                Instruction::Mask(m) => mask.copy_from_slice(m),
                Instruction::Mem((index, val)) => {
                    let v = apply_mask(*val, &mask);
                    let _ = self.mem.insert(*index, v);
                }
            }
        }
    }

    pub fn exec2(&mut self) {
        let mut mem = HashMap::new();
        let mut mask = [BitMap::Nop; 36];
        for ins in &self.instructions {
            match ins {
                Instruction::Mask(m) => mask.copy_from_slice(m),
                Instruction::Mem((index, val)) => {
                    let addresses = apply_mask_address(*index, &mask);
                    addresses.into_iter().for_each(|addr| { mem.insert(addr, *val); })
                }
            }
        }
        self.mem = mem;
    }
}

fn apply_mask_address(index: usize, mask: &[BitMap; 36]) -> Vec<usize> {
    let base = apply_mask2(index, mask);
    let mut result = Vec::new();
    // A vec of bit values 2, 8, 16 etc
    let floating_index_values = mask.iter()
        .enumerate()
        .filter(|(_, &b)| matches!(b, BitMap::Nop))
        .map(|(b, _)| 1 << (35 - b))
        .collect::<Vec<usize>>();
    // Construct bitmask
    for combo_val in 0..1 << floating_index_values.len() {
        let mask = floating_index_values.iter()
            .enumerate()
            .fold(0usize, |m, (i, v)| {
                if (1 << i) & combo_val > 0 { m + *v } else { m }
            });
        result.push(mask + base);
    }
    result
}

fn apply_mask(val: usize, mask: &[BitMap; 36]) -> usize {
    let mut result = val;
    mask.iter()
        .enumerate()
        .for_each(|(i, b)| {
            match b {
                BitMap::Nop => {}
                BitMap::One => result = result | (1 << (35 - i)),
                BitMap::Zero => {
                    let mask = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111 ^ (1 << (35 - i));
                    result = mask & result;
                }
            }
        });
    result
}

fn apply_mask2(val: usize, mask: &[BitMap; 36]) -> usize {
    let mut result = val;
    mask.iter()
        .enumerate()
        .for_each(|(i, b)| {
            match b {
                BitMap::Zero => {}
                BitMap::One => result = result | (1 << (35 - i)),
                BitMap::Nop => {
                    let mask = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111 ^ (1 << (35 - i));
                    result = mask & result;
                }
            }
        });
    result
}

#[derive(Clone)]
enum Instruction {
    Mask([BitMap; 36]),
    Mem((usize, usize)),
}

#[derive(Clone, Copy, Debug)]
enum BitMap {
    One,
    Zero,
    Nop,
}

impl BitMap {
    pub fn from_str(s: &str) -> [BitMap; 36] {
        let vec = s.chars().map(|c| {
            match c {
                'X' => BitMap::Nop,
                '1' => BitMap::One,
                '0' => BitMap::Zero,
                _ => panic!("Unknown bitmask {}", c)
            }
        }).collect::<Vec<Self>>();
        let mut result = [BitMap::Nop; 36];
        result.copy_from_slice(&vec);
        result
    }
}

fn read_data() -> Vec<String> {
    let values = fs::read_to_string("assets/day14.txt").expect("Could not load file");
    values
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
}