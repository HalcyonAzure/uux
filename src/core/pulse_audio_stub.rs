//! Non-Linux stand-in: PulseAudio is never selected, so cpal (CoreAudio) is used.
use anyhow::{Result, bail};
use serde_json::Value;
use std::collections::VecDeque;

#[allow(dead_code)]
pub fn available() -> bool {
    false
}
pub fn selection(_id: Option<&str>) -> Option<Option<&str>> {
    None
}
pub struct NativeStream;
impl NativeStream {
    pub fn open(_input: bool, _device: Option<&str>) -> Result<Self> {
        bail!("PulseAudio 仅在 Linux 上可用")
    }
    pub fn id(&self) -> String {
        String::new()
    }
    pub fn capture(&mut self, _queue: &mut VecDeque<f32>) -> Result<()> {
        Ok(())
    }
    pub fn playback(&mut self, _queue: &mut VecDeque<f32>) -> Result<()> {
        Ok(())
    }
}
pub fn devices() -> Result<Value> {
    bail!("PulseAudio 仅在 Linux 上可用")
}
