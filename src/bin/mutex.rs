use tokio::sync::Mutex;
use tokio::time;
use std::sync::Arc;
use tokio_buffer_repro as shared;

#[tokio::main]
async fn main() {
    let lock = Arc::new(Mutex::new(()));
    let _locked = lock.lock().await;
    let waiters = Arc::new(());

    let done = tokio::spawn(shared::stats(waiters.clone()));

    for _ in 0..shared::TASKS {
        let waiters = Arc::downgrade(&waiters);
        let lock = lock.clone();
        tokio::spawn(async move {
            let _w = waiters;
            let lock = lock.lock();
            let timeout = time::delay_for(shared::TIMEOUT);
            tokio::select! {
                _ = lock => {},
                _ = timeout => {},
            }
        });
        tokio::time::delay_for(shared::SPAWN_INTERVAL).await;
    }

    done.await.unwrap();
}
