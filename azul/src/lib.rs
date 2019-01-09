//! Azul is a free, functional, IMGUI-oriented GUI framework for rapid prototyping
//! of desktop applications written in Rust, supported by the Mozilla WebRender rendering
//! engine, using a CSS / DOM model for layout and styling.
//!
//! ## Concepts
//!
//! Azul is largely based on the IMGUI principle, in that you redraw the entire
//! screen every frame. To not make this too performance intensive, Azul provides
//! diffing and caching, as well as efficient callback handling and hit-testing.
//!
//! Managing your code can be done by creating "widgets", i.e. reusable components
//! that can register "default callbacks", for example a checkbox that toggles a
//! certain field if it is checked.
//!
//! Azul also has a standard library of widgets to use, see the [widgets] module.
//! Further, it provides a library for CSS parsing and handling (which takes care
//! of the layouting part) as well as DOM handling.
//!
//! ## Documentation
//!
//! Explaining all concepts and examples is too much material to be included in
//! this API reference. Please refer to the [wiki](https://github.com/maps4print/azul/wiki)
//! or use the links below to learn about how to use Azul.
//!
//! - [Getting Started](https://github.com/maps4print/azul/wiki/Getting-Started)
//! - [A simple counter](https://github.com/maps4print/azul/wiki/A-simple-counter)
//! - [Styling your app with CSS](https://github.com/maps4print/azul/wiki/Styling-your-application-with-CSS)
//! - [SVG drawing](https://github.com/maps4print/azul/wiki/SVG-drawing)
//! - [OpenGL drawing](https://github.com/maps4print/azul/wiki/OpenGL-drawing)
//! - [Timers, daemons, tasks and async IO](https://github.com/maps4print/azul/wiki/Timers,-daemons,-tasks-and-async-IO)
//! - [Two-way data binding](https://github.com/maps4print/azul/wiki/Two-way-data-binding)
//! - [Unit testing](https://github.com/maps4print/azul/wiki/Unit-testing)
//!
//! ## Hello world
//!
//! Note: Can currently not be tested on CI, since it opens a graphical window.
//!
//! ```no_run
//! extern crate azul;
//!
//! use azul::prelude::*;
//!
//! struct MyDataModel { }
//!
//! impl Layout for MyDataModel {
//!     fn layout(&self, _: WindowInfo<Self>) -> Dom<Self> {
//!         Dom::new(NodeType::Div)
//!     }
//! }
//!
//! fn main() {
//!     let app = App::new(MyDataModel { }, AppConfig::default());
//!     let window = Window::new(WindowCreateOptions::default(), css::native()).unwrap();
//!     app.run(window).unwrap();
//! }
//! ```
//!
//! If you run this code, you should get a window like this:
//!
//! ![Opening a blank window](https://raw.githubusercontent.com/maps4print/azul/master/doc/azul_tutorial_empty_window.png)
//!

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/azul_logo_full_min.svg.png",
    html_favicon_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/favicon.ico",
)]

#![deny(unused_must_use)]
#![deny(unreachable_patterns)]
#![deny(missing_copy_implementations)]
#![allow(dead_code)]

#[macro_use(warn, error, lazy_static)]
#[cfg_attr(feature = "svg", macro_use(implement_vertex, uniform))]
pub extern crate azul_dependencies;

pub use azul_dependencies::glium as glium;
pub use azul_dependencies::gleam as gleam;

pub(crate) use azul_dependencies::euclid;
pub(crate) use azul_dependencies::webrender;
pub(crate) use azul_dependencies::rusttype;
pub(crate) use azul_dependencies::app_units;
pub(crate) use azul_dependencies::unicode_normalization;
pub(crate) use azul_dependencies::tinyfiledialogs;
pub(crate) use azul_dependencies::clipboard2;
pub(crate) use azul_dependencies::font_loader;

#[cfg(feature = "logging")]
pub(crate) use azul_dependencies::log;
#[cfg(feature = "svg")]
pub(crate) use azul_dependencies::stb_truetype;
#[cfg(feature = "logging")]
pub(crate) use azul_dependencies::fern;
#[cfg(feature = "logging")]
pub(crate) use azul_dependencies::backtrace;
#[cfg(feature = "image_loading")]
pub(crate) use azul_dependencies::image;
#[cfg(feature = "svg")]
pub(crate) use azul_dependencies::lyon;
#[cfg(feature = "svg_parsing")]
pub(crate) use azul_dependencies::usvg;
#[cfg(feature = "faster-hashing")]
pub(crate) use azul_dependencies::twox_hash;

// #[cfg(not(target_os = "linux"))]
// use azul_dependencies::nfd;

#[cfg(feature = "css-parser")]
extern crate azul_css;
extern crate azul_native_style;
extern crate azul_css_parser;

#[macro_use]
mod macros;

/// Global application state, wrapping resources and app state
pub mod app;
/// Wrapper for the application data & application state
pub mod app_state;
/// Font & image resource handling, lookup and caching
pub mod app_resources;
#[cfg(any(feature = "css-parser", feature = "native-style"))]
pub mod css;
/// Daemon / timer system
pub mod daemon;
/// Handles default callbacks (such as an automatic text field update) via unsafe code
pub mod default_callbacks;
/// Bindings to the native file-chooser, color picker, etc. dialogs
pub mod dialogs;
/// DOM / HTML node handling
pub mod dom;
/// Re-exports of errors
pub mod error;
/// Font handling
pub mod font;
/// Async IO / task system
pub mod task;
/// Module for caching long texts (including their layout / character positions) across multiple frames
pub mod text_cache;
/// Text layout helper functions - useful for text layout outside of standard containers
pub mod text_layout;
/// The layout traits for creating a layout-able application
pub mod traits;
/// Built-in widgets
pub mod widgets;
/// Window handling
pub mod window;
/// Window state handling, event filtering
pub mod window_state;
/// DOM styling module
pub mod style;

/// UI Description & display list handling (webrender)
mod ui_description;
/// Converts the UI description (the styled HTML nodes)
/// to an actual display list (+ layout)
mod display_list;
/// Slab allocator for nodes, based on IDs (replaces kuchiki + markup5ever)
mod id_tree;
/// State handling for user interfaces
mod ui_state;
/// Image handling
mod images;
/// The compositor takes all textures (user-defined + the UI texture(s)) and draws them on
/// top of each other
mod compositor;
/// Default logger, can be turned off with `feature = "logging"`
#[cfg(feature = "logging")]
mod logging;
/// Flexbox-based UI solver
mod ui_solver;

// Faster implementation of a HashMap (optional, disabled by default, turn on with --feature="faster-hashing")

#[cfg(feature = "faster-hashing")]
type FastHashMap<T, U> = ::std::collections::HashMap<T, U, ::std::hash::BuildHasherDefault<::twox_hash::XxHash>>;
#[cfg(feature = "faster-hashing")]
type FastHashSet<T> = ::std::collections::HashSet<T, ::std::hash::BuildHasherDefault<::twox_hash::XxHash>>;
#[cfg(not(feature = "faster-hashing"))]
type FastHashMap<T, U> = ::std::collections::HashMap<T, U>;
#[cfg(not(feature = "faster-hashing"))]
type FastHashSet<T> = ::std::collections::HashSet<T>;

/// Quick exports of common types
pub mod prelude {
    pub use azul_css::ColorU;
    pub use app::{App, AppConfig};
    pub use app_state::AppState;
    pub use dom::DomHash;
    pub use dom::{
        Dom, NodeType, NodeData, Callback, On,
        UpdateScreen, Texture, GlTextureCallback,
        IFrameCallback, TabIndex,
    };
    pub use traits::{Layout, Modify};
    pub use window::{MonitorIter, Window, WindowCreateOptions, WindowId,
                     MouseMode, UpdateBehaviour, UpdateMode, HidpiAdjustedBounds,
                     WindowMonitorTarget, RendererType, WindowEvent, WindowInfo, ReadOnlyWindow};
    pub use window_state::{WindowState, KeyboardState, MouseState, DebugState};
    pub use images::{ImageType, ImageId};
    pub use text_cache::{TextCache, TextId};
    pub use glium::glutin::{
        dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize},
        VirtualKeyCode, ScanCode, Icon,
    };
    pub use azul_css::*;
    pub use rusttype::Font;
    pub use app_resources::AppResources;
    pub use daemon::{TerminateDaemon, DaemonId, DaemonCallback, Daemon};
    pub use default_callbacks::StackCheckedPointer;
    pub use text_layout::TextLayoutOptions;

    #[cfg(any(feature = "css-parser", feature = "native-style"))]
    pub use css;

    #[cfg(feature = "logging")]
    pub use log::LevelFilter;
}

