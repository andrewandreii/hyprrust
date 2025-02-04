use std::{future::Future, sync::Arc};

use hyprrust::connection::HyprlandConnection;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let conn = Arc::new(HyprlandConnection::new());
    let mut num_done = 0;
    const TOTAL: i32 = 5000;

    let mut handles: Vec<_> = Vec::new();
    for i in 1..TOTAL {
        let c = conn.clone();
        handles.push(
            tokio::spawn(async move {
                c.clone().send_raw_message("-j/clients").await;
            })
            .await,
        );
    }

    //for handle in handles {
    //    handle.await;
    //}
    println!("done");
}
