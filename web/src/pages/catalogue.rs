//! Catalogue : toutes les familles d'exercices, filtrables d'un seul appui.

use std::rc::Rc;

use yew::prelude::*;

use crate::composants::{PastilleRealisation, TitreFente};
use crate::exercices::{Categorie, EXERCICES, Exercice, ORDRE_CATEGORIES, Realisation, Seance};
use crate::icones::{Icone, Pictogramme};

#[derive(Properties, PartialEq)]
pub struct ProprietesCatalogue {
    pub on_seance_libre: Callback<Rc<Seance>>,
}

#[component]
pub fn Catalogue(p: &ProprietesCatalogue) -> Html {
    let ouvert = use_state(|| None::<&'static str>);
    // `None` : tous les modes de réalisation, ou toutes les familles.
    let filtre = use_state(|| None::<Realisation>);
    let famille = use_state(|| None::<Categorie>);

    let visibles: Vec<&'static Exercice> = EXERCICES
        .iter()
        .filter(|e| filtre.is_none_or(|f| e.realisation == f))
        .collect();
    let nombre_affiche = visibles
        .iter()
        .filter(|e| famille.is_none_or(|f| e.categorie == f))
        .count();

    let choisir_famille = |cat: Option<Categorie>| {
        let famille = famille.clone();
        Callback::from(move |_| famille.set(cat))
    };
    let choisir_filtre = |f: Option<Realisation>| {
        let filtre = filtre.clone();
        Callback::from(move |_| filtre.set(f))
    };

    let sections = ORDRE_CATEGORIES
        .iter()
        .enumerate()
        .filter(|&(_, &cat)| famille.is_none_or(|f| f == cat))
        .filter_map(|(rang, &cat)| {
            let liste: Vec<_> = visibles.iter().filter(|e| e.categorie == cat).copied().collect();
            if liste.is_empty() {
                return None;
            }
            let fam = cat.famille();
            Some(html! {
                <section key={fam.court}>
                    <h2 class="titre-section" data-revele="">
                        <span class="numero">{ format!("{:02}", rang + 1) }</span>
                        <Pictogramme nom={fam.icone} taille={21} />
                        { fam.titre }
                    </h2>
                    for ex in liste {
                        <CarteExercice
                            key={ex.id}
                            exercice={ex}
                            ouvert={*ouvert == Some(ex.id)}
                            on_basculer={
                                let ouvert = ouvert.clone();
                                Callback::from(move |_| {
                                    ouvert.set(if *ouvert == Some(ex.id) { None } else { Some(ex.id) })
                                })
                            }
                            on_lancer={p.on_seance_libre.reform(move |_| Rc::new(Seance::a_la_carte(ex)))}
                        />
                    }
                </section>
            })
        });

    html! {
        <>
            <header class="entete">
                <p class="surtitre-page">{ format!("{} exercices", EXERCICES.len()) }</p>
                <TitreFente texte="Tous les exercices" />
                <p class="chapo">
                    { "Touchez un exercice pour voir les consignes, ou lancez-le seul quand vous en avez envie." }
                </p>
            </header>

            <div class="familles" role="group" aria-label="Choisir une famille">
                <button
                    class={classes!(famille.is_none().then_some("actif"))}
                    aria-pressed={famille.is_none().to_string()}
                    onclick={choisir_famille(None)}
                >
                    { "Toutes" }
                </button>
                for cat in ORDRE_CATEGORIES {
                    <button
                        class={classes!((*famille == Some(cat)).then_some("actif"))}
                        aria-pressed={(*famille == Some(cat)).to_string()}
                        onclick={choisir_famille(if *famille == Some(cat) { None } else { Some(cat) })}
                    >
                        <Pictogramme nom={cat.famille().icone} taille={17} epaisseur={2.0} />
                        { cat.famille().court }
                    </button>
                }
            </div>

            <div class="segments" role="group" aria-label="Filtrer les exercices">
                <button class={classes!(filtre.is_none().then_some("actif"))} onclick={choisir_filtre(None)}>
                    { "Tous" }
                </button>
                <button
                    class={classes!((*filtre == Some(Realisation::Autonome)).then_some("actif"))}
                    onclick={choisir_filtre(Some(Realisation::Autonome))}
                >
                    <Pictogramme nom={Icone::Autonome} taille={17} epaisseur={2.0} />
                    { "Seul" }
                </button>
                <button
                    class={classes!((*filtre == Some(Realisation::TiercePersonne)).then_some("actif"))}
                    onclick={choisir_filtre(Some(Realisation::TiercePersonne))}
                >
                    <Pictogramme nom={Icone::Aide} taille={17} epaisseur={2.0} />
                    { "Avec de l'aide" }
                </button>
            </div>

            { for sections }

            if nombre_affiche == 0 {
                <p class="liste-vide">
                    { format!(
                        "Aucun exercice de cette famille ne se fait {}. Touchez « Toutes » pour revoir l'ensemble du catalogue.",
                        match *filtre {
                            None => "ainsi",
                            Some(Realisation::Autonome) => "en autonomie",
                            Some(Realisation::TiercePersonne) => "avec une tierce personne",
                        }
                    ) }
                </p>
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ProprietesCarteExercice {
    exercice: &'static Exercice,
    ouvert: bool,
    on_basculer: Callback<MouseEvent>,
    on_lancer: Callback<MouseEvent>,
}

#[component]
fn CarteExercice(p: &ProprietesCarteExercice) -> Html {
    let ex = p.exercice;
    html! {
        <div class="exercice">
            <button
                class="ligne-exercice"
                onclick={p.on_basculer.clone()}
                aria-expanded={p.ouvert.to_string()}
                data-revele=""
            >
                <span class="vignette">
                    <Pictogramme nom={ex.categorie.famille().icone} taille={22} />
                </span>
                <span class="infos">
                    <strong>{ ex.nom }</strong>
                    <span>{ format!("{} · {}", ex.dosage, ex.position.libelle()) }</span>
                </span>
                if ex.realisation == Realisation::TiercePersonne {
                    <span class="marque-aide" title={Realisation::TiercePersonne.titre()}>
                        <Pictogramme nom={Icone::Aide} taille={17} epaisseur={2.0} />
                    </span>
                }
                <span class={classes!("chevron", p.ouvert.then_some("ouvert"))} aria-hidden="true">
                    <Pictogramme nom={Icone::Pousse} taille={18} epaisseur={2.0} />
                </span>
            </button>

            if p.ouvert {
                <div class="carte detail-exercice">
                    <PastilleRealisation realisation={ex.realisation} />
                    <p class="objectif">{ ex.objectif }</p>
                    <ul class="etapes">
                        for etape in ex.etapes.iter() { <li>{ *etape }</li> }
                    </ul>
                    <button class="btn btn-secondaire" onclick={p.on_lancer.clone()}>
                        <Pictogramme nom={Icone::Lecture} taille={19} plein=true />
                        { "Faire cet exercice" }
                    </button>
                </div>
            }
        </div>
    }
}
