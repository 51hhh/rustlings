use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // Arc 本身不足以用于可变的共享状态，需要结合 Mutex
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // 在更新共享值之前，必须先获取锁
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印 JobStatus.jobs_done 的值
    let job_status = status.lock().unwrap();
    println!("Jobs done: {}", job_status.jobs_done);
}


