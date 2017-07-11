# xlib-programming-rust

This repository contains minimal X11 and OpenGL examples, written in Rust without any extra libraries (except for bindings 
to X11, but you can take them out if you want to). This repository exists because people told me that it'd be impossible to
create windows without libraries such as QT and GTK. But lo and behold - you can actually create windows without using 100s of
libraries. X11 is not "old" in a way that you can't work with it anymore.

These examples should also show how to set window attributes and draw with the X11 toolkit. This is important for situations
where you don't want to have a 30MB drawing toolkit and you want to reliably deliver simple windows (such as an error message)
to the user (where the overhead of invoking a extra toolkit would be too much). OpenGL may not be available on your system.

X11 is old. But it's not depreceated. Yes, everybody uses GTK and QT, but you don't have to use proprietary toolkits to draw 
a simple "Ok, cancel" message.

These people also told me that it would be unfeasible to write a GUI toolkit from scratch. While I do somehow agree with this,
it's neither impossible nor infeasible to write a basic 2D GUI toolkit using OpenGL.

These examples are targeted to run on anything > OpenGL 3.1 (because of shaders, you can of course strip out the OpenGL part
to make them run on lower requirements).

Each example is in the `/examples` folder and should compile on its own. The example that I'm currently working on is in the 
`/src` folder.

## Dependencies

- libGL.so
- libx11.so
