//! Catalogue d'exercices HemiFit.
//!
//! Profil visé : hémiparésie droite avec forte spasticité, déplacement
//! en fauteuil roulant et équilibre très altéré.
//!
//! Règles de sécurité non négociables :
//! - tout se fait assis avec le dos soutenu, ou allongé ;
//! - aucun exercice debout, aucun transfert non sécurisé ;
//! - la main gauche (saine) assiste le côté droit ;
//! - jamais de mouvement rapide ni forcé (réflexe spastique) ;
//! - étirements lents et prolongés, précédés de détente et de massage.
//!
//! Ce catalogue est dupliqué volontairement dans
//! `ios/HemiFit/Exercices.swift` : toute modification de l'un doit être
//! reportée à l'identique dans l'autre.

use crate::icones::Icone;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Categorie {
    Massage,
    Sensoriel,
    Main,
    Bras,
    Tronc,
    Jambe,
    Electrodes,
    Force,
}

/// Libellés et icône d'une famille d'exercices.
pub struct Famille {
    pub titre: &'static str,
    pub court: &'static str,
    pub icone: Icone,
}

impl Categorie {
    pub const fn famille(self) -> Famille {
        let (titre, court, icone) = match self {
            Categorie::Massage => ("Massage et détente", "Massage", Icone::Massage),
            Categorie::Sensoriel => ("Éveil sensoriel", "Sensoriel", Icone::Sensoriel),
            Categorie::Main => ("Main et doigts", "Main", Icone::Main),
            Categorie::Bras => ("Bras et épaule", "Bras", Icone::Bras),
            Categorie::Tronc => ("Tronc et posture", "Tronc", Icone::Tronc),
            Categorie::Jambe => ("Jambes et bassin", "Jambes", Icone::Jambe),
            Categorie::Electrodes => ("Électrodes", "Électrodes", Icone::Electrodes),
            Categorie::Force => ("Renforcement musculaire", "Muscles", Icone::Force),
        };
        Famille {
            titre,
            court,
            icone,
        }
    }
}

/// Ordre d'affichage des familles : on commence par la détente, qui
/// fait baisser le tonus, et on termine par les électrodes, qui
/// demandent du matériel.
pub const ORDRE_CATEGORIES: [Categorie; 8] = [
    Categorie::Massage,
    Categorie::Sensoriel,
    Categorie::Main,
    Categorie::Bras,
    Categorie::Tronc,
    Categorie::Jambe,
    Categorie::Force,
    Categorie::Electrodes,
];

/// Qui réalise l'exercice.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Realisation {
    Autonome,
    TiercePersonne,
}

impl Realisation {
    pub const fn titre(self) -> &'static str {
        match self {
            Realisation::Autonome => "En autonomie",
            Realisation::TiercePersonne => "Avec une tierce personne",
        }
    }

    pub const fn court(self) -> &'static str {
        match self {
            Realisation::Autonome => "Seul",
            Realisation::TiercePersonne => "Avec de l'aide",
        }
    }

    pub const fn icone(self) -> Icone {
        match self {
            Realisation::Autonome => Icone::Autonome,
            Realisation::TiercePersonne => Icone::Aide,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Position {
    Assis,
    Allonge,
}

impl Position {
    pub const fn libelle(self) -> &'static str {
        match self {
            Position::Assis => "assis",
            Position::Allonge => "allongé",
        }
    }
}

#[derive(Debug)]
pub struct Exercice {
    pub id: &'static str,
    pub nom: &'static str,
    pub categorie: Categorie,
    pub realisation: Realisation,
    /// Ce que l'exercice travaille, en une phrase.
    pub objectif: &'static str,
    /// Consignes pas à pas, phrases courtes.
    pub etapes: &'static [&'static str],
    /// Dosage lisible, ex. « 5 répétitions, tenir 30 s ».
    pub dosage: &'static str,
    /// Durée guidée en secondes pour le minuteur de séance.
    pub duree_sec: u32,
    pub position: Position,
}

/// Deux exercices sont identiques s'ils portent le même identifiant.
impl PartialEq for Exercice {
    fn eq(&self, autre: &Self) -> bool {
        self.id == autre.id
    }
}

pub static EXERCICES: &[Exercice] = &[
    // ————————————————— Massage et détente —————————————————
    Exercice {
        id: "detente-respiration",
        nom: "Détente et respiration",
        categorie: Categorie::Massage,
        realisation: Realisation::Autonome,
        objectif: "Faire baisser la spasticité avant de bouger : un corps détendu s'étire beaucoup mieux.",
        etapes: &[
            "Installez-vous bien calé, dos soutenu, bras droit posé sur un coussin.",
            "Inspirez lentement par le nez en comptant jusqu'à 4.",
            "Soufflez très lentement par la bouche en comptant jusqu'à 6, en laissant tomber les épaules.",
            "À chaque expiration, imaginez votre bras et votre main droite devenir lourds, chauds et mous.",
        ],
        dosage: "Environ 2 minutes de respiration lente",
        duree_sec: 120,
        position: Position::Assis,
    },
    Exercice {
        id: "massage-avant-bras",
        nom: "Massage de l'avant-bras",
        categorie: Categorie::Massage,
        realisation: Realisation::Autonome,
        objectif: "Détendre les muscles qui ferment la main : ce sont eux qui sont trop contractés.",
        etapes: &[
            "Posez l'avant-bras droit sur un coussin, paume vers le haut.",
            "Avec le pouce gauche, massez l'intérieur de l'avant-bras par petits cercles lents, du coude vers le poignet.",
            "Insistez doucement là où le muscle est dur, sans jamais provoquer de douleur.",
            "Terminez par de longs passages lisses, du coude vers la main.",
        ],
        dosage: "Environ 3 minutes",
        duree_sec: 180,
        position: Position::Assis,
    },
    Exercice {
        id: "massage-main",
        nom: "Massage de la main",
        categorie: Categorie::Massage,
        realisation: Realisation::Autonome,
        objectif: "Assouplir la paume et réveiller les sensations de la main droite.",
        etapes: &[
            "Main droite posée sur votre cuisse, paume vers le haut.",
            "Avec le pouce gauche, massez la paume par cercles lents, du centre vers les bords.",
            "Retournez la main : massez le dos de la main, entre les tendons.",
            "Reprenez chaque doigt : massez-le de la base vers le bout, puis étirez-le très doucement.",
        ],
        dosage: "Environ 3 minutes",
        duree_sec: 180,
        position: Position::Assis,
    },
    Exercice {
        id: "massage-pouce",
        nom: "Ouvrir l'espace du pouce",
        categorie: Categorie::Massage,
        realisation: Realisation::Autonome,
        objectif: "Empêcher le pouce de se bloquer dans la paume : c'est lui qui verrouille souvent toute la main.",
        etapes: &[
            "Prenez la main droite dans la gauche, paume vers le haut.",
            "Avec le pouce et l'index gauches, pincez doucement la peau entre le pouce et l'index droits.",
            "Massez cette zone par petits cercles, puis écartez très lentement le pouce de la paume.",
            "Maintenez le pouce écarté 20 secondes, sans forcer, puis relâchez.",
        ],
        dosage: "3 fois, tenir 20 s",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "massage-drainage",
        nom: "Drainage de la main vers l'épaule",
        categorie: Categorie::Massage,
        realisation: Realisation::Autonome,
        objectif: "Faire circuler : une main peu mobile a tendance à gonfler, surtout en fin de journée.",
        etapes: &[
            "Bras droit posé, si possible légèrement surélevé sur un coussin.",
            "Avec la paume gauche bien à plat, remontez lentement de la main vers le coude, en pressant doucement.",
            "Continuez du coude vers l'épaule, toujours dans ce sens (jamais l'inverse).",
            "Refaites le trajet complet, calmement, comme une vague qui remonte.",
        ],
        dosage: "10 remontées lentes",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "massage-mollet",
        nom: "Massage du mollet et du tendon d'Achille",
        categorie: Categorie::Massage,
        realisation: Realisation::Autonome,
        objectif: "Détendre le mollet, le muscle qui tire le pied en pointe et entretient l'équin.",
        etapes: &[
            "Freins bloqués, remontez le pied droit sur un tabouret bas ou sur le genou gauche, seulement si vous le faites sans vous déséquilibrer.",
            "Réchauffez d'abord : après la douche, ou avec une serviette chaude posée quelques minutes sur le mollet.",
            "Avec la main gauche, massez le mollet par pressions lentes, toujours de la cheville vers le genou.",
            "Insistez doucement là où le muscle est dur, sans jamais chercher la douleur.",
            "Terminez en pinçant le tendon d'Achille entre le pouce et l'index, de bas en haut.",
        ],
        dosage: "Environ 4 minutes",
        duree_sec: 240,
        position: Position::Assis,
    },
    Exercice {
        id: "massage-tibial-posterieur",
        nom: "Massage du bord interne de la jambe",
        categorie: Categorie::Massage,
        realisation: Realisation::Autonome,
        objectif: "Détendre le muscle qui tire le pied vers l'intérieur : c'est lui le principal responsable du varus.",
        etapes: &[
            "Repérez l'arête de l'os de la jambe, le tibia, sur le devant.",
            "Glissez le pouce gauche juste derrière son bord interne : le muscle recherché se trouve là, en profondeur.",
            "Massez par petits cercles lents le long de ce bord, de la cheville vers le genou.",
            "Ce muscle étant profond, augmentez la pression progressivement, sans jamais aller jusqu'à la douleur.",
            "Terminez par la plante du pied, du talon vers les orteils.",
        ],
        dosage: "Environ 3 minutes",
        duree_sec: 180,
        position: Position::Assis,
    },
    Exercice {
        id: "massage-epaule",
        nom: "Massage de l'épaule et de la nuque",
        categorie: Categorie::Massage,
        realisation: Realisation::Autonome,
        objectif: "Soulager l'épaule droite, souvent douloureuse quand le bras est peu actif.",
        etapes: &[
            "Avec la main gauche, massez le haut de l'épaule droite par pressions lentes.",
            "Remontez vers la nuque, puis redescendez vers l'omoplate.",
            "Soufflez lentement pendant le massage, en laissant l'épaule redescendre.",
            "Si l'épaule est douloureuse, restez très léger et parlez-en à votre kinésithérapeute.",
        ],
        dosage: "Environ 2 minutes",
        duree_sec: 120,
        position: Position::Assis,
    },
    // ————————————————— Éveil sensoriel —————————————————
    Exercice {
        id: "eveil-paume",
        nom: "Réveil de la main droite",
        categorie: Categorie::Sensoriel,
        realisation: Realisation::Autonome,
        objectif: "Réveiller les sensations de la main droite avant de la mobiliser.",
        etapes: &[
            "Bras droit posé sur une table ou un coussin.",
            "Avec la main gauche, frottez doucement la paume droite, du poignet vers les doigts.",
            "Variez les textures : tissu, éponge, brosse douce.",
            "Terminez par de petites pressions sur toute la main.",
        ],
        dosage: "Environ 2 minutes",
        duree_sec: 120,
        position: Position::Assis,
    },
    Exercice {
        id: "eveil-avant-bras",
        nom: "Caresses de l'avant-bras",
        categorie: Categorie::Sensoriel,
        realisation: Realisation::Autonome,
        objectif: "Stimuler la peau et abaisser le tonus de l'avant-bras droit.",
        etapes: &[
            "Posez l'avant-bras droit sur vos genoux ou une table.",
            "Avec la main gauche, caressez lentement du coude jusqu'à la main.",
            "Alternez le dessus et le dessous de l'avant-bras.",
        ],
        dosage: "Environ 1 minute 30",
        duree_sec: 90,
        position: Position::Assis,
    },
    Exercice {
        id: "main-miroir",
        nom: "Thérapie miroir",
        categorie: Categorie::Sensoriel,
        realisation: Realisation::Autonome,
        objectif: "Voir une main droite qui s'ouvre aide le cerveau à réapprendre le mouvement.",
        etapes: &[
            "Posez un miroir debout devant vous, tranche contre votre ventre, face réfléchissante vers la gauche.",
            "Cachez la main droite derrière le miroir ; regardez le reflet de la main gauche.",
            "Ouvrez et fermez lentement la main gauche en regardant le reflet.",
            "Pendant ce temps, essayez le même mouvement avec la main droite cachée, sans forcer.",
        ],
        dosage: "Environ 3 minutes",
        duree_sec: 180,
        position: Position::Assis,
    },
    Exercice {
        id: "main-imagerie",
        nom: "Imagerie du geste",
        categorie: Categorie::Sensoriel,
        realisation: Realisation::Autonome,
        objectif: "Activer les circuits du mouvement sans effort musculaire.",
        etapes: &[
            "Fermez les yeux, main droite posée confortablement.",
            "Ouvrez et fermez lentement la main gauche en observant bien la sensation.",
            "Puis imaginez très précisément le même mouvement avec la main droite.",
            "Visualisez les doigts qui se déplient, un par un, sans effort.",
        ],
        dosage: "Environ 2 minutes",
        duree_sec: 120,
        position: Position::Assis,
    },
    // ————————————————— Main et doigts —————————————————
    Exercice {
        id: "main-poignet-actif",
        nom: "Apprendre à bouger le poignet",
        categorie: Categorie::Main,
        realisation: Realisation::Autonome,
        objectif: "Réapprendre au poignet à se plier et se redresser : la gravité fait le mouvement, vous apprenez d'abord à le retenir.",
        etapes: &[
            "Avant-bras droit posé sur une table, main dans le vide au bord, paume vers le bas.",
            "Laissez la main pendre : la gravité plie le poignet toute seule.",
            "Avec la main gauche, remontez doucement la main droite à l'horizontale, puis laissez-la redescendre lentement.",
            "Essayez ensuite de retenir un peu la descente, ou de remonter d'un millimètre : retenir est plus facile que soulever.",
            "Terminez en laissant la main pendre et se détendre complètement.",
        ],
        dosage: "8 allers-retours doux",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "main-tenodese",
        nom: "L'astuce du poignet plié",
        categorie: Categorie::Main,
        realisation: Realisation::Autonome,
        objectif: "Poignet plié vers l'avant, les doigts se détendent et s'ouvrent plus facilement.",
        etapes: &[
            "Posez l'avant-bras droit sur la table ou votre cuisse.",
            "Avec la main gauche, pliez doucement le poignet droit vers l'avant. C'est la main gauche qui fait tout : le poignet droit se laisse porter.",
            "Les doigts se desserrent : profitez-en pour les ouvrir doucement avec la main gauche.",
            "Doigts ouverts, redressez très lentement le poignet, sans perdre l'ouverture.",
            "Si les doigts se referment, repliez le poignet et recommencez.",
        ],
        dosage: "5 essais tranquilles",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "main-ouverture",
        nom: "Ouverture de main assistée",
        categorie: Categorie::Main,
        realisation: Realisation::Autonome,
        objectif: "Ouvrir la main malgré la spasticité, en laissant le temps aux muscles de lâcher.",
        etapes: &[
            "Massez d'abord l'avant-bras et la paume pour préparer la main.",
            "Penchez légèrement le poignet vers l'avant : les doigts se laissent ouvrir plus facilement.",
            "Avec la main gauche, dépliez très lentement les doigts droits, en commençant par le pouce.",
            "Si les doigts résistent, ne forcez jamais : arrêtez-vous, soufflez, attendez que ça se relâche.",
            "Maintenez l'ouverture 30 secondes : c'est l'étirement prolongé qui calme la spasticité.",
        ],
        dosage: "3 ouvertures très lentes, tenir 30 s",
        duree_sec: 180,
        position: Position::Assis,
    },
    Exercice {
        id: "main-extension-active",
        nom: "Ouvrir avec de l'aide",
        categorie: Categorie::Main,
        realisation: Realisation::Autonome,
        objectif: "Vos doigts savent se fermer : on entraîne le mouvement inverse, l'ouverture.",
        etapes: &[
            "Main droite posée sur la cuisse, détendue.",
            "Serrez très légèrement le poing 3 secondes — cela, vous savez le faire.",
            "Puis arrêtez de serrer, soufflez, et essayez d'ouvrir les doigts, même d'un millimètre.",
            "Pendant que vous essayez, la main gauche accompagne et termine l'ouverture.",
            "L'essai compte autant que le résultat : c'est l'intention d'ouvrir qui réveille les muscles.",
        ],
        dosage: "6 essais, sans forcer",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "main-appui-paume",
        nom: "Appui sur la paume ouverte",
        categorie: Categorie::Main,
        realisation: Realisation::Autonome,
        objectif: "Un appui doux sur la main ouverte calme la spasticité des doigts.",
        etapes: &[
            "Ouvrez la main droite avec l'aide de la gauche, très lentement.",
            "Posez la paume droite bien à plat sur votre cuisse, doigts écartés si possible.",
            "Avec la main gauche posée par-dessus, appuyez très légèrement, comme pour ancrer la main.",
            "Gardez l'appui en respirant lentement ; si les doigts se replient, rouvrez-les calmement.",
        ],
        dosage: "3 appuis d'environ 30 s",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "main-relacher",
        nom: "Apprendre à relâcher",
        categorie: Categorie::Main,
        realisation: Realisation::Autonome,
        objectif: "Avec la spasticité, relâcher est plus difficile que serrer : c'est le relâchement qu'on entraîne.",
        etapes: &[
            "Posez la main droite sur votre cuisse ou sur une serviette roulée, sans rien tenir.",
            "Avec la main gauche, bercez doucement l'avant-bras droit, comme pour l'endormir.",
            "Soufflez lentement en imaginant la main qui fond, doigt par doigt.",
            "Si la main se referme, ne luttez pas : reprenez le bercement, puis rouvrez-la doucement.",
        ],
        dosage: "Environ 2 minutes",
        duree_sec: 120,
        position: Position::Assis,
    },
    // ————————————————— Bras et épaule —————————————————
    Exercice {
        id: "bras-glisser-table",
        nom: "Glisser sur la table",
        categorie: Categorie::Bras,
        realisation: Realisation::Autonome,
        objectif: "Mobiliser l'épaule et le coude droits en douceur.",
        etapes: &[
            "Installé face à une table, posez la main droite sur un linge.",
            "Avec la main gauche par-dessus la droite, faites glisser le linge vers l'avant.",
            "Allez aussi loin que confortable, sans décoller le dos du dossier.",
            "Revenez lentement vers vous.",
        ],
        dosage: "8 allers-retours lents",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "bras-elevation",
        nom: "Élévation mains croisées",
        categorie: Categorie::Bras,
        realisation: Realisation::Autonome,
        objectif: "Lever le bras droit avec l'aide du gauche, sans forcer l'épaule.",
        etapes: &[
            "Croisez les doigts, ou tenez le poignet droit avec la main gauche.",
            "Le bras gauche guide : montez lentement les deux bras vers l'avant, puis vers le haut.",
            "Montez seulement jusqu'où c'est confortable pour l'épaule droite.",
            "Redescendez encore plus lentement.",
        ],
        dosage: "8 montées lentes",
        duree_sec: 160,
        position: Position::Assis,
    },
    Exercice {
        id: "bras-coude",
        nom: "Coude plié, coude tendu",
        categorie: Categorie::Bras,
        realisation: Realisation::Autonome,
        objectif: "Entretenir la souplesse du coude droit.",
        etapes: &[
            "Bras droit posé sur les genoux ou une table.",
            "Avec la main gauche, pliez doucement le coude droit, la main vers l'épaule.",
            "Puis étendez-le doucement, le plus droit possible sans douleur.",
            "Respirez calmement pendant tout le mouvement.",
        ],
        dosage: "8 répétitions lentes",
        duree_sec: 140,
        position: Position::Assis,
    },
    Exercice {
        id: "bras-epaules",
        nom: "Épaules qui roulent",
        categorie: Categorie::Bras,
        realisation: Realisation::Autonome,
        objectif: "Détendre le cou et les deux épaules.",
        etapes: &[
            "Bien calé au fond du siège, bras relâchés.",
            "Haussez doucement les épaules vers les oreilles, puis relâchez.",
            "Roulez ensuite les épaules vers l'arrière, en cercles lents.",
        ],
        dosage: "10 mouvements lents",
        duree_sec: 90,
        position: Position::Assis,
    },
    // ————————————————— Tronc et posture —————————————————
    Exercice {
        id: "tronc-appuis",
        nom: "Soulagement des appuis",
        categorie: Categorie::Tronc,
        realisation: Realisation::Autonome,
        objectif: "Décharger régulièrement les points d'appui : c'est la meilleure prévention des rougeurs et des escarres.",
        etapes: &[
            "Freins du fauteuil bloqués, mains sur les accoudoirs.",
            "Poussez sur le bras gauche pour décoller légèrement la fesse droite, quelques secondes.",
            "Reposez-vous, puis penchez-vous doucement de l'autre côté pour soulager la fesse gauche.",
            "Restez toujours dans une amplitude petite et sûre : on cherche à soulager, pas à se pencher loin.",
        ],
        dosage: "3 fois de chaque côté, tenir 5 s",
        duree_sec: 120,
        position: Position::Assis,
    },
    Exercice {
        id: "tronc-bascule",
        nom: "Bascule du bassin",
        categorie: Categorie::Tronc,
        realisation: Realisation::Autonome,
        objectif: "Assouplir le bas du dos et retrouver le contrôle du bassin, base de la stabilité assise.",
        etapes: &[
            "Dos bien soutenu par le dossier, mains posées sur les cuisses.",
            "Basculez doucement le bassin vers l'arrière : le bas du dos s'arrondit.",
            "Puis basculez-le vers l'avant : le bas du dos se creuse légèrement.",
            "Mouvement lent et de petite amplitude ; le buste bouge à peine.",
        ],
        dosage: "10 bascules lentes",
        duree_sec: 130,
        position: Position::Assis,
    },
    Exercice {
        id: "tronc-rotation",
        nom: "Rotation douce du buste",
        categorie: Categorie::Tronc,
        realisation: Realisation::Autonome,
        objectif: "Garder de la mobilité dans le tronc, sans jamais se déséquilibrer.",
        etapes: &[
            "Dos soutenu, mains posées à plat sur les cuisses.",
            "Tournez lentement les épaules vers la gauche, en gardant le bassin immobile.",
            "Revenez au centre, puis tournez vers la droite.",
            "Gardez toujours un contact avec le dossier : c'est votre sécurité.",
        ],
        dosage: "8 rotations de chaque côté",
        duree_sec: 130,
        position: Position::Assis,
    },
    Exercice {
        id: "tronc-grandir",
        nom: "Se grandir",
        categorie: Categorie::Tronc,
        realisation: Realisation::Autonome,
        objectif: "Lutter contre l'affaissement du buste, fréquent quand on passe la journée assis.",
        etapes: &[
            "Dos soutenu, pieds posés sur les repose-pieds.",
            "Inspirez en imaginant un fil qui tire le sommet de votre tête vers le plafond.",
            "Le buste se redresse, les épaules descendent, le menton reste horizontal.",
            "Tenez 5 secondes en respirant, puis relâchez sans vous affaisser d'un coup.",
        ],
        dosage: "8 redressements, tenir 5 s",
        duree_sec: 120,
        position: Position::Assis,
    },
    // ————————————————— Jambes et bassin —————————————————
    Exercice {
        id: "jambe-genou",
        nom: "Extension du genou droit",
        categorie: Categorie::Jambe,
        realisation: Realisation::Autonome,
        objectif: "Entretenir la cuisse droite et la mobilité du genou, utiles pour les transferts.",
        etapes: &[
            "Bien calé au fond du siège, dos soutenu.",
            "Tendez doucement le genou droit pour lever le pied vers l'avant.",
            "Si la jambe ne monte pas seule, passez une serviette sous le mollet et aidez avec la main gauche.",
            "Redescendez lentement, sans laisser tomber le pied.",
        ],
        dosage: "8 extensions, même petites",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "jambe-cheville",
        nom: "Cheville en mouvement",
        categorie: Categorie::Jambe,
        realisation: Realisation::Autonome,
        objectif: "Garder la cheville souple et faire circuler le sang : essentiel quand on reste assis toute la journée.",
        etapes: &[
            "Pied droit posé à plat, ou sur le repose-pied.",
            "Essayez de relever la pointe du pied vers vous, puis de la pointer vers le bas.",
            "Si besoin, passez une serviette sous l'avant du pied et tirez doucement avec la main gauche.",
            "Terminez par des cercles lents avec la cheville, dans un sens puis dans l'autre.",
        ],
        dosage: "10 mouvements, puis 5 cercles",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "jambe-alternance",
        nom: "Alternance des appuis",
        categorie: Categorie::Jambe,
        realisation: Realisation::Autonome,
        objectif: "Réveiller l'alternance droite-gauche des jambes, qui entretient la commande motrice.",
        etapes: &[
            "Bien calé, pieds posés à plat.",
            "Décollez légèrement le talon droit, reposez-le. Puis le talon gauche.",
            "Alternez lentement, au rythme de votre respiration.",
            "Si le pied droit bouge peu, l'intention de le décoller compte déjà.",
        ],
        dosage: "Environ 1 minute 30",
        duree_sec: 90,
        position: Position::Assis,
    },
    Exercice {
        id: "cheville-eversion",
        nom: "Réveil du bord externe du pied",
        categorie: Categorie::Jambe,
        realisation: Realisation::Autonome,
        objectif: "Solliciter les muscles qui tournent le pied vers l'extérieur : ce sont eux qui manquent quand le pied part en varus.",
        etapes: &[
            "Pied droit posé à plat, ou soutenu par la main gauche.",
            "Frottez vivement le bord externe de la jambe, sous le genou, une dizaine de secondes : cela réveille les muscles endormis.",
            "Essayez ensuite de tourner la plante du pied vers l'extérieur, comme pour en montrer le bord externe.",
            "Même sans mouvement visible, l'intention compte : ce sont précisément ces muscles-là qu'il faut réapprendre à commander.",
            "Accompagnez le mouvement avec la main gauche et tenez la position 10 secondes.",
        ],
        dosage: "8 essais, tenir 10 s",
        duree_sec: 150,
        position: Position::Assis,
    },
    Exercice {
        id: "cheville-attelle",
        nom: "Préparer le pied avant l'attelle de nuit",
        categorie: Categorie::Jambe,
        realisation: Realisation::Autonome,
        objectif: "Une attelle se supporte beaucoup mieux sur un pied réchauffé, massé et déjà étiré.",
        etapes: &[
            "Massez le mollet puis le bord interne de la jambe pendant quelques minutes.",
            "Faites étirer la cheville avant d'enfiler l'attelle, ou étirez-la vous-même avec une sangle passée sous l'avant du pied.",
            "Vérifiez qu'aucun pli de chaussette ni couture ne reste sous le talon ou contre les malléoles.",
            "Ne forcez jamais le pied dans l'attelle : s'il ne rentre pas, c'est qu'il faut étirer davantage avant.",
            "Les premiers soirs, ne la gardez qu'une heure ou deux, puis augmentez progressivement sur une à deux semaines.",
        ],
        dosage: "Environ 4 minutes, chaque soir",
        duree_sec: 240,
        position: Position::Assis,
    },
    Exercice {
        id: "cheville-peau",
        nom: "Contrôle de la peau après l'attelle",
        categorie: Categorie::Jambe,
        realisation: Realisation::Autonome,
        objectif: "Repérer tôt un point de compression : c'est la précaution la plus importante avec une attelle.",
        etapes: &[
            "Au retrait de l'attelle, regardez le talon, les deux malléoles et tout le bord externe du pied.",
            "Servez-vous d'un miroir ou de l'appareil photo du téléphone pour les zones difficiles à voir.",
            "Une rougeur qui s'efface en quelques minutes est normale.",
            "Une rougeur qui persiste plus de vingt à trente minutes, une cloque ou une zone chaude : arrêtez l'attelle et signalez-le sans attendre.",
            "Regardez aussi, dans la journée, le bord du pied qui repose sur le repose-pied.",
        ],
        dosage: "Chaque matin, moins d'une minute",
        duree_sec: 60,
        position: Position::Assis,
    },
    Exercice {
        id: "jambe-pont",
        nom: "Pont tout doux",
        categorie: Categorie::Jambe,
        realisation: Realisation::Autonome,
        objectif: "Renforcer les fessiers et le bas du dos, ce qui facilite les transferts et l'installation au lit.",
        etapes: &[
            "Allongé sur le dos, genoux pliés, pieds à plat (aidez le pied droit avec la main gauche si besoin).",
            "Soulevez légèrement le bassin, juste quelques centimètres.",
            "Tenez 3 secondes, puis reposez très lentement.",
            "À faire uniquement bien installé au centre du lit, jamais près du bord.",
        ],
        dosage: "5 répétitions douces",
        duree_sec: 120,
        position: Position::Allonge,
    },
    // ————————————————— Électrodes —————————————————
    // Principe : on stimule les muscles AFFAIBLIS (ceux qui ouvrent la
    // main, ceux qui relèvent le pied), jamais les muscles spastiques.
    // Stimuler l'antagoniste renforce le muscle faible et fait baisser
    // en retour le tonus du muscle trop contracté.
    Exercice {
        id: "electrodes-main-pose",
        nom: "Main : poser les électrodes",
        categorie: Categorie::Electrodes,
        realisation: Realisation::Autonome,
        objectif: "Placer les électrodes sur les muscles qui ouvrent la main, à l'arrière de l'avant-bras.",
        etapes: &[
            "Appareil éteint. Peau propre et sèche, sans crème ni huile.",
            "Posez l'avant-bras droit sur une table, paume vers le bas, bien soutenu.",
            "Première électrode : à environ 5 cm sous le pli du coude, sur le DESSUS de l'avant-bras, côté externe.",
            "Deuxième électrode : 8 à 10 cm plus bas, dans le même axe, toujours sur le dessus de l'avant-bras.",
            "Appuyez sur toute la surface de chaque électrode pour qu'elle adhère bien à plat.",
            "Jamais du côté de la paume : ce sont les muscles qui ferment la main, on ne les stimule pas.",
        ],
        dosage: "Environ 3 minutes",
        duree_sec: 180,
        position: Position::Assis,
    },
    Exercice {
        id: "electrodes-main-seance",
        nom: "Main : séance d'électrostimulation",
        categorie: Categorie::Electrodes,
        realisation: Realisation::Autonome,
        objectif: "Faire travailler les extenseurs pour ouvrir la main, en accompagnant chaque contraction de votre propre intention.",
        etapes: &[
            "Montez l'intensité très progressivement, en partant de zéro.",
            "Le bon repère : le poignet et les doigts se relèvent et la main s'ouvre. Si les doigts se referment au contraire, les électrodes sont du mauvais côté : éteignez et repositionnez-les.",
            "L'intensité juste est celle qui donne une contraction visible et confortable. Jamais douloureuse.",
            "À chaque fois que le courant monte, essayez d'ouvrir la main vous-même, en même temps.",
            "C'est cette association du courant et de votre intention qui fait progresser : la stimulation seule, passive, apporte beaucoup moins.",
            "Entre deux contractions, laissez la main se reposer complètement.",
        ],
        dosage: "15 minutes, une à deux fois par jour",
        duree_sec: 900,
        position: Position::Assis,
    },
    Exercice {
        id: "electrodes-pied-pose",
        nom: "Pied : poser les électrodes",
        categorie: Categorie::Electrodes,
        realisation: Realisation::Autonome,
        objectif: "Placer les électrodes sur les muscles qui relèvent le pied et le tournent vers l'extérieur.",
        etapes: &[
            "Appareil éteint, jambe droite soutenue, peau propre et sèche.",
            "Repérez l'arête de l'os de la jambe, le tibia, sur le devant.",
            "Première électrode : 4 à 5 cm sous le genou, juste à l'EXTÉRIEUR de cette arête, sur le muscle.",
            "Deuxième électrode : 10 à 12 cm plus bas, toujours à l'extérieur de l'arête.",
            "Contre le varus, décalez la deuxième électrode un peu plus vers le côté externe de la jambe : le pied doit se relever en tournant légèrement vers l'extérieur, pas vers l'intérieur.",
            "Jamais sur le mollet : c'est le muscle qui tire le pied en pointe, on ne le stimule pas.",
            "Si vous n'atteignez pas confortablement votre jambe, faites poser les électrodes par la personne qui vous accompagne.",
        ],
        dosage: "Environ 3 minutes",
        duree_sec: 180,
        position: Position::Assis,
    },
    Exercice {
        id: "electrodes-pied-seance",
        nom: "Pied : séance d'électrostimulation",
        categorie: Categorie::Electrodes,
        realisation: Realisation::Autonome,
        objectif: "Entretenir les releveurs du pied et lutter contre l'installation du varus équin.",
        etapes: &[
            "Montez l'intensité très progressivement, en partant de zéro.",
            "Le bon repère : la pointe du pied se relève vers le tibia. Idéalement, le bord externe monte aussi.",
            "Si le pied se relève en partant vers l'intérieur, éteignez et décalez les électrodes plus vers l'extérieur de la jambe.",
            "Accompagnez chaque montée du courant en essayant vous-même de relever le pied.",
            "Intensité confortable, contraction visible, jamais de douleur.",
            "Si une crampe s'installe dans le mollet, arrêtez la séance et massez avant de reprendre plus tard.",
        ],
        dosage: "15 minutes, une à deux fois par jour",
        duree_sec: 900,
        position: Position::Assis,
    },
    Exercice {
        id: "electrodes-apres",
        nom: "Après la séance : peau et électrodes",
        categorie: Categorie::Electrodes,
        realisation: Realisation::Autonome,
        objectif: "Éviter les brûlures et faire durer les électrodes : deux minutes qui évitent bien des ennuis.",
        etapes: &[
            "Éteignez toujours l'appareil AVANT de décoller les électrodes.",
            "Regardez la peau sous chaque électrode. Une rougeur légère qui s'efface en quelques minutes est normale.",
            "Une rougeur qui persiste, une cloque, une sensation de brûlure : on arrête et on le signale.",
            "Remettez les électrodes sur leur film plastique et conservez-les au frais.",
            "Remplacez-les dès qu'elles n'adhèrent plus partout : une électrode qui décolle concentre le courant sur un point et peut brûler.",
            "Ces électrodes sont personnelles et ne se partagent pas.",
        ],
        dosage: "Après chaque séance, environ 2 minutes",
        duree_sec: 120,
        position: Position::Assis,
    },
    // ————————————————— Renforcement musculaire —————————————————
    // Après une longue immobilité, les deux côtés ont fondu. Le côté
    // gauche, seulement désentraîné, se renforce vite et porte tout :
    // transferts, poussées, assistance au côté droit. Le côté droit,
    // lui, s'entretient sans jamais être poussé dans la spasticité.
    Exercice {
        id: "force-gauche-poussees",
        nom: "Poussées sur les accoudoirs",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Renforcer le bras gauche, celui qui assure tous vos transferts.",
        etapes: &[
            "Freins bloqués, dos bien calé, main gauche à plat sur l'accoudoir.",
            "Poussez sur le bras gauche pour vous soulever de quelques centimètres seulement, ou simplement pour décharger l'appui.",
            "Soufflez pendant l'effort : ne bloquez jamais votre respiration.",
            "Tenez 3 secondes, puis reposez-vous complètement 10 secondes.",
            "Arrêtez la série quand le mouvement devient difficile à contrôler, pas quand vous n'en pouvez plus.",
        ],
        dosage: "3 séries de 6 poussées",
        duree_sec: 240,
        position: Position::Assis,
    },
    Exercice {
        id: "force-gauche-tirage",
        nom: "Tirage à l'élastique",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Renforcer le dos et contrer l'enroulement des épaules, inévitable en fauteuil.",
        etapes: &[
            "Fixez un élastique à une poignée de porte, ou passez-le autour d'un montant solide, à hauteur de poitrine.",
            "Tenez-le dans la main gauche, bras tendu devant vous.",
            "Tirez le coude vers l'arrière en serrant l'omoplate, sans hausser l'épaule.",
            "Revenez lentement : c'est le retour freiné qui muscle le plus.",
            "Soufflez pendant que vous tirez.",
        ],
        dosage: "3 séries de 10 tirages",
        duree_sec: 240,
        position: Position::Assis,
    },
    Exercice {
        id: "force-gauche-rotateurs",
        nom: "Rotateurs de l'épaule gauche",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Protéger l'épaule gauche, très sollicitée en fauteuil et exposée à l'usure.",
        etapes: &[
            "Coude gauche collé au corps, plié à angle droit, avant-bras horizontal.",
            "Tenez un élastique léger, son autre extrémité fixée à votre droite.",
            "Sans décoller le coude du corps, tournez l'avant-bras vers l'extérieur, lentement.",
            "Revenez encore plus lentement.",
            "Amplitude modérée et résistance légère : ces muscles sont petits, ils s'entraînent en finesse, jamais en force.",
        ],
        dosage: "3 séries de 12, résistance légère",
        duree_sec: 240,
        position: Position::Assis,
    },
    Exercice {
        id: "force-gauche-coude",
        nom: "Flexion du coude gauche",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Renforcer le bras gauche pour tout ce qu'il porte au quotidien.",
        etapes: &[
            "Assis, dos soutenu, bras gauche le long du corps.",
            "Tenez une charge : petit haltère, bouteille d'eau, ou simplement le poids de votre bras au début.",
            "Pliez le coude en comptant jusqu'à 2, puis redescendez en comptant jusqu'à 4.",
            "La descente freinée est ce qui construit le muscle : ne laissez jamais retomber la charge.",
            "Soufflez pendant la montée.",
        ],
        dosage: "3 séries de 10",
        duree_sec: 240,
        position: Position::Assis,
    },
    Exercice {
        id: "force-gauche-poigne",
        nom: "Renforcement de la poigne",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "La force de préhension de la main gauche conditionne vos transferts et votre sécurité.",
        etapes: &[
            "Serrez une balle souple ou une serviette roulée dans la main gauche.",
            "Serrez fort 5 secondes, puis relâchez complètement 5 secondes.",
            "Variez : serrage à pleine main, puis pincement entre le pouce et chaque doigt.",
            "Terminez en ouvrant grand la main et en écartant les doigts.",
        ],
        dosage: "10 serrages de 5 s",
        duree_sec: 180,
        position: Position::Assis,
    },
    Exercice {
        id: "force-droite-isometrique",
        nom: "Contractions du bras droit",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Entretenir le muscle du côté atteint sans réveiller la spasticité.",
        etapes: &[
            "Bras droit posé et bien soutenu, épaule détendue.",
            "Essayez de contracter le muscle sans produire de mouvement : une contraction retenue, quelques secondes.",
            "Trois secondes de contraction, puis dix secondes de repos complet.",
            "Si le bras se raidit ou se replie, la spasticité prend le dessus : arrêtez, soufflez, massez, reprenez plus doucement.",
            "Même une contraction à peine perceptible compte : c'est la commande que l'on entretient.",
        ],
        dosage: "8 contractions de 3 s",
        duree_sec: 200,
        position: Position::Assis,
    },
    Exercice {
        id: "force-droite-deux-mains",
        nom: "Poussée à deux mains",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Faire travailler le bras droit à l'effort, guidé et soutenu par le gauche.",
        etapes: &[
            "Doigts croisés ou mains jointes, avant-bras posés sur une table.",
            "Poussez les deux bras vers l'avant, le gauche menant le mouvement et le droit l'accompagnant.",
            "Allez lentement, sans à-coup, et revenez encore plus lentement.",
            "Demandez au bras droit de participer à chaque poussée, même un peu.",
            "Soufflez pendant la poussée.",
        ],
        dosage: "3 séries de 8 poussées",
        duree_sec: 220,
        position: Position::Assis,
    },
    Exercice {
        id: "force-tronc-gainage",
        nom: "Gainage assis",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Renforcer la ceinture qui vous tient assis : c'est elle qui rend les transferts sûrs.",
        etapes: &[
            "Freins bloqués. Décollez le dos du dossier de quelques centimètres seulement, mains sur les accoudoirs, prêtes à vous rattraper.",
            "Serrez le ventre comme pour rentrer le nombril, sans bloquer la respiration.",
            "Tenez 5 secondes en respirant normalement, puis reposez-vous contre le dossier.",
            "Si l'équilibre est incertain, gardez le contact avec le dossier et contractez simplement le ventre : le muscle travaille quand même.",
            "Jamais près d'un bord, jamais sans les freins.",
        ],
        dosage: "8 tenues de 5 s",
        duree_sec: 200,
        position: Position::Assis,
    },
    Exercice {
        id: "force-tronc-resistance",
        nom: "Tronc contre résistance",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Renforcer le tronc dans toutes les directions, sans jamais se déséquilibrer.",
        etapes: &[
            "Dos soutenu, main gauche posée à plat contre votre cuisse ou contre l'accoudoir.",
            "Poussez le tronc contre votre propre main, qui résiste : vers l'avant, puis sur le côté.",
            "Rien ne bouge : c'est un bras de fer contre vous-même, tout en retenue.",
            "Trois secondes de poussée, cinq secondes de repos, en soufflant pendant l'effort.",
            "L'amplitude étant nulle, il n'y a aucun risque de basculer.",
        ],
        dosage: "6 poussées dans chaque direction",
        duree_sec: 220,
        position: Position::Assis,
    },
    Exercice {
        id: "force-jambes-isometrique",
        nom: "Contractions des cuisses",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Entretenir la masse des cuisses, qui fond vite lorsqu'on ne marche plus.",
        etapes: &[
            "Assis, pieds posés, dos calé.",
            "Contractez la cuisse gauche comme pour tendre le genou, sans bouger le pied. Tenez 5 secondes.",
            "Faites de même à droite : même si le muscle répond peu, la contraction et l'intention comptent.",
            "Relâchez complètement entre chaque contraction.",
            "Cette contraction sans mouvement entretient aussi l'os, qui se déminéralise quand on ne marche plus.",
        ],
        dosage: "10 contractions de 5 s par jambe",
        duree_sec: 260,
        position: Position::Assis,
    },
    Exercice {
        id: "force-jambes-elastique",
        nom: "Extension de genou contre élastique",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Renforcer la cuisse avec une résistance progressive, réglable au plus juste.",
        etapes: &[
            "Passez un élastique léger autour des deux chevilles, ou autour d'un pied et d'un montant du fauteuil.",
            "Tendez lentement le genou gauche contre la résistance, puis revenez en freinant.",
            "À droite, accompagnez le mouvement avec la main gauche, ou allégez l'élastique jusqu'à ce que le mouvement devienne possible.",
            "Jamais d'à-coup : l'élastique reste tendu du début à la fin.",
            "N'augmentez la résistance que lorsque douze répétitions deviennent faciles.",
        ],
        dosage: "3 séries de 10 par jambe",
        duree_sec: 280,
        position: Position::Assis,
    },
    Exercice {
        id: "force-endurance",
        nom: "Endurance des bras et du souffle",
        categorie: Categorie::Force,
        realisation: Realisation::Autonome,
        objectif: "Retrouver du souffle et de l'endurance, perdus aussi sûrement que le muscle.",
        etapes: &[
            "Assis, dos calé, bras libres.",
            "Enchaînez des mouvements amples et continus du bras gauche : vers l'avant, vers le haut, sur le côté, comme une nage lente.",
            "Emmenez le bras droit dans le mouvement, mains jointes, s'il ne suit pas seul.",
            "Gardez un rythme où vous pouvez encore parler : c'est le bon repère d'intensité.",
            "Deux minutes d'effort, une minute de repos, puis on recommence.",
        ],
        dosage: "3 fois 2 minutes",
        duree_sec: 300,
        position: Position::Assis,
    },
    Exercice {
        id: "force-aide-droite",
        nom: "Renforcement guidé du côté droit",
        categorie: Categorie::Force,
        realisation: Realisation::TiercePersonne,
        objectif: "Une résistance dosée à la main permet de faire travailler le côté droit là où aucun élastique ne le peut.",
        etapes: &[
            "Personne aidante : massez d'abord le membre deux minutes. La spasticité baisse, et le muscle répond bien mieux.",
            "Placez votre main de façon à offrir une résistance douce au mouvement que la personne tente.",
            "Demandez un effort de trois secondes, puis un relâchement complet de dix secondes.",
            "Adaptez la résistance en continu : elle doit permettre au mouvement d'exister, jamais le bloquer.",
            "Arrêtez dès que le membre se raidit : c'est la spasticité qui prend le dessus, pas le muscle qui travaille.",
        ],
        dosage: "8 efforts par mouvement",
        duree_sec: 300,
        position: Position::Assis,
    },
    // ————————————————— Avec une tierce personne (le soir) —————————————————
    Exercice {
        id: "aide-massage-bras",
        nom: "Massage complet du bras droit",
        categorie: Categorie::Massage,
        realisation: Realisation::TiercePersonne,
        objectif: "Faire baisser le tonus de tout le membre avant les mobilisations : on masse toujours avant de mobiliser.",
        etapes: &[
            "Personne aidante : installez le bras droit posé et soutenu, la personne bien calée.",
            "Massez de la main vers l'épaule, par pressions larges et lentes, jamais dans l'autre sens.",
            "Insistez sur l'avant-bras, où les muscles fléchisseurs sont les plus contractés.",
            "Terminez par la main : paume, dos de la main, puis chaque doigt.",
            "Demandez régulièrement si la pression est confortable.",
        ],
        dosage: "Environ 5 minutes",
        duree_sec: 300,
        position: Position::Assis,
    },
    Exercice {
        id: "aide-epaule",
        nom: "Mobilisation passive de l'épaule",
        categorie: Categorie::Bras,
        realisation: Realisation::TiercePersonne,
        objectif: "Entretenir l'amplitude de l'épaule et prévenir l'enraidissement, très fréquent du côté atteint.",
        etapes: &[
            "Personne aidante : une main soutient le coude, l'autre tient l'avant-bras. Ne tirez jamais par la main seule.",
            "Montez lentement le bras vers l'avant, jusqu'à la limite confortable, sans jamais forcer.",
            "Redescendez tout aussi lentement.",
            "Puis écartez doucement le bras sur le côté, dans une amplitude modérée.",
            "Arrêtez immédiatement en cas de douleur : l'épaule du côté atteint est fragile.",
        ],
        dosage: "8 mouvements lents dans chaque direction",
        duree_sec: 240,
        position: Position::Assis,
    },
    Exercice {
        id: "aide-coude-poignet",
        nom: "Mobilisation du coude et du poignet",
        categorie: Categorie::Bras,
        realisation: Realisation::TiercePersonne,
        objectif: "Conserver la souplesse du coude et du poignet droits.",
        etapes: &[
            "Personne aidante : soutenez le bras, une main au-dessus du coude, l'autre au poignet.",
            "Pliez et tendez le coude lentement, dix fois.",
            "Puis tournez doucement l'avant-bras : paume vers le haut, paume vers le bas.",
            "Terminez par le poignet : fléchissez-le et étendez-le doucement, en tenant chaque position 15 secondes.",
        ],
        dosage: "10 mouvements, puis tenues de 15 s",
        duree_sec: 240,
        position: Position::Assis,
    },
    Exercice {
        id: "aide-doigts",
        nom: "Étirement prolongé des doigts",
        categorie: Categorie::Main,
        realisation: Realisation::TiercePersonne,
        objectif: "L'étirement long est ce qui calme le mieux la spasticité — plus efficace le soir, après le massage.",
        etapes: &[
            "Personne aidante : massez d'abord la main et l'avant-bras pendant deux bonnes minutes.",
            "Pliez légèrement le poignet vers l'avant : les doigts se desserrent d'eux-mêmes.",
            "Ouvrez les doigts un par un, en commençant par le pouce, très lentement.",
            "Main ouverte, maintenez l'étirement 60 secondes en redressant progressivement le poignet.",
            "Ne forcez jamais contre une résistance : attendez, le muscle finit par céder.",
        ],
        dosage: "3 étirements de 60 s",
        duree_sec: 300,
        position: Position::Assis,
    },
    Exercice {
        id: "aide-hanche-genou",
        nom: "Mobilisation de la hanche et du genou",
        categorie: Categorie::Jambe,
        realisation: Realisation::TiercePersonne,
        objectif: "Entretenir les articulations des jambes, peu sollicitées en fauteuil, et limiter l'enraidissement.",
        etapes: &[
            "À faire allongé sur le dos, bien installé au centre du lit.",
            "Personne aidante : une main sous le genou, l'autre sous le talon.",
            "Ramenez lentement le genou vers la poitrine, dans la limite du confort, puis rallongez la jambe.",
            "Répétez dix fois, sans à-coup, en soutenant toujours le poids de la jambe.",
            "Faites les deux jambes : la gauche en profite aussi.",
        ],
        dosage: "10 mouvements par jambe",
        duree_sec: 300,
        position: Position::Allonge,
    },
    Exercice {
        id: "aide-cheville-mollet",
        nom: "Étirement du pied en varus équin",
        categorie: Categorie::Jambe,
        realisation: Realisation::TiercePersonne,
        objectif: "Étirer l'équin ET le varus : remonter le pied ne suffit pas, il faut aussi tourner la plante vers l'extérieur.",
        etapes: &[
            "Allongé, jambe soutenue. Massez d'abord le mollet et le bord interne de la jambe deux à trois minutes : un muscle réchauffé s'étire bien mieux.",
            "Personne aidante : une main tient le talon par en dessous, l'autre se pose à plat sous l'avant du pied.",
            "Tirez d'abord le talon vers le bas, dans l'axe de la jambe. C'est ce geste qui décoince l'équin, avant même de remonter le pied.",
            "Puis remontez lentement l'avant du pied vers le tibia en tournant légèrement la plante vers l'extérieur. Sans cette rotation, on n'étire que la moitié du problème.",
            "Maintenez 60 secondes, genou tendu, en respirant calmement.",
            "Recommencez genou plié : les deux positions étirent des muscles différents du mollet, les deux sont nécessaires.",
            "Jamais d'à-coup, jamais de douleur. Si ça résiste, gardez la position et attendez que ça cède.",
        ],
        dosage: "3 fois genou tendu, 3 fois genou plié, 60 s",
        duree_sec: 420,
        position: Position::Allonge,
    },
    Exercice {
        id: "aide-drainage-jambes",
        nom: "Massage drainant des jambes",
        categorie: Categorie::Massage,
        realisation: Realisation::TiercePersonne,
        objectif: "Faire circuler et limiter le gonflement des jambes et des pieds, très fréquent après une journée assis.",
        etapes: &[
            "Allongé, jambes légèrement surélevées sur un coussin.",
            "Personne aidante : remontez à deux mains, de la cheville vers le genou, par pressions lentes.",
            "Continuez du genou vers la cuisse, toujours vers le cœur.",
            "Terminez par les pieds : massez la plante du talon vers les orteils, puis mobilisez chaque orteil.",
        ],
        dosage: "Environ 5 minutes",
        duree_sec: 300,
        position: Position::Allonge,
    },
    Exercice {
        id: "aide-installation-nuit",
        nom: "Installation pour la nuit",
        categorie: Categorie::Tronc,
        realisation: Realisation::TiercePersonne,
        objectif: "Une bonne installation prolonge l'effet des étirements pendant toute la nuit.",
        etapes: &[
            "Personne aidante : installez le bras droit posé sur un coussin, légèrement écarté du corps, main ouverte si possible.",
            "Évitez que la main reste serrée sous le corps ou coincée contre le flanc.",
            "Placez un coussin sous le mollet droit pour que le talon ne porte pas directement sur le matelas.",
            "Vérifiez que le pied droit n'est pas tourné vers l'intérieur ni figé en pointe.",
            "Demandez confirmation que la position est confortable avant de quitter la pièce.",
        ],
        dosage: "Environ 3 minutes",
        duree_sec: 180,
        position: Position::Allonge,
    },
];

fn par_id(id: &str) -> &'static Exercice {
    EXERCICES
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("Exercice inconnu : {id}"))
}

fn liste(ids: &[&str]) -> Vec<&'static Exercice> {
    ids.iter().map(|id| par_id(id)).collect()
}

#[derive(Clone, PartialEq, Debug)]
pub struct Seance {
    pub titre: &'static str,
    pub description: &'static str,
    pub exercices: Vec<&'static Exercice>,
    pub realisation: Realisation,
}

impl Seance {
    /// Un exercice seul, lancé depuis le catalogue.
    pub fn a_la_carte(ex: &'static Exercice) -> Self {
        Seance {
            titre: ex.nom,
            description: "Exercice à la carte",
            realisation: ex.realisation,
            exercices: vec![ex],
        }
    }

    pub fn duree_sec(&self) -> u32 {
        self.exercices.iter().map(|e| e.duree_sec).sum()
    }

    /// Durée totale arrondie à la minute.
    pub fn duree_totale_min(&self) -> u32 {
        (self.duree_sec() + 30) / 60
    }
}

/// Séance du jour, réalisable seul. Chaque séance commence par la
/// détente puis un massage : sur un membre spastique, c'est la
/// meilleure préparation avant de chercher à bouger.
///
/// `jour_semaine` : 0 = dimanche, 1 = lundi… 6 = samedi.
pub fn seance_du_jour(jour_semaine: u32) -> Seance {
    let (titre, description, ids): (&str, &str, &[&str]) = match jour_semaine {
        // lundi
        1 => (
            "Main et ouverture",
            "On masse la main, puis on travaille l'ouverture des doigts, tout en lenteur.",
            &[
                "detente-respiration",
                "massage-main",
                "massage-pouce",
                "main-tenodese",
                "main-ouverture",
                "main-extension-active",
            ],
        ),
        // jeudi
        4 => (
            "Main et poignet",
            "On détend l'avant-bras, puis on réapprend le mouvement du poignet.",
            &[
                "detente-respiration",
                "massage-avant-bras",
                "main-poignet-actif",
                "main-tenodese",
                "main-ouverture",
                "main-appui-paume",
            ],
        ),
        // mardi
        2 => (
            "Renforcement du haut du corps",
            "On reconstruit la force du bras gauche, celui qui porte tout, et on entretient le droit.",
            &[
                "detente-respiration",
                "force-gauche-poussees",
                "force-gauche-tirage",
                "force-gauche-rotateurs",
                "force-droite-isometrique",
            ],
        ),
        // vendredi
        5 => (
            "Bras et épaule",
            "Le bras droit bouge en douceur, guidé par le gauche.",
            &[
                "detente-respiration",
                "massage-avant-bras",
                "massage-epaule",
                "bras-glisser-table",
                "bras-elevation",
                "main-ouverture",
            ],
        ),
        // samedi
        6 => (
            "Renforcement du tronc et des jambes",
            "Gainage, cuisses et endurance : ce qui rend les transferts plus sûrs et les journées moins lourdes.",
            &[
                "detente-respiration",
                "force-tronc-gainage",
                "force-tronc-resistance",
                "force-jambes-isometrique",
                "force-endurance",
            ],
        ),
        // mercredi
        3 => (
            "Tronc et jambes",
            "Posture, appuis et souplesse des jambes, entièrement en sécurité dans le fauteuil.",
            &[
                "detente-respiration",
                "tronc-appuis",
                "tronc-bascule",
                "massage-mollet",
                "jambe-cheville",
                "cheville-eversion",
            ],
        ),
        // dimanche
        _ => (
            "Massage et détente",
            "Journée douce : on masse tout le côté droit, sans rien forcer.",
            &[
                "detente-respiration",
                "massage-main",
                "massage-avant-bras",
                "massage-drainage",
                "massage-epaule",
                "main-miroir",
            ],
        ),
    };
    Seance {
        titre,
        description,
        realisation: Realisation::Autonome,
        exercices: liste(ids),
    }
}

/// Séance du soir, réalisée par une tierce personne. Massage d'abord,
/// mobilisations ensuite, installation pour la nuit en dernier.
pub fn seance_du_soir() -> Seance {
    Seance {
        titre: "Séance du soir",
        description: "Massages et mobilisations passives, réalisés par une tierce personne. À faire de préférence après une douche chaude.",
        realisation: Realisation::TiercePersonne,
        exercices: liste(&[
            "aide-massage-bras",
            "aide-epaule",
            "aide-coude-poignet",
            "aide-doigts",
            "aide-hanche-genou",
            "aide-cheville-mollet",
            "aide-drainage-jambes",
            "aide-installation-nuit",
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_seance_ne_cite_que_des_exercices_connus() {
        for jour in 0..7 {
            assert!(!seance_du_jour(jour).exercices.is_empty());
        }
        assert_eq!(seance_du_soir().exercices.len(), 8);
    }

    #[test]
    fn chaque_seance_commence_par_la_detente() {
        for jour in 0..7 {
            assert_eq!(seance_du_jour(jour).exercices[0].id, "detente-respiration");
        }
    }

    #[test]
    fn les_identifiants_sont_uniques() {
        for (i, a) in EXERCICES.iter().enumerate() {
            assert!(EXERCICES[i + 1..].iter().all(|b| b.id != a.id), "{}", a.id);
        }
    }

    #[test]
    fn la_seance_du_soir_est_entierement_avec_aide() {
        assert!(
            seance_du_soir()
                .exercices
                .iter()
                .all(|e| e.realisation == Realisation::TiercePersonne)
        );
    }
}
