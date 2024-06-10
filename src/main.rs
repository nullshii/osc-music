use media_session::MediaSession;
use std::time::Duration;
use vrc_osc::VrcOsc;

mod media_session;
mod vrc_osc;

#[tokio::main]
async fn main() {
    tokio::spawn(check_music_update())
        .await
        .expect("Thread ERROR!!");
}

async fn check_music_update() {
    let vrc = VrcOsc::new("127.0.0.1:9000".to_string()).expect("Can't create VrcOsc Object!");
    let mut last_title = String::new();

    loop {
        tokio::time::sleep(Duration::from_millis(5_000)).await;

        let (artist, title) = get_data().await;

        if title == "" || last_title == title {
            continue;
        }

        let text = format!("{} - {}", artist, title);
        println!("{}", text);
        
        if let Err(e) = vrc.send_message(text) {
            println!("{}", e)
        }

        tokio::time::sleep(Duration::from_millis(10_000)).await;

        if let Err(e) = vrc.send_message(String::new()) {
            println!("{}", e)
        }

        last_title = title;
    }
}

async fn get_data() -> (String, String) {
    if let Ok(s) = MediaSession::new().await {
        return (s.get_artist(), s.get_title());
    }

    return (String::new(), String::new());
}
