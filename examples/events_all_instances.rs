/// Listens to all instances of hyprland and reports their events in the console
use std::io;

use hyprrust::{events::EventFilter, *};

#[tokio::main]
async fn main() {
    // Get all currently running instances
    let instances = HyprlandConnection::get_instances().expect("Unable to get instances");

    // Create connections for each of them
    let connections = instances
        .iter()
        .map(|inst| HyprlandConnection::new_with_instance(inst.clone()));

    let mut conn_num = 0;
    for mut conn in connections {
        // Start listener tasks for each connection
        let mut rx = conn
            .listen_to_events(EventFilter::new_include_all())
            .await
            .unwrap();

        // Listen to events in another task
        tokio::spawn(async move {
            loop {
                let ev = rx.recv().await.unwrap();
                println!("from conn #{}: {:?}", conn_num, ev);
            }
        });

        println!("Thread for connection #{} started.", conn_num);
        conn_num += 1;
    }

    // On enter quit
    let mut tmp = String::new();
    let _ = io::stdin().read_line(&mut tmp);
}
