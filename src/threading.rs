use rayon::prelude::*;

const ARRAY_SIZE: usize = 1024 * 1024 * 1;

fn crossbeam_spawn(chunk_count: usize) {
    let mut array = vec![0u8; ARRAY_SIZE];
    let chunk_size = array.len() / chunk_count;

    let timer = std::time::Instant::now();
    crossbeam::thread::scope(|s| {
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
        "crossbeam ({} chunks) : {}ms",
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

fn std_thread_spawn(chunk_count: usize) {
    let mut array = vec![0u8; ARRAY_SIZE];
    let chunk_size = array.len() / chunk_count;

    let timer = std::time::Instant::now();
    std::thread::scope(|s| {
        for chunk in array.chunks_mut(chunk_size) {
            s.spawn(move || {
                for item in chunk {
                    *item = rand::random::<u8>();
                }
            });
        }
    });
    let duration = timer.elapsed();

    println!(
        "std::thread ({} chunks) : {}ms",
        chunk_count,
        duration.as_millis()
    );
}

pub fn run() {
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

    // crossbeam for loop
    {
        crossbeam_spawn(1);
        crossbeam_spawn(2);
        crossbeam_spawn(4);
        crossbeam_spawn(8);
        crossbeam_spawn(16);
        crossbeam_spawn(32);
        crossbeam_spawn(64);
        crossbeam_spawn(128);
        crossbeam_spawn(256);
        crossbeam_spawn(512);
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

    // std thread spawn
    {
        std_thread_spawn(1);
        std_thread_spawn(2);
        std_thread_spawn(4);
        std_thread_spawn(8);
        std_thread_spawn(16);
        std_thread_spawn(32);
        std_thread_spawn(64);
        std_thread_spawn(128);
        std_thread_spawn(256);
        std_thread_spawn(512);
    }
}
