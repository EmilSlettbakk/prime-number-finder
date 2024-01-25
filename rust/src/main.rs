use std::{io, thread, sync::{Arc, Mutex}};

fn main() {
    const MAX_ALLOWED_THREADS: usize = 16; //Set the maximum amount of THREADS allowed.

    //Creates vector for storing THREADS and results from prime number calculations.
    let mut threads = Vec::new();
    let result = Arc::new(Mutex::new(Vec::new()));

    //Functions for taking in user input and handling errors.
    let mut input = String::new();
    println!("Start of prime scope: ");
    io::stdin().read_line(&mut input).unwrap();
    let start: u128 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input! Please enter a valid starting number");
            return;
        }
    };

    let mut input = String::new();
    println!("End of prime scope: ");
    io::stdin().read_line(&mut input).unwrap();
    let end: u128 = match input.trim().parse() {
        Ok(value) if value > start => value,
        _ => {
            println!("Invalid input! Please enter a valid ending number greater than the starting number.");
            return;
        }
    };

    let mut input = String::new();
    println!("Number of threads to use: ");
    io::stdin().read_line(&mut input).unwrap();
    let num_threads = match input.trim().parse() {
        Ok(value) if value > 0 && value <= MAX_ALLOWED_THREADS => value,
        _ => {
            println!("Invalid number of threads. Please enter a value between 1 and {}.", MAX_ALLOWED_THREADS);
            return;
        }
    };

    /*
    Split the number range into pieces so
    that each THREAD gets equal an workload.
    */
    let chunk_size = (end - start + 1) / num_threads as u128;

    //Assigns work for each thread and saves the result.
    for i in 0..num_threads {
        let thread_start = start + (i as u128) * chunk_size;
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

    //Sorts the results from each thread into ascending order by the prime number.
    let result = result.lock().unwrap();
    let mut sorted_result: Vec<_> = result.iter().cloned().collect();
    sorted_result.sort_by_key(|&(num, _)| num);

    //Prints the sorted results.
    for (num, thread_num) in sorted_result {
        println!("Thread {}: {} is a prime number!", thread_num, num);
    }
}

//Simple algorithm that checks if a number is a prime number by using modulo.
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
