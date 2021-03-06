#![doc(
    html_logo_url = "https://raw.githubusercontent.com/nannou-org/nannou/master/assets/images/logo.png"
)]

//! An open-source creative-coding toolkit for Rust.
//!
//! [**Nannou**](http://nannou.cc) is a collection of code aimed at making it easy for artists to
//! express themselves with simple, fast, reliable, portable code. Whether working on a 12-month
//! laser installation or a 5 minute sketch, this framework aims to give artists easy access to the
//! tools they need.
//!
//! If you're new to nannou, we recommend checking out [the
//! examples](https://github.com/nannou-org/nannou/tree/master/examples) to get an idea of how
//! nannou applications are structured and how the API works.

pub use conrod_core;
pub use conrod_vulkano;
pub use conrod_winit;
pub use daggy;
pub use find_folder;
use serde_derive;
pub use vulkano;
pub use vulkano_shaders;
pub use vulkano_win;
pub use winit;

pub use self::event::Event;
pub use self::frame::Frame;
pub use self::ui::Ui;
pub use crate::app::{App, LoopMode};
pub use crate::draw::Draw;

pub mod app;
pub mod audio;
pub mod color;
pub mod draw;
pub mod ease;
pub mod event;
pub mod frame;
pub mod geom;
pub mod image;
pub mod io;
pub mod math;
pub mod mesh;
pub mod noise;
pub mod osc;
pub mod prelude;
pub mod rand;
pub mod state;
pub mod time;
pub mod ui;
pub mod vk;
pub mod window;

/// Begin building the `App`.
///
/// The `model` argument is the function that the App will call to initialise your Model.
///
/// The Model can be thought of as the state that you would like to track throughout the
/// lifetime of your nannou program from start to exit.
///
/// The given function is called before any event processing begins within the application.
///
/// The Model that is returned by the function is the same model that will be passed to the
/// given event and view functions.
pub fn app<M: 'static>(model: app::ModelFn<M>) -> app::Builder<M, Event> {
    app::Builder::new(model)
}

/// Shorthand for building a simple app that has no model, handles no events and simply draws
/// to a single window.
///
/// This is useful for late night hack sessions where you just don't care about all that other
/// stuff, you just want to play around with some ideas or make something pretty.
pub fn sketch(view: app::SketchViewFn) {
    app::Builder::sketch(view)
}
