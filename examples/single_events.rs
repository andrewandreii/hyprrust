use futures::StreamExt;
use hyprrust::{
    events::{single_event::DetachedEventConnection, EventFilter},
    HyprlandConnection,
};

#[tokio::main]
async fn main() {
    let conn = HyprlandConnection::current().unwrap();
    let mut ev_conn =
        DetachedEventConnection::from_connection(conn, EventFilter::new_include_all())
            .await
            .unwrap();

    while let Some(ev) = ev_conn.next().await {
        println!("{:?}", ev);
    }
}
