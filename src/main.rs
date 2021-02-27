use pipewire::properties;
use portal::ScreenCast;
use std::{error::Error, ffi::CString, ptr};

mod portal;

/// # Run the Test Application
///
/// We have two main moving parts here. First we make D-Bus calls to obtain a
/// ScreenCast session and start it. Once we have done that we connect to
/// the raw video using Pipewire.
fn main() -> Result<(), Box<dyn Error>> {

    // - - - - - - - - - - - - - - PORTAL - - - - - - - - - - - - - -
    
    let screen_cast = ScreenCast::new()?.start(None)?;

    // - - - - - - - - - - - - - - PIPEWIRE - - - - - - - - - - - - - -

    pipewire::init();
    let pw_loop = pipewire::MainLoop::new()?;
    let pw_context = pipewire::Context::new(&pw_loop)?;

    println!("PW Context: {0:?}", pw_context);

    // FIXME: Add safe bindings so we don't need the unsafe block here...
    // let core = pw_context.connect_fd(pipe_fd.into_fd(), None)?;

    unsafe {
        let pw_core = pipewire_sys::pw_context_connect_fd(
            pw_context.as_ptr(),
            screen_cast.pipewire_fd(),
            ptr::null_mut(),
            0,
        );
        println!("Core:: {0:?}", pw_core);
        // FIXME: add listener to the core so we can observe errors.

        let stream_name = CString::new("Test stream")?;
        use pipewire_sys as pw_sys;
        let stream = pipewire_sys::pw_stream_new(
            pw_core,
            stream_name.as_ptr(),
            properties! {
                "media.type" => "Video",
                "media.category" => "Capture",
                "media.role" => "Screen"
            }
            .as_ptr(),
        );
        println!("Stream: {0:?}", stream);

        // TODO: listen to the stream events.
    }

    pw_loop.run();

    drop(pw_loop);
    unsafe {
        pipewire::deinit();
    }

    Ok(())
}
