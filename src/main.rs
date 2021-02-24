use dbus::{
    arg::{RefArg, Variant},
    blocking::Connection,
    Message, Path,
};
use std::{collections::HashMap, error::Error, time::Duration};

mod generated;

use generated::{OrgFreedesktopPortalRequestResponse, OrgFreedesktopPortalScreenCast};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new session and work out our session's sender token. Portal
    // requests will send responses to paths based on this token.
    let con = Connection::new_session()?;
    let sender_token = String::from(&con.unique_name().replace(".", "_")[1..]);
    println!("Connection::{:?}", sender_token);

    // Create a proxy pointing to the main desktop portal. We can then call
    // ScreenCast interface method on this.
    let desktop_proxy = con.with_proxy(
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        Duration::from_secs(20),
    );

    // Grab the supported cursor and source types. These are packed bitfields
    println!(
        "cursor modes: {:?}",
        desktop_proxy.available_cursor_modes()?
    );
    println!(
        "source types: {:?}",
        desktop_proxy.available_source_types()?
    );

    // Portal requests return their results via messages to a `Response` object.
    // To protect against race conditions we open a proxy to the expected
    // response object before making the call.
    let (s, r) = std::sync::mpsc::channel();
    let request_id = "test1";
    let resp_path = Path::new(format!(
        "/org/freedesktop/portal/desktop/request/{0}/{1}",
        sender_token, request_id
    ))?;
    println!("@response path: {:?}", resp_path);
    let resp_proxy = con.with_proxy(
        "org.freedesktop.portal.Desktop",
        resp_path,
        Duration::from_secs(20),
    );
    let id = resp_proxy.match_signal(
        move |a: OrgFreedesktopPortalRequestResponse, _: &Connection, _: &Message| {
            println!("GOT: {:?}", a.response);
            println!("GOT: {:?}", a.results);
            s.send(
                a.results
                    .get("session_handle")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )
            .is_ok()
        },
    )?;

    // Make the initail call to open the session.
    let mut session_args = HashMap::<String, Variant<Box<dyn RefArg>>>::new();
    session_args.insert(
        "handle_token".into(),
        Variant(Box::new(String::from(request_id))),
    );
    session_args.insert(
        "session_handle_token".into(),
        Variant(Box::new(String::from(request_id))),
    );
    let session = desktop_proxy.create_session(session_args)?;
    println!("session request: {:?}", session);

    // Pump the event loop until we receive our expected result
    loop {
        if let Ok(data) = r.try_recv() {
            println!("Recieved: {:?}", data);
            break;
        } else {
            con.process(Duration::from_millis(100))?;
        }
    }
    resp_proxy.match_stop(id, true)?;

    // let select_args = HashMap::<String, Variant<Box<dyn RefArg>>>::new();
    // let sources = proxy.select_sources(session, select_args);
    // println!("Sources: {:?}", sources);
    Ok(())
}