use dbus::{Message, Path, arg::{RefArg, Variant}, blocking::{Connection, Proxy}};
use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

mod generated;

use generated::{OrgFreedesktopPortalRequestResponse, OrgFreedesktopPortalScreenCast};

struct ConnectionState {
    connection: Connection,
    sender_token: String,
}

impl ConnectionState {

    /// Open a new D-Bus connection to use for all our requests
    pub fn open_new() -> Result<Self, dbus::Error> {
        // Create a new session and work out our session's sender token. Portal
        // requests will send responses to paths based on this token.
        let connection = Connection::new_session()?;
        let sender_token = String::from(&connection.unique_name().replace(".", "_")[1..]);
        println!("Connection::{:?}", sender_token);      
        Ok(ConnectionState {
            connection,
            sender_token,
        })
    }

    /// Create a proxy to the main desktop portal object
    pub fn desktop_proxy(&self) -> Proxy<&Connection> {
        self.connection.with_proxy(
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            Duration::from_secs(20),
        )
    }
}

fn proxied_request<TResponse, RequestHandler, ResponseHandler>(state: &ConnectionState, make_request: RequestHandler, mut on_response: ResponseHandler)
    
-> Result<TResponse, Box<dyn Error>> 
where ResponseHandler: FnMut(OrgFreedesktopPortalRequestResponse) -> TResponse + Send + 'static,
          RequestHandler: FnOnce(&str) -> Result<(), Box<dyn Error>>,
          TResponse: Send + Sync + 'static
{
    // Portal requests return their results via messages to a `Response` object.
    // To protect against race conditions we open a proxy to the expected
    // response object before making the call.
    let (s, r) = std::sync::mpsc::channel();
    let request_id = format!("screencap{0}", rand::random::<usize>());
    let resp_path = Path::new(format!(
        "/org/freedesktop/portal/desktop/request/{0}/{1}",
        state.sender_token, request_id
    ))?;
    println!("@response path: {:?}", resp_path);
    let resp_proxy = state.connection.with_proxy(
        "org.freedesktop.portal.Desktop",
        resp_path,
        Duration::from_secs(20),
    );
    let id = resp_proxy.match_signal(
        move |a: OrgFreedesktopPortalRequestResponse, _: &Connection, _: &Message| {
            let res = on_response(a);
            s.send(res).is_ok()
        },
    )?;

    make_request(&request_id)?;

    // Pump the event loop until we receive our expected result
    loop {
        if let Ok(data) = r.try_recv() {
            resp_proxy.match_stop(id, true)?;
            return Ok(data);
        } else {
            state.connection.process(Duration::from_millis(100))?;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let state = ConnectionState::open_new()?;

    // Create a proxy pointing to the main desktop portal. We can then call
    // ScreenCast interface method on this.
    let desktop_proxy = state.desktop_proxy();

    // Grab the supported cursor and source types. These are packed bitfields
    println!(
        "cursor modes: {:?}",
        desktop_proxy.available_cursor_modes()?
    );
    println!(
        "source types: {:?}",
        desktop_proxy.available_source_types()?
    );

    let session = proxied_request(
        &state,
        |request_id| {
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
            state.desktop_proxy().create_session(session_args)?;
            Ok(())
        },
        |a| {
            println!("GOT: {:?}", a.response);
            println!("GOT: {:?}", a.results);
            a.results
                .get("session_handle")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned()
        })?;

    proxied_request(&state,
        |request_id| {
            let session = dbus::Path::from(&session);
            let mut select_args = HashMap::<String, Variant<Box<dyn RefArg>>>::new();
            select_args.insert("handle_token".into(), Variant(Box::new(String::from(request_id))));
            select_args.insert("types".into(), Variant(Box::new(desktop_proxy.available_source_types()?)));
            desktop_proxy.select_sources(session, select_args)?;
            Ok(())
        }, |_| {
            ()
        })?;

    proxied_request(&state,
        |request_id| {
            let session = dbus::Path::from(&session);
            let mut select_args = HashMap::<String, Variant<Box<dyn RefArg>>>::new();
            select_args.insert("handle_token".into(), Variant(Box::new(String::from(request_id))));
            desktop_proxy.start(session, "", select_args)?;
            Ok(())
        }, |response| {
            println!("GOT screenshare {0:#?}", response);
            ()
        })?;

    Ok(())
}
