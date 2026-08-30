/**
 * Catalogue d'exercices HemiFit.
 *
 * Profil visé : hémiparésie droite avec forte spasticité, déplacement
 * en fauteuil roulant et équilibre très altéré.
 *
 * Règles de sécurité non négociables :
 * - tout se fait assis avec le dos soutenu, ou allongé ;
 * - aucun exercice debout, aucun transfert non sécurisé ;
 * - la main gauche (saine) assiste le côté droit ;
 * - jamais de mouvement rapide ni forcé (réflexe spastique) ;
 * - étirements lents et prolongés, précédés de détente et de massage.
 */

import type { NomIcone } from "../Icones";

export type Categorie =
  | "massage"
  | "sensoriel"
  | "main"
  | "bras"
  | "tronc"
  | "jambe";

/** Qui réalise l'exercice. */
export type Realisation = "autonome" | "tierce-personne";

export interface Exercice {
  id: string;
  nom: string;
  categorie: Categorie;
  realisation: Realisation;
  /** Ce que l'exercice travaille, en une phrase. */
  objectif: string;
  /** Consignes pas à pas, phrases courtes. */
  etapes: string[];
  /** Dosage lisible, ex. « 5 répétitions, tenir 30 s ». */
  dosage: string;
  /** Durée guidée en secondes pour le minuteur de séance. */
  dureeSec: number;
  position: "assis" | "allongé";
}

export const CATEGORIES: Record<
  Categorie,
  { titre: string; icone: NomIcone }
> = {
  massage: { titre: "Massage et détente", icone: "massage" },
  sensoriel: { titre: "Éveil sensoriel", icone: "sensoriel" },
  main: { titre: "Main et doigts", icone: "main" },
  bras: { titre: "Bras et épaule", icone: "bras" },
  tronc: { titre: "Tronc et posture", icone: "tronc" },
  jambe: { titre: "Jambes et bassin", icone: "jambe" },
};

export const REALISATIONS: Record<
  Realisation,
  { titre: string; court: string; icone: NomIcone }
> = {
  autonome: {
    titre: "En autonomie",
    court: "Seul",
    icone: "autonome",
  },
  "tierce-personne": {
    titre: "Avec une tierce personne",
    court: "Avec de l'aide",
    icone: "aide",
  },
};

export const EXERCICES: Exercice[] = [
  // ————————————————— Massage et détente —————————————————
  {
    id: "detente-respiration",
    nom: "Détente et respiration",
    categorie: "massage",
    realisation: "autonome",
    objectif:
      "Faire baisser la spasticité avant de bouger : un corps détendu s'étire beaucoup mieux.",
    etapes: [
      "Installez-vous bien calé, dos soutenu, bras droit posé sur un coussin.",
      "Inspirez lentement par le nez en comptant jusqu'à 4.",
      "Soufflez très lentement par la bouche en comptant jusqu'à 6, en laissant tomber les épaules.",
      "À chaque expiration, imaginez votre bras et votre main droite devenir lourds, chauds et mous.",
    ],
    dosage: "Environ 2 minutes de respiration lente",
    dureeSec: 120,
    position: "assis",
  },
  {
    id: "massage-avant-bras",
    nom: "Massage de l'avant-bras",
    categorie: "massage",
    realisation: "autonome",
    objectif:
      "Détendre les muscles qui ferment la main : ce sont eux qui sont trop contractés.",
    etapes: [
      "Posez l'avant-bras droit sur un coussin, paume vers le haut.",
      "Avec le pouce gauche, massez l'intérieur de l'avant-bras par petits cercles lents, du coude vers le poignet.",
      "Insistez doucement là où le muscle est dur, sans jamais provoquer de douleur.",
      "Terminez par de longs passages lisses, du coude vers la main.",
    ],
    dosage: "Environ 3 minutes",
    dureeSec: 180,
    position: "assis",
  },
  {
    id: "massage-main",
    nom: "Massage de la main",
    categorie: "massage",
    realisation: "autonome",
    objectif: "Assouplir la paume et réveiller les sensations de la main droite.",
    etapes: [
      "Main droite posée sur votre cuisse, paume vers le haut.",
      "Avec le pouce gauche, massez la paume par cercles lents, du centre vers les bords.",
      "Retournez la main : massez le dos de la main, entre les tendons.",
      "Reprenez chaque doigt : massez-le de la base vers le bout, puis étirez-le très doucement.",
    ],
    dosage: "Environ 3 minutes",
    dureeSec: 180,
    position: "assis",
  },
  {
    id: "massage-pouce",
    nom: "Ouvrir l'espace du pouce",
    categorie: "massage",
    realisation: "autonome",
    objectif:
      "Empêcher le pouce de se bloquer dans la paume : c'est lui qui verrouille souvent toute la main.",
    etapes: [
      "Prenez la main droite dans la gauche, paume vers le haut.",
      "Avec le pouce et l'index gauches, pincez doucement la peau entre le pouce et l'index droits.",
      "Massez cette zone par petits cercles, puis écartez très lentement le pouce de la paume.",
      "Maintenez le pouce écarté 20 secondes, sans forcer, puis relâchez.",
    ],
    dosage: "3 fois, tenir 20 s",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "massage-drainage",
    nom: "Drainage de la main vers l'épaule",
    categorie: "massage",
    realisation: "autonome",
    objectif:
      "Faire circuler : une main peu mobile a tendance à gonfler, surtout en fin de journée.",
    etapes: [
      "Bras droit posé, si possible légèrement surélevé sur un coussin.",
      "Avec la paume gauche bien à plat, remontez lentement de la main vers le coude, en pressant doucement.",
      "Continuez du coude vers l'épaule, toujours dans ce sens (jamais l'inverse).",
      "Refaites le trajet complet, calmement, comme une vague qui remonte.",
    ],
    dosage: "10 remontées lentes",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "massage-epaule",
    nom: "Massage de l'épaule et de la nuque",
    categorie: "massage",
    realisation: "autonome",
    objectif:
      "Soulager l'épaule droite, souvent douloureuse quand le bras est peu actif.",
    etapes: [
      "Avec la main gauche, massez le haut de l'épaule droite par pressions lentes.",
      "Remontez vers la nuque, puis redescendez vers l'omoplate.",
      "Soufflez lentement pendant le massage, en laissant l'épaule redescendre.",
      "Si l'épaule est douloureuse, restez très léger et parlez-en à votre kinésithérapeute.",
    ],
    dosage: "Environ 2 minutes",
    dureeSec: 120,
    position: "assis",
  },

  // ————————————————— Éveil sensoriel —————————————————
  {
    id: "eveil-paume",
    nom: "Réveil de la main droite",
    categorie: "sensoriel",
    realisation: "autonome",
    objectif: "Réveiller les sensations de la main droite avant de la mobiliser.",
    etapes: [
      "Bras droit posé sur une table ou un coussin.",
      "Avec la main gauche, frottez doucement la paume droite, du poignet vers les doigts.",
      "Variez les textures : tissu, éponge, brosse douce.",
      "Terminez par de petites pressions sur toute la main.",
    ],
    dosage: "Environ 2 minutes",
    dureeSec: 120,
    position: "assis",
  },
  {
    id: "eveil-avant-bras",
    nom: "Caresses de l'avant-bras",
    categorie: "sensoriel",
    realisation: "autonome",
    objectif: "Stimuler la peau et abaisser le tonus de l'avant-bras droit.",
    etapes: [
      "Posez l'avant-bras droit sur vos genoux ou une table.",
      "Avec la main gauche, caressez lentement du coude jusqu'à la main.",
      "Alternez le dessus et le dessous de l'avant-bras.",
    ],
    dosage: "Environ 1 minute 30",
    dureeSec: 90,
    position: "assis",
  },
  {
    id: "main-miroir",
    nom: "Thérapie miroir",
    categorie: "sensoriel",
    realisation: "autonome",
    objectif:
      "Voir une main droite qui s'ouvre aide le cerveau à réapprendre le mouvement.",
    etapes: [
      "Posez un miroir debout devant vous, tranche contre votre ventre, face réfléchissante vers la gauche.",
      "Cachez la main droite derrière le miroir ; regardez le reflet de la main gauche.",
      "Ouvrez et fermez lentement la main gauche en regardant le reflet.",
      "Pendant ce temps, essayez le même mouvement avec la main droite cachée, sans forcer.",
    ],
    dosage: "Environ 3 minutes",
    dureeSec: 180,
    position: "assis",
  },
  {
    id: "main-imagerie",
    nom: "Imagerie du geste",
    categorie: "sensoriel",
    realisation: "autonome",
    objectif: "Activer les circuits du mouvement sans effort musculaire.",
    etapes: [
      "Fermez les yeux, main droite posée confortablement.",
      "Ouvrez et fermez lentement la main gauche en observant bien la sensation.",
      "Puis imaginez très précisément le même mouvement avec la main droite.",
      "Visualisez les doigts qui se déplient, un par un, sans effort.",
    ],
    dosage: "Environ 2 minutes",
    dureeSec: 120,
    position: "assis",
  },

  // ————————————————— Main et doigts —————————————————
  {
    id: "main-poignet-actif",
    nom: "Apprendre à bouger le poignet",
    categorie: "main",
    realisation: "autonome",
    objectif:
      "Réapprendre au poignet à se plier et se redresser : la gravité fait le mouvement, vous apprenez d'abord à le retenir.",
    etapes: [
      "Avant-bras droit posé sur une table, main dans le vide au bord, paume vers le bas.",
      "Laissez la main pendre : la gravité plie le poignet toute seule.",
      "Avec la main gauche, remontez doucement la main droite à l'horizontale, puis laissez-la redescendre lentement.",
      "Essayez ensuite de retenir un peu la descente, ou de remonter d'un millimètre : retenir est plus facile que soulever.",
      "Terminez en laissant la main pendre et se détendre complètement.",
    ],
    dosage: "8 allers-retours doux",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "main-tenodese",
    nom: "L'astuce du poignet plié",
    categorie: "main",
    realisation: "autonome",
    objectif:
      "Poignet plié vers l'avant, les doigts se détendent et s'ouvrent plus facilement.",
    etapes: [
      "Posez l'avant-bras droit sur la table ou votre cuisse.",
      "Avec la main gauche, pliez doucement le poignet droit vers l'avant. C'est la main gauche qui fait tout : le poignet droit se laisse porter.",
      "Les doigts se desserrent : profitez-en pour les ouvrir doucement avec la main gauche.",
      "Doigts ouverts, redressez très lentement le poignet, sans perdre l'ouverture.",
      "Si les doigts se referment, repliez le poignet et recommencez.",
    ],
    dosage: "5 essais tranquilles",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "main-ouverture",
    nom: "Ouverture de main assistée",
    categorie: "main",
    realisation: "autonome",
    objectif:
      "Ouvrir la main malgré la spasticité, en laissant le temps aux muscles de lâcher.",
    etapes: [
      "Massez d'abord l'avant-bras et la paume pour préparer la main.",
      "Penchez légèrement le poignet vers l'avant : les doigts se laissent ouvrir plus facilement.",
      "Avec la main gauche, dépliez très lentement les doigts droits, en commençant par le pouce.",
      "Si les doigts résistent, ne forcez jamais : arrêtez-vous, soufflez, attendez que ça se relâche.",
      "Maintenez l'ouverture 30 secondes : c'est l'étirement prolongé qui calme la spasticité.",
    ],
    dosage: "3 ouvertures très lentes, tenir 30 s",
    dureeSec: 180,
    position: "assis",
  },
  {
    id: "main-extension-active",
    nom: "Ouvrir avec de l'aide",
    categorie: "main",
    realisation: "autonome",
    objectif:
      "Vos doigts savent se fermer : on entraîne le mouvement inverse, l'ouverture.",
    etapes: [
      "Main droite posée sur la cuisse, détendue.",
      "Serrez très légèrement le poing 3 secondes — cela, vous savez le faire.",
      "Puis arrêtez de serrer, soufflez, et essayez d'ouvrir les doigts, même d'un millimètre.",
      "Pendant que vous essayez, la main gauche accompagne et termine l'ouverture.",
      "L'essai compte autant que le résultat : c'est l'intention d'ouvrir qui réveille les muscles.",
    ],
    dosage: "6 essais, sans forcer",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "main-appui-paume",
    nom: "Appui sur la paume ouverte",
    categorie: "main",
    realisation: "autonome",
    objectif:
      "Un appui doux sur la main ouverte calme la spasticité des doigts.",
    etapes: [
      "Ouvrez la main droite avec l'aide de la gauche, très lentement.",
      "Posez la paume droite bien à plat sur votre cuisse, doigts écartés si possible.",
      "Avec la main gauche posée par-dessus, appuyez très légèrement, comme pour ancrer la main.",
      "Gardez l'appui en respirant lentement ; si les doigts se replient, rouvrez-les calmement.",
    ],
    dosage: "3 appuis d'environ 30 s",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "main-relacher",
    nom: "Apprendre à relâcher",
    categorie: "main",
    realisation: "autonome",
    objectif:
      "Avec la spasticité, relâcher est plus difficile que serrer : c'est le relâchement qu'on entraîne.",
    etapes: [
      "Posez la main droite sur votre cuisse ou sur une serviette roulée, sans rien tenir.",
      "Avec la main gauche, bercez doucement l'avant-bras droit, comme pour l'endormir.",
      "Soufflez lentement en imaginant la main qui fond, doigt par doigt.",
      "Si la main se referme, ne luttez pas : reprenez le bercement, puis rouvrez-la doucement.",
    ],
    dosage: "Environ 2 minutes",
    dureeSec: 120,
    position: "assis",
  },

  // ————————————————— Bras et épaule —————————————————
  {
    id: "bras-glisser-table",
    nom: "Glisser sur la table",
    categorie: "bras",
    realisation: "autonome",
    objectif: "Mobiliser l'épaule et le coude droits en douceur.",
    etapes: [
      "Installé face à une table, posez la main droite sur un linge.",
      "Avec la main gauche par-dessus la droite, faites glisser le linge vers l'avant.",
      "Allez aussi loin que confortable, sans décoller le dos du dossier.",
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
    realisation: "autonome",
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
    realisation: "autonome",
    objectif: "Entretenir la souplesse du coude droit.",
    etapes: [
      "Bras droit posé sur les genoux ou une table.",
      "Avec la main gauche, pliez doucement le coude droit, la main vers l'épaule.",
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
    realisation: "autonome",
    objectif: "Détendre le cou et les deux épaules.",
    etapes: [
      "Bien calé au fond du siège, bras relâchés.",
      "Haussez doucement les épaules vers les oreilles, puis relâchez.",
      "Roulez ensuite les épaules vers l'arrière, en cercles lents.",
    ],
    dosage: "10 mouvements lents",
    dureeSec: 90,
    position: "assis",
  },

  // ————————————————— Tronc et posture —————————————————
  {
    id: "tronc-appuis",
    nom: "Soulagement des appuis",
    categorie: "tronc",
    realisation: "autonome",
    objectif:
      "Décharger régulièrement les points d'appui : c'est la meilleure prévention des rougeurs et des escarres.",
    etapes: [
      "Freins du fauteuil bloqués, mains sur les accoudoirs.",
      "Poussez sur le bras gauche pour décoller légèrement la fesse droite, quelques secondes.",
      "Reposez-vous, puis penchez-vous doucement de l'autre côté pour soulager la fesse gauche.",
      "Restez toujours dans une amplitude petite et sûre : on cherche à soulager, pas à se pencher loin.",
    ],
    dosage: "3 fois de chaque côté, tenir 5 s",
    dureeSec: 120,
    position: "assis",
  },
  {
    id: "tronc-bascule",
    nom: "Bascule du bassin",
    categorie: "tronc",
    realisation: "autonome",
    objectif:
      "Assouplir le bas du dos et retrouver le contrôle du bassin, base de la stabilité assise.",
    etapes: [
      "Dos bien soutenu par le dossier, mains posées sur les cuisses.",
      "Basculez doucement le bassin vers l'arrière : le bas du dos s'arrondit.",
      "Puis basculez-le vers l'avant : le bas du dos se creuse légèrement.",
      "Mouvement lent et de petite amplitude ; le buste bouge à peine.",
    ],
    dosage: "10 bascules lentes",
    dureeSec: 130,
    position: "assis",
  },
  {
    id: "tronc-rotation",
    nom: "Rotation douce du buste",
    categorie: "tronc",
    realisation: "autonome",
    objectif: "Garder de la mobilité dans le tronc, sans jamais se déséquilibrer.",
    etapes: [
      "Dos soutenu, mains posées à plat sur les cuisses.",
      "Tournez lentement les épaules vers la gauche, en gardant le bassin immobile.",
      "Revenez au centre, puis tournez vers la droite.",
      "Gardez toujours un contact avec le dossier : c'est votre sécurité.",
    ],
    dosage: "8 rotations de chaque côté",
    dureeSec: 130,
    position: "assis",
  },
  {
    id: "tronc-grandir",
    nom: "Se grandir",
    categorie: "tronc",
    realisation: "autonome",
    objectif:
      "Lutter contre l'affaissement du buste, fréquent quand on passe la journée assis.",
    etapes: [
      "Dos soutenu, pieds posés sur les repose-pieds.",
      "Inspirez en imaginant un fil qui tire le sommet de votre tête vers le plafond.",
      "Le buste se redresse, les épaules descendent, le menton reste horizontal.",
      "Tenez 5 secondes en respirant, puis relâchez sans vous affaisser d'un coup.",
    ],
    dosage: "8 redressements, tenir 5 s",
    dureeSec: 120,
    position: "assis",
  },

  // ————————————————— Jambes et bassin —————————————————
  {
    id: "jambe-genou",
    nom: "Extension du genou droit",
    categorie: "jambe",
    realisation: "autonome",
    objectif:
      "Entretenir la cuisse droite et la mobilité du genou, utiles pour les transferts.",
    etapes: [
      "Bien calé au fond du siège, dos soutenu.",
      "Tendez doucement le genou droit pour lever le pied vers l'avant.",
      "Si la jambe ne monte pas seule, passez une serviette sous le mollet et aidez avec la main gauche.",
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
    realisation: "autonome",
    objectif:
      "Garder la cheville souple et faire circuler le sang : essentiel quand on reste assis toute la journée.",
    etapes: [
      "Pied droit posé à plat, ou sur le repose-pied.",
      "Essayez de relever la pointe du pied vers vous, puis de la pointer vers le bas.",
      "Si besoin, passez une serviette sous l'avant du pied et tirez doucement avec la main gauche.",
      "Terminez par des cercles lents avec la cheville, dans un sens puis dans l'autre.",
    ],
    dosage: "10 mouvements, puis 5 cercles",
    dureeSec: 150,
    position: "assis",
  },
  {
    id: "jambe-alternance",
    nom: "Alternance des appuis",
    categorie: "jambe",
    realisation: "autonome",
    objectif:
      "Réveiller l'alternance droite-gauche des jambes, qui entretient la commande motrice.",
    etapes: [
      "Bien calé, pieds posés à plat.",
      "Décollez légèrement le talon droit, reposez-le. Puis le talon gauche.",
      "Alternez lentement, au rythme de votre respiration.",
      "Si le pied droit bouge peu, l'intention de le décoller compte déjà.",
    ],
    dosage: "Environ 1 minute 30",
    dureeSec: 90,
    position: "assis",
  },
  {
    id: "jambe-pont",
    nom: "Pont tout doux",
    categorie: "jambe",
    realisation: "autonome",
    objectif:
      "Renforcer les fessiers et le bas du dos, ce qui facilite les transferts et l'installation au lit.",
    etapes: [
      "Allongé sur le dos, genoux pliés, pieds à plat (aidez le pied droit avec la main gauche si besoin).",
      "Soulevez légèrement le bassin, juste quelques centimètres.",
      "Tenez 3 secondes, puis reposez très lentement.",
      "À faire uniquement bien installé au centre du lit, jamais près du bord.",
    ],
    dosage: "5 répétitions douces",
    dureeSec: 120,
    position: "allongé",
  },

  // ————————————————— Avec une tierce personne (le soir) —————————————————
  {
    id: "aide-massage-bras",
    nom: "Massage complet du bras droit",
    categorie: "massage",
    realisation: "tierce-personne",
    objectif:
      "Faire baisser le tonus de tout le membre avant les mobilisations : on masse toujours avant de mobiliser.",
    etapes: [
      "Personne aidante : installez le bras droit posé et soutenu, la personne bien calée.",
      "Massez de la main vers l'épaule, par pressions larges et lentes, jamais dans l'autre sens.",
      "Insistez sur l'avant-bras, où les muscles fléchisseurs sont les plus contractés.",
      "Terminez par la main : paume, dos de la main, puis chaque doigt.",
      "Demandez régulièrement si la pression est confortable.",
    ],
    dosage: "Environ 5 minutes",
    dureeSec: 300,
    position: "assis",
  },
  {
    id: "aide-epaule",
    nom: "Mobilisation passive de l'épaule",
    categorie: "bras",
    realisation: "tierce-personne",
    objectif:
      "Entretenir l'amplitude de l'épaule et prévenir l'enraidissement, très fréquent du côté atteint.",
    etapes: [
      "Personne aidante : une main soutient le coude, l'autre tient l'avant-bras. Ne tirez jamais par la main seule.",
      "Montez lentement le bras vers l'avant, jusqu'à la limite confortable, sans jamais forcer.",
      "Redescendez tout aussi lentement.",
      "Puis écartez doucement le bras sur le côté, dans une amplitude modérée.",
      "Arrêtez immédiatement en cas de douleur : l'épaule du côté atteint est fragile.",
    ],
    dosage: "8 mouvements lents dans chaque direction",
    dureeSec: 240,
    position: "assis",
  },
  {
    id: "aide-coude-poignet",
    nom: "Mobilisation du coude et du poignet",
    categorie: "bras",
    realisation: "tierce-personne",
    objectif: "Conserver la souplesse du coude et du poignet droits.",
    etapes: [
      "Personne aidante : soutenez le bras, une main au-dessus du coude, l'autre au poignet.",
      "Pliez et tendez le coude lentement, dix fois.",
      "Puis tournez doucement l'avant-bras : paume vers le haut, paume vers le bas.",
      "Terminez par le poignet : fléchissez-le et étendez-le doucement, en tenant chaque position 15 secondes.",
    ],
    dosage: "10 mouvements, puis tenues de 15 s",
    dureeSec: 240,
    position: "assis",
  },
  {
    id: "aide-doigts",
    nom: "Étirement prolongé des doigts",
    categorie: "main",
    realisation: "tierce-personne",
    objectif:
      "L'étirement long est ce qui calme le mieux la spasticité — plus efficace le soir, après le massage.",
    etapes: [
      "Personne aidante : massez d'abord la main et l'avant-bras pendant deux bonnes minutes.",
      "Pliez légèrement le poignet vers l'avant : les doigts se desserrent d'eux-mêmes.",
      "Ouvrez les doigts un par un, en commençant par le pouce, très lentement.",
      "Main ouverte, maintenez l'étirement 60 secondes en redressant progressivement le poignet.",
      "Ne forcez jamais contre une résistance : attendez, le muscle finit par céder.",
    ],
    dosage: "3 étirements de 60 s",
    dureeSec: 300,
    position: "assis",
  },
  {
    id: "aide-hanche-genou",
    nom: "Mobilisation de la hanche et du genou",
    categorie: "jambe",
    realisation: "tierce-personne",
    objectif:
      "Entretenir les articulations des jambes, peu sollicitées en fauteuil, et limiter l'enraidissement.",
    etapes: [
      "À faire allongé sur le dos, bien installé au centre du lit.",
      "Personne aidante : une main sous le genou, l'autre sous le talon.",
      "Ramenez lentement le genou vers la poitrine, dans la limite du confort, puis rallongez la jambe.",
      "Répétez dix fois, sans à-coup, en soutenant toujours le poids de la jambe.",
      "Faites les deux jambes : la gauche en profite aussi.",
    ],
    dosage: "10 mouvements par jambe",
    dureeSec: 300,
    position: "allongé",
  },
  {
    id: "aide-cheville-mollet",
    nom: "Étirement du mollet et de la cheville",
    categorie: "jambe",
    realisation: "tierce-personne",
    objectif:
      "Empêcher le pied de se figer en pointe : c'est l'un des enraidissements les plus gênants en fauteuil.",
    etapes: [
      "Allongé, jambe tendue et soutenue.",
      "Personne aidante : posez une main à plat sous la plante du pied, l'autre au-dessus du genou.",
      "Poussez très lentement le pied vers le tibia, jusqu'à sentir l'étirement du mollet.",
      "Maintenez 30 secondes en respirant, sans jamais forcer par à-coups.",
      "Répétez trois fois, puis faites quelques cercles doux avec la cheville.",
    ],
    dosage: "3 étirements de 30 s par pied",
    dureeSec: 300,
    position: "allongé",
  },
  {
    id: "aide-drainage-jambes",
    nom: "Massage drainant des jambes",
    categorie: "massage",
    realisation: "tierce-personne",
    objectif:
      "Faire circuler et limiter le gonflement des jambes et des pieds, très fréquent après une journée assis.",
    etapes: [
      "Allongé, jambes légèrement surélevées sur un coussin.",
      "Personne aidante : remontez à deux mains, de la cheville vers le genou, par pressions lentes.",
      "Continuez du genou vers la cuisse, toujours vers le cœur.",
      "Terminez par les pieds : massez la plante du talon vers les orteils, puis mobilisez chaque orteil.",
    ],
    dosage: "Environ 5 minutes",
    dureeSec: 300,
    position: "allongé",
  },
  {
    id: "aide-installation-nuit",
    nom: "Installation pour la nuit",
    categorie: "tronc",
    realisation: "tierce-personne",
    objectif:
      "Une bonne installation prolonge l'effet des étirements pendant toute la nuit.",
    etapes: [
      "Personne aidante : installez le bras droit posé sur un coussin, légèrement écarté du corps, main ouverte si possible.",
      "Évitez que la main reste serrée sous le corps ou coincée contre le flanc.",
      "Placez un coussin sous le mollet droit pour que le talon ne porte pas directement sur le matelas.",
      "Vérifiez que le pied droit n'est pas tourné vers l'intérieur ni figé en pointe.",
      "Demandez confirmation que la position est confortable avant de quitter la pièce.",
    ],
    dosage: "Environ 3 minutes",
    dureeSec: 180,
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
  realisation: Realisation;
}

/**
 * Séance du jour, réalisable seul. Chaque séance commence par la
 * détente puis un massage : sur un membre spastique, c'est la
 * meilleure préparation avant de chercher à bouger.
 */
export function seanceDuJour(date: Date = new Date()): Seance {
  const jour = date.getDay(); // 0 = dimanche

  switch (jour) {
    case 1: // lundi
      return {
        titre: "Main et ouverture",
        description:
          "On masse la main, puis on travaille l'ouverture des doigts, tout en lenteur.",
        realisation: "autonome",
        exercices: parId(
          "detente-respiration",
          "massage-main",
          "massage-pouce",
          "main-tenodese",
          "main-ouverture",
          "main-extension-active",
        ),
      };
    case 4: // jeudi
      return {
        titre: "Main et poignet",
        description:
          "On détend l'avant-bras, puis on réapprend le mouvement du poignet.",
        realisation: "autonome",
        exercices: parId(
          "detente-respiration",
          "massage-avant-bras",
          "main-poignet-actif",
          "main-tenodese",
          "main-ouverture",
          "main-appui-paume",
        ),
      };
    case 2: // mardi
    case 5: // vendredi
      return {
        titre: "Bras et épaule",
        description: "Le bras droit bouge en douceur, guidé par le gauche.",
        realisation: "autonome",
        exercices: parId(
          "detente-respiration",
          "massage-avant-bras",
          "massage-epaule",
          "bras-glisser-table",
          "bras-elevation",
          "main-ouverture",
        ),
      };
    case 3: // mercredi
    case 6: // samedi
      return {
        titre: "Tronc et jambes",
        description:
          "Posture, appuis et souplesse des jambes, entièrement en sécurité dans le fauteuil.",
        realisation: "autonome",
        exercices: parId(
          "detente-respiration",
          "tronc-appuis",
          "tronc-bascule",
          "tronc-grandir",
          "jambe-cheville",
          "jambe-genou",
        ),
      };
    default: // dimanche
      return {
        titre: "Massage et détente",
        description:
          "Journée douce : on masse tout le côté droit, sans rien forcer.",
        realisation: "autonome",
        exercices: parId(
          "detente-respiration",
          "massage-main",
          "massage-avant-bras",
          "massage-drainage",
          "massage-epaule",
          "main-miroir",
        ),
      };
  }
}

/**
 * Séance du soir, réalisée par une tierce personne. Massage d'abord,
 * mobilisations ensuite, installation pour la nuit en dernier.
 */
export function seanceDuSoir(): Seance {
  return {
    titre: "Séance du soir",
    description:
      "Massages et mobilisations passives, réalisés par une tierce personne. À faire de préférence après une douche chaude.",
    realisation: "tierce-personne",
    exercices: parId(
      "aide-massage-bras",
      "aide-epaule",
      "aide-coude-poignet",
      "aide-doigts",
      "aide-hanche-genou",
      "aide-cheville-mollet",
      "aide-drainage-jambes",
      "aide-installation-nuit",
    ),
  };
}

export function dureeTotaleMin(seance: Seance): number {
  const sec = seance.exercices.reduce((t, e) => t + e.dureeSec, 0);
  return Math.round(sec / 60);
}
