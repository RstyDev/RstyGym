mod app;
mod libs;
mod menu;
mod today;
mod commands;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
