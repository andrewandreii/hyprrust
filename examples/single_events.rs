use hyprrust::{
    events::{single_event::DetachedEventConnection, EventFilter},
    HyprlandConnection,
};

#[tokio::main]
async fn main() {
    let conn = HyprlandConnection::new();
    let mut ev_conn =
        DetachedEventConnection::from_connection(conn, Some(EventFilter::new_include_all()))
            .await
            .unwrap();

    while let Ok(ev) = ev_conn.next_event().await {
        println!("{:?}", ev);
    }
}
