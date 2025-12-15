pub mod entities;
pub mod utils;

#[cfg(feature = "ssr")]
pub mod backend;
#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub mod frontend;

#[macro_use]
pub mod macros;


#[cfg(feature = "ssr")]
use crate::backend::infrastructure::web::run;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
use frontend::app::App;

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}


#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {}
