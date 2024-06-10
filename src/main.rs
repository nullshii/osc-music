use media_session::MediaSession;
use media_status::MediaStatus;
use std::time::Duration;
use vrc_osc::VrcOsc;

mod media_session;
mod media_status;
mod vrc_osc;

#[tokio::main]
async fn main() {
    tokio::spawn(check_music_update())
        .await
        .expect("Thread ERROR!!");
}

async fn check_music_update() {
    let vrc = VrcOsc::new("127.0.0.1:9000".to_string()).expect("Can't create VrcOsc Object!");

    loop {
        tokio::time::sleep(Duration::from_millis(10_000)).await;

        let (artist, title, status) = get_data().await;

        if title == "" || status != MediaStatus::Playing {
            continue;
        }

        let text = format!("{} - {}", artist, title);
        println!("{}", text);

        if let Err(e) = vrc.send_message(text) {
            println!("{}", e)
        }
    }
}

async fn get_data() -> (String, String, MediaStatus) {
    if let Ok(s) = MediaSession::new().await {
        return (s.get_artist(), s.get_title(), s.get_status());
    }

    return (String::new(), String::new(), MediaStatus::Closed);
}
