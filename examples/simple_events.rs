#![feature(never_type)]
use hyprrust::connection::HyprlandConnection;

// A sync version of events is not yet available

#[tokio::main]
async fn main() -> ! {
    let mut conn = HyprlandConnection::new();

    // Make sure you don't wait for IO
    tokio::spawn(async move {
        // If everything went ok, we get a Receiver
        // which will return events if they occur
        match conn.listen_to_events().await {
            Ok(mut rx) => {
                // See hyprrust::ctl::data for events
                while let Ok(ev) = rx.recv().await {
                    println!("Got {:?}", ev);
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    loop {
        // Doing some other important business
    }
}
