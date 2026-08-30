/**
 * Catalogue d'exercices HemiFit.
 *
 * Conçu pour une hémiparésie droite : tous les exercices se font
 * assis ou allongé, en toute sécurité, et la main gauche (saine)
 * vient assister le côté droit. Jamais de douleur : on s'arrête
 * dès que ça tire trop.
 */

export type Categorie = "main" | "bras" | "jambe" | "sensoriel";

export interface Exercice {
  id: string;
  nom: string;
  categorie: Categorie;
  /** Ce que l'exercice travaille, en une phrase. */
  objectif: string;
  /** Consignes pas à pas, phrases courtes. */
  etapes: string[];
  /** Dosage lisible, ex. « 5 répétitions, tenir 10 s ». */
  dosage: string;
  /** Durée guidée en secondes pour le minuteur de séance. */
  dureeSec: number;
  /** Position de départ. */
  position: "assis" | "allongé";
}

export const CATEGORIES: Record<Categorie, { titre: string; emoji: string }> = {
  sensoriel: { titre: "Éveil sensoriel", emoji: "🫱" },
  main: { titre: "Main & doigts", emoji: "✋" },
  bras: { titre: "Bras & épaule", emoji: "💪" },
  jambe: { titre: "Jambe & pré-marche", emoji: "🦶" },
};

export const EXERCICES: Exercice[] = [
  // ——— Éveil sensoriel ———
  {
    id: "eveil-paume",
    nom: "Réveil de la main droite",
    categorie: "sensoriel",
    objectif: "Réveiller les sensations de la main droite avant de bouger.",
    etapes: [
      "Asseyez-vous confortablement, le bras droit posé sur une table ou un coussin.",
      "Avec la main gauche, frottez doucement la paume droite, du poignet vers les doigts.",
      "Massez chaque doigt l'un après l'autre, sans forcer.",
      "Terminez par de petites pressions douces sur toute la main.",
    ],
    dosage: "Environ 2 minutes, en douceur",
    dureeSec: 120,
    position: "assis",
  },
  {
    id: "eveil-avant-bras",
    nom: "Caresses de l'avant-bras",
    categorie: "sensoriel",
    objectif: "Stimuler la peau et détendre l'avant-bras droit.",
    etapes: [
      "Posez l'avant-bras droit sur vos genoux ou une table.",
      "Avec la main gauche, caressez lentement du coude jusqu'à la main.",
      "Variez : paume, dos de la main, textures différentes (tissu, éponge).",
    ],
    dosage: "Environ 1 minute 30",
    dureeSec: 90,
    position: "assis",
  },

  // ——— Main & doigts ———
  {
    id: "main-ouverture",
    nom: "Ouverture de main assistée",
    categorie: "main",
    objectif: "Assouplir les doigts et travailler l'ouverture de la main droite.",
    etapes: [
      "Posez la main droite sur vos genoux, paume vers le haut si possible.",
      "Avec la main gauche, dépliez très doucement les doigts droits, un par un.",
      "Quand la main est ouverte, maintenez l'ouverture 10 secondes.",
      "Relâchez, laissez la main se reposer, puis recommencez.",
      "Jamais de douleur : si ça tire trop, ouvrez moins grand.",
    ],
    dosage: "5 répétitions, tenir 10 s",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "main-poignet",
    nom: "Étirement doux du poignet",
    categorie: "main",
    objectif: "Garder le poignet droit souple.",
    etapes: [
      "Coude droit posé sur la table, avant-bras vertical si possible.",
      "Avec la main gauche, amenez doucement la main droite vers l'arrière (paume vers l'avant).",
      "Tenez 10 secondes en respirant calmement.",
      "Revenez au repos, puis penchez doucement la main vers l'avant.",
    ],
    dosage: "4 répétitions dans chaque sens, tenir 10 s",
    dureeSec: 160,
    position: "assis",
  },
  {
    id: "main-presse",
    nom: "Presse douce",
    categorie: "main",
    objectif: "Essayer de serrer puis de relâcher, même un tout petit peu.",
    etapes: [
      "Placez une balle en mousse souple (ou une chaussette roulée) dans la main droite.",
      "Essayez de serrer, même très légèrement. Chaque petit mouvement compte.",
      "Puis, avec l'aide de la main gauche, rouvrez doucement les doigts.",
      "L'ouverture est aussi importante que le serrage.",
    ],
    dosage: "8 essais, sans forcer",
    dureeSec: 120,
    position: "assis",
  },
  {
    id: "main-imagerie",
    nom: "Imagerie du geste",
    categorie: "main",
    objectif: "Faire travailler le cerveau : imaginer la main droite qui s'ouvre.",
    etapes: [
      "Fermez les yeux, main droite posée confortablement.",
      "Ouvrez et fermez lentement la main GAUCHE en observant bien la sensation.",
      "Puis imaginez, très précisément, le même mouvement avec la main droite.",
      "Visualisez les doigts qui se déplient, un par un, sans effort.",
    ],
    dosage: "Environ 2 minutes",
    dureeSec: 120,
    position: "assis",
  },

  // ——— Bras & épaule ———
  {
    id: "bras-glisser-table",
    nom: "Glisser sur la table",
    categorie: "bras",
    objectif: "Mobiliser l'épaule et le coude droits en douceur.",
    etapes: [
      "Assis face à une table, posez la main droite sur un linge ou un torchon.",
      "Avec la main gauche par-dessus la droite, faites glisser le linge vers l'avant.",
      "Allez aussi loin que confortable, sans décoller le buste.",
      "Revenez lentement vers vous.",
    ],
    dosage: "8 allers-retours lents",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "bras-elevation",
    nom: "Élévation mains croisées",
    categorie: "bras",
    objectif: "Lever le bras droit avec l'aide du gauche, sans forcer l'épaule.",
    etapes: [
      "Croisez les doigts, ou tenez le poignet droit avec la main gauche.",
      "Le bras gauche guide : montez lentement les deux bras vers l'avant, puis vers le haut.",
      "Montez seulement jusqu'où c'est confortable pour l'épaule droite.",
      "Redescendez encore plus lentement.",
    ],
    dosage: "8 montées lentes",
    dureeSec: 160,
    position: "assis",
  },
  {
    id: "bras-coude",
    nom: "Coude plié, coude tendu",
    categorie: "bras",
    objectif: "Entretenir la souplesse du coude droit.",
    etapes: [
      "Bras droit posé sur les genoux ou une table.",
      "Avec la main gauche, pliez doucement le coude droit (la main vers l'épaule).",
      "Puis étendez-le doucement, le plus droit possible sans douleur.",
      "Respirez calmement pendant tout le mouvement.",
    ],
    dosage: "8 répétitions lentes",
    dureeSec: 140,
    position: "assis",
  },
  {
    id: "bras-epaules",
    nom: "Épaules qui roulent",
    categorie: "bras",
    objectif: "Détendre le cou et les deux épaules.",
    etapes: [
      "Assis bien droit, bras relâchés.",
      "Haussez doucement les épaules vers les oreilles, puis relâchez.",
      "Roulez ensuite les épaules vers l'arrière, en cercles lents.",
    ],
    dosage: "10 mouvements lents",
    dureeSec: 90,
    position: "assis",
  },

  // ——— Jambe & pré-marche ———
  {
    id: "jambe-transfert",
    nom: "Transferts d'appui assis",
    categorie: "jambe",
    objectif: "Préparer la marche : apprendre à mettre du poids côté droit.",
    etapes: [
      "Assis sur une chaise stable, pieds bien à plat au sol.",
      "Penchez doucement le buste vers la droite : le poids passe sur la fesse droite.",
      "Tenez 3 secondes, revenez au centre, puis penchez à gauche.",
      "Gardez toujours un appui sûr, près d'une table si besoin.",
    ],
    dosage: "10 transferts de chaque côté",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "jambe-genou",
    nom: "Extension du genou droit",
    categorie: "jambe",
    objectif: "Renforcer la cuisse droite, indispensable pour se lever et marcher.",
    etapes: [
      "Assis au fond de la chaise, dos soutenu.",
      "Tendez doucement le genou droit pour lever le pied vers l'avant.",
      "Si la jambe ne monte pas seule, passez une serviette sous la cuisse ou le mollet et aidez avec la main gauche.",
      "Redescendez lentement, sans laisser tomber le pied.",
    ],
    dosage: "8 extensions, même petites",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "jambe-cheville",
    nom: "Cheville en mouvement",
    categorie: "jambe",
    objectif: "Garder la cheville droite souple pour poser le pied à plat.",
    etapes: [
      "Assis, jambe droite légèrement tendue, talon au sol.",
      "Essayez de relever la pointe du pied droit vers vous, puis de la pointer.",
      "Si besoin, passez une serviette sous l'avant du pied et tirez doucement avec la main gauche.",
    ],
    dosage: "10 mouvements doux",
    dureeSec: 130,
    position: "assis",
  },
  {
    id: "jambe-marche-assise",
    nom: "Marche assise",
    categorie: "jambe",
    objectif: "Réveiller le rythme de la marche, en toute sécurité sur la chaise.",
    etapes: [
      "Assis bien droit, pieds à plat.",
      "Décollez légèrement le talon droit, reposez-le. Puis le talon gauche.",
      "Alternez comme une marche lente, au rythme de votre respiration.",
      "Si le pied droit bouge peu, même une intention de mouvement compte.",
    ],
    dosage: "Environ 1 minute, à votre rythme",
    dureeSec: 60,
    position: "assis",
  },
  {
    id: "jambe-pont",
    nom: "Pont tout doux",
    categorie: "jambe",
    objectif: "Renforcer les fessiers et le dos, utiles pour les transferts.",
    etapes: [
      "Allongé sur le dos, genoux pliés, pieds à plat (aidez le pied droit avec la main gauche si besoin).",
      "Soulevez légèrement le bassin, juste quelques centimètres.",
      "Tenez 3 secondes, puis reposez très lentement.",
      "À ne faire que si vous êtes installé en sécurité sur un lit ferme ou un tapis.",
    ],
    dosage: "5 répétitions douces",
    dureeSec: 120,
    position: "allongé",
  },
];

export function exercicesParCategorie(cat: Categorie): Exercice[] {
  return EXERCICES.filter(e => e.categorie === cat);
}

function parId(...ids: string[]): Exercice[] {
  return ids.map(id => {
    const ex = EXERCICES.find(e => e.id === id);
    if (!ex) throw new Error(`Exercice inconnu : ${id}`);
    return ex;
  });
}

export interface Seance {
  titre: string;
  description: string;
  exercices: Exercice[];
}

/**
 * Séance du jour : chaque jour commence par l'éveil sensoriel,
 * puis alterne les priorités (main, bras, jambe) au fil de la semaine.
 * Le dimanche est une séance plus courte et très douce.
 */
export function seanceDuJour(date: Date = new Date()): Seance {
  const jour = date.getDay(); // 0 = dimanche

  switch (jour) {
    case 1: // lundi
    case 4: // jeudi
      return {
        titre: "Main & doigts",
        description: "Aujourd'hui, on prend soin de la main droite.",
        exercices: parId(
          "eveil-paume",
          "main-ouverture",
          "main-poignet",
          "main-presse",
          "jambe-transfert",
        ),
      };
    case 2: // mardi
    case 5: // vendredi
      return {
        titre: "Bras & épaule",
        description: "Le bras droit bouge en douceur, guidé par le gauche.",
        exercices: parId(
          "eveil-avant-bras",
          "bras-epaules",
          "bras-glisser-table",
          "bras-elevation",
          "main-ouverture",
        ),
      };
    case 3: // mercredi
    case 6: // samedi
      return {
        titre: "Jambe & pré-marche",
        description: "On prépare la marche, pas à pas, bien assis.",
        exercices: parId(
          "jambe-transfert",
          "jambe-genou",
          "jambe-cheville",
          "jambe-marche-assise",
          "main-ouverture",
        ),
      };
    default: // dimanche
      return {
        titre: "Douceur du dimanche",
        description: "Séance courte et relaxante, pour garder le rythme.",
        exercices: parId("eveil-paume", "main-imagerie", "bras-epaules"),
      };
  }
}

export function dureeTotaleMin(seance: Seance): number {
  const sec = seance.exercices.reduce((t, e) => t + e.dureeSec, 0);
  return Math.round(sec / 60);
}
