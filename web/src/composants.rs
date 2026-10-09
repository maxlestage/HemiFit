//! Petits composants partagés entre les écrans.

use yew::prelude::*;

use crate::exercices::Realisation;
use crate::icones::{Icone, Pictogramme};
use crate::mouvement::{self, Animation, use_scene_prete};

/* ————————————————— Titre en fente ————————————————— */

#[derive(Properties, PartialEq)]
pub struct ProprietesTitre {
    pub texte: AttrValue,
    #[prop_or_default]
    pub classe: Classes,
}

/// Titre principal : chaque mot sort d'une fente, l'un après l'autre.
/// Le texte complet reste lisible tel quel par les lecteurs d'écran.
#[component]
pub fn TitreFente(p: &ProprietesTitre) -> Html {
    let mots = p.texte.split(' ').enumerate().map(|(i, mot)| {
        html! {
            <>
                if i > 0 { {" "} }
                <span class="fente"><span style={format!("--i:{i}")}>{ mot.to_owned() }</span></span>
            </>
        }
    });
    html! {
        <h1 class={classes!("titre-fente", p.classe.clone())}>
            <span class="lecteur-ecran">{ p.texte.clone() }</span>
            <span aria-hidden="true">{ for mots }</span>
        </h1>
    }
}

/* ————————————————— Compteur ————————————————— */

#[derive(Properties, PartialEq)]
pub struct ProprietesCompteur {
    pub valeur: u32,
}

/// Grand chiffre qui monte jusqu'à sa valeur quand le rideau est levé.
#[component]
pub fn Compteur(p: &ProprietesCompteur) -> Html {
    let scene = use_scene_prete();
    let affiche = use_state(|| {
        if mouvement::mouvement_permis() {
            0
        } else {
            p.valeur
        }
    });
    let animation = use_mut_ref(|| None::<Animation>);
    {
        let affiche = affiche.clone();
        use_effect_with((p.valeur, scene), move |&(cible, scene)| {
            let depart = *affiche;
            if !mouvement::mouvement_permis() || depart == cible {
                affiche.set(cible);
            } else if scene {
                *animation.borrow_mut() = Some(mouvement::animer(1300.0, move |t| {
                    let ecart = f64::from(cible) - f64::from(depart);
                    let valeur = f64::from(depart) + ecart * mouvement::sortie_douce(t);
                    affiche.set(valeur.round() as u32);
                }));
            }
        });
    }
    html! {
        <span class="grand-chiffre">
            <span class="lecteur-ecran">{ p.valeur }</span>
            <span aria-hidden="true">{ *affiche }</span>
        </span>
    }
}

/* ————————————————— Pastille de réalisation ————————————————— */

#[derive(Properties, PartialEq)]
pub struct ProprietesPastille {
    pub realisation: Realisation,
}

/// Pastille indiquant qui réalise l'exercice ou la séance.
#[component]
pub fn PastilleRealisation(p: &ProprietesPastille) -> Html {
    let classe = match p.realisation {
        Realisation::Autonome => classes!("pastille-mode"),
        Realisation::TiercePersonne => classes!("pastille-mode", "pastille-aide"),
    };
    html! {
        <span class={classe} title={p.realisation.titre()}>
            <Pictogramme nom={p.realisation.icone()} taille={15} epaisseur={2.0} />
            { p.realisation.court() }
        </span>
    }
}

/* ————————————————— Bandeau de sécurité ————————————————— */

#[derive(Properties, PartialEq)]
pub struct ProprietesSecurite {
    pub texte: AttrValue,
}

#[component]
pub fn BandeauSecurite(p: &ProprietesSecurite) -> Html {
    html! {
        <div class="bandeau-securite" data-revele="">
            <Pictogramme nom={Icone::Info} taille={20} />
            <p>{ p.texte.clone() }</p>
        </div>
    }
}

/* ————————————————— Bandeau des principes ————————————————— */

const PRINCIPES: [&str; 6] = [
    "Lentement, jamais forcé",
    "Assis, le dos soutenu",
    "La main gauche guide la droite",
    "Masser avant de bouger",
    "Respirer sans bloquer",
    "L'intention compte déjà",
];

/// Les principes défilent lentement ; un appui arrête ou relance le défilement.
#[component]
pub fn BandeauPrincipes() -> Html {
    let en_pause = use_state(|| false);
    let basculer = {
        let en_pause = en_pause.clone();
        Callback::from(move |_| en_pause.set(!*en_pause))
    };
    let groupe = || {
        html! {
            <span class="bandeau-groupe">
                for principe in PRINCIPES {
                    <span class="principe">{ principe }</span>
                }
            </span>
        }
    };
    html! {
        <section class="bandeau" data-revele="">
            <ul class="lecteur-ecran">
                for principe in PRINCIPES { <li>{ principe }</li> }
            </ul>
            <button
                class={classes!("bandeau-defilant", en_pause.then_some("en-pause"))}
                onclick={basculer}
                aria-pressed={(*en_pause).to_string()}
                aria-label={if *en_pause { "Relancer le défilement des principes" } else { "Arrêter le défilement des principes" }}
            >
                <span class="bandeau-piste" aria-hidden="true">
                    { groupe() }
                    { groupe() }
                </span>
            </button>
        </section>
    }
}

/* ————————————————— Manifeste ————————————————— */

#[derive(Properties, PartialEq)]
pub struct ProprietesManifeste {
    pub texte: AttrValue,
}

/// Une phrase qui s'allume mot à mot quand elle entre à l'écran.
#[component]
pub fn Manifeste(p: &ProprietesManifeste) -> Html {
    let mots = p.texte.split(' ').enumerate().map(|(i, mot)| {
        html! {
            <>
                if i > 0 { {" "} }
                <span class="mot" style={format!("--i:{i}")}>{ mot.to_owned() }</span>
            </>
        }
    });
    html! { <p class="manifeste" data-revele="">{ for mots }</p> }
}
