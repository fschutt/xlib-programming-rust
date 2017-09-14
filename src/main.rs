#![allow(dead_code)]
#![allow(unused_variables)]

extern crate x11_dl;

use std::ffi::CString;
use x11_dl::xlib::{True, False, GrabModeAsync, CurrentTime, XEvent};
use x11_dl::xf86vmode::XF86VidModeModeInfo;

const GL_TRUE: i32 = 1;
const GL_FALSE: i32 = 0;

const GL_DEPTH_TEST: GLenum = 0x0B71;
const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;

// window decorations for motif
const MWM_DECOR_ALL: u64      = 1;
const MWM_DECOR_BORDER: u64   = 1 << 1;
const MWM_DECOR_RESIZEH: u64  = 1 << 2;
const MWM_DECOR_TITLE: u64    = 1 << 3;
const MWM_DECOR_MENU: u64     = 1 << 4;
const MWM_DECOR_MINIMIZE: u64 = 1 << 5;
const MWM_DECOR_MAXIMIZE: u64 = 1 << 6;

// clientDecoration, transientDecoration

type GLenum =       u32;
type GLboolean =    u8;
type GLbitfield =   u32;
type GLbyte =       i8;
type GLshort =      i16;
type GLint =        i32;
type GLsizei =      i32;
type GLubyte =      u8;
type GLushort =     u16;
type GLuint =       u8;
type GLfloat =      f32;
type GLclampf =     f32;
type GLdouble =     f64;
type GLclampd =     f64;
type GLvoid =       ();


#[link(kind = "dylib", name = "GL")]
extern {
    fn glEnable(cap: GLenum) -> ();
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> ();
    fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> ();
    fn glClear(mask: GLbitfield) -> ();
}

/// Window hints (needed for changing fullscreen and windowed mode)
#[repr(C)]
pub struct Hints {
    flags: u64,
    functions: u64,
    decorations: u64,
    input_mode: i64,
    status: u64,
}

/// Toggle fullscreen / windowing
/// Returns the ID of the _MOTIF_WM_HINTS property, so you can refer to this property later on
/// (may or may not be useful)
fn toggle_fullscreen_windowed(display: &mut x11_dl::xlib::Display, window: u64, xlib: &x11_dl::xlib::Xlib)
-> Result<(), String>
{
    // Old code, does not work correctly, but may still be useful
/*
    let default_screen = unsafe { (xlib.XDefaultScreen)(display) };
    let default_screen_ptr = unsafe { (xlib.XScreenOfDisplay)(display, default_screen) };
    let mut width = unsafe { (xlib.XWidthOfScreen)(default_screen_ptr) };
    let mut height = unsafe { (xlib.XHeightOfScreen)(default_screen_ptr) };

    // Some WMs do not respect this.
    // toggle_foreground_window(display, window, xlib)?;

    let _xf86 = x11_dl::xf86vmode::Xf86vmode::open();
    let xf86 = match _xf86 {
        Ok(x) => x,
        Err(xerr) => return Err(String::from(xerr.detail())),
    };

    let mut mode_count: i32 = unsafe { std::mem::uninitialized() };
    let mut modes: *mut *mut XF86VidModeModeInfo = std::ptr::null_mut();
    let video_mode_info = unsafe { (xf86.XF86VidModeGetAllModeLines)(display, default_screen, &mut mode_count, &mut modes) };
    if video_mode_info == 0 { return Err("no appropriate video mode found".into()); }
    if modes.is_null() { return Err("could not find any video modes".into()); }
    let modes_slice = unsafe { std::slice::from_raw_parts_mut(modes, mode_count as usize) };

    let best_mode = get_best_video_mode(xlib, display, default_screen, &xf86, &mut width, &mut height);
    if best_mode.is_none() { unsafe { (xlib.XFree)(modes as *mut std::os::raw::c_void) }; return Err("could not find a good video mode".into()); }
    let best_mode = best_mode.unwrap();

    // Initialize video modes
    // todo: figure out best video mode
    // for some reason, other libraries leave this value unchecked. It is not documented what the return value actually represents
    let _switch_successful = unsafe { (xf86.XF86VidModeSwitchToMode)(display, default_screen, modes_slice[best_mode]) };
    unsafe { (xlib.XFree)(modes as *mut std::os::raw::c_void) };

    // There is zero documentation on the return values of these functions. They seem to work regardless of what they return
    unsafe {
        let _xf86_vidmode_set_viewport_successful = (xf86.XF86VidModeSetViewPort)(display, default_screen, 0, 0);
        // if xf86_vidmode_set_viewport_successful != 0 { return Err("xf86_vidmode_set_viewport not successful {:?}".into())}
        let _x_move_resize_window = (xlib.XMoveResizeWindow)(display, window, 0, 0, width as u32, height as u32);
        // if x_move_resize_window != 0 { return Err("x_move_resize_window not successful".into()); }
        let _x_map_raised = (xlib.XMapRaised)(display,window);
        // if x_map_raised != 0 { return Err("x_map_raised not successful".into()); }
        let _x_grab_pointer = (xlib.XGrabPointer)(display, window, True, 0, GrabModeAsync, GrabModeAsync, window, 0, CurrentTime);
        // if x_grab_pointer != 0 { return Err("x_grab_pointer not successful".into()); }
        let _x_grab_keyboard = (xlib.XGrabKeyboard)(display, window, False, GrabModeAsync, GrabModeAsync, CurrentTime);
        // if x_grab_keyboard != 0 { return Err("x_grab_keyboard not successful".into()); }
    }

    Ok(())
*/

    /// Use _NET_WM_FULLSCREEN_MONITORS for making a fullscreen window
    use x11_dl::xlib::{ClientMessageData, XClientMessageEvent, ClientMessage,
                       SubstructureRedirectMask, SubstructureNotifyMask};

    let net_wm_state_fullscreen = CString::new("_NET_WM_FULLSCREEN_MONITORS").unwrap();
    let property = unsafe { (xlib.XInternAtom)(display, net_wm_state_fullscreen.as_ptr(), True) };
    if property == 0 { return Err("Could not set atom".into()); }

    let mut data = ClientMessageData::new();
    data.set_long(0, 1);
    data.set_long(1, 1);
    data.set_long(2, 1);
    data.set_long(3, 1);
    data.set_long(4, 1);

    let event = XClientMessageEvent {
        type_: ClientMessage,
        serial: unsafe { ::std::mem::uninitialized() },
        send_event: True,
        display: display,
        window: window,
        message_type: property,
        format: 32,
        data: data,
    };

    let mut x_event = XEvent::from(event);

    let result = unsafe {
        (xlib.XSendEvent)(display,
            (xlib.XRootWindow)(display, (xlib.XDefaultScreen)(display)),
            False,
            SubstructureRedirectMask | SubstructureNotifyMask,
            &mut x_event)
    };

    println!("result maximize: {:?}", result);

    Ok(())
}

fn get_best_video_mode(xlib: &x11_dl::xlib::Xlib, display: &mut x11_dl::xlib::Display,
                       screen: i32, xf86: &x11_dl::xf86vmode::Xf86vmode,
                       width: &mut i32, height: &mut i32)
-> Option<usize>
{
    let mut mode_count: i32 = unsafe { std::mem::zeroed() };
    let mut modes: *mut *mut XF86VidModeModeInfo = std::ptr::null_mut();

    if unsafe { (xf86.XF86VidModeGetAllModeLines)(display, screen, &mut mode_count, &mut modes) } != 0 {

        let mut best_mode: usize = 0;
        let mut best_match = ::std::i32::MAX;
        let modes_slice = unsafe { std::slice::from_raw_parts_mut(modes, mode_count as usize) };

        // let item = (*slice[i]).hdisplay;

        for (idx, item) in modes_slice.iter().enumerate() {
            if item.is_null() || (*item).is_null() { continue; }
            let hdisplay = unsafe { (**item).hdisplay };
            let vdisplay = unsafe { (**item).vdisplay };
            let cur_match = (*width  - hdisplay  as i32) * (*width  - hdisplay  as i32) +
                            (*height - vdisplay as i32) * (*height - vdisplay as i32);

            if cur_match < best_match  {
                best_match = cur_match;
                best_mode  = idx;
            }
        }

        if !modes_slice[best_mode].is_null() {
            *width  = unsafe { (*modes_slice[best_mode]).hdisplay as i32 };
            *height = unsafe { (*modes_slice[best_mode]).vdisplay  as i32 };
        }

        unsafe { (xlib.XFree)(modes as *mut std::os::raw::c_void) };
        return Some(best_mode);
    }

    None
}

/*
fn toggle_foreground_window(display: &mut x11_dl::xlib::Display, window: u64, xlib: &x11_dl::xlib::Xlib)
-> Result<(), String>
{
    use x11_dl::xlib::{ClientMessageData, XClientMessageEvent, SubstructureRedirectMask, SubstructureNotifyMask};

    let net_wm_state_fullscreen = CString::new("_NET_WM_STATE_FULLSCREEN").unwrap();
    let property = unsafe { (xlib.XInternAtom)(display, net_wm_state_fullscreen.as_ptr(), True) };
    if property == 0 { return Err("Could not set atom".into()); }

    let mut data = ClientMessageData::new();
    data.set_long(0, 2); // _NET_WM_STATE_TOGGLE
    data.set_long(1, property as i64);
    data.set_long(2, 0);    // no second property to toggle
    data.set_long(3, 1);
    data.set_long(4, 0);

    let event = XClientMessageEvent {
        type_: ::x11_dl::xlib::ClientMessage,
        serial: 0,                  // ???
        send_event: True,
        display: display,
        window: window,
        message_type: property, // _NET_WM_STATE
        format: 32,
        data: data,
    };


    let mut x_event = XEvent::from(event);
    unsafe { (xlib.XSendEvent)(display, window, False, SubstructureRedirectMask | SubstructureNotifyMask, &mut x_event); }

    Ok(())
}
*/

fn toggle_borders(display: &mut x11_dl::xlib::Display, window: u64, xlib: &x11_dl::xlib::Xlib)
-> Result<u64, String>
{
    // credit: https://tonyobryan.com/index.php?article=9
    let mut hints: Hints = unsafe { std::mem::uninitialized() };
    hints.flags = 2;        // Specify that we're changing the window decorations.
    hints.decorations = 0;  // 0 (false) means that window decorations should go bye-bye.

    let motif_wm_hints = CString::new("_MOTIF_WM_HINTS").unwrap();

    let property = unsafe { (xlib.XInternAtom)(display, motif_wm_hints.as_ptr(), True) };
    if property == 0 { return Err(String::from("Could not set XInternAtom")); }

    let hints_ptr: *const u8 = &hints as *const _ as *const u8;
    unsafe { (xlib.XChangeProperty)(display, window, property, property, 32, x11_dl::xlib::PropModeReplace, hints_ptr, 5) };
    Ok(property)

}

fn main() {
    let xlib = match x11_dl::xlib::Xlib::open() {
        Ok(x) => x,
        Err(xerr) => panic!("Error: {}", xerr.detail()),
    };

    let display_int = 0_i8;
    let dpy = unsafe { (xlib.XOpenDisplay)(&display_int) };

    let mut display = {
        if dpy.is_null() {
            panic!("Error opening connection to X Server!");
        } else {
            unsafe { &mut*dpy }
        }
    };

    // get root window
    let root = unsafe { (xlib.XDefaultRootWindow)(display) };

    let glx_ext = match x11_dl::glx::Glx::open() {
        Ok(ext) => ext,
        Err(xerr) => panic!("Error: {}", xerr.detail()),
    };

    let mut att = [x11_dl::glx::GLX_RGBA, x11_dl::glx::GLX_DEPTH_SIZE, 24, x11_dl::glx::GLX_DOUBLEBUFFER, x11_dl::glx::GLX_NONE];

    let vi = unsafe { (glx_ext.glXChooseVisual)(dpy, 0, &mut att[0]) };

    let visual_info = { if vi.is_null() {
            panic!("Display does not meet minimum requirements: RGBA buffer, 24-bit depth, double-buffered display");
        } else {
            unsafe { &mut*vi }
        }
    };

    let cmap = unsafe { (xlib.XCreateColormap)(display, root, visual_info.visual, x11_dl::xlib::AllocNone) };

    let mut window_attributes: x11_dl::xlib::XSetWindowAttributes = unsafe { std::mem::uninitialized() };
    window_attributes.event_mask = x11_dl::xlib::ExposureMask | x11_dl::xlib::KeyPressMask;
    window_attributes.colormap = cmap;

    // construct window
    let window = unsafe { (xlib.XCreateWindow)(display, root, 0, 0, 600, 600, 0, visual_info.depth,
                                            1 /* InputOutput */, visual_info.visual,
                                            x11_dl::xlib::CWColormap | x11_dl::xlib::CWEventMask,
                                            &mut window_attributes) };

    let window_title = CString::new("illustrate!").unwrap();

    // toggle_borders(display, window, &xlib).unwrap();
    toggle_fullscreen_windowed(&mut display, window, &xlib).unwrap();

    // show window
    unsafe { (xlib.XMapWindow)(display, window) };
    unsafe { (xlib.XStoreName)(display, window, window_title.as_ptr()) };

    let glc = unsafe { (glx_ext.glXCreateContext)(display, &mut *visual_info, ::std::ptr::null_mut(), GL_TRUE) };
    unsafe { (glx_ext.glXMakeCurrent)(display, window, glc) };

    unsafe { glEnable(GL_DEPTH_TEST) };

    let mut cur_xevent = x11_dl::xlib::XEvent { pad: [0;24] };
    let mut cur_window_attributes: x11_dl::xlib::XWindowAttributes = unsafe { std::mem::uninitialized() };

    // todo: poll events?

    // todo: setup opengl 3.1 or 3.3

    loop {
        unsafe { (xlib.XNextEvent)(display, &mut cur_xevent) };

        let cur_event_type = cur_xevent.get_type();

        match cur_event_type {
            x11_dl::xlib::Expose => {
                unsafe { (xlib.XGetWindowAttributes)(display, window, &mut cur_window_attributes) };
                unsafe { glViewport(0, 0, cur_window_attributes.width, cur_window_attributes.height) };

                /* do drawing here */
                unsafe { glClearColor(1.0, 1.0, 1.0, 0.0) };
                unsafe { glClear(GL_COLOR_BUFFER_BIT) };

                unsafe { (glx_ext.glXSwapBuffers)(display, window) };
            },
            x11_dl::xlib::KeyPress => {
                unsafe { (glx_ext.glXMakeCurrent)(display, 0 /* None ? */, ::std::ptr::null_mut()) };
                unsafe { (glx_ext.glXDestroyContext)(display, glc) };
                unsafe { (xlib.XDestroyWindow)(display, window) };
                unsafe { (xlib.XCloseDisplay)(display) };
                break;
            },
            _ => { },
        }
    }
}
