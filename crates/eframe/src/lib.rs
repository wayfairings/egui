//! eframe - the [`egui`] framework crate
//!
//! If you are planning to write an app for web or native,
//! and want to use [`egui`] for everything, then `eframe` is for you!
//!
//! To get started, see the [examples](https://github.com/emilk/egui/tree/main/examples).
//! To learn how to set up `eframe` for web and native, go to <https://github.com/emilk/eframe_template/> and follow the instructions there!
//!
//! In short, you implement [`App`] (especially [`App::update`]) and then
//! call [`crate::run_native`] from your `main.rs`, and/or use `eframe::WebRunner` from your `lib.rs`.
//!
//! ## Compiling for web
//! You need to install the `wasm32` target with `rustup target add wasm32-unknown-unknown`.
//!
//! Build the `.wasm` using `cargo build --target wasm32-unknown-unknown`
//! and then use [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) to generate the JavaScript glue code.
//!
//! See the [`eframe_template` repository](https://github.com/emilk/eframe_template/) for more.
//!
//! ## Simplified usage
//! If your app is only for native, and you don't need advanced features like state persistence,
//! then you can use the simpler function [`run_simple_native`].
//!
//! ## Usage, native:
//! ``` no_run
//! use eframe::egui;
//!
//! fn main() {
//!     let native_options = eframe::NativeOptions::default();
//!     eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
//! }
//!
//! #[derive(Default)]
//! struct MyEguiApp {}
//!
//! impl MyEguiApp {
//!     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//!         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
//!         // Restore app state using cc.storage (requires the "persistence" feature).
//!         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
//!         // for e.g. egui::PaintCallback.
//!         Self::default()
//!     }
//! }
//!
//! impl eframe::App for MyEguiApp {
//!    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//!        egui::CentralPanel::default().show(ctx, |ui| {
//!            ui.heading("Hello World!");
//!        });
//!    }
//! }
//! ```
//!
//! ## Usage, web:
//! ``` no_run
//! # #[cfg(target_arch = "wasm32")]
//! use wasm_bindgen::prelude::*;
//!
//! /// Your handle to the web app from JavaScript.
//! # #[cfg(target_arch = "wasm32")]
//! #[derive(Clone)]
//! #[wasm_bindgen]
//! pub struct WebHandle {
//!     runner: eframe::WebRunner,
//! }
//!
//! # #[cfg(target_arch = "wasm32")]
//! #[wasm_bindgen]
//! impl WebHandle {
//!     /// Installs a panic hook, then returns.
//!     #[expect(clippy::new_without_default)]
//!     #[wasm_bindgen(constructor)]
//!     pub fn new() -> Self {
//!         // Redirect [`log`] message to `console.log` and friends:
//!         eframe::WebLogger::init(log::LevelFilter::Debug).ok();
//!
//!         Self {
//!             runner: eframe::WebRunner::new(),
//!         }
//!     }
//!
//!     /// Call this once from JavaScript to start your app.
//!     #[wasm_bindgen]
//!     pub async fn start(&self, canvas: web_sys::HtmlCanvasElement) -> Result<(), wasm_bindgen::JsValue> {
//!         self.runner
//!             .start(
//!                 canvas,
//!                 eframe::WebOptions::default(),
//!                 Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))),)
//!             )
//!             .await
//!     }
//!
//!     // The following are optional:
//!
//!     /// Shut down eframe and clean up resources.
//!     #[wasm_bindgen]
//!     pub fn destroy(&self) {
//!         self.runner.destroy();
//!     }
//!
//!     /// Example on how to call into your app from JavaScript.
//!     #[wasm_bindgen]
//!     pub fn example(&self) {
//!         if let Some(app) = self.runner.app_mut::<MyEguiApp>() {
//!             app.example();
//!         }
//!     }
//!
//!     /// The JavaScript can check whether or not your app has crashed:
//!     #[wasm_bindgen]
//!     pub fn has_panicked(&self) -> bool {
//!         self.runner.has_panicked()
//!     }
//!
//!     #[wasm_bindgen]
//!     pub fn panic_message(&self) -> Option<String> {
//!         self.runner.panic_summary().map(|s| s.message())
//!     }
//!
//!     #[wasm_bindgen]
//!     pub fn panic_callstack(&self) -> Option<String> {
//!         self.runner.panic_summary().map(|s| s.callstack())
//!     }
//! }
//! ```
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!
//! ## Instrumentation
//! This crate supports using the [profiling](https://crates.io/crates/profiling) crate for instrumentation.
//! You can enable features on the profiling crates in your application to add instrumentation for all
//! crates that support it, including egui. See the profiling crate docs for more information.
//! ```toml
//! [dependencies]
//! profiling = "1.0"
//! [features]
//! profile-with-puffin = ["profiling/profile-with-puffin"]
//! ```
//!

#![warn(missing_docs)] // let's keep eframe well-documented
#![allow(clippy::needless_doctest_main)]

// Limitation imposed by `accesskit_winit`:
// https://github.com/AccessKit/accesskit/tree/accesskit-v0.18.0/platforms/winit#android-activity-compatibility`
#[cfg(all(
    target_os = "android",
    feature = "accesskit",
    feature = "android-native-activity"
))]
compile_error!("`accesskit` feature is only available with `android-game-activity`");

// Re-export all useful libraries:
pub use {egui, egui::emath, egui::epaint};

#[cfg(feature = "glow")]
pub use {egui_glow, glow};

#[cfg(feature = "wgpu")]
pub use {egui_wgpu, wgpu};

mod epi;

// Re-export everything in `epi` so `eframe` users don't have to care about what `epi` is:
pub use epi::*;

pub(crate) mod stopwatch;

// ----------------------------------------------------------------------------
// When compiling for web

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen;

#[cfg(target_arch = "wasm32")]
pub use web_sys;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(target_arch = "wasm32")]
pub use web::{WebLogger, WebRunner};

// ----------------------------------------------------------------------------
// When compiling natively

#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(feature = "glow", feature = "wgpu"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(feature = "glow", feature = "wgpu"))]
pub use native::run::EframeWinitApplication;

#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[cfg(any(feature = "glow", feature = "wgpu"))]
pub use native::run::EframePumpStatus;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(feature = "glow", feature = "wgpu"))]
#[cfg(feature = "persistence")]
pub use native::file_storage::storage_dir;

#[cfg(not(target_arch = "wasm32"))]
pub mod icon_data;

/// This is how you start a native (desktop) app.
///
/// The first argument is name of your app, which is an identifier
/// used for the save location of persistence (see [`App::save`]).
/// It is also used as the application id on wayland.
/// If you set no title on the viewport, the app id will be used
/// as the title.
///
/// For details about application ID conventions, see the
/// [Desktop Entry Spec](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#desktop-file-id)
///
/// Call from `fn main` like this:
/// ``` no_run
/// use eframe::egui;
///
/// fn main() -> eframe::Result {
///     let native_options = eframe::NativeOptions::default();
///     eframe::run_native("MyApp", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))))
/// }
///
/// #[derive(Default)]
/// struct MyEguiApp {}
///
/// impl MyEguiApp {
///     fn new(cc: &eframe::CreationContext<'_>) -> Self {
///         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
///         // Restore app state using cc.storage (requires the "persistence" feature).
///         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
///         // for e.g. egui::PaintCallback.
///         Self::default()
///     }
/// }
///
/// impl eframe::App for MyEguiApp {
///    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
///        egui::CentralPanel::default().show(ctx, |ui| {
///            ui.heading("Hello World!");
///        });
///    }
/// }
/// ```
///
/// # Errors
/// This function can fail if we fail to set up a graphics context.
#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(feature = "glow", feature = "wgpu"))]
#[allow(clippy::needless_pass_by_value, clippy::allow_attributes)]
pub fn run_native(
    app_name: &str,
    mut native_options: NativeOptions,
    app_creator: AppCreator<'_>,
) -> Result {
    let renderer = init_native(app_name, &mut native_options);

    match renderer {
        #[cfg(feature = "glow")]
        Renderer::Glow => {
            log::debug!("Using the glow renderer");
            native::run::run_glow(app_name, native_options, app_creator)
        }

        #[cfg(feature = "wgpu")]
        Renderer::Wgpu => {
            log::debug!("Using the wgpu renderer");
            native::run::run_wgpu(app_name, native_options, app_creator)
        }
    }
}

/// Provides a proxy for your native eframe application to run on your own event loop.
///
/// See `run_native` for details about `app_name`.
///
/// Call from `fn main` like this:
/// ``` no_run
/// use eframe::{egui, UserEvent};
/// use winit::event_loop::{ControlFlow, EventLoop};
///
/// fn main() -> eframe::Result {
///     let native_options = eframe::NativeOptions::default();
///     let eventloop = EventLoop::<UserEvent>::with_user_event().build()?;
///     eventloop.set_control_flow(ControlFlow::Poll);
///
///     let mut winit_app = eframe::create_native(
///         "MyExtApp",
///         native_options,
///         Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
///         &eventloop,
///     );
///
///     eventloop.run_app(&mut winit_app)?;
///
///     Ok(())
/// }
///
/// #[derive(Default)]
/// struct MyEguiApp {}
///
/// impl MyEguiApp {
///     fn new(cc: &eframe::CreationContext<'_>) -> Self {
///         Self::default()
///     }
/// }
///
/// impl eframe::App for MyEguiApp {
///    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
///        egui::CentralPanel::default().show(ctx, |ui| {
///            ui.heading("Hello World!");
///        });
///    }
/// }
/// ```
///
/// See the `external_eventloop` example for a more complete example.
#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(feature = "glow", feature = "wgpu"))]
pub fn create_native<'a>(
    app_name: &str,
    mut native_options: NativeOptions,
    app_creator: AppCreator<'a>,
    event_loop: &winit::event_loop::EventLoop,
) -> EframeWinitApplication<'a> {
    let renderer = init_native(app_name, &mut native_options);

    match renderer {
        #[cfg(feature = "glow")]
        Renderer::Glow => {
            log::debug!("Using the glow renderer");
            EframeWinitApplication::new(native::run::create_glow(
                app_name,
                native_options,
                app_creator,
                event_loop,
            ))
        }

        #[cfg(feature = "wgpu")]
        Renderer::Wgpu => {
            log::debug!("Using the wgpu renderer");
            EframeWinitApplication::new(native::run::create_wgpu(
                app_name,
                native_options,
                app_creator,
                event_loop,
            ))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(feature = "glow", feature = "wgpu"))]
fn init_native(app_name: &str, native_options: &mut NativeOptions) -> Renderer {
    #[cfg(not(feature = "__screenshot"))]
    assert!(
        std::env::var("EFRAME_SCREENSHOT_TO").is_err(),
        "EFRAME_SCREENSHOT_TO found without compiling with the '__screenshot' feature"
    );

    if native_options.viewport.title.is_none() {
        native_options.viewport.title = Some(app_name.to_owned());
    }

    let renderer = native_options.renderer;

    #[cfg(all(feature = "glow", feature = "wgpu"))]
    {
        match native_options.renderer {
            Renderer::Glow => "glow",
            Renderer::Wgpu => "wgpu",
        };
        log::info!("Both the glow and wgpu renderers are available. Using {renderer}.");
    }

    renderer
}

// ----------------------------------------------------------------------------

/// The simplest way to get started when writing a native app.
///
/// This does NOT support persistence of custom user data. For that you need to use [`run_native`].
/// However, it DOES support persistence of egui data (window positions and sizes, how far the user has scrolled in a
/// [`ScrollArea`](egui::ScrollArea), etc.) if the persistence feature is enabled.
///
/// # Example
/// ``` no_run
/// fn main() -> eframe::Result {
///     // Our application state:
///     let mut name = "Arthur".to_owned();
///     let mut age = 42;
///
///     let options = eframe::NativeOptions::default();
///     eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
///         egui::CentralPanel::default().show(ctx, |ui| {
///             ui.heading("My egui Application");
///             ui.horizontal(|ui| {
///                 let name_label = ui.label("Your name: ");
///                 ui.text_edit_singleline(&mut name)
///                     .labelled_by(name_label.id);
///             });
///             ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
///             if ui.button("Increment").clicked() {
///                 age += 1;
///             }
///             ui.label(format!("Hello '{name}', age {age}"));
///         });
///     })
/// }
/// ```
///
/// # Errors
/// This function can fail if we fail to set up a graphics context.
#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(feature = "glow", feature = "wgpu"))]
pub fn run_simple_native(
    app_name: &str,
    native_options: NativeOptions,
    update_fun: impl FnMut(&egui::Context, &mut Frame) + 'static,
) -> Result {
    struct SimpleApp<U> {
        update_fun: U,
    }

    impl<U: FnMut(&egui::Context, &mut Frame) + 'static> App for SimpleApp<U> {
        fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
            (self.update_fun)(ctx, frame);
        }
    }

    run_native(
        app_name,
        native_options,
        Box::new(|_cc| Ok(Box::new(SimpleApp { update_fun }))),
    )
}

// ----------------------------------------------------------------------------

/// The different problems that can occur when trying to run `eframe`.
#[derive(Debug)]
pub enum Error {
    /// Something went wrong in user code when creating the app.
    AppCreation(Box<dyn std::error::Error + Send + Sync>),

    /// An error from [`winit`].
    #[cfg(not(target_arch = "wasm32"))]
    Winit(winit::error::RequestError),

    /// An error from [`winit::event_loop::EventLoop`].
    #[cfg(not(target_arch = "wasm32"))]
    WinitEventLoop(winit::error::EventLoopError),

    /// An error from [`glutin`] when using [`glow`].
    #[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
    Glutin(glutin::error::Error),

    /// An error from [`glutin`] when using [`glow`].
    #[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
    NoGlutinConfigs(glutin::config::ConfigTemplate, Box<dyn std::error::Error>),

    /// An error from [`glutin`] when using [`glow`].
    #[cfg(feature = "glow")]
    OpenGL(egui_glow::PainterError),

    /// An error from [`wgpu`].
    #[cfg(feature = "wgpu")]
    Wgpu(egui_wgpu::WgpuError),
}

impl std::error::Error for Error {}

#[cfg(not(target_arch = "wasm32"))]
impl From<winit::error::RequestError> for Error {
    #[inline]
    fn from(err: winit::error::RequestError) -> Self {
        Self::Winit(err)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<winit::error::EventLoopError> for Error {
    #[inline]
    fn from(err: winit::error::EventLoopError) -> Self {
        Self::WinitEventLoop(err)
    }
}

#[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
impl From<glutin::error::Error> for Error {
    #[inline]
    fn from(err: glutin::error::Error) -> Self {
        Self::Glutin(err)
    }
}

#[cfg(feature = "glow")]
impl From<egui_glow::PainterError> for Error {
    #[inline]
    fn from(err: egui_glow::PainterError) -> Self {
        Self::OpenGL(err)
    }
}

#[cfg(feature = "wgpu")]
impl From<egui_wgpu::WgpuError> for Error {
    #[inline]
    fn from(err: egui_wgpu::WgpuError) -> Self {
        Self::Wgpu(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AppCreation(err) => write!(f, "app creation error: {err}"),

            #[cfg(not(target_arch = "wasm32"))]
            Self::Winit(err) => {
                write!(f, "winit error: {err}")
            }

            #[cfg(not(target_arch = "wasm32"))]
            Self::WinitEventLoop(err) => {
                write!(f, "winit EventLoopError: {err}")
            }

            #[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
            Self::Glutin(err) => {
                write!(f, "glutin error: {err}")
            }

            #[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
            Self::NoGlutinConfigs(template, err) => {
                write!(
                    f,
                    "Found no glutin configs matching the template: {template:?}. Error: {err}"
                )
            }

            #[cfg(feature = "glow")]
            Self::OpenGL(err) => {
                write!(f, "egui_glow: {err}")
            }

            #[cfg(feature = "wgpu")]
            Self::Wgpu(err) => {
                write!(f, "WGPU error: {err}")
            }
        }
    }
}

/// Short for `Result<T, eframe::Error>`.
pub type Result<T = (), E = Error> = std::result::Result<T, E>;
