use std::{io, thread, sync::{Arc, Mutex}};

fn main() {
    let mut threads = Vec::new();
    let result = Arc::new(Mutex::new(Vec::new()));

    let mut input = String::new();
    println!("Start of prime scope: ");
    io::stdin().read_line(&mut input).unwrap();
    let start: u128 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("End of prime scope: ");
    io::stdin().read_line(&mut input).unwrap();
    let end: u128 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Number of threads to use: ");
    io::stdin().read_line(&mut input).unwrap();
    let num_threads = input.trim().parse().unwrap();

    let chunk_size = (end - start + 1) / num_threads as u128;

    for i in 0..num_threads {
        let thread_start = start + i * chunk_size;
        let thread_end = if i == num_threads - 1 {
            end
        } else {
            thread_start + chunk_size
        };

        let result_clone = Arc::clone(&result);

        threads.push(thread::spawn(move || {
            let mut local_result = Vec::new();
            for num in thread_start..thread_end {
                if is_prime(num) {
                    local_result.push((num, i + 1));
                }
            }

            let mut result = result_clone.lock().unwrap();
            result.extend(local_result);
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }

    let result = result.lock().unwrap();
    let mut sorted_result: Vec<_> = result.iter().cloned().collect();
    sorted_result.sort_by_key(|&(num, _)| num);

    for (num, thread_num) in sorted_result {
        println!("Thread {}: {} is a prime number!", thread_num, num);
    }
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..=(n as f64).sqrt() as u128 {
        if n % a == 0 {
            return false;
        }
    }
    true
}