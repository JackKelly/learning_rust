use std::time::Duration;

use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Builder::new_multi_thread().worker_threads(4).build()?;

    rt.block_on(async {
        task(2, Duration::from_secs(1));
        task(10, Duration::from_millis(100));
        Ok(())
    })
}

fn task(n: u8, d: Duration) {
    for i in 0..n {
        tokio::spawn(async move {
            println!(
                "** Starting {d:?} task {i} ^^! {:?}",
                std::thread::current().id()
            );
            std::thread::sleep(d);
            println!(
                "** Finished {d:?} task {i} ##! {:?}",
                std::thread::current().id()
            );
        });
    }
}
