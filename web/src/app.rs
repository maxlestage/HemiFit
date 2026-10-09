//! L'application : quatre onglets en bas d'écran, et le lecteur de séance
//! en plein écran par-dessus.

use std::iter;
use std::rc::Rc;

use yew::prelude::*;

use crate::dates;
use crate::exercices::{self, Seance};
use crate::icones::{Icone, Pictogramme};
use crate::lecteur::LecteurSeance;
use crate::mouvement;
use crate::pages::{accueil::Accueil, catalogue::Catalogue, conseils::Conseils, progres::Progres};
use crate::progression;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Onglet {
    Accueil,
    Exercices,
    Progres,
    Conseils,
}

impl Onglet {
    const TOUS: [Onglet; 4] = [
        Onglet::Accueil,
        Onglet::Exercices,
        Onglet::Progres,
        Onglet::Conseils,
    ];

    fn libelle(self) -> &'static str {
        match self {
            Onglet::Accueil => "Accueil",
            Onglet::Exercices => "Exercices",
            Onglet::Progres => "Progrès",
            Onglet::Conseils => "Conseils",
        }
    }

    fn icone(self) -> Icone {
        match self {
            Onglet::Accueil => Icone::Accueil,
            Onglet::Exercices => Icone::Exercices,
            Onglet::Progres => Icone::Progres,
            Onglet::Conseils => Icone::Conseils,
        }
    }

    fn rang(self) -> usize {
        self as usize
    }
}

#[component]
pub fn App() -> Html {
    let onglet = use_state(|| Onglet::Accueil);
    let seance_en_cours = use_state(|| None::<Rc<Seance>>);
    let historique = use_state(|| Rc::new(progression::charger_historique()));

    // Les animations démarrent une fois l'application affichée.
    use_effect_with((), |_| mouvement::demarrer());

    // Chaque changement d'écran repart du haut de la page.
    use_effect_with((*onglet, seance_en_cours.is_some()), |_| {
        mouvement::haut_de_page()
    });

    if let Some(seance) = (*seance_en_cours).clone() {
        let quitter = {
            let seance_en_cours = seance_en_cours.clone();
            Callback::from(move |_| seance_en_cours.set(None))
        };
        let terminee = {
            let (seance_en_cours, historique, onglet) =
                (seance_en_cours.clone(), historique.clone(), onglet.clone());
            Callback::from(move |nouvel_historique| {
                historique.set(Rc::new(nouvel_historique));
                seance_en_cours.set(None);
                onglet.set(Onglet::Progres);
            })
        };
        return html! {
            <LecteurSeance seance={seance} on_quitter={quitter} on_terminee={terminee} />
        };
    }

    let aujourdhui = dates::aujourdhui();
    let demarrer = {
        let seance_en_cours = seance_en_cours.clone();
        Callback::from(move |seance: Rc<Seance>| seance_en_cours.set(Some(seance)))
    };

    let page = match *onglet {
        Onglet::Accueil => html! {
            <Accueil
                seance={Rc::new(exercices::seance_du_jour(aujourdhui.jour_semaine()))}
                soir={Rc::new(exercices::seance_du_soir())}
                historique={(*historique).clone()}
                aujourdhui={aujourdhui}
                on_demarrer={demarrer}
            />
        },
        Onglet::Exercices => html! { <Catalogue on_seance_libre={demarrer} /> },
        Onglet::Progres => html! {
            <Progres historique={(*historique).clone()} aujourdhui={aujourdhui} />
        },
        Onglet::Conseils => html! { <Conseils /> },
    };

    // Le contenu est reconstruit à chaque onglet : son entrée se rejoue.
    let contenu = html! {
        <main class="contenu" key={onglet.rang()}>{ page }</main>
    };

    html! {
        <>
            { for iter::once(contenu) }

            <nav
                class="nav-basse"
                aria-label="Navigation principale"
                style={format!("--actif:{}", onglet.rang())}
            >
                for cible in Onglet::TOUS {
                    <BoutonOnglet
                        actif={*onglet == cible}
                        icone={cible.icone()}
                        libelle={cible.libelle()}
                        onclick={
                            let onglet = onglet.clone();
                            Callback::from(move |_| onglet.set(cible))
                        }
                    />
                }
            </nav>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ProprietesBoutonOnglet {
    actif: bool,
    icone: Icone,
    libelle: AttrValue,
    onclick: Callback<MouseEvent>,
}

#[component]
fn BoutonOnglet(p: &ProprietesBoutonOnglet) -> Html {
    html! {
        <button
            class={classes!(p.actif.then_some("actif"))}
            onclick={p.onclick.clone()}
            aria-current={p.actif.then_some("page")}
        >
            <Pictogramme nom={p.icone} taille={23} epaisseur={if p.actif { 2.0 } else { 1.7 }} />
            { p.libelle.clone() }
        </button>
    }
}
