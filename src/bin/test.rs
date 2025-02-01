use hyprrust::connection::*;

#[tokio::main]
async fn main() {
    let instances = HyprlandConnection::get_instances();
    let connections = instances
        .iter()
        .map(|inst| HyprlandConnection::new_with_instance(inst.clone()));

    let mut conn_num = 0;
    for mut conn in connections {
        let mut rx = conn.listen_to_events().await.unwrap();
        tokio::spawn(async move {
            loop {
                let ev = rx.recv().await.unwrap();
                println!("from conn #{}: {:?}", conn_num, ev);
            }
        });
        println!("Thread for connection #{} started.", conn_num);
        conn_num += 1;
    }

    loop {}
}
