use hyprrust::{events::EventFilter, HyprlandConnection};

// A sync version of events is not yet available

#[tokio::main]
async fn main() {
    let mut conn = HyprlandConnection::new();

    // Make sure you don't wait for IO
    tokio::spawn(async move {
        // If everything went ok, we get a Receiver
        // which will return events if they occur
        match conn.listen_to_events(EventFilter::new_include_all()).await {
            Ok(mut rx) => {
                // See hyprrust::ctl::data for events
                loop {
                    match rx.recv().await {
                        Ok(ev) => println!("Got {:?}", ev),
                        Err(e) => println!("err: {}", e),
                    }
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
