use core::panic;
use std::time::Duration;

use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create both runtimes (rt):
    let main_rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    let cpu_rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;

    // Tokio reports an error if one runtime is dropped within another runtime. So we must drop
    // the cpu runtime in the main thread, and only move a clone of the handle into the main_rt.
    // This idea is adapted from https://stackoverflow.com/a/61763027
    let cpu_rt_handle = cpu_rt.handle().clone();

    main_rt.block_on(async {
        println!("Starting main_rt! {:?}", std::thread::current().id());

        println!("tokio::time::sleep on main_rt");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Finished tokio_time::sleep on main_rt");

        let cpu_task = cpu_rt_handle.spawn(async {
            println!(
                "Starting task on cpu_rt_handle! {:?}",
                std::thread::current().id()
            );
            std::thread::sleep(Duration::from_secs(3));
            // This panic is correctly propagated:
            // panic!("Test panic from cpu_rt_handle");
            println!(
                "Ending task on cpu_rt_handle! {:?}",
                std::thread::current().id()
            );
        });

        let main_task = main_rt.spawn(async {
            println!(
                "Spawning task on main_rt! {:?}",
                std::thread::current().id()
            );
            std::thread::sleep(Duration::from_secs(3));
            println!("Ending task on main_rt! {:?}", std::thread::current().id());
        });

        let _ = cpu_task.await;
        let _ = main_task.await;

        println!("Ending main_rt! {:?}", std::thread::current().id());
    });

    Ok(())
}
