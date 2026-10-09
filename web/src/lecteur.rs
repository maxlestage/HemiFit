//! Lecteur de séance guidée : un exercice à la fois, un grand minuteur,
//! et des boutons larges en bas d'écran, accessibles au pouce gauche.

use std::f64::consts::PI;
use std::iter;
use std::rc::Rc;

use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::composants::TitreFente;
use crate::dates;
use crate::exercices::{Realisation, Seance};
use crate::icones::{Icone, Pictogramme};
use crate::progression::{self, SeanceTerminee};

#[derive(Properties, PartialEq)]
pub struct ProprietesLecteur {
    pub seance: Rc<Seance>,
    pub on_quitter: Callback<()>,
    pub on_terminee: Callback<Vec<SeanceTerminee>>,
}

#[component]
pub fn LecteurSeance(p: &ProprietesLecteur) -> Html {
    let indice = use_state(|| 0usize);
    let fin = use_state(|| false);

    if *fin {
        return html! {
            <FinDeSeance seance={p.seance.clone()} on_terminee={p.on_terminee.clone()} />
        };
    }

    let total = p.seance.exercices.len();
    let exercice = p.seance.exercices[*indice];
    let dernier = *indice + 1 == total;

    let suivant = {
        let (indice, fin) = (indice.clone(), fin.clone());
        Callback::from(move |_| {
            if dernier {
                fin.set(true);
            } else {
                indice.set(*indice + 1);
            }
            crate::mouvement::haut_de_page_seance();
        })
    };

    let famille = exercice.categorie.famille();
    // Le corps est reconstruit à chaque exercice : son entrée se rejoue et
    // le minuteur repart de zéro.
    let corps = html! {
        <div class="seance-corps" key={*indice}>
            <p class="compteur-exercice">{ format!("Exercice {} sur {}", *indice + 1, total) }</p>

            <div class="entete-carte">
                <span class="badge">
                    <Pictogramme nom={famille.icone} taille={16} />
                    { format!("{} · {}", famille.titre, exercice.position.libelle()) }
                </span>
                if exercice.realisation == Realisation::TiercePersonne {
                    <span class="pastille-mode pastille-aide">
                        <Pictogramme nom={Icone::Aide} taille={15} epaisseur={2.0} />
                        { Realisation::TiercePersonne.court() }
                    </span>
                }
            </div>

            <TitreFente texte={exercice.nom} classe="titre-exercice" />
            <p class="objectif">{ exercice.objectif }</p>
            <p class="dosage">{ exercice.dosage }</p>

            <Minuteur duree_sec={exercice.duree_sec} />

            <ul class="etapes">
                for etape in exercice.etapes.iter() { <li>{ *etape }</li> }
            </ul>
        </div>
    };

    html! {
        <div class="seance-plein-ecran" id="seance" role="region" aria-label="Séance en cours">
            <div class="seance-progression" aria-hidden="true">
                for i in 0..total {
                    <span class={classes!((i <= *indice).then_some("fait"))} />
                }
            </div>

            { for iter::once(corps) }

            <div class="zone-boutons-bas">
                <button class="btn btn-principal" onclick={suivant}>
                    if dernier {
                        <Pictogramme nom={Icone::Valide} taille={20} epaisseur={2.4} />
                        { "Terminer la séance" }
                    } else {
                        <Pictogramme nom={Icone::Lecture} taille={19} plein=true />
                        { "Exercice suivant" }
                    }
                </button>
                <button class="btn btn-discret" onclick={p.on_quitter.reform(|_| ())}>
                    { "Arrêter la séance" }
                </button>
            </div>
        </div>
    }
}

/// Rayon et circonférence de l'anneau du minuteur.
const RAYON: f64 = 94.0;
const CIRCONFERENCE: f64 = 2.0 * PI * RAYON;

#[derive(Properties, PartialEq)]
struct ProprietesMinuteur {
    duree_sec: u32,
}

/// Minuteur circulaire avec pause.
#[component]
fn Minuteur(p: &ProprietesMinuteur) -> Html {
    let restant = use_state(|| p.duree_sec);
    let en_pause = use_state(|| false);
    {
        let restant = restant.clone();
        use_effect_with((*restant, *en_pause), move |&(secondes, pause)| {
            let minuterie = (!pause && secondes > 0)
                .then(|| Timeout::new(1000, move || restant.set(secondes - 1)));
            move || drop(minuterie)
        });
    }

    let proportion = if p.duree_sec > 0 {
        f64::from(*restant) / f64::from(p.duree_sec)
    } else {
        0.0
    };
    let basculer = {
        let en_pause = en_pause.clone();
        Callback::from(move |_| en_pause.set(!*en_pause))
    };

    html! {
        <div class="bloc-minuteur">
            <div class={classes!("minuteur-anneau", (*restant == 0).then_some("fini"))}>
                <svg viewBox="0 0 208 208" aria-hidden="true">
                    <circle class="piste" cx="104" cy="104" r={RAYON.to_string()} />
                    <circle
                        class="jauge"
                        cx="104"
                        cy="104"
                        r={RAYON.to_string()}
                        stroke-dasharray={format!("{CIRCONFERENCE:.2}")}
                        stroke-dashoffset={format!("{:.2}", CIRCONFERENCE * (1.0 - proportion))}
                        style={format!("--circonference:{CIRCONFERENCE:.2}")}
                    />
                </svg>
                if *restant > 0 {
                    <span class="minuteur-texte" aria-live="polite">
                        { format!("{}:{:02}", *restant / 60, *restant % 60) }
                    </span>
                } else {
                    <span class="minuteur-fini" aria-live="polite">
                        <Pictogramme nom={Icone::Valide} taille={40} epaisseur={2.2} />
                        { "Terminé" }
                    </span>
                }
            </div>

            if *restant > 0 {
                <button class="btn btn-secondaire btn-pause" onclick={basculer}>
                    if *en_pause {
                        <Pictogramme nom={Icone::Lecture} taille={18} plein=true />
                        { "Reprendre" }
                    } else {
                        <Pictogramme nom={Icone::Pause} taille={18} />
                        { "Pause" }
                    }
                </button>
            }
        </div>
    }
}

const RESSENTIS: [(u8, &str); 3] = [(1, "Difficile"), (2, "Correct"), (3, "Bien")];

#[derive(Properties, PartialEq)]
struct ProprietesFin {
    seance: Rc<Seance>,
    on_terminee: Callback<Vec<SeanceTerminee>>,
}

/// Écran de fin : ressenti puis enregistrement de la séance.
#[component]
fn FinDeSeance(p: &ProprietesFin) -> Html {
    let ressenti = use_state(|| None::<u8>);

    let enregistrer = {
        let (seance, ressenti, on_terminee) =
            (p.seance.clone(), ressenti.clone(), p.on_terminee.clone());
        Callback::from(move |_| {
            let historique = progression::enregistrer_seance(SeanceTerminee {
                date: dates::aujourdhui().to_string(),
                titre: seance.titre.to_owned(),
                minutes: seance.duree_totale_min().max(1),
                exercices_faits: seance.exercices.len() as u32,
                ressenti: *ressenti,
            });
            on_terminee.emit(historique);
        })
    };

    html! {
        <div class="seance-plein-ecran fin-de-seance">
            <div class="felicitations">
                <span class="sceau-fin">
                    <Pictogramme nom={Icone::Valide} taille={44} epaisseur={2.2} />
                </span>
                <TitreFente texte="Séance terminée" />
                <p>
                    { "Chaque séance renforce les nouveaux chemins de votre cerveau. Vous pouvez \
                       en être fier." }
                </p>
            </div>

            <h2 class="question-ressenti">{ "Comment vous sentez-vous ?" }</h2>
            <div class="choix-ressenti">
                for (valeur, libelle) in RESSENTIS {
                    <button
                        class={classes!((*ressenti == Some(valeur)).then_some("choisi"))}
                        aria-pressed={(*ressenti == Some(valeur)).to_string()}
                        onclick={
                            let ressenti = ressenti.clone();
                            Callback::from(move |_| ressenti.set(Some(valeur)))
                        }
                    >
                        <span class="jauge-ressenti" aria-hidden="true">
                            for n in 1..=3u8 {
                                <i class={classes!((n <= valeur).then_some("remplie"))} />
                            }
                        </span>
                        { libelle }
                    </button>
                }
            </div>

            <div class="zone-boutons-bas">
                <button class="btn btn-principal" onclick={enregistrer}>
                    <Pictogramme nom={Icone::Valide} taille={20} epaisseur={2.4} />
                    { "Enregistrer la séance" }
                </button>
            </div>
        </div>
    }
}
