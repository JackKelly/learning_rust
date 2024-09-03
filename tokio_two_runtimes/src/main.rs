use core::panic;
use std::time::Duration;

use tokio::runtime::Builder;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpu_rt = Builder::new_multi_thread().worker_threads(2).build()?;
    let io_rt = Builder::new_multi_thread().worker_threads(2).build()?;

    let cpu_handle = cpu_rt.spawn(async {
        println!("Starting task on cpu_rt! {:?}", std::thread::current().id());
        std::thread::sleep(Duration::from_secs(3));
        println!("Ending task on cpu_rt! {:?}", std::thread::current().id());
    });

    let io_handle = io_rt.spawn(async {
        println!("Starting task on io_rt! {:?}", std::thread::current().id());
        panic!("Panicing from io_rt!");
        std::thread::sleep(Duration::from_secs(3));
        println!("Ending task on io_rt! {:?}", std::thread::current().id());
    });

    cpu_handle.await?;
    io_handle.await?;

    Ok(())
}
