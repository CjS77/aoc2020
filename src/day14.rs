use std::collections::HashMap;
use regex::Regex;
use crate::bits::{bit_array, assign_bits, set_bit, clear_bit, read_data};

pub fn day14a() -> String {
    let lines = read_data("assets/day14.txt");
    let mut machine = Machine::read_program(lines);
    machine.exec();
    let res = machine.mem.values().into_iter().sum::<usize>();
    format!("{}", res)
}

pub fn day14b() -> String {
    let lines = read_data("assets/day14.txt");
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
    let floating_indices = mask.iter()
        .enumerate()
        .filter(|(_, &b)| matches!(b, BitMap::Nop))
        .map(|(i, _)| 35 - i)
        .collect::<Vec<usize>>();
    // Construct bitmask
    for combo_val in 0..(1 << floating_indices.len()) {
        let arr = bit_array(combo_val);
        let iter = floating_indices.iter().copied()
            .zip(arr.iter().copied());
        result.push(assign_bits(base, iter));
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
                BitMap::One => result = set_bit(result, 35 - i),
                BitMap::Zero => result = clear_bit(result, 35 - i),
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
                BitMap::One => result = set_bit(result, 35 - i),
                BitMap::Nop => result = clear_bit(result, 35 - i),
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
