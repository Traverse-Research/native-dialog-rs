use super::Dialog;
use crate::{MessageType, Modal};
use raw_window_handle::RawWindowHandle;

pub struct MessageAlert<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub(crate) modal: Modal,
}

impl Dialog for MessageAlert<'_> {
    type Output = ();
}

pub struct MessageConfirm<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub(crate) modal: Modal,
}

impl Dialog for MessageConfirm<'_> {
    type Output = bool;
}
