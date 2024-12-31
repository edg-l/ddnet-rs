#![warn(unused)]

pub mod actionfeed;

use api::GRAPHICS;
use ui_generic::traits::UiPageInterface;

pub use api_ui::ui_impl::*;
pub use api_ui_game::render::*;

#[no_mangle]
fn mod_ui_new() -> Box<dyn UiPageInterface<()>> {
    Box::new(actionfeed::page::ActionfeedPage::new(
        &GRAPHICS.with(|g| (*g).clone()),
    ))
}