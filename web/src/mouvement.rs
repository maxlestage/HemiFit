//! Mouvement : les animations du site, dans l'esprit de zamocorp.com.
//!
//! - un rideau d'ouverture, une fois par session : l'anneau de la marque se
//!   trace, un compteur monte jusqu'à 100, puis le rideau se lève ;
//! - les titres sortent d'une fente, mot par mot, quand le rideau se lève ;
//! - les cartes montent en cascade quand elles entrent à l'écran, et les
//!   filets des sections se déroulent ;
//! - à la souris seulement : boutons magnétiques et anneau qui suit le
//!   pointeur.
//!
//! Garde-fous :
//! - avec « réduire les animations », rien ne bouge et rien n'est masqué ;
//! - ce module ne crée, ne déplace ni ne supprime aucun nœud géré par Yew :
//!   il pose seulement des classes sur `<html>` et des attributs `data-*` ;
//! - seul le contenu encore sous la ligne de flottaison attend d'être
//!   révélé : rien ne reste caché après un défilement complet.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gloo_events::EventListener;
use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    Document, Element, HtmlElement, IntersectionObserver, IntersectionObserverEntry,
    IntersectionObserverInit, MutationObserver, MutationObserverInit, MutationRecord, PointerEvent,
    Window,
};
use yew::prelude::*;

/// Événement émis sur `document` quand le rideau se lève.
const EVENEMENT_SCENE: &str = "hemifit-scene";
/// Le rideau ne joue son ouverture complète qu'une fois par session.
const CLE_RIDEAU: &str = "hemifit.rideau";
/// Durée minimale du rideau à la première ouverture : le temps que
/// l'anneau se trace et que le compteur atteigne 100.
const DUREE_RIDEAU_MS: f64 = 1900.0;
/// Durée de la levée du rideau (voir `#rideau` dans styles.css).
const LEVEE_RIDEAU_MS: u32 = 1100;
/// Éléments révélés au défilement (voir `[data-revele]` dans styles.css).
const SELECTEUR_REVELE: &str = "[data-revele]:not([data-vu])";
/// Nombre de crans de décalage dans une même vague de révélation.
const CRANS_MAX: u32 = 7;

thread_local! {
    static SCENE_PRETE: Cell<bool> = const { Cell::new(false) };
}

fn fenetre() -> Window {
    web_sys::window().expect("fenêtre du navigateur")
}

fn document() -> Document {
    fenetre().document().expect("document")
}

fn racine() -> Element {
    document().document_element().expect("élément <html>")
}

fn media(requete: &str) -> bool {
    fenetre()
        .match_media(requete)
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

/// Faux si la personne a demandé de réduire les animations.
pub fn mouvement_permis() -> bool {
    !media("(prefers-reduced-motion: reduce)")
}

/// Vrai une fois le rideau levé : les entrées peuvent se jouer.
pub fn scene_prete() -> bool {
    SCENE_PRETE.get()
}

/// Remonte en haut de la page, sans animation : on change d'écran.
pub fn haut_de_page() {
    fenetre().scroll_to_with_x_and_y(0.0, 0.0);
}

/// Remonte en haut de la séance en cours, à chaque nouvel exercice.
pub fn haut_de_page_seance() {
    if let Some(seance) = document().get_element_by_id("seance") {
        seance.set_scroll_top(0);
    }
}

/// Lance les animations ; à appeler une fois, après le premier rendu.
pub fn demarrer() {
    let permis = mouvement_permis();
    if permis {
        let _ = racine().class_list().add_1("mouvement");
        observer_revelations();
        if media("(hover: hover) and (pointer: fine)") {
            magnetisme();
            curseur();
        }
    }
    preparer_rideau(permis);
}

/* ————————————————— Rideau ————————————————— */

fn preparer_rideau(permis: bool) {
    let Some(rideau) = document().get_element_by_id("rideau") else {
        lever_rideau();
        return;
    };
    let stockage = fenetre().session_storage().ok().flatten();
    let deja_vu = stockage
        .as_ref()
        .and_then(|s| s.get_item(CLE_RIDEAU).ok().flatten())
        .is_some();
    if let Some(s) = &stockage {
        let _ = s.set_item(CLE_RIDEAU, "1");
    }

    // Le rideau a été affiché dès le premier rendu de la page, pendant le
    // chargement du WebAssembly : on ne le prolonge que pour finir
    // l'ouverture, et seulement la première fois.
    let attente = if permis && !deja_vu {
        let ecoule = fenetre().performance().map_or(0.0, |p| p.now());
        (DUREE_RIDEAU_MS - ecoule).max(0.0)
    } else {
        0.0
    };
    if deja_vu {
        let _ = racine().class_list().add_1("rideau-bref");
    }

    // Toucher le rideau le lève aussitôt.
    EventListener::once(&rideau, "pointerdown", |_| lever_rideau()).forget();
    Timeout::new(attente as u32, lever_rideau).forget();
}

fn lever_rideau() {
    if SCENE_PRETE.replace(true) {
        return;
    }
    let _ = racine().class_list().add_2("rideau-leve", "scene-prete");
    if let Ok(evenement) = web_sys::CustomEvent::new(EVENEMENT_SCENE) {
        let _ = document().dispatch_event(&evenement);
    }
    let delai = if mouvement_permis() {
        LEVEE_RIDEAU_MS
    } else {
        0
    };
    Timeout::new(delai, || {
        if let Some(rideau) = document().get_element_by_id("rideau") {
            rideau.remove();
        }
    })
    .forget();
}

/// Vrai quand le rideau est levé ; provoque un nouveau rendu à ce moment-là.
#[hook]
pub fn use_scene_prete() -> bool {
    let prete = use_state(scene_prete);
    {
        let prete = prete.clone();
        use_effect_with((), move |_| {
            let ecoute = if scene_prete() {
                prete.set(true);
                None
            } else {
                Some(EventListener::once(
                    &document(),
                    EVENEMENT_SCENE,
                    move |_| prete.set(true),
                ))
            };
            move || drop(ecoute)
        });
    }
    *prete
}

/* ————————————————— Révélations au défilement ————————————————— */

fn observer_revelations() {
    let rappel = Closure::<dyn FnMut(js_sys::Array, IntersectionObserver)>::new(
        |entrees: js_sys::Array, observateur: IntersectionObserver| {
            let mut cran = 0;
            for entree in entrees.iter() {
                let entree: IntersectionObserverEntry = entree.unchecked_into();
                if !entree.is_intersecting() {
                    continue;
                }
                let element = entree.target();
                // Les éléments d'une même vague arrivent l'un après l'autre.
                let _ = element.set_attribute("data-delai", &cran.min(CRANS_MAX).to_string());
                let _ = element.set_attribute("data-vu", "");
                observateur.unobserve(&element);
                cran += 1;
            }
        },
    );
    let options = IntersectionObserverInit::new();
    // On révèle un peu avant le bas de l'écran, sans attendre qu'une
    // fraction de l'élément soit visible : les blocs très hauts aussi.
    options.set_root_margin("0px 0px -6% 0px");
    options.set_threshold(&JsValue::from_f64(0.0));
    let Ok(observateur) =
        IntersectionObserver::new_with_options(rappel.as_ref().unchecked_ref(), &options)
    else {
        // Navigateur trop ancien : tout reste visible.
        let _ = racine().class_list().remove_1("mouvement");
        return;
    };
    rappel.forget();

    observer_dans(&document().into(), &observateur);

    // Les écrans sont rendus par Yew au fil de la navigation : chaque
    // nouvel élément à révéler est pris en charge dès son insertion.
    let observateur_insertions = observateur.clone();
    let rappel_insertions = Closure::<dyn FnMut(js_sys::Array, MutationObserver)>::new(
        move |mutations: js_sys::Array, _: MutationObserver| {
            for mutation in mutations.iter() {
                let mutation: MutationRecord = mutation.unchecked_into();
                let ajouts = mutation.added_nodes();
                for i in 0..ajouts.length() {
                    if let Some(noeud) = ajouts.item(i) {
                        observer_dans(&noeud, &observateur_insertions);
                    }
                }
            }
        },
    );
    if let (Ok(insertions), Some(corps)) = (
        MutationObserver::new(rappel_insertions.as_ref().unchecked_ref()),
        document().body(),
    ) {
        let options = MutationObserverInit::new();
        options.set_child_list(true);
        options.set_subtree(true);
        let _ = insertions.observe_with_options(&corps, &options);
    }
    rappel_insertions.forget();
}

fn observer_dans(noeud: &web_sys::Node, observateur: &IntersectionObserver) {
    let Some(element) = noeud.dyn_ref::<Element>() else {
        if let Some(document) = noeud.dyn_ref::<Document>() {
            observer_liste(document.query_selector_all(SELECTEUR_REVELE), observateur);
        }
        return;
    };
    if element.matches(SELECTEUR_REVELE).unwrap_or(false) {
        observateur.observe(element);
    }
    observer_liste(element.query_selector_all(SELECTEUR_REVELE), observateur);
}

fn observer_liste(liste: Result<web_sys::NodeList, JsValue>, observateur: &IntersectionObserver) {
    let Ok(liste) = liste else { return };
    for i in 0..liste.length() {
        if let Some(element) = liste.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
            observateur.observe(&element);
        }
    }
}

/* ————————————————— Boucle d'animation ————————————————— */

/// Animation en cours ; elle s'arrête quand on la lâche.
pub struct Animation {
    active: Rc<Cell<bool>>,
}

impl Drop for Animation {
    fn drop(&mut self) {
        self.active.set(false);
    }
}

type RappelImage = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

fn image_suivante(rappel: &RappelImage) {
    if let Some(r) = rappel.borrow().as_ref() {
        let _ = fenetre().request_animation_frame(r.as_ref().unchecked_ref());
    }
}

/// Appelle `a_chaque_image` avec l'avancement (de 0 à 1) pendant `duree_ms`.
pub fn animer(duree_ms: f64, mut a_chaque_image: impl FnMut(f64) + 'static) -> Animation {
    let active = Rc::new(Cell::new(true));
    let rappel: RappelImage = Rc::new(RefCell::new(None));
    let suite = rappel.clone();
    let encore = active.clone();
    let mut debut = None;
    *rappel.borrow_mut() = Some(Closure::new(move |instant: f64| {
        if !encore.get() {
            // On lâche la boucle : elle sera libérée au retour.
            suite.borrow_mut().take();
            return;
        }
        let depart = *debut.get_or_insert(instant);
        let avancement = ((instant - depart) / duree_ms).clamp(0.0, 1.0);
        a_chaque_image(avancement);
        if avancement < 1.0 {
            image_suivante(&suite);
        } else {
            suite.borrow_mut().take();
        }
    }));
    image_suivante(&rappel);
    Animation { active }
}

/// Décélération marquée, comme une main qui se pose.
pub fn sortie_douce(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(4)
}

/* ————————————————— Souris : magnétisme et curseur ————————————————— */

/// Les boutons principaux se laissent attirer par le pointeur.
fn magnetisme() {
    let actuel: Rc<RefCell<Option<HtmlElement>>> = Rc::new(RefCell::new(None));

    fn relacher(element: &HtmlElement) {
        let style = element.style();
        let _ = style.remove_property("--mx");
        let _ = style.remove_property("--my");
    }

    let suivi = actuel.clone();
    EventListener::new(&document(), "pointermove", move |evenement| {
        let Some(evenement) = evenement.dyn_ref::<PointerEvent>() else {
            return;
        };
        let cible = evenement
            .target()
            .and_then(|c| c.dyn_into::<Element>().ok())
            .and_then(|c| c.closest(".btn-principal, .magnetique").ok().flatten())
            .and_then(|c| c.dyn_into::<HtmlElement>().ok());

        let mut actuel = suivi.borrow_mut();
        if let Some(precedent) = actuel.as_ref()
            && cible.as_ref() != Some(precedent)
        {
            relacher(precedent);
        }
        if let Some(bouton) = &cible {
            // Écart au centre, ramené entre -1 et 1 quelle que soit la largeur
            // du bouton : l'attraction reste de quelques pixels.
            let cadre = bouton.get_bounding_client_rect();
            let dx = (f64::from(evenement.client_x()) - (cadre.left() + cadre.width() / 2.0))
                / (cadre.width() / 2.0).max(1.0);
            let dy = (f64::from(evenement.client_y()) - (cadre.top() + cadre.height() / 2.0))
                / (cadre.height() / 2.0).max(1.0);
            let style = bouton.style();
            let _ = style.set_property("--mx", &format!("{:.1}px", dx.clamp(-1.0, 1.0) * 8.0));
            let _ = style.set_property("--my", &format!("{:.1}px", dy.clamp(-1.0, 1.0) * 5.0));
        }
        *actuel = cible;
    })
    .forget();

    EventListener::new(&racine(), "pointerleave", move |_| {
        if let Some(precedent) = actuel.borrow_mut().take() {
            relacher(&precedent);
        }
    })
    .forget();
}

/// Un anneau suit le pointeur avec un léger retard, et s'agrandit au-dessus
/// de ce qui se touche.
fn curseur() {
    let Ok(anneau) = document().create_element("div") else {
        return;
    };
    anneau.set_class_name("curseur");
    let _ = anneau.set_attribute("aria-hidden", "true");
    let Some(corps) = document().body() else {
        return;
    };
    let _ = corps.append_child(&anneau);
    let Ok(anneau) = anneau.dyn_into::<HtmlElement>() else {
        return;
    };

    // Position visée et position affichée.
    let vise = Rc::new(Cell::new((-100.0_f64, -100.0_f64)));
    let affiche = Rc::new(Cell::new((-100.0_f64, -100.0_f64)));
    let en_route = Rc::new(Cell::new(false));
    let boucle: RappelImage = Rc::new(RefCell::new(None));

    {
        let (vise, affiche, en_route, suite, anneau) = (
            vise.clone(),
            affiche.clone(),
            en_route.clone(),
            boucle.clone(),
            anneau.clone(),
        );
        *boucle.borrow_mut() = Some(Closure::new(move |_: f64| {
            let (vx, vy) = vise.get();
            let (ax, ay) = affiche.get();
            let (nx, ny) = (ax + (vx - ax) * 0.2, ay + (vy - ay) * 0.2);
            affiche.set((nx, ny));
            let _ = anneau.style().set_property(
                "transform",
                &format!("translate3d({nx:.1}px, {ny:.1}px, 0)"),
            );
            if (vx - nx).abs() + (vy - ny).abs() > 0.3 {
                image_suivante(&suite);
            } else {
                en_route.set(false);
            }
        }));
    }

    let classes = anneau.class_list();
    EventListener::new(&document(), "pointermove", move |evenement| {
        let Some(evenement) = evenement.dyn_ref::<PointerEvent>() else {
            return;
        };
        if evenement.pointer_type() != "mouse" {
            return;
        }
        vise.set((
            f64::from(evenement.client_x()),
            f64::from(evenement.client_y()),
        ));
        let survol = evenement
            .target()
            .and_then(|c| c.dyn_into::<Element>().ok())
            .and_then(|c| c.closest("button, a, [role=button]").ok().flatten())
            .is_some();
        let _ = classes.toggle_with_force("survol", survol);
        let _ = classes.add_1("visible");
        if !en_route.replace(true) {
            image_suivante(&boucle);
        }
    })
    .forget();

    let classes = anneau.class_list();
    EventListener::new(&racine(), "pointerleave", move |_| {
        let _ = classes.remove_1("visible");
    })
    .forget();
}
