use crate::IsItDark;

use std::io;

use winreg::RegKey;

#[derive(Default)]
pub(crate) struct Windows;

impl Windows {
    fn dark() -> Result<bool, io::Error> {
        RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
            .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
            .and_then(|key| key.get_value::<u32, _>("AppsUseLightTheme").map(|n| n == 0))
    }
    fn enable_dark(dark: bool) -> Result<bool, io::Error> {
        let val: u32 = if dark { 0 } else { 1 };
        RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
            .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
            .and_then(|key| key.set_value("AppsUseLightTheme", &val).map(|_| dark))
    }
}

impl IsItDark for Windows {
    type Error = std::io::Error;
    fn is_dark(&self) -> Result<bool, Self::Error> {
        Self::dark()
    }

    fn is_light(&self) -> Result<bool, Self::Error> {
        Self::dark().map(|s| !s)
    }
    fn set_dark(&mut self) -> Result<bool, Self::Error> {
        Self::enable_dark(true)
    }
    fn set_light(&mut self) -> Result<bool, Self::Error> {
        Self::enable_dark(false)
    }
}
