#[cfg(all(feature = "default_dark", feature = "default_light"))]
compile_error!(
    "feature \"default_dark\" and feature \"default_light\" cannot be enabled at the same time"
);

use std::error::Error;

//#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;
// #[cfg(target_os="macos")]
mod macos;

mod time;

pub use crate::time::SmartTime;

#[cfg(feature = "default_dark")]
pub(crate) const DEFAULT_DARK: bool = true;

#[cfg(any(
    feature = "default_light",
    not(any(feature = "default_dark", feature = "default_light"))
))]
pub(crate) const DEFAULT_DARK: bool = false;

/// Trait to detect if it is dark
pub trait IsItDark
where
    Self: Sized,
{
    /// Error returned by theme handling
    type Error;
    /// Detect if theme is currently set to dark
    fn is_dark(&self) -> Result<bool, Self::Error>
    where
        Self::Error: Error;
    /// Detect if theme is currently set to light
    fn is_light(&self) -> Result<bool, Self::Error>
    where
        Self::Error: Error,
    {
        self.is_dark().map(|d| !d)
    }
    /// Set current theme to light
    fn set_light(&mut self) -> Result<bool, Self::Error>
    where
        Self::Error: Error;
    /// Set current theme to dark
    fn set_dark(&mut self) -> Result<bool, Self::Error>
    where
        Self::Error: Error;
}

mod generic {
    use cfg_if::cfg_if;

    use crate::SmartTime;
    #[cfg(target_os = "windows")]
    #[derive(Default)]
    pub struct GenericDetectDark {
        sys: crate::windows::Windows,
        smart_time: Option<SmartTime>,
    }

    #[cfg(target_os = "linux")]
    #[derive(Default)]
    pub struct GenericDetectDark {
        sys: crate::linux::Linux,
        smart_time: Option<SmartTime>,
    }

    #[cfg(target_os = "macos")]
    #[derive(Default)]
    pub struct GenericDetectDark {
        sys: crate::linux::Linux,
        smart_time: Option<SmartTime>,
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    #[derive(Default)]
    pub struct GenericDetectDark {
        sys: SmartTime,
        smart_time: Option<SmartTime>,
    }
    impl GenericDetectDark {
        pub fn new(time_fallback: Option<crate::time::SmartTime>) -> Self {
            cfg_if! {
                if #[cfg(any(target_os="windows", target_os="macos", target_os="linux"))] {
                    GenericDetectDark {
                        sys: Default::default(),
                        smart_time: time_fallback
                    }
                } else if #[cfg(not(any(target_os="windows", target_os="macos", target_os="linux")))] {
                    Self {
                        sys: time_fallback.unwrap_or_default(),
                        smart_time: None
                    }
                } else {
                    compile_error!("This should be unreachable, please file an report")
                }
            }
        }
    }

    impl crate::IsItDark for GenericDetectDark {
        type Error = std::io::Error;
        fn is_dark(&self) -> Result<bool, Self::Error> {
            self.sys.is_dark().or_else(|_| {
                if let Some(t) = self.smart_time {
                    t.is_dark()
                } else {
                    Ok(crate::DEFAULT_DARK)
                }
            })
        }
        fn set_dark(&mut self) -> Result<bool, Self::Error> {
            self.sys.set_dark()
        }
        fn set_light(&mut self) -> Result<bool, Self::Error> {
            self.sys.set_light()
        }
    }
}

pub use generic::GenericDetectDark;
