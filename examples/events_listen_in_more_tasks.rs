use std::future::{pending, Future};

use hyprrust::{events::HyprlandEvent, HyprlandConnection};
use tokio::sync::broadcast;

fn simple_listener(
    task_num: i32,
    mut rx: broadcast::Receiver<HyprlandEvent>,
) -> impl Future<Output = ()> {
    async move {
        while let Ok(ev) = rx.recv().await {
            println!("Task #{} got: {:?}", task_num, ev);
        }
    }
}

#[tokio::main]
async fn main() {
    let mut conn = HyprlandConnection::new();

    let rx = conn.listen_to_events(None).await.unwrap();
    let task1 = tokio::spawn(simple_listener(1, rx));

    let rx = conn.listen_to_events(None).await.unwrap();
    let task2 = tokio::spawn(simple_listener(2, rx));

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    task1.abort();
    task2.abort();
    // Connection remains open.

    let rx = conn.listen_to_events(None).await.unwrap();
    tokio::spawn(simple_listener(3, rx));

    pending::<()>().await;
}
