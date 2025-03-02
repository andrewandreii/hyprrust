use hyprrust::{
    events::{single_event_sync::DetachedEventConnection, EventFilter},
    HyprlandConnection,
};

fn main() {
    let conn = HyprlandConnection::new();
    let mut ev_conn =
        DetachedEventConnection::from_connection(conn, Some(EventFilter::new_include_all()))
            .unwrap();

    while let Ok(ev) = ev_conn.next_event() {
        println!("{:?}", ev);
    }
}
