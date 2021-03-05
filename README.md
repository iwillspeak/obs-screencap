# obs-screencap

Experimentation to produce an OBS source which uses the [ScreenCapture portal](https://flatpak.github.io/xdg-desktop-portal/portal-docs.html#gdbus-org.freedesktop.portal.ScreenCast).

## TODO:

 * [x] Open a desktop portal and start a screencast session.
 * [x] Connect to the screencast stream with PipeWire.
 * [ ] Add an OBS module.
 * [ ] CI and build ergonomics.

### Structure

Interacaction with the ScreenCast desktop portal is contained within the
`portal-screencast` crate. This crate provides a simplified API around the
portal through blocking D-Bus requests. It handles negotiating a new screen cast
and returns the metadata required to connect to it with Pipewire.

For interacting with Pipewire all code should live ina `pipewire` module. This
should take a `portal::ScreenCastStream` and allow executing a callback on
each frame.

With these components an OBS plugin should be trivial.