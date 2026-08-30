//
//  Exercices.swift
//  HemiFit
//
//  Catalogue d'exercices pour hémiparésie droite avec forte
//  spasticité : tout se fait assis ou allongé, la main gauche
//  (saine) assiste le côté droit.
//
//  Règles face à la spasticité : jamais de mouvement rapide ni
//  forcé (le muscle spastique résiste d'autant plus qu'on l'étire
//  vite), des étirements lents et prolongés, de la détente avant
//  l'effort, et on entraîne le relâchement plutôt que le serrage.
//  Règle d'or : jamais de douleur.
//

import Foundation

enum CategorieExercice: String, CaseIterable, Identifiable {
    case sensoriel, main, bras, jambe

    var id: String { rawValue }

    var titre: String {
        switch self {
        case .sensoriel: "Éveil sensoriel"
        case .main: "Main & doigts"
        case .bras: "Bras & épaule"
        case .jambe: "Jambe & pré-marche"
        }
    }

    var emoji: String {
        switch self {
        case .sensoriel: "🫱"
        case .main: "✋"
        case .bras: "💪"
        case .jambe: "🦶"
        }
    }
}

enum PositionDepart: String {
    case assis, allonge = "allongé"
}

struct Exercice: Identifiable, Hashable {
    let id: String
    let nom: String
    let categorie: CategorieExercice
    /// Ce que l'exercice travaille, en une phrase.
    let objectif: String
    /// Consignes pas à pas, phrases courtes.
    let etapes: [String]
    /// Dosage lisible, ex. « 5 répétitions, tenir 10 s ».
    let dosage: String
    /// Durée guidée en secondes pour le minuteur.
    let dureeSec: Int
    let position: PositionDepart
}

struct Seance {
    let titre: String
    let description: String
    let exercices: [Exercice]

    var dureeMinutes: Int {
        max(1, exercices.reduce(0) { $0 + $1.dureeSec } / 60)
    }
}

enum Catalogue {
    static let exercices: [Exercice] = [
        // ——— Éveil sensoriel ———
        Exercice(
            id: "detente-respiration",
            nom: "Détente et respiration",
            categorie: .sensoriel,
            objectif: "Faire baisser la spasticité avant de bouger : un corps détendu s'étire beaucoup mieux.",
            etapes: [
                "Installez-vous confortablement, dos soutenu, bras droit posé sur un coussin.",
                "Inspirez lentement par le nez en comptant jusqu'à 4.",
                "Soufflez très lentement par la bouche en comptant jusqu'à 6, en laissant tomber les épaules.",
                "À chaque expiration, imaginez votre bras et votre main droite devenir lourds, chauds et mous.",
            ],
            dosage: "Environ 2 minutes de respiration lente",
            dureeSec: 120,
            position: .assis
        ),
        Exercice(
            id: "eveil-paume",
            nom: "Réveil de la main droite",
            categorie: .sensoriel,
            objectif: "Réveiller les sensations de la main droite avant de bouger.",
            etapes: [
                "Asseyez-vous confortablement, le bras droit posé sur une table ou un coussin.",
                "Avec la main gauche, frottez doucement la paume droite, du poignet vers les doigts.",
                "Massez chaque doigt l'un après l'autre, sans forcer.",
                "Terminez par de petites pressions douces sur toute la main.",
            ],
            dosage: "Environ 2 minutes, en douceur",
            dureeSec: 120,
            position: .assis
        ),
        Exercice(
            id: "eveil-avant-bras",
            nom: "Caresses de l'avant-bras",
            categorie: .sensoriel,
            objectif: "Stimuler la peau et détendre l'avant-bras droit.",
            etapes: [
                "Posez l'avant-bras droit sur vos genoux ou une table.",
                "Avec la main gauche, caressez lentement du coude jusqu'à la main.",
                "Variez : paume, dos de la main, textures différentes (tissu, éponge).",
            ],
            dosage: "Environ 1 minute 30",
            dureeSec: 90,
            position: .assis
        ),

        // ——— Main & doigts ———
        Exercice(
            id: "main-ouverture",
            nom: "Ouverture de main assistée",
            categorie: .main,
            objectif: "Ouvrir la main malgré la spasticité : très lentement, en laissant le temps aux muscles de lâcher.",
            etapes: [
                "Commencez par masser doucement l'avant-bras et la paume pour préparer la main.",
                "Astuce : penchez d'abord le poignet droit légèrement vers l'avant, les doigts se laissent ouvrir plus facilement.",
                "Avec la main gauche, dépliez TRÈS lentement les doigts droits, en commençant par le pouce.",
                "Si les doigts résistent (c'est la spasticité), ne forcez jamais : arrêtez-vous, soufflez, et attendez que ça se relâche tout seul.",
                "Quand la main est ouverte, maintenez l'ouverture 30 secondes : c'est l'étirement prolongé qui calme la spasticité.",
                "Relâchez doucement et laissez la main se reposer avant de recommencer.",
            ],
            dosage: "3 ouvertures très lentes, tenir 30 s",
            dureeSec: 180,
            position: .assis
        ),
        Exercice(
            id: "main-poignet",
            nom: "Étirement doux du poignet",
            categorie: .main,
            objectif: "Garder le poignet droit souple.",
            etapes: [
                "Coude droit posé sur la table, avant-bras vertical si possible.",
                "Avec la main gauche, amenez doucement la main droite vers l'arrière (paume vers l'avant).",
                "Si le poignet résiste, n'insistez pas : gardez la position et attendez, la spasticité cède avec la lenteur.",
                "Tenez 15 secondes en respirant calmement.",
                "Revenez au repos, puis penchez doucement la main vers l'avant.",
            ],
            dosage: "3 répétitions dans chaque sens, tenir 15 s",
            dureeSec: 160,
            position: .assis
        ),
        Exercice(
            id: "main-poignet-actif",
            nom: "Apprendre à bouger le poignet",
            categorie: .main,
            objectif: "Réapprendre au poignet droit à se plier et se redresser : la gravité fait le mouvement, vous apprenez d'abord à le retenir.",
            etapes: [
                "Posez l'avant-bras droit sur la table, la main dans le vide au bord de la table, paume vers le bas.",
                "Laissez la main pendre : la gravité plie le poignet toute seule, vous n'avez rien à faire.",
                "Avec la main gauche, remontez doucement la main droite à l'horizontale, puis laissez-la redescendre lentement.",
                "Après quelques allers-retours guidés, essayez de retenir un peu la descente, ou de remonter d'un millimètre : retenir est plus facile que soulever, c'est par là qu'on commence.",
                "Terminez en laissant la main pendre et se détendre complètement.",
            ],
            dosage: "8 allers-retours doux",
            dureeSec: 150,
            position: .assis
        ),
        Exercice(
            id: "main-tenodese",
            nom: "L'astuce du poignet plié",
            categorie: .main,
            objectif: "Utiliser un réflexe naturel : poignet plié vers l'avant, les doigts se détendent et s'ouvrent plus facilement.",
            etapes: [
                "Posez l'avant-bras droit sur la table ou votre cuisse.",
                "Avec la main gauche, pliez doucement le poignet droit vers l'avant (la main descend vers le sol). C'est la main gauche qui fait tout : le poignet droit n'a rien à faire, il se laisse porter.",
                "Vous sentirez les doigts se desserrer un peu : profitez-en pour les ouvrir doucement avec la main gauche.",
                "Doigts ouverts, redressez très lentement le poignet, sans perdre l'ouverture.",
                "Si les doigts se referment, repliez le poignet et recommencez : c'est normal, ça se gagne petit à petit.",
            ],
            dosage: "5 essais tranquilles",
            dureeSec: 150,
            position: .assis
        ),
        Exercice(
            id: "main-extension-active",
            nom: "Ouvrir avec de l'aide",
            categorie: .main,
            objectif: "Vos doigts savent se fermer : on entraîne le mouvement inverse, l'ouverture, avec assistance.",
            etapes: [
                "Main droite posée sur la cuisse, détendue.",
                "Serrez très légèrement le poing 3 secondes — ça, vous savez faire.",
                "Puis arrêtez de serrer, soufflez, et essayez d'OUVRIR les doigts, même d'un millimètre.",
                "Pendant que vous essayez, la main gauche accompagne et termine l'ouverture en douceur.",
                "L'essai compte autant que le résultat : c'est l'intention d'ouvrir qui réveille les muscles endormis.",
            ],
            dosage: "6 essais, sans forcer",
            dureeSec: 150,
            position: .assis
        ),
        Exercice(
            id: "main-miroir",
            nom: "Thérapie miroir",
            categorie: .main,
            objectif: "Tromper (gentiment) le cerveau : voir une main droite qui s'ouvre l'aide à réapprendre le mouvement.",
            etapes: [
                "Posez un miroir debout devant vous, tranche contre votre ventre, face réfléchissante vers la gauche.",
                "Cachez la main droite derrière le miroir ; regardez le reflet de la main gauche.",
                "Ouvrez et fermez lentement la main gauche en regardant le reflet : on dirait la main droite qui bouge.",
                "Pendant ce temps, essayez de faire le même mouvement avec la main droite cachée, sans forcer.",
            ],
            dosage: "Environ 3 minutes",
            dureeSec: 180,
            position: .assis
        ),
        Exercice(
            id: "main-relacher",
            nom: "Apprendre à relâcher",
            categorie: .main,
            objectif: "Avec la spasticité, relâcher est plus difficile que serrer : c'est le relâchement qu'on entraîne.",
            etapes: [
                "Posez la main droite sur votre cuisse ou sur une serviette roulée, sans rien tenir.",
                "Avec la main gauche, bercez doucement l'avant-bras droit de petits mouvements lents, comme pour l'endormir.",
                "Soufflez lentement en imaginant la main qui fond, doigt par doigt.",
                "Si la main se referme, ne luttez pas : reprenez le bercement, puis rouvrez-la tout doucement.",
            ],
            dosage: "Environ 2 minutes, tout en douceur",
            dureeSec: 120,
            position: .assis
        ),
        Exercice(
            id: "main-appui-paume",
            nom: "Appui sur la paume ouverte",
            categorie: .main,
            objectif: "Mettre un peu de poids sur la main ouverte : un appui doux qui calme la spasticité des doigts.",
            etapes: [
                "Ouvrez la main droite avec l'aide de la gauche, très lentement.",
                "Posez la paume droite bien à plat sur votre cuisse, doigts écartés si possible.",
                "Avec la main gauche posée par-dessus, appuyez très légèrement, comme pour ancrer la main.",
                "Gardez l'appui en respirant lentement ; si les doigts se replient, rouvrez-les calmement, sans jamais forcer.",
            ],
            dosage: "3 appuis d'environ 30 s",
            dureeSec: 150,
            position: .assis
        ),
        Exercice(
            id: "main-imagerie",
            nom: "Imagerie du geste",
            categorie: .main,
            objectif: "Faire travailler le cerveau : imaginer la main droite qui s'ouvre.",
            etapes: [
                "Fermez les yeux, main droite posée confortablement.",
                "Ouvrez et fermez lentement la main GAUCHE en observant bien la sensation.",
                "Puis imaginez, très précisément, le même mouvement avec la main droite.",
                "Visualisez les doigts qui se déplient, un par un, sans effort.",
            ],
            dosage: "Environ 2 minutes",
            dureeSec: 120,
            position: .assis
        ),

        // ——— Bras & épaule ———
        Exercice(
            id: "bras-glisser-table",
            nom: "Glisser sur la table",
            categorie: .bras,
            objectif: "Mobiliser l'épaule et le coude droits en douceur.",
            etapes: [
                "Assis face à une table, posez la main droite sur un linge ou un torchon.",
                "Avec la main gauche par-dessus la droite, faites glisser le linge vers l'avant.",
                "Allez aussi loin que confortable, sans décoller le buste.",
                "Revenez lentement vers vous.",
            ],
            dosage: "8 allers-retours lents",
            dureeSec: 150,
            position: .assis
        ),
        Exercice(
            id: "bras-elevation",
            nom: "Élévation mains croisées",
            categorie: .bras,
            objectif: "Lever le bras droit avec l'aide du gauche, sans forcer l'épaule.",
            etapes: [
                "Croisez les doigts, ou tenez le poignet droit avec la main gauche.",
                "Le bras gauche guide : montez lentement les deux bras vers l'avant, puis vers le haut.",
                "Montez seulement jusqu'où c'est confortable pour l'épaule droite.",
                "Redescendez encore plus lentement.",
            ],
            dosage: "8 montées lentes",
            dureeSec: 160,
            position: .assis
        ),
        Exercice(
            id: "bras-coude",
            nom: "Coude plié, coude tendu",
            categorie: .bras,
            objectif: "Entretenir la souplesse du coude droit.",
            etapes: [
                "Bras droit posé sur les genoux ou une table.",
                "Avec la main gauche, pliez doucement le coude droit (la main vers l'épaule).",
                "Puis étendez-le doucement, le plus droit possible sans douleur.",
                "Respirez calmement pendant tout le mouvement.",
            ],
            dosage: "8 répétitions lentes",
            dureeSec: 140,
            position: .assis
        ),
        Exercice(
            id: "bras-epaules",
            nom: "Épaules qui roulent",
            categorie: .bras,
            objectif: "Détendre le cou et les deux épaules.",
            etapes: [
                "Assis bien droit, bras relâchés.",
                "Haussez doucement les épaules vers les oreilles, puis relâchez.",
                "Roulez ensuite les épaules vers l'arrière, en cercles lents.",
            ],
            dosage: "10 mouvements lents",
            dureeSec: 90,
            position: .assis
        ),

        // ——— Jambe & pré-marche ———
        Exercice(
            id: "jambe-transfert",
            nom: "Transferts d'appui assis",
            categorie: .jambe,
            objectif: "Préparer la marche : apprendre à mettre du poids côté droit.",
            etapes: [
                "Assis sur une chaise stable, pieds bien à plat au sol.",
                "Penchez doucement le buste vers la droite : le poids passe sur la fesse droite.",
                "Tenez 3 secondes, revenez au centre, puis penchez à gauche.",
                "Gardez toujours un appui sûr, près d'une table si besoin.",
            ],
            dosage: "10 transferts de chaque côté",
            dureeSec: 150,
            position: .assis
        ),
        Exercice(
            id: "jambe-genou",
            nom: "Extension du genou droit",
            categorie: .jambe,
            objectif: "Renforcer la cuisse droite, indispensable pour se lever et marcher.",
            etapes: [
                "Assis au fond de la chaise, dos soutenu.",
                "Tendez doucement le genou droit pour lever le pied vers l'avant.",
                "Si la jambe ne monte pas seule, passez une serviette sous la cuisse ou le mollet et aidez avec la main gauche.",
                "Redescendez lentement, sans laisser tomber le pied.",
            ],
            dosage: "8 extensions, même petites",
            dureeSec: 150,
            position: .assis
        ),
        Exercice(
            id: "jambe-cheville",
            nom: "Cheville en mouvement",
            categorie: .jambe,
            objectif: "Garder la cheville droite souple pour poser le pied à plat.",
            etapes: [
                "Assis, jambe droite légèrement tendue, talon au sol.",
                "Essayez de relever la pointe du pied droit vers vous, puis de la pointer.",
                "Si besoin, passez une serviette sous l'avant du pied et tirez doucement avec la main gauche.",
            ],
            dosage: "10 mouvements doux",
            dureeSec: 130,
            position: .assis
        ),
        Exercice(
            id: "jambe-marche-assise",
            nom: "Marche assise",
            categorie: .jambe,
            objectif: "Réveiller le rythme de la marche, en toute sécurité sur la chaise.",
            etapes: [
                "Assis bien droit, pieds à plat.",
                "Décollez légèrement le talon droit, reposez-le. Puis le talon gauche.",
                "Alternez comme une marche lente, au rythme de votre respiration.",
                "Si le pied droit bouge peu, même une intention de mouvement compte.",
            ],
            dosage: "Environ 1 minute, à votre rythme",
            dureeSec: 60,
            position: .assis
        ),
        Exercice(
            id: "jambe-pont",
            nom: "Pont tout doux",
            categorie: .jambe,
            objectif: "Renforcer les fessiers et le dos, utiles pour les transferts.",
            etapes: [
                "Allongé sur le dos, genoux pliés, pieds à plat (aidez le pied droit avec la main gauche si besoin).",
                "Soulevez légèrement le bassin, juste quelques centimètres.",
                "Tenez 3 secondes, puis reposez très lentement.",
                "À ne faire que si vous êtes installé en sécurité sur un lit ferme ou un tapis.",
            ],
            dosage: "5 répétitions douces",
            dureeSec: 120,
            position: .allonge
        ),
    ]

    static func exercice(_ id: String) -> Exercice {
        guard let ex = exercices.first(where: { $0.id == id }) else {
            fatalError("Exercice inconnu : \(id)")
        }
        return ex
    }

    static func parCategorie(_ categorie: CategorieExercice) -> [Exercice] {
        exercices.filter { $0.categorie == categorie }
    }

    /// Séance du jour : éveil sensoriel d'abord, puis alternance des
    /// priorités (main, bras, jambe) au fil de la semaine.
    /// Le dimanche est une séance courte et très douce.
    static func seanceDuJour(date: Date = .now) -> Seance {
        let jour = Calendar.current.component(.weekday, from: date) // 1 = dimanche

        switch jour {
        case 2, 5: // lundi, jeudi
            return Seance(
                titre: "Main & doigts",
                description: "Aujourd'hui, on prend soin de la main droite : détente d'abord, la spasticité déteste la lenteur.",
                exercices: ["detente-respiration", "eveil-paume", "main-poignet-actif", "main-tenodese", "main-ouverture", "main-extension-active"].map(exercice)
            )
        case 3, 6: // mardi, vendredi
            return Seance(
                titre: "Bras & épaule",
                description: "Le bras droit bouge en douceur, guidé par le gauche.",
                exercices: ["detente-respiration", "eveil-avant-bras", "bras-epaules", "bras-glisser-table", "bras-elevation", "main-ouverture"].map(exercice)
            )
        case 4, 7: // mercredi, samedi
            return Seance(
                titre: "Jambe & pré-marche",
                description: "On prépare la marche, pas à pas, bien assis.",
                exercices: ["detente-respiration", "jambe-transfert", "jambe-genou", "jambe-cheville", "jambe-marche-assise", "main-ouverture"].map(exercice)
            )
        default: // dimanche
            return Seance(
                titre: "Douceur du dimanche",
                description: "Séance courte et relaxante, pour garder le rythme.",
                exercices: ["detente-respiration", "main-miroir", "main-imagerie", "bras-epaules"].map(exercice)
            )
        }
    }
}
