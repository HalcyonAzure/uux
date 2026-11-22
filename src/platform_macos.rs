use anyhow::{Context, Result, bail};
use std::process::{Child, Command, Stdio};

pub fn autostart_enabled() -> bool {
    false
}
pub fn set_autostart(enabled: bool) -> Result<()> {
    if enabled {
        bail!("macOS 版暂不支持开机自启");
    }
    Ok(())
}

/// `caffeinate -w <pid>` keeps the display and system awake and exits on its
/// own if UUX dies, so a crash never leaves the Mac pinned awake.
pub struct Inhibitor {
    child: Option<Child>,
}
impl Inhibitor {
    pub async fn acquire() -> Result<Self> {
        let child = Command::new("/usr/bin/caffeinate")
            .args(["-d", "-i", "-w", &std::process::id().to_string()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("无法启动 caffeinate 阻止休眠")?;
        Ok(Self { child: Some(child) })
    }
    pub async fn release(mut self) {
        self.stop();
    }
    fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
impl Drop for Inhibitor {
    fn drop(&mut self) {
        self.stop();
    }
}
