//! SigmaOS-cinnamon-menus — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_cinnamon_menus_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-cinnamon-menus: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-cinnamon-menus: ok");
}
