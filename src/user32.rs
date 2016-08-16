// Wrappers around items provided by Winuser.h, User32.lib, and User32.dll
extern crate user32;

use winapi;

use std;
use std::os::windows::ffi::OsStrExt;

// TODO(zeffron 2016-08-15) Turn this into a fleshed out type with structs
// and lifetime management.
pub struct Window {
    // TODO(zeffron 2016-08-15) We should use NonZero or equivalent once it
    // stabilizes
    handle: winapi::HWND,
}

// TODO(zeffron 2016-08-15) Turn this into a fleshed out type with structs
// and lifetime management.
pub struct Menu {
    // TODO(zeffron 2016-08-15) We should use NonZero or equivalent once it
    // stabilizes
    handle: winapi::HMENU,
}

// TODO(zeffron 2016-08-15) Turn this into a fleshed out type with structs
// and lifetime management.
pub struct Module {
    // TODO(zeffron 2016-08-15) We should use NonZero or equivalent once it
    // stabilizes
    handle: winapi::HINSTANCE,
}

bitflags! {
    pub flags WindowStyle : u32 {
        const BORDER = winapi::WS_BORDER,
        const CAPTION = winapi::WS_CAPTION,
        const CHILD = winapi::WS_CHILD,
        const CHILD_WINDOW = winapi::WS_CHILDWINDOW,
        const CLIP_CHILDREN = winapi::WS_CLIPCHILDREN,
        const CLIP_SIBLINGS = winapi::WS_CLIPSIBLINGS,
        const DISABLED = winapi::WS_DISABLED,
        const DIALOG_FRAME = winapi::WS_DLGFRAME,
        const GROUP = winapi::WS_GROUP,
        const HORIZONTAL_SCROLLBAR = winapi::WS_HSCROLL,
        const ICONIC = winapi::WS_ICONIC,
        const MAXIMIZE = winapi::WS_MAXIMIZE,
        const MAXIMIZE_BOX = winapi::WS_MAXIMIZEBOX,
        const MINIMIZE = winapi::WS_MINIMIZE,
        const MINIMIZE_BOX = winapi::WS_MINIMIZEBOX,
        const OVERLAPPED = winapi::WS_OVERLAPPED,
        const OVERLAPPED_WINDOW = winapi::WS_OVERLAPPEDWINDOW,
        const POPUP = winapi::WS_POPUP,
        const POPUP_WINDOW = winapi::WS_POPUPWINDOW,
        const SIZE_BOX = winapi::WS_SIZEBOX,
        const SYSTEM_MENU = winapi::WS_SYSMENU,
        const TAB_STOP = winapi::WS_TABSTOP,
        const THICK_FRAME = winapi::WS_THICKFRAME,
        const TILED = winapi::WS_TILED,
        const TILED_WINDOW = winapi::WS_TILEDWINDOW,
        const VISIBLE = winapi::WS_VISIBLE,
        const VERTICAL_SCROLLBAR = winapi::WS_VSCROLL,
    }
}

bitflags!{
    pub flags WindowStyleExtended : u32 {
        const ACCEPT_FILES = winapi::WS_EX_ACCEPTFILES,
        const APP_WINDOW = winapi::WS_EX_APPWINDOW,
        const CLIENT_EDGE = winapi::WS_EX_CLIENTEDGE,
        const COMPOSITED = winapi::WS_EX_COMPOSITED,
        const CONTEXT_HELP = winapi::WS_EX_CONTEXTHELP,
        const CONTROL_PARENT = winapi::WS_EX_CONTROLPARENT,
        const DIALOG_MODAL_FRAME = winapi::WS_EX_DLGMODALFRAME,
        const LAYERED = winapi::WS_EX_LAYERED,
        const LAYOUT_RTL = winapi::WS_EX_LAYOUTRTL,
        const LEFT = winapi::WS_EX_LEFT,
        const LEFT_SCROLL_BAR = winapi::WS_EX_LEFTSCROLLBAR,
        const LTR_READING = winapi::WS_EX_LTRREADING,
        const MDI_CHILD = winapi::WS_EX_MDICHILD,
        const NO_ACTIVATE = winapi::WS_EX_NOACTIVATE,
        const NO_INHERIT_LAYOUT = winapi::WS_EX_NOINHERITLAYOUT,
        const NO_PARENT_NOTIFY = winapi::WS_EX_NOPARENTNOTIFY,
        const NO_REDIRECTION_BITMAP = winapi::WS_EX_NOREDIRECTIONBITMAP,
        // const OVERLAPPED_WINDOW = winapi::WS_EX_OVERLAPPEDWINDOW,
        const PALETTE_WINDOW = winapi::WS_EX_PALETTEWINDOW,
        const RIGHT = winapi::WS_EX_RIGHT,
        const RIGHT_SCROLLBAR = winapi::WS_EX_RIGHTSCROLLBAR,
        const RTL_READING = winapi::WS_EX_RTLREADING,
        const STATIC_EDGE = winapi::WS_EX_STATICEDGE,
        const TOOL_WINDOW = winapi::WS_EX_TOOLWINDOW,
        const TOPMOST = winapi::WS_EX_TOPMOST,
        // const TRANSPARENT = winapi::WS_EX_TRANSPARENT,
        const WINDOW_EDGE = winapi::WS_EX_WINDOWEDGE,
    }
}

pub enum WindowClassName {
    String(String),
    Atom(u16),
}

pub fn create_window_extended(
    extended_style : WindowStyleExtended,
    class_name : Option<WindowClassName>,
    window_name : Option<String>,
    style : WindowStyle,
    x : Option<i32>,
    y : Option<i32>,
    width : Option<i32>,
    height : Option<i32>,
    parent_window : Option<Window>,
    menu : Option<Menu>,
    module : Option<Module>,
    parameter : winapi::LPVOID,
) -> Result<Window, std::io::Error> {
    let window_name = match window_name {
        Some(ref val) => Some(std::ffi::OsStr::new(&val).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()),
        None => None,
    };
    let class_name_2 = match class_name {
        Some(WindowClassName::String(ref val)) => Some(std::ffi::OsStr::new(&val).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()),
        _ => None,
    };
    let window_handle = unsafe {
        user32::CreateWindowExW(
            extended_style.bits() as u32,
            match class_name_2 {
                Some(ref val) => val.as_ptr(),
                None => match class_name {
                    Some(WindowClassName::Atom(val)) => val as winapi::LPCWSTR,
                    None => std::ptr::null_mut(),
                    _ => panic!("The string case should have been translated"),
                }
            },
            match window_name {
                Some(ref val) => val.as_ptr(),
                None => std::ptr::null(),
            },
            style.bits() as u32,
            x.unwrap_or(winapi::CW_USEDEFAULT),
            y.unwrap_or(winapi::CW_USEDEFAULT),
            width.unwrap_or(winapi::CW_USEDEFAULT),
            height.unwrap_or(winapi::CW_USEDEFAULT),
            parent_window.unwrap_or(
                Window {
                    handle: std::ptr::null_mut(),
                }
            ).handle,
            menu.unwrap_or(
                Menu {
                    handle: std::ptr::null_mut(),
                }
            ).handle,
            module.unwrap_or(
                Module {
                    handle: std::ptr::null_mut(),
                }
            ).handle,
            parameter,
        )
    };
    match window_handle {
        val if val == std::ptr::null_mut() => Err(std::io::Error::last_os_error()),
        val => Ok(Window { handle: val }),
    }
}