//! Suivi de progression, sauvegardé localement sur l'appareil.
//!
//! Le format et la clé de stockage sont ceux de la version précédente du
//! site : l'historique déjà enregistré dans le navigateur est repris tel
//! quel, rien n'est jamais remis à zéro.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::dates::Jour;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeanceTerminee {
    /// Date au format AAAA-MM-JJ (heure locale).
    pub date: String,
    pub titre: String,
    pub minutes: u32,
    pub exercices_faits: u32,
    /// Ressenti après la séance : 1 = difficile, 2 = correct, 3 = bien.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ressenti: Option<u8>,
}

#[cfg(target_arch = "wasm32")]
const CLE: &str = "hemifit.progression.v1";

/// Lit les entrées brutes : une entrée illisible est conservée telle quelle
/// lors de l'écriture suivante, plutôt que d'être effacée.
#[cfg(target_arch = "wasm32")]
fn lire_brut() -> Vec<serde_json::Value> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(CLE).ok().flatten())
        .and_then(|brut| serde_json::from_str(&brut).ok())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn charger_historique() -> Vec<SeanceTerminee> {
    lire_brut()
        .into_iter()
        .filter_map(|v| serde_json::from_value(v).ok())
        .collect()
}

#[cfg(target_arch = "wasm32")]
pub fn enregistrer_seance(seance: SeanceTerminee) -> Vec<SeanceTerminee> {
    let mut brut = lire_brut();
    if let Ok(valeur) = serde_json::to_value(&seance) {
        brut.push(valeur);
    }
    if let (Some(stockage), Ok(texte)) = (
        web_sys::window().and_then(|w| w.local_storage().ok().flatten()),
        serde_json::to_string(&brut),
    ) {
        // Stockage indisponible (navigation privée…) : l'app continue sans sauvegarde.
        let _ = stockage.set_item(CLE, &texte);
    }
    charger_historique()
}

fn jours_actifs(historique: &[SeanceTerminee]) -> BTreeSet<Jour> {
    historique
        .iter()
        .filter_map(|s| Jour::depuis_iso(&s.date))
        .collect()
}

pub fn seance_faite_aujourdhui(historique: &[SeanceTerminee], aujourdhui: Jour) -> bool {
    jours_actifs(historique).contains(&aujourdhui)
}

/// Nombre de jours consécutifs avec au moins une séance, en comptant aujourd'hui ou hier.
pub fn serie_en_cours(historique: &[SeanceTerminee], aujourdhui: Jour) -> u32 {
    let jours = jours_actifs(historique);
    let mut curseur = aujourdhui;
    // La série n'est pas cassée si la séance du jour n'est pas encore faite.
    if !jours.contains(&curseur) {
        curseur = curseur.veille();
    }
    let mut serie = 0;
    while jours.contains(&curseur) {
        serie += 1;
        curseur = curseur.veille();
    }
    serie
}

/// Meilleure série jamais atteinte. Elle n'est jamais perdue : une
/// interruption, même longue, n'efface pas ce qui a été accompli.
pub fn meilleure_serie(historique: &[SeanceTerminee]) -> u32 {
    let mut meilleure = 0;
    let mut courante = 0;
    let mut precedent: Option<Jour> = None;
    for jour in jours_actifs(historique) {
        courante = match precedent {
            Some(veille) if jour.jours_depuis(veille) == 1 => courante + 1,
            _ => 1,
        };
        meilleure = meilleure.max(courante);
        precedent = Some(jour);
    }
    meilleure
}

/// Nombre de jours depuis la dernière séance ; `None` si aucune séance
/// n'a jamais été faite.
pub fn jours_depuis_derniere_seance(
    historique: &[SeanceTerminee],
    aujourdhui: Jour,
) -> Option<i64> {
    jours_actifs(historique)
        .last()
        .map(|&derniere| aujourdhui.jours_depuis(derniere).max(0))
}

pub fn seances_sur_7_jours(historique: &[SeanceTerminee], aujourdhui: Jour) -> usize {
    historique
        .iter()
        .filter_map(|s| Jour::depuis_iso(&s.date))
        .filter(|&j| (0..7).contains(&aujourdhui.jours_depuis(j)))
        .count()
}

pub fn minutes_totales(historique: &[SeanceTerminee]) -> u32 {
    historique.iter().map(|s| s.minutes).sum()
}

#[derive(Clone, PartialEq)]
pub struct JourActivite {
    pub jour: Jour,
    pub actif: bool,
}

/// Les 7 derniers jours, du plus ancien à aujourd'hui, pour la bande d'activité.
pub fn derniers_7_jours(historique: &[SeanceTerminee], aujourdhui: Jour) -> Vec<JourActivite> {
    let faits = jours_actifs(historique);
    (0..7)
        .rev()
        .map(|i| {
            let jour = aujourdhui.decaler(-i);
            JourActivite {
                jour,
                actif: faits.contains(&jour),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seance(date: &str) -> SeanceTerminee {
        SeanceTerminee {
            date: date.into(),
            titre: "Essai".into(),
            minutes: 15,
            exercices_faits: 6,
            ressenti: None,
        }
    }

    fn jour(texte: &str) -> Jour {
        Jour::depuis_iso(texte).unwrap()
    }

    #[test]
    fn lit_le_format_de_la_version_precedente() {
        let brut = r#"[{"date":"2026-10-08","titre":"Main et poignet","minutes":17,"exercicesFaits":6,"ressenti":3},
                       {"date":"2026-10-09","titre":"Bras et épaule","minutes":16,"exercicesFaits":6}]"#;
        let h: Vec<SeanceTerminee> = serde_json::from_str(brut).unwrap();
        assert_eq!(h[0].ressenti, Some(3));
        assert_eq!(h[1].ressenti, None);
        assert_eq!(h[1].exercices_faits, 6);
        // Le ressenti absent n'est pas réécrit en « null ».
        assert!(!serde_json::to_string(&h[1]).unwrap().contains("ressenti"));
    }

    #[test]
    fn la_serie_tient_tant_que_la_journee_n_est_pas_finie() {
        let h = [seance("2026-10-07"), seance("2026-10-08")];
        assert_eq!(serie_en_cours(&h, jour("2026-10-09")), 2);
        assert_eq!(serie_en_cours(&h, jour("2026-10-10")), 0);
    }

    #[test]
    fn la_meilleure_serie_n_est_jamais_perdue() {
        let h = [
            seance("2026-01-01"),
            seance("2026-01-02"),
            seance("2026-01-03"),
            seance("2026-09-30"),
        ];
        assert_eq!(meilleure_serie(&h), 3);
        assert_eq!(
            jours_depuis_derniere_seance(&h, jour("2026-10-09")),
            Some(9)
        );
    }

    #[test]
    fn semaine_glissante() {
        let h = [
            seance("2026-10-02"),
            seance("2026-10-03"),
            seance("2026-10-09"),
            seance("2026-10-09"),
        ];
        assert_eq!(seances_sur_7_jours(&h, jour("2026-10-09")), 3);
        let semaine = derniers_7_jours(&h, jour("2026-10-09"));
        assert_eq!(semaine.len(), 7);
        assert!(semaine[0].actif && semaine[6].actif && !semaine[1].actif);
    }
}
