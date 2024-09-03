use std::time::Duration;

use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_rt = Builder::new_multi_thread().worker_threads(2).build()?;
    let cpu_rt = Builder::new_multi_thread().worker_threads(2).build()?;

    // Adapted from https://stackoverflow.com/a/61763027

    // Tokio reports an error if one runtime is dropped within another runtime. So we must drop
    // the cpu runtime in the main thread, and only move a clone of the handle into the main_rt
    let cpu_rt_handle = cpu_rt.handle().clone();

    main_rt.block_on(async {
        println!("Starting main_rt! {:?}", std::thread::current().id());

        let _ = cpu_rt_handle
            .spawn(async {
                println!(
                    "Starting task on cpu_rt_handle! {:?}",
                    std::thread::current().id()
                );
                std::thread::sleep(Duration::from_secs(3));
                println!(
                    "Ending task on cpu_rt_handle! {:?}",
                    std::thread::current().id()
                );
            })
            .await;

        println!("Ending main_rt! {:?}", std::thread::current().id());
    });

    Ok(())
}
