use num::checked_pow;    

fn digit_in(num: u128, avoid_list: &Vec<i32>) -> bool {
    // Check if a digit is present in the number, using modulus
    let mut n = num;
    loop {
        if avoid_list.contains(&((n % 10) as i32)) {return true;}
        n /= 10;
        if n == 0 {return false;}
    }
}

fn search_power(base: u128,
                min_power: usize,
                avoid_list: Vec<i32>) -> Option<u128> {
    let mut p = min_power;
    loop {
        p += 1;
        let calc = checked_pow(base, p)?; // Returns None on overflow
        if !digit_in(calc, &avoid_list) {return Some(calc);}
    }
}

fn main() {
    println!("Searching for powers of 16 without digits 1, 2, 4, or 8...");
    match search_power(16, 4, vec![1, 2, 4, 8]) {
        Some(x) => println!("Found one: {}", x),
        None => println!("Couldn't find any :(")
    }
}
