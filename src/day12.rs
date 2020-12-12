use std::fs;

pub fn day12a() -> String {
    let mut nav = NavComputer::default();
    let ins = read_data();
    let pos = nav.exec(&ins);
    println!("{}, {}", pos.0, pos.1);
    format!("{:?}", nav.travelled())
}

pub fn day12b() -> String {
    let mut nav = NavComputer::new();
    let ins = read_data();
    let pos = nav.exec_wp(&ins);
    println!("{}, {}", pos.0, pos.1);
    format!("{:?}", nav.travelled())
}

fn read_data() -> Vec<Instruction> {
    let values = fs::read_to_string("assets/day12.txt").expect("Could not load file");
    values
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(Instruction::from_str)
        .collect()
}

#[derive(Default)]
struct NavComputer {
    origin: (i32, i32),
    position: (i32, i32),
    waypoint: (i32, i32),
    facing: i32
}

impl NavComputer {
    pub fn new() -> Self {
        Self {
            waypoint: (10, 1),
            .. Self::default()
        }
    }

    pub fn travelled(&self) -> i32 {
        (self.origin.0 - self.position.0).abs() + (self.origin.1 - self.position.1).abs()
    }

    pub fn mov_wp(&mut self, inst: &Instruction) {
        match inst {
            Instruction::North(val) => {
                self.waypoint.1 += val;
            }
            Instruction::South(val) => {
                self.waypoint.1 -= val;
            }
            Instruction::East(val) => {
                self.waypoint.0 += val;
            }
            Instruction::West(val) => {
                self.waypoint.0 -= val;
            }
            Instruction::Left(val) => {
                self.waypoint = match val {
                    0 | 360 => self.waypoint,
                    90 => (-self.waypoint.1, self.waypoint.0),
                    180 => (-self.waypoint.0, -self.waypoint.1),
                    270 => (self.waypoint.1, -self.waypoint.0),
                    _ => panic!(format!("Invalid left {}", val))
                }
            }
            Instruction::Right(val) => {
                self.waypoint = match val {
                    0 | 360 => self.waypoint,
                    90 => (self.waypoint.1, -self.waypoint.0),
                    180 => (-self.waypoint.0, -self.waypoint.1),
                    270 => (-self.waypoint.1, self.waypoint.0),
                    _ => panic!(format!("Invalid right {}", val))
                }
            }
            Instruction::Fwd(val) => {
                self.position.0 += self.waypoint.0 * val;
                self.position.1 += self.waypoint.1 * val;
            }
        };
    }

    pub fn mov(&mut self, inst: &Instruction) {
        match inst {
            Instruction::North(val) => {
                self.position.1 += val;
            }
                Instruction::South(val) => {
                    self.position.1 -= val;
                }
            Instruction::East(val) => {
                self.position.0 += val;
            }
                Instruction::West(val) => {
                    self.position.0 -= val;
                }
            Instruction::Left(val) => {
                self.facing = (self.facing - val/90 + 4) % 4;
            }
                Instruction::Right(val) => {
                    self.facing = (self.facing + val/90) % 4;
                }
            Instruction::Fwd(val) => {
                match self.facing {
                    0 => self.mov(&Instruction::East(*val)),
                    1 => self.mov(&Instruction::South(*val)),
                    2 => self.mov(&Instruction::West(*val)),
                    3 => self.mov(&Instruction::North(*val)),
                    _ => {
                        panic!("{:?}", self.facing);

                    }
                }
            }
        }
    }

    pub fn exec(&mut self, inst: &[Instruction]) -> (i32, i32) {
        inst.iter().for_each(|i| self.mov(i));
        self.position
    }

    pub fn exec_wp(&mut self, inst: &[Instruction]) -> (i32, i32) {
        inst.iter().for_each(|i| self.mov_wp(i));
        self.position
    }
}

#[derive(Clone)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Fwd(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let ins = s.as_bytes()[0] as char;
        let val = s[1..].parse::<i32>().unwrap();
        match ins {
            'N'  => Instruction::North(val),
            'S'  => Instruction::South(val),
            'E'  => Instruction::East(val),
            'W'  => Instruction::West(val),
            'L'  => Instruction::Left(val),
            'R'  => Instruction::Right(val),
            'F'  => Instruction::Fwd(val),
            _ => panic!("Unknown instruction"),
        }
    }
}