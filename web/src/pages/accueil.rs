//! Accueil : la séance du jour, la séance du soir et la série en cours.

use std::rc::Rc;

use yew::prelude::*;

use crate::composants::{
    BandeauPrincipes, BandeauSecurite, Compteur, Manifeste, PastilleRealisation, TitreFente,
};
use crate::dates::{self, Jour};
use crate::exercices::{Realisation, Seance};
use crate::icones::{Icone, Pictogramme};
use crate::progression::{self, SeanceTerminee};

#[derive(Properties, PartialEq)]
pub struct ProprietesAccueil {
    pub seance: Rc<Seance>,
    pub soir: Rc<Seance>,
    pub historique: Rc<Vec<SeanceTerminee>>,
    pub aujourdhui: Jour,
    pub on_demarrer: Callback<Rc<Seance>>,
}

#[component]
pub fn Accueil(p: &ProprietesAccueil) -> Html {
    let h = p.historique.as_slice();
    let faite_aujourdhui = progression::seance_faite_aujourdhui(h, p.aujourdhui);
    let serie = progression::serie_en_cours(h, p.aujourdhui);
    let absence = progression::jours_depuis_derniere_seance(h, p.aujourdhui);

    let heure = dates::heure();
    let salutation = if heure < 12 {
        "Bonjour"
    } else if heure < 18 {
        "Bon après-midi"
    } else {
        "Bonsoir"
    };

    let demarrer = |seance: &Rc<Seance>| {
        let seance = seance.clone();
        p.on_demarrer.reform(move |_| seance.clone())
    };

    html! {
        <>
            <header class="entete">
                <p class="surtitre-page">{ p.aujourdhui.en_toutes_lettres() }</p>
                <TitreFente texte={salutation} />
                <p class="chapo">{ "Chaque petit mouvement compte. On avance en douceur." }</p>
            </header>

            if let Some(absence) = absence.filter(|&a| a >= 7) {
                <div class="carte" data-revele="">
                    <h2>{ "Content de vous revoir" }</h2>
                    <p>
                        { if absence >= 60 {
                            "Cela fait un moment, et ce n'est pas grave du tout."
                        } else {
                            "Quelques jours sans séance, et alors ?"
                        } }
                        { " Une pause n'efface rien de ce que vous avez déjà construit : votre \
                           meilleure série reste inscrite dans vos progrès." }
                    </p>
                    <p>
                        { "On reprend tranquillement, là où vous en êtes aujourd'hui. C'est le \
                           seul endroit d'où on peut repartir." }
                    </p>
                </div>
            }

            if serie > 0 {
                <div class="carte carte-serie" data-revele="">
                    <span class="icone-serie">
                        <Pictogramme nom={Icone::Serie} taille={30} plein=true />
                    </span>
                    <div>
                        <Compteur valeur={serie} />
                        <p>
                            { if serie == 1 {
                                "jour de suite. La régularité commence ici."
                            } else {
                                "jours de suite. La régularité, c'est votre force."
                            } }
                        </p>
                    </div>
                </div>
            }

            <CarteSeance
                seance={p.seance.clone()}
                surtitre="Séance du jour"
                vedette=true
                libelle_bouton={if faite_aujourdhui { "Refaire la séance" } else { "Commencer la séance" }}
                on_demarrer={demarrer(&p.seance)}
            />

            if faite_aujourdhui {
                <div class="carte carte-faite" data-revele="">
                    <Pictogramme nom={Icone::Valide} taille={22} epaisseur={2.2} />
                    <div>
                        <h3>{ "Séance du jour déjà faite" }</h3>
                        <p>{ "L'important est la régularité, pas la quantité. Reposez-vous." }</p>
                    </div>
                </div>
            }

            <CarteSeance
                seance={p.soir.clone()}
                surtitre="Séance du soir"
                libelle_bouton="Ouvrir la séance du soir"
                on_demarrer={demarrer(&p.soir)}
            />

            <BandeauPrincipes />

            <Manifeste texte="Lentement, assis, la main gauche qui guide la droite. Chaque intention compte, même quand le mouvement ne se voit pas encore." />

            <BandeauSecurite texte="HemiFit accompagne votre rééducation mais ne remplace pas votre \
                kinésithérapeute ni votre médecin. Faites-leur valider ces exercices, et arrêtez \
                tout mouvement qui fait mal." />
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ProprietesCarteSeance {
    seance: Rc<Seance>,
    surtitre: AttrValue,
    #[prop_or_default]
    vedette: bool,
    libelle_bouton: AttrValue,
    on_demarrer: Callback<MouseEvent>,
}

#[component]
fn CarteSeance(p: &ProprietesCarteSeance) -> Html {
    let seance = &p.seance;
    html! {
        <div class={classes!("carte", p.vedette.then_some("carte-vedette"))} data-revele="">
            if p.vedette {
                // L'anneau de la marque, tracé en filigrane quand la carte apparaît.
                <svg class="filigrane" viewBox="0 0 1024 1024" aria-hidden="true">
                    <circle cx="512" cy="512" r="290" />
                </svg>
            }
            <div class="entete-carte">
                <span class="surtitre">
                    <Pictogramme nom={if p.vedette { Icone::Horloge } else { Icone::Soir }} taille={16} />
                    { format!("{} · {} min", p.surtitre, seance.duree_totale_min()) }
                </span>
                <PastilleRealisation realisation={seance.realisation} />
            </div>

            <h2>{ seance.titre }</h2>
            <p>{ seance.description }</p>
            <p class="detail-seance">
                { format!("{} exercices, ", seance.exercices.len()) }
                { match seance.realisation {
                    Realisation::Autonome => "réalisables seul, assis ou allongé.",
                    Realisation::TiercePersonne => "réalisés par la personne qui vous accompagne.",
                } }
            </p>

            <button class="btn btn-principal" onclick={p.on_demarrer.clone()}>
                <Pictogramme nom={Icone::Lecture} taille={20} plein=true />
                { p.libelle_bouton.clone() }
            </button>
        </div>
    }
}
