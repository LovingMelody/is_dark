use std::process;

use crate::IsItDark;

#[derive(Default)]
pub(crate) struct MacOS;

impl MacOS {
    fn dark() -> Result<bool, std::io::Error> {
        process::Command::new("defaults")
            .arg("read")
            .arg("g")
            .arg("AppleInterfaceStyle")
            .output()
            .map(|o| o.stdout.starts_with(b"Dark"))
    }
    fn enable_dark(dark: bool) -> Result<bool, std::io::Error> {
        process::Command::new("osascript")
            .arg("-e")
            .arg(&format!(
                " 
tell application \"System Events\"
    tell appearance preferences
        set dark mode to {:?}
    end tell
end tell
",
                dark
            ))
            .status()
            .map(|s| s.success())
    }
}

impl IsItDark for MacOS {
    type Error = std::io::Error;
    fn is_dark(&self) -> Result<bool, Self::Error> {
        Self::dark()
    }
    fn is_light(&self) -> Result<bool, Self::Error> {
        Self::dark().map(|d| !d)
    }
    fn set_dark(&mut self) -> Result<bool, Self::Error> {
        Self::enable_dark(true)
    }
    fn set_light(&mut self) -> Result<bool, Self::Error> {
        Self::enable_dark(false)
    }
}
