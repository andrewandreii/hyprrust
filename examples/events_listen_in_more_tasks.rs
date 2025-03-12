use std::future::pending;

use hyprrust::{
    events::{EventFilter, HyprlandEvent},
    HyprlandConnection,
};
use tokio::sync::broadcast;

async fn simple_listener(task_num: i32, mut rx: broadcast::Receiver<HyprlandEvent>) {
    while let Ok(ev) = rx.recv().await {
        println!("Task #{} got: {:?}", task_num, ev);
    }
}

#[tokio::main]
async fn main() {
    let mut conn = HyprlandConnection::current().unwrap();

    let rx = conn
        .listen_to_events(EventFilter::new_include_all())
        .await
        .unwrap();
    let task1 = tokio::spawn(simple_listener(1, rx));

    let rx = conn
        .listen_to_events(EventFilter::new_include_all())
        .await
        .unwrap();
    let task2 = tokio::spawn(simple_listener(2, rx));

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    task1.abort();
    task2.abort();
    // Connection remains open.

    let rx = conn
        .listen_to_events(EventFilter::new_include_all())
        .await
        .unwrap();
    tokio::spawn(simple_listener(3, rx));

    pending::<()>().await;
}
