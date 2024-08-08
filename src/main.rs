use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::VecDeque;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut chars = vec!['哼', '啊', '嗯'];
    chars.shuffle(&mut thread_rng());
    let homo_prefix = "114514";
    let queue: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    queue.lock().unwrap().push_back(String::new());

    let start = Instant::now();
    let count = Arc::new(Mutex::new(0u64));
    let found = Arc::new(Mutex::new(false));

    loop {
        let current_batch: Vec<String> = {
            let mut queue_lock = queue.lock().unwrap();
            let mut batch = Vec::new();
            for _ in 0..100 {
                if let Some(item) = queue_lock.pop_front() {
                    batch.push(item);
                }
            }
            batch
        };

        if current_batch.is_empty() {
            break;
        }

        current_batch.par_iter().for_each(|current| {
            for &c in &chars {
                let mut new_string = current.clone();
                new_string.push(c);

                let mut hasher = Sha256::new();
                hasher.update(new_string.as_bytes());
                let result = hasher.finalize();
                let hash_hex = format!("{:x}", result);

                {
                    let mut count_lock = count.lock().unwrap();
                    *count_lock += 1;
                    let elapsed = start.elapsed();
                    let speed = *count_lock as f64 / elapsed.as_secs_f64();
                    print!(
                        "\r数量：{} | 当前：{} | 速度：{:.2} hashes/sec",
                        *count_lock, new_string, speed
                    );
                    std::io::stdout().flush().unwrap();
                }

                if hash_hex.starts_with(homo_prefix) {
                    let elapsed = start.elapsed();
                    println!("\n找到恶臭字符串：{} -> {} | 耗时：{:.2?}", new_string, hash_hex, elapsed);
                }

                queue.lock().unwrap().push_back(new_string);
            }
        });

        if *found.lock().unwrap() {
            break;
        }
    }
}
