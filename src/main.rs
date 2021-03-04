use pipewire::{
    properties,
    spa::Direction,
    stream::{Stream, StreamFlags},
    Context, MainLoop,
};
use portal_screencast::ScreenCast;
use std::{error::Error, io::Write};

mod haxx {
    //! HAXX: These functions build the SPA_POD structures for us because doing so
    //!       from Rust is akward.

    extern "C" {
        pub fn build_video_params() -> *const core::ffi::c_void;
    }
}

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
    let fd = unsafe { libc::fcntl(screen_cast.pipewire_fd(), libc::F_DUPFD_CLOEXEC, 3) };
    let core = pw_context.connect_fd(fd, None)?;

    let _listener = core
        .add_listener_local()
        .info(|i| println!("INFO: {0:#?}", i))
        .error(|e, f, g, h| println!("ERR: {0},{1},{2},{3}", e, f, g, h))
        .done(|d, e| println!("DONE: {0},{1}", d, e))
        .register();

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

    let _stream_listener = stream
        .add_local_listener()
        .state_changed(|old, new| println!("State: {0:?} -> {1:?}", old, new))
        .param_changed(|x, y| {
            println!("Param: {0:?} {1:?}", x, y);
        })
        .process(|| {
            println!("On process");
            let _ = std::io::stdout().lock().flush();
        })
        .register()?;

    let param = unsafe { haxx::build_video_params() };
    let connected = stream.connect(
        Direction::Input,
        Some(screen_cast.streams().next().unwrap().pipewire_node()),
        StreamFlags::AUTOCONNECT | StreamFlags::MAP_BUFFERS,
        &mut [param as *const _],
    )?;
    println!("Stream: {0:?} (connected: {1:?})", stream, connected);

    pw_loop.run();

    println!("DONE");

    drop(pw_loop);
    unsafe {
        pipewire::deinit();
    }

    Ok(())
}
