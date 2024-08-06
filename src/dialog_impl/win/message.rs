use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::{MessageType, Modal, Result};
use raw_window_handle::RawWindowHandle;
use winapi::um::winuser::{MB_APPLMODAL, MB_SYSTEMMODAL, MB_TASKMODAL};

impl DialogImpl for MessageAlert<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        message_box(MessageBoxParams {
            title: self.title,
            text: self.text,
            typ: self.typ,
            owner: self.owner,
            ask: false,
            modal: self.modal,
        })?;
        Ok(())
    }
}

impl DialogImpl for MessageConfirm<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        message_box(MessageBoxParams {
            title: self.title,
            text: self.text,
            typ: self.typ,
            owner: self.owner,
            ask: true,
            modal: self.modal,
        })
    }
}

struct MessageBoxParams<'a> {
    title: &'a str,
    text: &'a str,
    typ: MessageType,
    owner: Option<RawWindowHandle>,
    ask: bool,
    modal: Modal,
}

fn message_box(params: MessageBoxParams) -> Result<bool> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::shared::windef::HWND;
    use winapi::um::winuser::{
        MessageBoxW, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MB_YESNO,
    };

    let owner = match params.owner {
        Some(RawWindowHandle::Win32(handle)) => handle.hwnd.get() as HWND,
        _ => null_mut(),
    };

    let text: Vec<u16> = OsStr::new(params.text)
        .encode_wide()
        .chain(once(0))
        .collect();

    let caption: Vec<u16> = OsStr::new(params.title)
        .encode_wide()
        .chain(once(0))
        .collect();

    let u_type = match params.typ {
        MessageType::Info => MB_ICONINFORMATION,
        MessageType::Warning => MB_ICONWARNING,
        MessageType::Error => MB_ICONERROR,
    } | if params.ask { MB_YESNO } else { MB_OK };

    let modal = match params.modal {
        Modal::App => MB_APPLMODAL,
        Modal::System => MB_SYSTEMMODAL,
        Modal::Task => MB_TASKMODAL,
    };

    let ret = super::with_visual_styles(|| unsafe {
        MessageBoxW(owner, text.as_ptr(), caption.as_ptr(), u_type | modal)
    });

    match ret {
        0 => Err(std::io::Error::last_os_error().into()),
        x => Ok(x == IDYES),
    }
}
