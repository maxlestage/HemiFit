//! Progrès : série, meilleure série, minutes cumulées et dernières séances.

use std::rc::Rc;

use yew::prelude::*;

use crate::composants::{Compteur, TitreFente};
use crate::dates::Jour;
use crate::icones::{Icone, Pictogramme};
use crate::progression::{self, SeanceTerminee};

#[derive(Properties, PartialEq)]
pub struct ProprietesProgres {
    pub historique: Rc<Vec<SeanceTerminee>>,
    pub aujourdhui: Jour,
}

#[component]
pub fn Progres(p: &ProprietesProgres) -> Html {
    let h = p.historique.as_slice();
    let cette_semaine = progression::seances_sur_7_jours(h, p.aujourdhui);

    html! {
        <>
            <header class="entete">
                <p class="surtitre-page">{ "Suivi" }</p>
                <TitreFente texte="Mes progrès" />
                <p class="chapo">{ "La régularité compte plus que la performance." }</p>
            </header>

            <div class="grille-stats">
                <Tuile icone={Icone::Serie} valeur={progression::serie_en_cours(h, p.aujourdhui)} legende="jours de suite" />
                <Tuile icone={Icone::Record} valeur={progression::meilleure_serie(h)} legende="meilleure série, jamais perdue" />
                <Tuile icone={Icone::Valide} valeur={h.len() as u32} legende="séances au total" />
                <Tuile icone={Icone::Horloge} valeur={progression::minutes_totales(h)} legende="minutes de rééducation" />
            </div>

            <div class="carte" data-revele="">
                <h3>{ "Cette semaine" }</h3>
                <p>
                    { if cette_semaine > 0 {
                        format!(
                            "{cette_semaine} {} sur les 7 derniers jours.",
                            if cette_semaine == 1 { "séance" } else { "séances" }
                        )
                    } else {
                        "Aucune séance ces 7 derniers jours. La prochaine vous attend, tranquillement.".into()
                    } }
                </p>
                <div class="semaine">
                    for (i, jour) in progression::derniers_7_jours(h, p.aujourdhui).into_iter().enumerate() {
                        <div class="jour" key={jour.jour.to_string()}>
                            <div
                                class={classes!("pastille", jour.actif.then_some("actif"))}
                                style={format!("--i:{i}")}
                                title={jour.jour.to_string()}
                            >
                                if jour.actif {
                                    <Pictogramme nom={Icone::Valide} taille={16} epaisseur={2.6} />
                                }
                            </div>
                            <span class="etiquette">{ jour.jour.initiale() }</span>
                        </div>
                    }
                </div>
            </div>

            <div class="carte" data-revele="">
                <h3>{ "Rien de tout cela ne se perd" }</h3>
                <p>
                    { "Ces minutes sont du travail réel accompli par votre cerveau. Une pause, même \
                       de plusieurs mois, ne les efface pas : vous reprendrez là où vous en êtes, \
                       jamais à zéro." }
                </p>
            </div>

            <div class="carte" data-revele="">
                <h2>{ "Dernières séances" }</h2>
                if h.is_empty() {
                    <p>
                        { "Aucune séance pour l'instant. La première est la plus importante, et \
                           elle vous attend sur l'accueil." }
                    </p>
                }
                for s in h.iter().rev().take(14) {
                    <div class="historique-ligne">
                        <div>
                            <strong>{ s.titre.clone() }</strong>
                            <div class="date">
                                { Jour::depuis_iso(&s.date).map_or_else(|| s.date.clone(), Jour::en_toutes_lettres) }
                            </div>
                        </div>
                        <div class="minutes">{ format!("{} min", s.minutes) }</div>
                    </div>
                }
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ProprietesTuile {
    icone: Icone,
    valeur: u32,
    legende: AttrValue,
}

#[component]
fn Tuile(p: &ProprietesTuile) -> Html {
    html! {
        <div class="carte tuile" data-revele="">
            <span class="tuile-icone">
                <Pictogramme nom={p.icone} taille={20} />
            </span>
            <Compteur valeur={p.valeur} />
            <p>{ p.legende.clone() }</p>
        </div>
    }
}
