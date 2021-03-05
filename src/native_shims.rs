//! Glue code for working with raw SPA_POD data. These functions build and parse
//! the SPA_POD structures for us because doing so from Rust is akward.

use std::os::raw;

extern "C" {
    /// Build the video parameters strucure
    ///
    /// This POD should be an object defining our supported video formats. It
    /// is used when connecting to a pipewire node to begin the negotiations.
    pub fn build_video_params() -> *const ::libspa_sys::spa_pod;

    /// Build the stream parameters
    ///
    /// Called when we are finishing the format negotiation. This produces the
    /// stream parameters we need to set to complete negotiation.
    pub fn build_stream_param() -> *const ::libspa_sys::spa_pod;

    /// Shim to parse a format from an SPA POD.
    pub fn spa_format_parse_rs(
        format: *const ::libspa_sys::spa_pod,
        media_type: *mut u32,
        media_subtype: *mut u32,
    ) -> raw::c_int;

    pub fn spa_format_video_raw_parse_rs(
        format: *const ::libspa_sys::spa_pod,
        info: *mut ::libspa_sys::spa_video_info_raw,
    ) -> raw::c_int;
}
