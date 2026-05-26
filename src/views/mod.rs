//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`Home`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod browse;
mod history;
mod library;
mod navbar;
mod settings;
mod updates;

pub use browse::Browse;
pub use history::History;
pub use library::Library;
pub use navbar::Navbar;
pub use settings::Settings;
pub use updates::Updates;
