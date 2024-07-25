use crossbeam_utils::thread;
use jobsys;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

const ARRAY_SIZE: usize = 1024 * 1024 * 1024;

fn jobsys_for_each(thread_count: usize, job_capacity: usize) {
    let job_system = jobsys::JobSystem::new(thread_count, job_capacity).unwrap();
    let job_scope = jobsys::JobScope::new_from_system(&job_system);
    let mut array = vec![0u8; ARRAY_SIZE];

    let timer = std::time::Instant::now();
    job_scope
        .for_each(&mut array, |slice: &mut [u8], _start, _end| {
            for i in 0..slice.len() {
                slice[i] = rand::random::<u8>();
            }
        })
        .expect("Failed to run jobs");
    let duration = timer.elapsed();

    println!(
        "jobsys.for_each ({} threads,{} jobs) : {}ms",
        thread_count,
        job_capacity,
        duration.as_millis()
    );
}

fn crossbeam_spawn(chunk_count: usize) {
    let mut array = vec![0u8; ARRAY_SIZE];
    let chunk_size = array.len() / chunk_count;

    let timer = std::time::Instant::now();
    thread::scope(|s| {
        for chunk in array.chunks_mut(chunk_size) {
            s.spawn(|_| {
                for item in chunk {
                    *item = rand::random::<u8>();
                }
            });
        }
    })
    .unwrap();
    let duration = timer.elapsed();

    println!(
        "crossbeam.scope ({} chunks) : {}ms",
        chunk_count,
        duration.as_millis()
    );
}

async fn tokio_spawn(chunk_count: usize) {
    let array = vec![0u8; ARRAY_SIZE];
    let chunk_size = array.len() / chunk_count;

    let array = Arc::new(Mutex::new(vec![0u8; ARRAY_SIZE]));

    let mut handles = vec![];

    let timer = std::time::Instant::now();
    for i in 0..chunk_count {
        let array_clone = array.clone();
        let handle = tokio::task::spawn(async move {
            let start = i * chunk_size;
            let mut array = array_clone.lock().unwrap();
            let end = std::cmp::min(start + chunk_size, array.len());
            for i in start..end {
                array[i] = rand::random::<u8>();
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    let duration = timer.elapsed();

    println!(
        "tokio async ({} chunks) : {}ms",
        chunk_count,
        duration.as_millis()
    );
}

fn threadpool_spawn(worker_count: usize, chunk_count: usize) {
    let array = vec![0u8; ARRAY_SIZE];
    let chunk_size = array.len() / chunk_count;

    let pool = ThreadPool::new(worker_count);

    let mut array = Arc::new(Mutex::new(array));

    let timer = std::time::Instant::now();
    for i in 0..chunk_count {
        let array_clone = array.clone();
        pool.execute(move || {
            let start = i * chunk_size;
            let mut array = array_clone.lock().unwrap();
            let end = std::cmp::min(start + chunk_size, array.len());
            for i in start..end {
                array[i] = rand::random::<u8>();
            }
        });
    }
    pool.join();
    let duration = timer.elapsed();

    println!(
        "threadpool ({} workers, {} chunks) : {}ms",
        worker_count,
        chunk_count,
        duration.as_millis()
    );
}

fn rayon_spawn(chunk_count: usize) {
    let mut array = vec![0u8; ARRAY_SIZE];
    let chunk_size = array.len() / chunk_count;

    let timer = std::time::Instant::now();
    rayon::scope(|s| {
        for chunk in array.chunks_mut(chunk_size) {
            s.spawn(|_| {
                for item in chunk {
                    *item = rand::random::<u8>();
                }
            })
        }
    });
    let duration = timer.elapsed();

    println!(
        "rayon ({} chunks) : {}ms",
        chunk_count,
        duration.as_millis()
    );
}

pub async fn run() {
    // normal for loop
    {
        let mut array = vec![0u8; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        for i in 0..ARRAY_SIZE {
            array[i] = rand::random::<u8>();
        }
        let duration = timer.elapsed();

        println!("for : {}ms", duration.as_millis());
    }

    // normal iter_mut
    {
        let mut array = vec![0u8; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        array.iter_mut().for_each(|i| *i = rand::random::<u8>());
        let duration = timer.elapsed();

        println!("iter_mut : {}ms", duration.as_millis());
    }

    // crossbeam for loop
    {
        crossbeam_spawn(1); // mega slow
        crossbeam_spawn(2); // bad
        crossbeam_spawn(4); // fast
        crossbeam_spawn(8); // fastest (prob related to cpu)
        crossbeam_spawn(16); // fast
        crossbeam_spawn(32);
        crossbeam_spawn(64);
        crossbeam_spawn(128);
        crossbeam_spawn(256);
        crossbeam_spawn(512);
    }

    // rayon for loop using par_iter_mut
    {
        let mut array = vec![0u8; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        array.par_iter_mut().for_each(|i| {
            *i = rand::random::<u8>();
        });
        let duration = timer.elapsed();

        println!("rayon.par_iter_mut : {}ms", duration.as_millis());
    }

    // rayon manual spawn
    {
        rayon_spawn(1);
        rayon_spawn(2);
        rayon_spawn(4);
        rayon_spawn(8);
        rayon_spawn(16);
        rayon_spawn(32);
        rayon_spawn(64);
        rayon_spawn(128);
        rayon_spawn(256);
        rayon_spawn(512);
    }

    // // spawn tokio
    // {
    //     tokio_spawn(1).await;
    //     tokio_spawn(2).await;
    //     tokio_spawn(4).await;
    //     tokio_spawn(8).await;
    //     tokio_spawn(16).await;
    //     tokio_spawn(32).await;
    //     tokio_spawn(64).await;
    //     tokio_spawn(128).await;
    //     tokio_spawn(256).await;
    //     tokio_spawn(512).await;
    // }

    // // spawn threadpool
    // {
    //     threadpool_spawn(1, 1);
    //     threadpool_spawn(1, 2);
    //     threadpool_spawn(1, 4);
    //     threadpool_spawn(1, 8);
    //     threadpool_spawn(1, 16);
    //     threadpool_spawn(1, 32);
    //     threadpool_spawn(1, 64);
    //     threadpool_spawn(1, 128);
    //     threadpool_spawn(1, 256);
    //     threadpool_spawn(1, 512);

    //     threadpool_spawn(4, 1);
    //     threadpool_spawn(4, 2);
    //     threadpool_spawn(4, 4);
    //     threadpool_spawn(4, 8);
    //     threadpool_spawn(4, 16);
    //     threadpool_spawn(4, 32);
    //     threadpool_spawn(4, 64);
    //     threadpool_spawn(4, 128);
    //     threadpool_spawn(4, 256);
    //     threadpool_spawn(4, 512);
    // }

    // // spawn jobsys jobs
    // {
    //     jobsys_for_each(0, 2);
    //     jobsys_for_each(0, 4);
    //     jobsys_for_each(0, 8);
    //     jobsys_for_each(0, 16);
    //     jobsys_for_each(0, 32);
    //     jobsys_for_each(0, 64);
    //     jobsys_for_each(0, 128);
    //     jobsys_for_each(0, 256);
    //     jobsys_for_each(0, 512);
    //     jobsys_for_each(0, 1024);

    //     jobsys_for_each(4, 8);
    //     jobsys_for_each(4, 16);
    //     jobsys_for_each(4, 32);
    //     jobsys_for_each(4, 64);
    //     jobsys_for_each(4, 128);
    //     jobsys_for_each(4, 256);
    //     jobsys_for_each(4, 512);
    //     jobsys_for_each(4, 1024);
    // }
}
