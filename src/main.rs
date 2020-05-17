use rand::prelude::{thread_rng, Rng};
use std::cmp::Ordering;
use std::time;

#[macro_use]
extern crate log;
extern crate simple_logger;

/// Print progress
fn print_progress(index: isize) {
    if index % 10000 == 0 {
        trace!("Completed {} iterations.", index);
    }
}

/// Generate a random whole number between 1 and 20 and return it
fn roll_d20() -> u8 {
    thread_rng().gen_range(1, 21)
}

/// Roll twice and return the higher result
fn roll_adv() -> u8 {
    let roll_1 = roll_d20();
    let roll_2 = roll_d20();

    match roll_1.cmp(&roll_2) {
        Ordering::Less => roll_2,
        Ordering::Greater => roll_1,
        Ordering::Equal => roll_1,
    }
}

/// Roll twice and return the lower result
fn roll_dis() -> u8 {
    let roll_1 = roll_d20();
    let roll_2 = roll_d20();

    match roll_1.cmp(&roll_2) {
        Ordering::Less => roll_1,
        Ordering::Greater => roll_2,
        Ordering::Equal => roll_1,
    }
}

/// Roll at disadvantage twice and get the higher result
fn roll_adv_dis() -> u8 {
    let roll_1 = roll_dis();
    let roll_2 = roll_dis();

    match roll_1.cmp(&roll_2) {
        Ordering::Less => roll_2,
        Ordering::Greater => roll_1,
        Ordering::Equal => roll_1,
    }
}

/// Roll at advantage twice and get the lower result
fn roll_dis_adv() -> u8 {
    let roll_1 = roll_adv();
    let roll_2 = roll_adv();

    match roll_1.cmp(&roll_2) {
        Ordering::Less => roll_1,
        Ordering::Greater => roll_2,
        Ordering::Equal => roll_1,
    }
}

fn get_vec_average(vals: &[u8]) -> u8 {
    let mut sum = 0;
    let length = vals.len();
    if length == 0 {
        error!("How is the length of this vector 0?");
        return 0;
    }
    for val in vals.iter() {
        sum += *val as usize;
    }
    (sum as usize / length) as u8
}

fn get_percent_occurrence(num: u8, vect: Vec<u8>) -> f32 {
    let mut count = 0;
    for i in vect.iter() {
        if i == &num {
            count += 1
        };
    }
    count as f32 / vect.len() as f32
}

fn display_stats(vals: &[u8]) {
    info!("Median value: {}", vals[vals.len() / 2 - 1]);
    info!("Average value: {}", get_vec_average(vals));
    info!("Percent Distribution:");
    for i in 1..21 {
        info!(
            "{}: {}%",
            i,
            get_percent_occurrence(i, vals.to_vec()) * 100.0
        );
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let roll_count = 100_000_001;
    let start = time::Instant::now();
    info!("Hello, time to roll!");
    {
        let new_start = time::Instant::now();
        let mut vec: Vec<u8> = Vec::new();
        info!("Rolling 1d20 {} times", roll_count - 1);
        for _ in 1..roll_count {
            vec.push(roll_d20());
        }
        vec.sort();
        display_stats(&vec);
        info!("Execution time: {}s", new_start.elapsed().as_secs_f32());
    }
    {
        let new_start = time::Instant::now();
        let mut vec: Vec<u8> = Vec::new();
        info!("Rolling AdvDis {} times", roll_count - 1);
        for _ in 1..roll_count {
            vec.push(roll_adv_dis());
        }
        vec.sort();
        display_stats(&vec);
        info!("Execution time: {}", new_start.elapsed().as_secs_f32());
    }
    {
        let new_start = time::Instant::now();
        let mut vec: Vec<u8> = Vec::new();
        info!("Rolling DisAdv {} times", roll_count - 1);
        for _ in 1..roll_count {
            vec.push(roll_dis_adv());
        }
        vec.sort();
        display_stats(&vec);
        info!("Execution time: {}", new_start.elapsed().as_secs_f32());
    }
    info!("Total Elapsed time: {}", start.elapsed().as_secs_f32());
}
