use std::process;

use crate::IsItDark;

// FIXME: Only detects gtk

#[derive(Default)]
pub(crate) struct Linux;

impl IsItDark for Linux {
    type Error = std::io::Error;

    fn is_dark(&self) -> Result<bool, Self::Error>
    where
        Self::Error: std::error::Error,
    {
        process::Command::new("gsettings")
            .arg("get")
            .arg("org.gnome.desktop.interface")
            .arg("gtk-theme")
            .output()
            .and_then(|o| {
                String::from_utf8(o.stdout)
                    .map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                    })
                    .map(|s| s.ends_with("-dark"))
            })
    }

    fn is_light(&self) -> Result<bool, Self::Error>
    where
        Self::Error: std::error::Error,
    {
        todo!()
    }

    fn set_light(&mut self) -> Result<bool, Self::Error>
    where
        Self::Error: std::error::Error,
    {
        todo!()
    }

    fn set_dark(&mut self) -> Result<bool, Self::Error>
    where
        Self::Error: std::error::Error,
    {
        todo!()
    }
}
