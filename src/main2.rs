#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate x11_dl;
extern crate dbus;

use std::ffi::CString;
use std::os::raw::c_ulong;

const GL_TRUE: i32 = 1;
const GL_FALSE: i32 = 0;

const GL_DEPTH_TEST: GLenum = 0x0B71;
const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;

// Application-specific dbus path
const APP_MAIN_MENU_OBJECT: &'static str = "com/maps4print/AppMenu/Main";
const APP_MAIN_MENU_IFACE: &'static str = "com.maps4print.AppMenu.Main";

const DBUS_NAME: &'static str = "com.canonical.AppMenu.Registrar";
const REG_OBJECT: &'static str = "/com/canonical/AppMenu/Registrar";
const REG_IFACE: &'static str = "com.canonical.AppMenu.Registrar";
const DEBUG_IFACE: &'static str = "com.canonical.AppMenu.Renderer";
const DEBUG_OBJECT: &'static str = "/com/canonical/AppMenu/Renderer";

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
        Err(xerr) => panic!("Error opening GLX: {}", xerr.detail()),
    };

    let mut att = [x11_dl::glx::GLX_RGBA, x11_dl::glx::GLX_DEPTH_SIZE, 24, x11_dl::glx::GLX_DOUBLEBUFFER, x11_dl::glx::GLX_NONE];

    let vi = unsafe { (glx_ext.glXChooseVisual)(dpy, 0, &mut att[0]) };

    let mut visual_info = { if vi.is_null() {
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

    // show window
    unsafe { (xlib.XMapWindow)(display, window) };
    unsafe { (xlib.XStoreName)(display, window, window_title.as_ptr()) };

    let glc = unsafe { (glx_ext.glXCreateContext)(display, &mut *visual_info, ::std::ptr::null_mut(), GL_TRUE) };
    unsafe { (glx_ext.glXMakeCurrent)(display, window, glc) };

    unsafe { glEnable(GL_DEPTH_TEST) };

    let mut cur_xevent = x11_dl::xlib::XEvent { pad: [0;24] };
    let mut cur_window_attributes: x11_dl::xlib::XWindowAttributes = unsafe { std::mem::uninitialized() };
/*
    // Add menus to window
    let conn = dbus::Connection::get_private(dbus::BusType::Session).unwrap();
    let factory = dbus::tree::Factory::new_sync::<()>();
    let event_queue = factory.tree(());
    // add_menus_to_window(window, &conn, &factory, &mut event_queue);
    // for _ in event_queue.run(&conn, conn.iter(1000)) { println!("hello from event queue!");}
*/

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

    // remove_menus_window(window);
}

/*
fn add_menus_to_window<D: dbus::tree::DataType, M: dbus::tree::MethodType<D>>(window: c_ulong, conn: &dbus::Connection, factory: &dbus::tree::Factory<M, D>, tree: &mut dbus::tree::Tree<M, D>)
{
    use dbus::{Message, BusType, Connection, NameFlag};
    use dbus::tree::Factory;

    use dbus::MessageItem::*;
    use dbus::tree::DataType;

    // the window is the X11 id.
    // First, prepare and insert the menus and callbacks
    conn.register_name(APP_MAIN_MENU_OBJECT, NameFlag::ReplaceExisting as u32).unwrap();
    let a = factory.object_path(APP_MAIN_MENU_IFACE, ());
    tree.insert(a.introspectable());
    let t = factory.interface("com.example.dbustest", ());
    tree.insert(
        t.add_m(
            factory.method("Hello", (), |m| {

                let s = format!("Hello {}!", m.msg.sender().unwrap());
                Ok(vec!(m.msg.method_return().append1(s)))
            }).outarg::<&str,_>("reply")
        )
    );

    tree.set_registered(&conn, true).unwrap();

    // Then, call com.canonical.AppMenu.Registrar.RegisterWindow()
    // RegisterWindow(X11 window ID, path to the menus) -> () - add menus to a window
    let mut m = Message::new_method_call(DBUS_NAME, REG_OBJECT, REG_IFACE, "RegisterWindow").unwrap();

    // this is the "(uo)" thing you see everywhere
    m.append_items(&[UInt32(window as u32), Str(APP_MAIN_MENU_OBJECT.into())]);

    let r = conn.send(m).unwrap();
}

fn remove_menus_window(window: c_ulong)
{
    use dbus::{Message, BusType, Connection, NameFlag};
    use dbus::tree::Factory;

    use dbus::MessageItem::*;

    let c = Connection::get_private(BusType::Session).unwrap();
    let mut m = Message::new_method_call(DBUS_NAME, REG_OBJECT, REG_IFACE, "UnRegisterWindow").unwrap();
    m.append_items(&[UInt32(window as u32)]);

    let r = c.send(m).unwrap();
    println!("reply unregister: {:?}", r);
}
*/
