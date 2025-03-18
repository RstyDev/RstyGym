mod app;
mod commands;
mod libs;
mod menu;
mod today;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
