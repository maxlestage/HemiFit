//! Dates du calendrier local, sans heure.
//!
//! Les calculs (veille, lendemain, jour de la semaine) sont faits en Rust
//! pur, ce qui permet de les tester hors du navigateur ; seule la date du
//! jour est lue auprès du navigateur, en heure locale.

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Jour {
    /// Nombre de jours depuis le 1er janvier 1970.
    rang: i64,
}

const JOURS_SEMAINE: [&str; 7] = [
    "dimanche", "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi",
];

const MOIS: [&str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

impl Jour {
    /// Construit une date à partir de l'année, du mois (1–12) et du jour (1–31).
    pub fn depuis_amj(annee: i64, mois: u32, jour: u32) -> Self {
        // Algorithme « days from civil » de Howard Hinnant.
        let a = if mois <= 2 { annee - 1 } else { annee };
        let ere = a.div_euclid(400);
        let annee_ere = a - ere * 400;
        let m = i64::from(mois);
        let jour_annee = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(jour) - 1;
        let jour_ere = annee_ere * 365 + annee_ere / 4 - annee_ere / 100 + jour_annee;
        Jour {
            rang: ere * 146_097 + jour_ere - 719_468,
        }
    }

    /// Année, mois (1–12) et jour (1–31).
    pub fn amj(self) -> (i64, u32, u32) {
        let z = self.rang + 719_468;
        let ere = z.div_euclid(146_097);
        let jour_ere = z - ere * 146_097;
        let annee_ere = (jour_ere - jour_ere / 1460 + jour_ere / 36_524 - jour_ere / 146_096) / 365;
        let jour_annee = jour_ere - (365 * annee_ere + annee_ere / 4 - annee_ere / 100);
        let mp = (5 * jour_annee + 2) / 153;
        let jour = (jour_annee - (153 * mp + 2) / 5 + 1) as u32;
        let mois = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let annee = annee_ere + ere * 400 + i64::from(mois <= 2);
        (annee, mois, jour)
    }

    /// Lit une date au format AAAA-MM-JJ.
    pub fn depuis_iso(texte: &str) -> Option<Self> {
        let mut morceaux = texte.splitn(3, '-');
        let annee = morceaux.next()?.parse().ok()?;
        let mois = morceaux.next()?.parse().ok()?;
        let jour = morceaux.next()?.parse().ok()?;
        ((1..=12).contains(&mois) && (1..=31).contains(&jour))
            .then(|| Jour::depuis_amj(annee, mois, jour))
    }

    pub fn decaler(self, jours: i64) -> Self {
        Jour {
            rang: self.rang + jours,
        }
    }

    pub fn veille(self) -> Self {
        self.decaler(-1)
    }

    /// Nombre de jours écoulés depuis `avant`.
    pub fn jours_depuis(self, avant: Jour) -> i64 {
        self.rang - avant.rang
    }

    /// 0 = dimanche, 1 = lundi… 6 = samedi.
    pub fn jour_semaine(self) -> u32 {
        // Le 1er janvier 1970 était un jeudi.
        (self.rang + 4).rem_euclid(7) as u32
    }

    /// Initiale du jour, ex. « L » pour lundi.
    pub fn initiale(self) -> &'static str {
        ["D", "L", "M", "M", "J", "V", "S"][self.jour_semaine() as usize]
    }

    /// Ex. « lundi 5 octobre ».
    pub fn en_toutes_lettres(self) -> String {
        let (_, mois, jour) = self.amj();
        format!(
            "{} {} {}",
            JOURS_SEMAINE[self.jour_semaine() as usize],
            jour,
            MOIS[mois as usize - 1]
        )
    }
}

/// Format AAAA-MM-JJ, celui qui est enregistré dans l'historique.
impl fmt::Display for Jour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (annee, mois, jour) = self.amj();
        write!(f, "{annee:04}-{mois:02}-{jour:02}")
    }
}

/// La date du jour, en heure locale.
#[cfg(target_arch = "wasm32")]
pub fn aujourdhui() -> Jour {
    let d = js_sys::Date::new_0();
    Jour::depuis_amj(
        i64::from(d.get_full_year()),
        d.get_month() + 1,
        d.get_date(),
    )
}

/// L'heure locale (0–23), pour la salutation.
#[cfg(target_arch = "wasm32")]
pub fn heure() -> u32 {
    js_sys::Date::new_0().get_hours()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aller_retour_iso() {
        for texte in ["1970-01-01", "2000-02-29", "2024-12-31", "2026-10-09"] {
            assert_eq!(Jour::depuis_iso(texte).unwrap().to_string(), texte);
        }
    }

    #[test]
    fn jours_de_la_semaine() {
        // Le 9 octobre 2026 est un vendredi.
        let j = Jour::depuis_iso("2026-10-09").unwrap();
        assert_eq!(j.jour_semaine(), 5);
        assert_eq!(j.en_toutes_lettres(), "vendredi 9 octobre");
        assert_eq!(j.decaler(2).jour_semaine(), 0);
    }

    #[test]
    fn changement_de_mois_et_d_annee() {
        let j = Jour::depuis_iso("2026-03-01").unwrap();
        assert_eq!(j.veille().to_string(), "2026-02-28");
        let j = Jour::depuis_iso("2027-01-01").unwrap();
        assert_eq!(j.veille().to_string(), "2026-12-31");
    }

    #[test]
    fn date_invalide() {
        assert!(Jour::depuis_iso("2026-13-01").is_none());
        assert!(Jour::depuis_iso("n'importe quoi").is_none());
    }
}
