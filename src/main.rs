use std::time::Duration;
use vrc_osc::VrcOsc;
use media_session::MediaSession;

mod vrc_osc;
mod media_session;

#[tokio::main]
async fn main() {
    let vrc = VrcOsc::new("127.0.0.1:9000".to_string()).expect("Can't create VrcOsc Object!");

    let mut last_title = String::new();

    loop {
        if let Ok(s) = MediaSession::new().await {
            let title = s.get_title();

            if last_title == title {
                continue;
            }

            let text = format!("{} - {}", s.get_artist(), title);

            if let Err(e) = vrc.send_message(text) {
                println!("{}", e)
            }

            last_title = title;
        }

        tokio::time::sleep(Duration::from_millis(2000)).await;
    }
}