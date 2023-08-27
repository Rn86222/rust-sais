mod sais;
use sais::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn check_suffix_array(suffix_array: &Vec<usize>, string: String) {
    let length = suffix_array.len();
    for i in 0..(length - 1) {
        if string[suffix_array[i]..] > string[suffix_array[i + 1]..] {
            println!("failed.");
            println!("index = {}", i);
            return;
        }
        print!(
            "\rChecking... {:2}%",
            (i as f32 / length as f32 * 100.) as i32
        );
    }
    println!("\nAll passed!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if argc <= 1 {
        eprintln!("Usage: ./sais filename [-c]");
        eprintln!("   -c: check validity of suffix array.");
        return;
    }
    if let Ok(mut file) = File::open(args[1].clone()) {
        let mut contents = String::new();

        if let Ok(_) = file.read_to_string(&mut contents) {
            let start_time = Instant::now();
            let string = contents.chars().collect();
            print!("Start... ");
            let suffix_array = sais(&string);
            let end_time = start_time.elapsed();
            println!("End {:?}", end_time);
            if argc >= 3 && args[2] == "-c" {
                check_suffix_array(&suffix_array, string);
            }
        } else {
            eprintln!("Cannot read file.");
        }
    } else {
        eprintln!("No such file.");
    }
}
