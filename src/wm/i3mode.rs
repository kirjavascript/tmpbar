use std::env;
use std::fmt;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};
use std::thread;
use egui::Context;

#[derive(Debug)]
pub enum I3IpcError {
    EnvVarNotSet,
    ConnectionFailed(std::io::Error),
    SendFailed(std::io::Error),
    ReceiveFailed(std::io::Error),
    UnexpectedResponse,
}

impl fmt::Display for I3IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            I3IpcError::EnvVarNotSet => write!(f, "I3SOCK environment variable not set"),
            I3IpcError::ConnectionFailed(e) => write!(f, "failed to connect to i3 IPC socket: {}", e),
            I3IpcError::SendFailed(e) => write!(f, "failed to send message to i3 IPC socket: {}", e),
            I3IpcError::ReceiveFailed(e) => write!(f, "failed to receive message from i3 IPC socket: {}", e),
            I3IpcError::UnexpectedResponse => write!(f, "unexpected response format from i3"),
        }
    }
}

pub struct I3Mode {
    current_mode: Arc<Mutex<String>>,
}

impl I3Mode {
    pub fn new(ctx: Context) -> Self {
        let mode = I3Mode {
            current_mode: Arc::new(Mutex::new(String::from("default"))),
        };

        mode.try_listen(ctx).ok();

        mode
    }

    pub fn get(&self) -> String {
        self.current_mode.lock().unwrap().clone()
    }

    pub fn try_listen(&self, ctx: Context) -> Result<(), I3IpcError> {
        if env::var("I3SOCK").is_err() {
            return Err(I3IpcError::EnvVarNotSet);
        }

        let socket_path = env::var("I3SOCK").map_err(|_| I3IpcError::EnvVarNotSet)?;
        let mut stream = UnixStream::connect(&socket_path)
            .map_err(I3IpcError::ConnectionFailed)?;

        self.subscribe_to_events(&mut stream)?;

        let mode = Arc::clone(&self.current_mode);

        thread::spawn(move || {
            loop {
                let mut header = [0; 14]; // 6 bytes magic string + 4 bytes length + 4 bytes type
                if stream.read_exact(&mut header).is_err() {
                    break;
                }

                let event_type = u32::from_ne_bytes([header[10], header[11], header[12], header[13]]);

                if event_type & 0x80000000 == 0 {
                    continue;
                }

                let payload_len = u32::from_ne_bytes([header[6], header[7], header[8], header[9]]) as usize;
                let mut payload = vec![0; payload_len];
                if stream.read_exact(&mut payload).is_err() {
                    break;
                }

                // we got a mode event
                if event_type & 0x7FFFFFFF == 2 {
                    let payload = String::from_utf8_lossy(payload.as_slice());

                    fn extract_change(json: &str) -> Option<&str> {
                        let prop = json.find("\"change\"")? + 8;
                        let start = &json[prop..].find("\"")? + 1 + prop;

                        let end = json[start..].find('"')? + start;
                        Some(&json[start..end])
                    }

                    if let Some(next_mode) = extract_change(&payload) {
                        *mode.lock().unwrap() = next_mode.to_string();
                        ctx.request_repaint();
                    }
                }
            }
        });

        Ok(())
    }

    fn subscribe_to_events(&self, stream: &mut UnixStream) -> Result<(), I3IpcError> {
        self.send_message(stream, 2, "[\"mode\"]".as_bytes())?;

        let (_type, payload) = self.receive_message(stream)?;

        if let Ok(payload_str) = String::from_utf8(payload) {
            if payload_str.contains("\"success\":true") {
                return Ok(());
            }
        }

        Err(I3IpcError::UnexpectedResponse)
    }

    fn send_message(&self, stream: &mut UnixStream, message_type: u32, payload: &[u8]) -> Result<(), I3IpcError> {
        let len = payload.len() as u32;
        let len_bytes = len.to_ne_bytes();
        let type_bytes = message_type.to_ne_bytes();

        stream.write_all(b"i3-ipc").map_err(I3IpcError::SendFailed)?;
        stream.write_all(&len_bytes).map_err(I3IpcError::SendFailed)?;
        stream.write_all(&type_bytes).map_err(I3IpcError::SendFailed)?;
        stream.write_all(payload).map_err(I3IpcError::SendFailed)?;

        Ok(())
    }

    fn receive_message(&self, stream: &mut UnixStream) -> Result<(u32, Vec<u8>), I3IpcError> {
        let mut header = [0; 14];
        stream.read_exact(&mut header).map_err(I3IpcError::ReceiveFailed)?;

        let payload_len = u32::from_ne_bytes([header[6], header[7], header[8], header[9]]) as usize;
        let message_type = u32::from_ne_bytes([header[10], header[11], header[12], header[13]]);

        let mut payload = vec![0; payload_len];
        stream.read_exact(&mut payload).map_err(I3IpcError::ReceiveFailed)?;

        Ok((message_type, payload))
    }
}
