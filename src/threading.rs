use jobsys;
use rayon::prelude::*;

const ARRAY_SIZE: usize = 1024 * 1024 * 256;

fn jobsys_for_each(thread_count: usize, job_capacity: usize) {
    let job_system = jobsys::JobSystem::new(thread_count, job_capacity).unwrap();
    let job_scope = jobsys::JobScope::new_from_system(&job_system);
    let mut array = vec![0u32; ARRAY_SIZE];

    let timer = std::time::Instant::now();
    job_scope
        .for_each(&mut array, |slice: &mut [u32], _start, _end| {
            for i in 0..slice.len() {
                slice[i] = rand::random::<u32>();
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

pub async fn run() {
    // normal for loop
    {
        let mut array = vec![0u32; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        for i in 0..ARRAY_SIZE {
            array[i] = rand::random::<u32>();
        }
        let duration = timer.elapsed();

        println!("for : {}ms", duration.as_millis());
    }

    // normal iter_mut
    {
        let mut array = vec![0u32; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        array.iter_mut().for_each(|i| *i = rand::random::<u32>());
        let duration = timer.elapsed();

        println!("iter_mut : {}ms", duration.as_millis());
    }

    // rayon for loop using par_iter
    {
        let mut array = vec![0u32; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        array.par_iter_mut().for_each(|i| {
            *i = rand::random::<u32>();
        });
        let duration = timer.elapsed();

        println!("rayon.par_iter_mut : {}ms", duration.as_millis());
    }

    // spawn tokio sync
    {
        let mut array = vec![0u32; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        let task = tokio::task::spawn_blocking(move || {
            array.iter_mut().for_each(|i| *i = rand::random::<u32>());
        });
        task.await.unwrap();
        let duration = timer.elapsed();

        println!("tokio sync : {}ms", duration.as_millis());
    }

    // spawn tokio async
    {
        let mut array = vec![0u32; ARRAY_SIZE];

        let timer = std::time::Instant::now();
        tokio::task::spawn(async move {
            array.iter_mut().for_each(|i| *i = rand::random::<u32>());
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
        jobsys_for_each(4, 512);
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
