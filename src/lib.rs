use std::error::Error;

// Only if feature is windows and target_os is windows
#[cfg(all(target_os = "linux", feature = "os"))]
mod linux;
#[cfg(all(target_os = "macos", feature = "os"))]
mod macos;
#[cfg(all(target_os = "windows", feature = "os"))]
mod windows;

mod time;

pub use crate::time::SmartTime;
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
    #[cfg(all(target_os = "windows", feature = "os"))]
    #[derive(Default)]
    pub struct GenericDetectDark {
        sys: crate::windows::Windows,
        smart_time: Option<SmartTime>,
    }

    #[cfg(all(target_os = "linux", feature = "os"))]
    #[derive(Default)]
    pub struct GenericDetectDark {
        sys: crate::linux::Linux,
        smart_time: Option<SmartTime>,
    }

    #[cfg(all(target_os = "macos", feature = "os"))]
    #[derive(Default)]
    pub struct GenericDetectDark {
        sys: crate::linux::Linux,
        smart_time: Option<SmartTime>,
    }

    #[cfg(any(
        not(any(target_os = "windows", target_os = "macos", target_os = "linux")),
        not(feature = "os")
    ))]
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
            self.sys.is_dark().or_else(|e| {
                if let Some(t) = self.smart_time {
                    t.is_dark()
                } else {
                    Err(e)
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
