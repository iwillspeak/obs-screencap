use pipewire::{
    properties,
    stream::{Stream, StreamDirection},
    Context, MainLoop,
};
use portal::ScreenCast;
use std::error::Error;

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
    let pw_loop = MainLoop::new()?;
    let pw_context = Context::new(&pw_loop)?;
    let core = pw_context.connect_fd(screen_cast.pipewire_fd(), None)?;

    use pipewire_sys as pw_sys;

    let mut stream = Stream::new(
        &core,
        "test-screencap",
        properties! {
            "media.type" => "Video",
            "media.category" => "Capture",
            "media.role" => "Screen"
        },
    )?;
    println!("Stream: {0:?}", stream);

    let connected = stream.connect(
        StreamDirection::INPUT,
        Some(screen_cast.streams().next().unwrap().pipewire_node()),
        &mut [],
    )?;
    println!("Stream: {0:?} (connected: {1:?})", stream, connected);

    pw_loop.run();

    drop(pw_loop);
    unsafe {
        pipewire::deinit();
    }

    Ok(())
}
