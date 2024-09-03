use core::panic;
use std::time::Duration;

use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create both runtimes (rt):
    let main_rt = Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("main_rt")
        .enable_all()
        .build()?;
    let cpu_rt = Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("cpu_rt")
        .enable_all()
        .build()?;

    // Tokio reports an error if one runtime is dropped within another runtime. So we must drop
    // the cpu runtime in the main thread, and only move a clone of the handle into the main_rt.
    // This idea is adapted from https://stackoverflow.com/a/61763027
    let cpu_rt_handle = cpu_rt.handle().clone();

    main_rt.block_on(async {
        println!("{} Starting main_rt!", thread_and_runtime_id());

        // Sleep on main runtime, and wait for it to finish:
        println!("tokio::time::sleep on main_rt");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Finished tokio_time::sleep on main_rt");

        // Spawn tasks on the CPU runtime:
        let cpu_task = cpu_rt_handle.spawn(async {
            println!(
                "{} Starting tasks on cpu_rt_handle!",
                thread_and_runtime_id()
            );
            task(2, Duration::from_secs(1));
            println!("{} Ending tasks on cpu_rt_handle!", thread_and_runtime_id());
            // This panic is correctly propagated:
            // panic!("Test panic from cpu_rt_handle");
        });

        // Spawn tasks on the main runtime:
        let main_task = main_rt.spawn(async {
            println!("{} Spawning tasks on main_rt!", thread_and_runtime_id());
            task(1, Duration::from_secs(1));
            task(10, Duration::from_millis(100));
            println!("{} Ending tasks on main_rt!", thread_and_runtime_id());
        });

        // Wait for the two spawns to finish:
        let _ = cpu_task.await;
        let _ = main_task.await;

        println!("{} Ending main_rt!", thread_and_runtime_id());
    });

    Ok(())
}

/// Spawn n tasks, where each task waits d duration.
fn task(n: u8, d: Duration) {
    for i in 0..n {
        tokio::spawn(async move {
            println!("{} ** Starting {d:?} task {i} ^^!", thread_and_runtime_id());
            std::thread::sleep(d);
            println!("{} ** Finished {d:?} task {i} ##!", thread_and_runtime_id());
        });
    }
}

fn thread_and_runtime_id() -> String {
    format!(
        "{:?} Runtime{:?}",
        std::thread::current().id(),
        tokio::runtime::Handle::current().id(),
    )
}
