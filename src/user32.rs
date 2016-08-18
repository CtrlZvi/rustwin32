// Wrappers around items provided by Winuser.h, User32.lib, and User32.dll
extern crate user32;

use windef::*;

use winapi;

use std;
use std::os::windows::ffi::OsStrExt;

// TODO(zeffron 2016-08-15) Turn this into a fleshed out type with structs
// and lifetime management.
pub struct Window {
    // TODO(zeffron 2016-08-15) We should use NonZero or equivalent once it
    // stabilizes
    // FIXME(zeffron 2016-08-16) This is public for now to allow the sample to
    // pass a Window to default_window_procedure while still taking a
    // winapi::HWND as its parameter.
    pub handle: winapi::HWND,
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
    // FIXME(zeffron 2016-08-16) This is public for now to allow the sample to
    // not need to pass in a module to register_class_extended. This seems to
    // indicate that it is acceptable to pass a null module to
    // RegisterClassEx, but that is based on a sample project which could be
    // violating best practice.
    // This need to stop being public, and can be done by either writing a
    // function that will make them, or by optionally taking them (in the short
    // term).
    pub handle: winapi::HINSTANCE,
}

// TODO(zeffron 2016-08-15) Turn this into a fleshed out type with structs
// and lifetime management.
pub struct Icon {
    // TODO(zeffron 2016-08-15) We should use NonZero or equivalent once it
    // stabilizes
    handle: winapi::HICON,
}

// TODO(zeffron 2016-08-15) Turn this into a fleshed out type with structs
// and lifetime management.
pub struct Cursor {
    // TODO(zeffron 2016-08-15) We should use NonZero or equivalent once it
    // stabilizes
    handle: winapi::HCURSOR,
}

// TODO(zeffron 2016-08-15) Turn this into a fleshed out type with structs
// and lifetime management.
pub struct Brush {
    // TODO(zeffron 2016-08-15) We should use NonZero or equivalent once it
    // stabilizes
    handle: winapi::HBRUSH,
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
    // TODO(zeffron 2016-08-15) See if we can enforce the 256 character limit
    // with the type system.
    String(String),
    Atom(u16),
}

pub fn create_window_extended(
    extended_style : WindowStyleExtended,
    class_name : WindowClassName,
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
    let window_handle = unsafe {
        user32::CreateWindowExW(
            extended_style.bits() as u32,
            match class_name {
                WindowClassName::String(ref val) => std::ffi::OsStr::new(&val).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr(),
                WindowClassName::Atom(atom) => atom as *const u16,
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
            parent_window.unwrap_or(Window { handle: std::ptr::null_mut(), }).handle,
            menu.unwrap_or(Menu { handle: std::ptr::null_mut(), }).handle,
            module.unwrap_or(Module { handle: std::ptr::null_mut(), }).handle,
            parameter,
        )
    };
    match window_handle {
        val if val == std::ptr::null_mut() => Err(std::io::Error::last_os_error()),
        val => Ok(Window { handle: val }),
    }
}

pub struct WindowClassExtended {
    pub style : WindowClassStyle,
    pub window_procedure : WindowProcedure,
    pub class_extra : i32,
    pub window_extra : i32,
    // TODO(zeffron 2016-08-15) I think this can be null, but the documentation
    // doesn't say so. It needs to be looked into. If it can't be null, we
    // should use NonZero or equivalent when it stabilizes
    pub module : Module,
    pub icon : Option<Icon>,
    pub cursor : Option<Cursor>,
    pub background_brush : Option<Brush>,
    pub menu_name : Option<String>,
    pub class_name : WindowClassName,
    pub small_icon : Option<Icon>,
}

bitflags! {
    pub flags WindowClassStyle : u32 {
        const BYTE_ALIGN_CLIENT = winapi::CS_BYTEALIGNCLIENT,
        const BYTE_ALIGN_WINDOW = winapi::CS_BYTEALIGNWINDOW,
        const CLASS_DEVICE_CONTEXT = winapi::CS_CLASSDC,
        const DROP_SHADOW = winapi::CS_DROPSHADOW,
        const DOUBLE_CLICKS = winapi::CS_DBLCLKS,
        const GLOBAL_CLASS = winapi::CS_GLOBALCLASS,
        const HORIZONTAL_REDRAW = winapi::CS_HREDRAW,
        const NO_CLOSE = winapi::CS_NOCLOSE,
        const OWN_DEVICE_CONTEXT = winapi::CS_OWNDC,
        const PARENT_DEVICE_CONTEXT = winapi::CS_PARENTDC,
        const SAVE_BITS = winapi::CS_SAVEBITS,
        const VERTICAL_REDRAW = winapi::CS_VREDRAW,
    }
}

// TODO(zeffron 2016-08-15) Figure out how to make this into a proper type.
// It looks like to do it the way we want to we'll need to figure out how to
// make a closure passable as a FFI callback
pub type WindowProcedure = winapi::WNDPROC;

pub fn register_class_extended(class : &WindowClassExtended) -> Result<u16, std::io::Error> {
    let window_class = winapi::WNDCLASSEXW {
        cbSize : std::mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT,
        style : class.style.bits(),
        lpfnWndProc : match class.window_procedure {
            // TODO(zeffron 2016-08-16) We should have a compile time
            // assertion that Window and winapi::HWND are the same size, if
            // possible.
            // If not, we should have a runtime assertion in debug builds.
            Some(procedure) => unsafe { Some(std::mem::transmute(procedure)) },
            None => None,
        },
        cbClsExtra : class.class_extra,
        cbWndExtra : class.window_extra,
        hInstance : class.module.handle,
        hIcon : class.icon.as_ref().unwrap_or(&Icon { handle: std::ptr::null_mut() }).handle,
        hCursor : class.cursor.as_ref().unwrap_or(&Cursor { handle: std::ptr::null_mut() }).handle,
        hbrBackground : class.background_brush.as_ref().unwrap_or(&Brush { handle: std::ptr::null_mut() }).handle,
        lpszMenuName : match class.menu_name {
            Some(ref menu_name) => std::ffi::OsStr::new(&menu_name).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr(),
            None => std::ptr::null(),
        },
        lpszClassName : match class.class_name {
            WindowClassName::String(ref class_name) => std::ffi::OsStr::new(&class_name).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr(),
            WindowClassName::Atom(atom) => atom as winapi::LPCWSTR,
        },
        hIconSm : class.small_icon.as_ref().unwrap_or(&Icon { handle: std::ptr::null_mut() }).handle,
    };
    let atom = unsafe { user32::RegisterClassExW(&window_class) };
    match atom {
        0 =>  Err(std::io::Error::last_os_error()),
        val => Ok(val),
    }
}

pub fn adjust_window_rectangle_extended(rect: &mut Rectangle, style: WindowStyle, has_menu: bool, extended_style: WindowStyleExtended) -> Result<(), std::io::Error> {
    let result = unsafe { user32::AdjustWindowRectEx(rect, style.bits(), has_menu as i32, extended_style.bits())};
    match result {
        0 =>  Err(std::io::Error::last_os_error()),
        _ => Ok(()),
    }
}

pub fn default_window_procedure(window: Option<Window>, message_identifier: u32, parameter1: u64, parameter2: i64) -> i64 {
    unsafe {
        user32::DefWindowProcW(
            match window {
                Some(window) => window.handle,
                None => std::ptr::null_mut(),
            },
            message_identifier as winapi::UINT,
            parameter1 as winapi::WPARAM,
            parameter2 as winapi::LPARAM,
        )
    }
}