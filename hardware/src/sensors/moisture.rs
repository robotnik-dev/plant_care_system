use std::sync::mpsc::{Receiver, Sender, channel};
use tokio::time::interval;

const READ_INTERVAL: f32 = 1.0;
const READ_PIN: u8 = 17;

pub struct MoistureSensor {
    sender: Sender<f64>,
    receiver: Receiver<f64>,
}

impl Default for MoistureSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl MoistureSensor {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }

    pub fn subscribe(&self) -> &Receiver<f64> {
        &self.receiver
    }

    pub async fn start_reading(&self) -> Result<(), String> {
        let sender = self.sender.clone();

        tokio::task::spawn(async move {
            let mut interval = interval(core::time::Duration::from_secs(READ_INTERVAL as u64));

            loop {
                interval.tick().await;
                let Ok(reading) = read_moisture_value(READ_PIN) else {
                    return Err::<(), std::string::String>(String::from(
                        "Failed to read moisture pins",
                    ));
                };
                if let Err(err) = sender.send(reading).map_err(|e| e.to_string()) {
                    return Err::<(), std::string::String>(err.to_string());
                }
            }
        });
        Ok(())
    }
}

fn read_moisture_value(_pin: u8) -> Result<f64, String> {
    // Actual sensor reading implementation
    // This is a placeholder
    Ok(42.5)
}
