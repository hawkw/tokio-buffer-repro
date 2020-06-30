use tokio::sync::mpsc;
use tokio::time;
use std::sync::Arc;
use tokio_buffer_repro as shared;

#[tokio::main]
async fn main() {
    let (mut tx, mut _rx) = mpsc::channel(16);
    let waiters = Arc::new(());
    
    // fill up the channel
    for _ in 0..16i32 {
        let _ = tx.send(()).await;
    }

    let done = tokio::spawn(shared::stats(waiters.clone()));

    for _ in 0..shared::TASKS {
        let waiters = Arc::downgrade(&waiters);
        let mut tx = tx.clone();
        tokio::spawn(async move {
            let _w = waiters;
            let send = tx.send(());
            let timeout = time::delay_for(shared::TIMEOUT);
            tokio::select! {
                _ = send => {},
                _ = timeout => {},
            }
        });
        tokio::time::delay_for(shared::SPAWN_INTERVAL).await;
    }

    done.await.unwrap();
}
