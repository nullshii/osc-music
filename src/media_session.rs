use core::fmt;

use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties,
};

pub struct MediaSession {
    // session: GlobalSystemMediaTransportControlsSession,
    properties: GlobalSystemMediaTransportControlsSessionMediaProperties,
    // timeline: GlobalSystemMediaTransportControlsSessionTimelineProperties,
}

impl MediaSession {
    pub async fn new() -> Result<Self, windows::core::Error> {
        let mp = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
        let session = mp.GetCurrentSession()?;
        let properties = session.TryGetMediaPropertiesAsync()?.await?;
        // let timeline = session.GetTimelineProperties()?;
        Ok(Self {
            // session,
            properties,
            // timeline,
        })
    }

    pub fn get_artist(&self) -> String {
        self.properties.Artist().unwrap_or_default().to_string()
    }

    pub fn get_title(&self) -> String {
        self.properties.Title().unwrap_or_default().to_string()
    }

    // pub fn get_position(&self) -> HumanDurationData {
    //     self.timeline.Position().unwrap_or_default().cleanup()
    // }

    // pub fn get_duration(&self) -> HumanDurationData {
    //     self.timeline.EndTime().unwrap_or_default().cleanup()
    // }
}

impl fmt::Display for MediaSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.get_title(), self.get_artist(),)
    }
}
