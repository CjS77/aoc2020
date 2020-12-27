use fnv::FnvHashMap;
use crate::bits::is_set;

pub fn day25a() -> String {
    let crypto = Crypto::new();
    println!("k=8, Pa={}", pubkey_slow(8));
    println!("k=8, Pa={}", crypto.pubkey(8));
    println!("k=11, Pa={}", pubkey_slow(11));
    println!("k=11, Pa={}", crypto.pubkey(11));

    let ka = brute_force(Pa, &crypto);
    let kb = brute_force(Pb, &crypto);
    println!("ka = {}, Pk = {}", ka, crypto.pubkey(ka));
    println!("kb = {}, Pk = {}", kb, crypto.pubkey(kb));
    println!("kb = {}, Pk = {}", kb, pubkey_slow(kb));
    format!("key = {} / {}", dh(ka, Pb), dh(kb, Pa))
}


pub fn day25b() -> String {
    format!("{}", 1)
}

const Pa: usize = 12090988;
const Pb: usize = 240583;

const P: usize = 20201227;
const G: usize = 7;

pub struct Crypto {
    table: FnvHashMap<usize, usize>
}

impl Crypto {
    pub fn new() -> Self {
        let mut g = G;
        let mut table = FnvHashMap::default();
        table.insert(1, G);
        let mut k = 2usize;
        while k < P {
            let pk = mult(2, g);
            table.insert(k, pk);
            k *= 2;
            g = pk;
        }
        println!("{:?}", table);
        Self { table }
    }

    pub fn pubkey(&self, k: usize) -> usize {
        let mut result: usize = 1;
        for i in 0..32 {
            if is_set(k, i) {
                result = (result * self.table.get(&(1 << i)).unwrap()) % P;
            }
        }
        result % P
    }
}

fn mult(k: usize, g: usize) -> usize {
    let mut value = 1usize;
    for _ in 0..k {
        value = (value * g) % P;
    }
    value
}

fn pubkey_slow(k: usize) -> usize {
    mult(k, G)
}

fn scalar_mult(scalar: usize, p: usize) -> usize {
    mult(scalar, p)
}

fn dh(k: usize, pk: usize) -> usize {
    scalar_mult(k, pk)
}

fn brute_force(pk: usize, c: &Crypto) -> usize {
    for k in 1..P {
        if c.pubkey(k) == pk { return k; }
    }
    panic!("No solution")
}