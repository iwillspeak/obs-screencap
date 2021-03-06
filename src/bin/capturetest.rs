use obs_portal_screencap::native_shims;
use pipewire::{
    properties,
    spa::Direction,
    stream::{Stream, StreamFlags},
    Context, MainLoop,
};
use portal_screencast::ScreenCast;
use std::{cell::RefCell, error::Error, mem, rc::Rc};

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

    let _listener = core
        .add_listener_local()
        .info(|i| println!("INFO: {0:#?}", i))
        .error(|e, f, g, h| println!("ERR: {0},{1},{2},{3}", e, f, g, h))
        .done(|d, e| println!("DONE: {0},{1}", d, e))
        .register();

    use pipewire_sys as pw_sys;

    let stream = Rc::new(RefCell::new(Stream::new(
        &core,
        "test-screencap",
        properties! {
            "media.type" => "Video",
            "media.category" => "Capture",
            "media.role" => "Screen"
        },
    )?));
    println!("Stream: {0:?}", stream);

    let param_changed_stream = stream.clone();
    let process_stream = stream.clone();

    let _stream_listener = stream
        .borrow_mut()
        .add_local_listener()
        .io_changed(|x, y, z| {
            println!("IO change: , {0:?}, {1:?}, {2:?}", x, y, z);
        })
        .state_changed(|old, new| println!("State: {0:?} -> {1:?}", old, new))
        .param_changed(move |id, param| {
            if param.is_null() || id != libspa_sys::spa_param_type_SPA_PARAM_Format {
                return;
            }

            unsafe {
                let mut media_type = mem::zeroed();
                let mut media_subtype = mem::zeroed();

                native_shims::spa_format_parse_rs(param, &mut media_type, &mut media_subtype);

                let mut vidya_format = mem::zeroed();
                native_shims::spa_format_video_raw_parse_rs(param, &mut vidya_format);

                println!("Media: {}({})", media_type, media_subtype);
                println!("Format: {:#?}", vidya_format);
            }

            let param = unsafe { native_shims::build_stream_param() };
            param_changed_stream
                .borrow_mut()
                .update_params(&mut [param as _])
                .unwrap()
        })
        .process(move || {
            let mut stream = process_stream.borrow_mut();
            let (buff, size, spa_buff) = unsafe {
                let buff = stream.dequeue_buffer();
                let size = (*buff).size;
                let spa_buff = *(*buff).buffer;
                (buff, size, spa_buff)
            };
            println!(
                "got buffer: {0:?} (size={1}) spa={2:#?}",
                buff, size, &spa_buff
            );
            unsafe {
                stream.queue_buffer(buff);
            }
        })
        .register()?;

    let param = unsafe { native_shims::build_video_params() };
    stream.borrow_mut().connect(
        Direction::Input,
        Some(screen_cast.streams().next().unwrap().pipewire_node()),
        StreamFlags::AUTOCONNECT | StreamFlags::MAP_BUFFERS,
        &mut [param as *const _],
    )?;
    println!("Stream: {0:?}", stream);

    pw_loop.run();

    println!("DONE");

    drop(pw_loop);

    unsafe {
        pipewire::deinit();
    }

    Ok(())
}
