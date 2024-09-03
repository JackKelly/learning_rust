use std::time::Duration;

use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start Tokio runtime on separate thread for CPU-bound tasks:
    let cpu_thread_handle = std::thread::spawn(|| {
        let cpu_rt = Builder::new_multi_thread()
            .worker_threads(2)
            .build()
            .expect("Building Tokio runtime for CPU-bound tasks");
        cpu_rt.block_on(async {
            println!("Starting task on cpu_rt! {:?}", std::thread::current().id());
            std::thread::sleep(Duration::from_secs(3));
            println!("Ending task on cpu_rt! {:?}", std::thread::current().id());
        })
    });

    // Start Tokio runtime for IO tasks on local thread:
    let io_rt = Builder::new_multi_thread().worker_threads(2).build()?;
    io_rt.block_on(async {
        println!("Starting task on io_rt! {:?}", std::thread::current().id());
        std::thread::sleep(Duration::from_secs(3));
        println!("Ending task on io_rt! {:?}", std::thread::current().id());
    });

    cpu_thread_handle.join().unwrap();

    Ok(())
}
