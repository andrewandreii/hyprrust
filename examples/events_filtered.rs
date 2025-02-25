use hyprrust::events::{event_name, EventFilter};
use hyprrust::HyprlandConnection;

#[tokio::main]
async fn main() {
    let mut conn = HyprlandConnection::new();
    let filter = [
        event_name!(HyprlandEvent::ActiveWindow),
        event_name!(HyprlandEvent::WindowTitleV2),
    ]
    .iter()
    .collect::<EventFilter>();

    let mut rx = conn.listen_to_events(Some(filter)).await.unwrap();
    while let Ok(ev) = rx.recv().await {
        println!("got {:?}", ev);
    }
}
