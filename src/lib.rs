use tokio::time;
use std::sync::Arc;
use std::time::Duration;

use sysinfo::{ProcessExt, System, SystemExt};

pub const TASKS: usize = 100_000;
pub const TIMEOUT: Duration = Duration::from_secs(1);
pub const SPAWN_INTERVAL: Duration = Duration::from_millis(1);

pub async fn stats(waiters: Arc<()>) {
    let mut interval = time::interval(Duration::from_secs(2));
    let pid = sysinfo::get_current_pid().expect("we should have a PID...what is this, windows?");
    let mut sys = System::new();
    loop {
        interval.tick().await;
        sys.refresh_process(pid);
        let process = sys.get_process(pid).expect("our pid exists!");
        let tasks = Arc::weak_count(&waiters);
        println!(
            "{}: waiters: {:>6}; RSS: {:>6} kb; virt: {:>6} kb;",
            process.name(),
            tasks,
            process.memory(), 
            process.virtual_memory()
        );
        if tasks == 0 {
            return;
        }
    }

}