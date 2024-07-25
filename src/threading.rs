use crossbeam_utils::thread;
use jobsys;
use rayon::prelude::*;

const ARRAY_SIZE: usize = 1024 * 1024 * 512;

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
                for item in chunk.iter_mut() {
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
        array
            .iter_mut()
            .for_each(|i| *i = rand::random::<u8>());
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

    // spawn tokio sync
    {
        let mut array = vec![0u8; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        let task = tokio::task::spawn_blocking(move || {
            array
                .iter_mut()
                .for_each(|i| *i = rand::random::<u8>());
        });
        task.await.unwrap();
        let duration = timer.elapsed();

        println!("tokio sync : {}ms", duration.as_millis());
    }

    // spawn tokio async
    {
        let mut array = vec![0u8; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        tokio::task::spawn(async move {
            array
                .iter_mut()
                .for_each(|i| *i = rand::random::<u8>());
        })
        .await
        .unwrap();
        let duration = timer.elapsed();

        println!("tokio async : {}ms", duration.as_millis());
    }

    // spawn jobsys jobs
    {
        // jobsys_for_each(0, 2);
        // jobsys_for_each(0, 4);
        // jobsys_for_each(0, 8);
        // jobsys_for_each(0, 16);
        // jobsys_for_each(0, 32);
        // jobsys_for_each(0, 64);
        // jobsys_for_each(0, 128);
        // jobsys_for_each(0, 256);
        // jobsys_for_each(0, 512);
        // jobsys_for_each(0, 1024);

        // jobsys_for_each(0, ARRAY_SIZE);
        // jobsys_for_each(0, ARRAY_SIZE / 2);
        // jobsys_for_each(0, ARRAY_SIZE / 4);
        // jobsys_for_each(0, ARRAY_SIZE / 8);
        // jobsys_for_each(0, ARRAY_SIZE / 16);
        // jobsys_for_each(0, ARRAY_SIZE / 32);
        // jobsys_for_each(0, ARRAY_SIZE / 64);

        // jobsys_for_each(4, 8);
        // jobsys_for_each(4, 16);
        // jobsys_for_each(4, 32);
        // jobsys_for_each(4, 64);
        // jobsys_for_each(4, 128);
        // jobsys_for_each(4, 256);
        // jobsys_for_each(4, 512);
        // jobsys_for_each(4, 1024);

        // jobsys_for_each(4, ARRAY_SIZE);
        // jobsys_for_each(4, ARRAY_SIZE / 2);
        // jobsys_for_each(4, ARRAY_SIZE / 4);
        // jobsys_for_each(4, ARRAY_SIZE / 8);
        // jobsys_for_each(4, ARRAY_SIZE / 16);
        // jobsys_for_each(4, ARRAY_SIZE / 32);
        // jobsys_for_each(4, ARRAY_SIZE / 64);
    }
}
