//! Glue code for working with raw SPA_POD data. These functions build and parse
//! the SPA_POD structures for us because doing so from Rust is akward.

extern "C" {
    /// Build the video parameters strucure
    ///
    /// This POD should be an object defining our supported video formats. It
    /// is used when connecting to a pipewire node to begin the negotiations.
    pub fn build_video_params() -> *const core::ffi::c_void;

    /// Build the stream parameters
    ///
    /// Called when we are finishing the format negotiation. This produces the
    /// stream parameters we need to set to complete negotiation.
    pub fn build_stream_param() -> *const core::ffi::c_void;
}