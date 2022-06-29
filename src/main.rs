use rand::prelude::*;

fn gen_rand_arr<const SIZE: usize>(rng: &mut ThreadRng, s: u8, n: u8) -> [u8; SIZE] {
    let mut arr = [0; SIZE];
    for x in &mut arr {
        *x = rng.gen_range(s..n);
    }
    arr
}

fn is_win(choice: &[u8; 6], winning: &[u8; 6]) -> bool {
    for num in winning.iter() {
        if !choice.contains(num) {
            return false;
        }
    }
    true
}

fn main() {
    let mut rng = thread_rng();

    let mut choice: [u8; 6];
    let mut winning: [u8; 6];
    let mut tries: u64 = 0;

    println!("Trying...");
    
    loop {
        choice = gen_rand_arr(&mut rng, 1, 58);
        winning = gen_rand_arr(&mut rng, 1, 58);

        tries += 1;

        if is_win(&choice, &winning) {
            println!("Won after {} tries!", tries);
            break;
        }
    }
}
