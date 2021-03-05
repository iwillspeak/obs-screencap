# obs-screencap

Experimentation to produce an OBS source which uses the [ScreenCapture portal](https://flatpak.github.io/xdg-desktop-portal/portal-docs.html#gdbus-org.freedesktop.portal.ScreenCast).

## TODO:

 * [x] Open a desktop portal and start a screencast session.
 * [x] Connect to the screencast stream with PipeWire.
 * [ ] Add an OBS module.
 * [ ] CI and build ergonomics.

### Structure

Desktop portal interaction code should be in a `portal` module. This should provide an interface such as:

```rust
impl ScreenCast {
    pub fn new() -> Result<Self,PortalError>
    pub fn source_types(&self) -> SourceType // A bitfield enum of the types
    pub fn set_source_types(&self, types: SourceType)
    pub fn start(self) -> Result<OpenScreenCast,PortalError>
}

impl OpenScreenCast {
    fn pipewire_fd(&self) -> RawFd
    fn streams(&self) -> Iter<Item=ScreenCastStream>>
    fn close(self) -> Result<(),PortalError>
}

impl ScreenCastStream {
    pub fn pipewire_node(&self) -> u64;
    pub fn stream_type(&self) -> SourceType;
    pub fn size(&self) -> (u64, u64);
    pub fn position(&self) -> (u64, u64);
}
```

This will be backed by some private structs: `ConnectionState` to manage our D-Bus connection; `Request`, and `Session` to handle interacting with request and session proxies.

For interacting with Pipewire all code should live ina `pipewire` module. This
should take a `portal::ScreenCastStream` and allow executing a callback on
each frame.

With these components an OBS plugin should be trivial.