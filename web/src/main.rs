//! HemiFit — site web en Yew, compilé en WebAssembly.
//!
//! Rééducation douce d'une hémiparésie droite spastique, en fauteuil
//! roulant : tout se fait assis ou allongé, lentement, la main gauche
//! assistant la droite. Les données restent dans le navigateur.

// Hors navigateur, seuls les tests utilisent le code.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

mod dates;
mod exercices;
mod icones;
mod progression;

#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod composants;
#[cfg(target_arch = "wasm32")]
mod lecteur;
#[cfg(target_arch = "wasm32")]
mod mouvement;
#[cfg(target_arch = "wasm32")]
mod pages;

#[cfg(target_arch = "wasm32")]
fn main() {
    let racine = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("app"))
        .expect("élément #app dans index.html");
    yew::Renderer::<app::App>::with_root(racine).render();
}

/// Hors navigateur, le binaire ne sert qu'aux tests (`cargo test`).
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
