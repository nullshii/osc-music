use core::fmt;
use std::{io::Error, net::UdpSocket};

use rosc::{encoder, OscError, OscMessage, OscPacket};

pub struct VrcOsc {
    socket: UdpSocket,
    address: String,
}

impl VrcOsc {
    pub fn new(address: String) -> Result<Self, Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        Ok(Self { socket, address })
    }

    pub fn send_message(&self, text: String) -> Result<(), MessageError> {
        let message = OscPacket::Message(OscMessage {
            addr: "/chatbox/input".to_string(),
            args: vec![
                rosc::OscType::String(text),
                rosc::OscType::Bool(true),
                rosc::OscType::Bool(false),
            ],
        });

        let bytes = encoder::encode(&message)?;
        self.socket.send_to(&bytes, &self.address)?;
        Ok(())
    }
}

// Helper stuff
pub enum MessageError {
    GeneralError(Error),
    OscError(OscError),
}

impl From<OscError> for MessageError {
    fn from(error: OscError) -> Self {
        MessageError::OscError(error)
    }
}

impl From<Error> for MessageError {
    fn from(error: Error) -> Self {
        MessageError::GeneralError(error)
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}