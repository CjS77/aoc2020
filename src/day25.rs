use fnv::FnvHashMap;
use crate::bits::is_set;
use std::time;

const Pa: usize = 12090988;
const Pb: usize = 240583;

pub fn day25a() -> String {

    let mut crypto = None;
    let t = time_it(|| {
        crypto.replace(Crypto::new());
    });
    println!("Set up crypto table in {} µs", t);
    let crypto = crypto.unwrap();

    let mut pa=0;
    let mut pb=0;
    let mut pc=0;
    let t = time_it(|| {
        pa = pubkey_slow(8);
        pb = pubkey_slow(11);
        pc = pubkey_slow(18_365_783);
    });
    println!("Pa = {}, Pb = {}, Pc = {}, Using pubkey_slow: {} µs", pa, pb, pc, t);

    let t = time_it(|| {
        pa = crypto.pubkey(8);
        pb = crypto.pubkey(11);
        pc = crypto.pubkey(18_365_783);
    });
    println!("Pa = {}, Pb = {}, Pc = {}, Using crypto.pubkey: {} µs", pa, pb, pc, t);

    // let ka = brute_force(Pa, &crypto);
    let mut ka = 0;

    let t = time_it(|| {
        ka = naiive_crack(Pa, &crypto);
    });
    println!("ka = {}, key = {}, Using naive_crack: {} µs", ka, dh(ka, Pb), t);

    let t = time_it(|| {
        ka = quick_crack(Pa);
    });
    println!("ka = {}, key = {}, Using quick_crack: {} µs", ka, dh(ka, Pb), t);

    dh(ka, Pb).to_string()
}

fn time_it<F: FnOnce()>(f: F) -> u64 {
    let now = time::Instant::now();
    f();
    now.elapsed().as_micros() as u64
}

const P: usize = 20201227;
const G: usize = 7;

fn quick_crack(pubkey: usize) -> usize {
    let mut k = 1;
    let mut pk = G;
    while pk != pubkey {
        k += 1;
        pk = G*pk % P;
    }
    k
}


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
        // println!("{:?}", table);
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

fn naiive_crack(pk: usize, c: &Crypto) -> usize {
    for k in 1..P {
        if c.pubkey(k) == pk { return k; }
    }
    panic!("No solution")
}