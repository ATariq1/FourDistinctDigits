extern crate rand; 
use std::collections::HashMap;

fn generate(len:u64) -> u64 {

    let mut ret:u64 = 1;

    for _x in 0..len {
        ret = ret*(10_u64) + (rand::random::<u64>()%(2_u64));
    }

    return ret;
}

fn get_diff(a:u64, b:u64, len:u64) -> u64 {

    let mut result:u64 = if a > b {a -b} else {b-a};
    let result_len = result.to_string().len();

    for _x in 0..(len - (result_len as u64))+1 {
        result = result*10_u64;
    }

    result
}

fn main() {
    
    let len:u64 = 15;
    let n:u64   = 6969;

    let mut mod_map: HashMap<u64, u64> = HashMap::new();

    let answer = loop {

        let current = generate(len);
        let mod_n   = current%n;
        println!("{} {}", current, mod_n);

        if mod_n == 0_u64 {
            break current;
        }

        if mod_map.contains_key(&mod_n) {
            let previous = mod_map.get(&mod_n).unwrap();
            if current != *previous {
                break get_diff(current,*previous,len) ;
            }
        } else {
            mod_map.insert(mod_n, current);
        }

    };

    println!("{}", answer);
}
