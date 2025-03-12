use hyprrust::{
    events::{single_event_sync::DetachedEventConnection, EventFilter},
    HyprlandConnection,
};

fn main() {
    let conn = HyprlandConnection::current().unwrap();
    let ev_conn =
        DetachedEventConnection::from_connection(conn, EventFilter::new_include_all()).unwrap();

    for ev in ev_conn {
        println!("{:?}", ev);
    }
}
