use hyprrust::events::{EventFilter, HyprlandEventType};
use hyprrust::HyprlandConnection;

#[tokio::main]
async fn main() {
    let mut conn = HyprlandConnection::current().unwrap();

    // This filter includes everything present in the list
    let mut filter = [
        HyprlandEventType::ActiveWindow,
        HyprlandEventType::ActiveWindowV2,
        HyprlandEventType::WindowTitle,
        HyprlandEventType::WindowTitleV2,
        HyprlandEventType::DestroyWorkspace,
        HyprlandEventType::DestroyWorkspaceV2,
    ]
    .iter()
    .collect::<EventFilter>();

    // Set the filter so it includes everything except what was in the list
    filter.set_include(false);

    let mut rx = conn.listen_to_events(filter).await.unwrap();
    while let Ok(ev) = rx.recv().await {
        println!("got {:?}", ev);
    }
}
