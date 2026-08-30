import { useMemo, useState } from "react";
import {
  CATEGORIES,
  EXERCICES,
  REALISATIONS,
  dureeTotaleMin,
  seanceDuJour,
  seanceDuSoir,
  type Categorie,
  type Exercice,
  type Realisation,
  type Seance,
} from "./data/exercises";
import {
  chargerHistorique,
  derniers7Jours,
  joursDepuisDerniereSeance,
  meilleureSerie,
  minutesTotales,
  seanceFaiteAujourdhui,
  seancesSur7Jours,
  serieEnCours,
  type SeanceTerminee,
} from "./lib/progression";
import { Icone, type NomIcone } from "./Icones";
import { LecteurSeance } from "./LecteurSeance";

type Onglet = "accueil" | "exercices" | "progres" | "conseils";

export function App() {
  const [onglet, setOnglet] = useState<Onglet>("accueil");
  const [seanceEnCours, setSeanceEnCours] = useState<Seance | null>(null);
  const [historique, setHistorique] = useState<SeanceTerminee[]>(() =>
    chargerHistorique(),
  );

  const seance = useMemo(() => seanceDuJour(), []);
  const soir = useMemo(() => seanceDuSoir(), []);

  if (seanceEnCours) {
    return (
      <LecteurSeance
        seance={seanceEnCours}
        onQuitter={() => setSeanceEnCours(null)}
        onTerminee={nouvelHistorique => {
          setHistorique(nouvelHistorique);
          setSeanceEnCours(null);
          setOnglet("progres");
        }}
      />
    );
  }

  return (
    <>
      <main className="contenu">
        {onglet === "accueil" && (
          <Accueil
            seance={seance}
            soir={soir}
            historique={historique}
            onDemarrer={setSeanceEnCours}
          />
        )}
        {onglet === "exercices" && (
          <Exercices onSeanceLibre={setSeanceEnCours} />
        )}
        {onglet === "progres" && <Progres historique={historique} />}
        {onglet === "conseils" && <Conseils />}
      </main>

      <nav className="nav-basse" aria-label="Navigation principale">
        <BoutonOnglet
          actif={onglet === "accueil"}
          icone="accueil"
          libelle="Accueil"
          onClick={() => setOnglet("accueil")}
        />
        <BoutonOnglet
          actif={onglet === "exercices"}
          icone="exercices"
          libelle="Exercices"
          onClick={() => setOnglet("exercices")}
        />
        <BoutonOnglet
          actif={onglet === "progres"}
          icone="progres"
          libelle="Progrès"
          onClick={() => setOnglet("progres")}
        />
        <BoutonOnglet
          actif={onglet === "conseils"}
          icone="conseils"
          libelle="Conseils"
          onClick={() => setOnglet("conseils")}
        />
      </nav>
    </>
  );
}

function BoutonOnglet(props: {
  actif: boolean;
  icone: NomIcone;
  libelle: string;
  onClick: () => void;
}) {
  return (
    <button
      className={props.actif ? "actif" : ""}
      onClick={props.onClick}
      aria-current={props.actif ? "page" : undefined}
    >
      <Icone nom={props.icone} taille={23} epaisseur={props.actif ? 2 : 1.7} />
      {props.libelle}
    </button>
  );
}

/** Pastille indiquant qui réalise l'exercice ou la séance. */
function PastilleRealisation({ realisation }: { realisation: Realisation }) {
  const r = REALISATIONS[realisation];
  return (
    <span
      className={
        realisation === "autonome"
          ? "pastille-mode"
          : "pastille-mode pastille-aide"
      }
    >
      <Icone nom={r.icone} taille={15} epaisseur={2} />
      {r.court}
    </span>
  );
}

/* ————————————————— Accueil ————————————————— */

function Accueil(props: {
  seance: Seance;
  soir: Seance;
  historique: SeanceTerminee[];
  onDemarrer: (s: Seance) => void;
}) {
  const faiteAujourdhui = seanceFaiteAujourdhui(props.historique);
  const serie = serieEnCours(props.historique);
  const absence = joursDepuisDerniereSeance(props.historique);
  const retourApresPause = absence !== null && absence >= 7;

  const heure = new Date().getHours();
  const salutation =
    heure < 12 ? "Bonjour" : heure < 18 ? "Bon après-midi" : "Bonsoir";

  return (
    <>
      <header className="entete">
        <h1>{salutation}</h1>
        <p>Chaque petit mouvement compte. On avance en douceur.</p>
      </header>

      {retourApresPause && (
        <div className="carte">
          <h2>Content de vous revoir</h2>
          <p>
            {absence >= 60
              ? "Cela fait un moment, et ce n'est pas grave du tout."
              : "Quelques jours sans séance, et alors ?"}{" "}
            Une pause n'efface rien de ce que vous avez déjà construit : votre
            meilleure série reste inscrite dans vos progrès.
          </p>
          <p>
            On reprend tranquillement, là où vous en êtes aujourd'hui. C'est le
            seul endroit d'où on peut repartir.
          </p>
        </div>
      )}

      {serie > 0 && (
        <div className="carte carte-serie">
          <span className="icone-serie">
            <Icone nom="serie" taille={30} plein />
          </span>
          <div>
            <span className="grand-chiffre">{serie}</span>
            <p>
              {serie === 1
                ? "jour de suite. La régularité commence ici."
                : "jours de suite. La régularité, c'est votre force."}
            </p>
          </div>
        </div>
      )}

      <CarteSeance
        seance={props.seance}
        surtitre="Séance du jour"
        vedette
        libelleBouton={
          faiteAujourdhui ? "Refaire la séance" : "Commencer la séance"
        }
        onDemarrer={() => props.onDemarrer(props.seance)}
      />

      {faiteAujourdhui && (
        <div className="carte carte-faite">
          <Icone nom="valide" taille={22} epaisseur={2.2} />
          <div>
            <h3>Séance du jour déjà faite</h3>
            <p>
              L'important est la régularité, pas la quantité. Reposez-vous.
            </p>
          </div>
        </div>
      )}

      <CarteSeance
        seance={props.soir}
        surtitre="Séance du soir"
        onDemarrer={() => props.onDemarrer(props.soir)}
        libelleBouton="Ouvrir la séance du soir"
      />

      <div className="bandeau-securite">
        <Icone nom="info" taille={20} />
        <p>
          HemiFit accompagne votre rééducation mais ne remplace pas votre
          kinésithérapeute ni votre médecin. Faites-leur valider ces exercices,
          et arrêtez tout mouvement qui fait mal.
        </p>
      </div>
    </>
  );
}

function CarteSeance(props: {
  seance: Seance;
  surtitre: string;
  vedette?: boolean;
  libelleBouton: string;
  onDemarrer: () => void;
}) {
  const { seance } = props;
  return (
    <div className={props.vedette ? "carte carte-vedette" : "carte"}>
      <div className="entete-carte">
        <span className="surtitre">
          <Icone nom={props.vedette ? "horloge" : "soir"} taille={16} />
          {props.surtitre} · {dureeTotaleMin(seance)} min
        </span>
        <PastilleRealisation realisation={seance.realisation} />
      </div>

      <h2>{seance.titre}</h2>
      <p>{seance.description}</p>
      <p className="detail-seance">
        {seance.exercices.length} exercices,{" "}
        {seance.realisation === "autonome"
          ? "réalisables seul, assis ou allongé."
          : "réalisés par la personne qui vous accompagne."}
      </p>

      <button className="btn btn-principal" onClick={props.onDemarrer}>
        <Icone nom="lecture" taille={20} plein />
        {props.libelleBouton}
      </button>
    </div>
  );
}

/* ————————————————— Exercices ————————————————— */

type Filtre = "tous" | Realisation;

function Exercices(props: { onSeanceLibre: (s: Seance) => void }) {
  const [ouvert, setOuvert] = useState<string | null>(null);
  const [filtre, setFiltre] = useState<Filtre>("tous");

  const ordre: Categorie[] = [
    "massage",
    "sensoriel",
    "main",
    "bras",
    "tronc",
    "jambe",
  ];

  const visibles = EXERCICES.filter(
    e => filtre === "tous" || e.realisation === filtre,
  );

  return (
    <>
      <header className="entete">
        <h1>Tous les exercices</h1>
        <p>
          Touchez un exercice pour voir les consignes, ou lancez-le seul quand
          vous en avez envie.
        </p>
      </header>

      <div className="segments" role="group" aria-label="Filtrer les exercices">
        <button
          className={filtre === "tous" ? "actif" : ""}
          onClick={() => setFiltre("tous")}
        >
          Tous
        </button>
        <button
          className={filtre === "autonome" ? "actif" : ""}
          onClick={() => setFiltre("autonome")}
        >
          <Icone nom="autonome" taille={17} epaisseur={2} />
          Seul
        </button>
        <button
          className={filtre === "tierce-personne" ? "actif" : ""}
          onClick={() => setFiltre("tierce-personne")}
        >
          <Icone nom="aide" taille={17} epaisseur={2} />
          Avec de l'aide
        </button>
      </div>

      {ordre.map(cat => {
        const liste = visibles.filter(e => e.categorie === cat);
        if (liste.length === 0) return null;
        return (
          <section key={cat}>
            <h2 className="titre-section">
              <Icone nom={CATEGORIES[cat].icone} taille={21} />
              {CATEGORIES[cat].titre}
            </h2>
            {liste.map(ex => (
              <CarteExercice
                key={ex.id}
                exercice={ex}
                ouvert={ouvert === ex.id}
                onToggle={() => setOuvert(ouvert === ex.id ? null : ex.id)}
                onLancer={() =>
                  props.onSeanceLibre({
                    titre: ex.nom,
                    description: "Exercice à la carte",
                    realisation: ex.realisation,
                    exercices: [ex],
                  })
                }
              />
            ))}
          </section>
        );
      })}
    </>
  );
}

function CarteExercice(props: {
  exercice: Exercice;
  ouvert: boolean;
  onToggle: () => void;
  onLancer: () => void;
}) {
  const ex = props.exercice;
  return (
    <div>
      <button
        className="ligne-exercice"
        onClick={props.onToggle}
        aria-expanded={props.ouvert}
      >
        <span className="vignette">
          <Icone nom={CATEGORIES[ex.categorie].icone} taille={22} />
        </span>
        <span className="infos">
          <strong>{ex.nom}</strong>
          <span>
            {ex.dosage} · {ex.position}
          </span>
        </span>
        {ex.realisation === "tierce-personne" && (
          <span className="marque-aide" title="Avec une tierce personne">
            <Icone nom="aide" taille={17} epaisseur={2} />
          </span>
        )}
        <span
          className={props.ouvert ? "chevron ouvert" : "chevron"}
          aria-hidden="true"
        >
          <Icone nom="pousse" taille={18} epaisseur={2} />
        </span>
      </button>

      {props.ouvert && (
        <div className="carte">
          <PastilleRealisation realisation={ex.realisation} />
          <p className="objectif">{ex.objectif}</p>
          <ul className="etapes">
            {ex.etapes.map((etape, i) => (
              <li key={i}>{etape}</li>
            ))}
          </ul>
          <button className="btn btn-secondaire" onClick={props.onLancer}>
            <Icone nom="lecture" taille={19} plein />
            Faire cet exercice
          </button>
        </div>
      )}
    </div>
  );
}

/* ————————————————— Progrès ————————————————— */

function Progres(props: { historique: SeanceTerminee[] }) {
  const h = props.historique;
  const recentes = [...h].reverse().slice(0, 14);
  const cetteSemaine = seancesSur7Jours(h);

  return (
    <>
      <header className="entete">
        <h1>Mes progrès</h1>
        <p>La régularité compte plus que la performance.</p>
      </header>

      <div className="grille-stats">
        <Tuile icone="serie" valeur={serieEnCours(h)} legende="jours de suite" />
        <Tuile
          icone="record"
          valeur={meilleureSerie(h)}
          legende="meilleure série, jamais perdue"
        />
        <Tuile icone="valide" valeur={h.length} legende="séances au total" />
        <Tuile
          icone="horloge"
          valeur={minutesTotales(h)}
          legende="minutes de rééducation"
        />
      </div>

      <div className="carte">
        <h3>Cette semaine</h3>
        <p>
          {cetteSemaine > 0
            ? `${cetteSemaine} ${cetteSemaine === 1 ? "séance" : "séances"} sur les 7 derniers jours.`
            : "Aucune séance ces 7 derniers jours. La prochaine vous attend, tranquillement."}
        </p>
        <div className="semaine">
          {derniers7Jours(h).map(jour => (
            <div className="jour" key={jour.date}>
              <div
                className={jour.actif ? "pastille actif" : "pastille"}
                title={jour.date}
              >
                {jour.actif && <Icone nom="valide" taille={16} epaisseur={2.6} />}
              </div>
              <span className="etiquette">{jour.etiquette}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="carte">
        <h3>Rien de tout cela ne se perd</h3>
        <p>
          Ces minutes sont du travail réel accompli par votre cerveau. Une
          pause, même de plusieurs mois, ne les efface pas : vous reprendrez là
          où vous en êtes, jamais à zéro.
        </p>
      </div>

      <div className="carte">
        <h2>Dernières séances</h2>
        {recentes.length === 0 && (
          <p>
            Aucune séance pour l'instant. La première est la plus importante,
            et elle vous attend sur l'accueil.
          </p>
        )}
        {recentes.map((s, i) => (
          <div className="historique-ligne" key={`${s.date}-${i}`}>
            <div>
              <strong>{s.titre}</strong>
              <div className="date">{formaterDate(s.date)}</div>
            </div>
            <div className="minutes">{s.minutes} min</div>
          </div>
        ))}
      </div>
    </>
  );
}

function Tuile(props: { icone: NomIcone; valeur: number; legende: string }) {
  return (
    <div className="carte tuile">
      <span className="tuile-icone">
        <Icone nom={props.icone} taille={20} />
      </span>
      <span className="grand-chiffre">{props.valeur}</span>
      <p>{props.legende}</p>
    </div>
  );
}

function formaterDate(iso: string): string {
  const [a, m, j] = iso.split("-").map(Number);
  const d = new Date(a!, m! - 1, j!);
  return d.toLocaleDateString("fr-FR", {
    weekday: "long",
    day: "numeric",
    month: "long",
  });
}

/* ————————————————— Conseils ————————————————— */

const CONSEILS: { titre: string; texte: string }[] = [
  {
    titre: "Il n'est jamais trop tard pour progresser",
    texte:
      "On a longtemps cru que tout se jouait dans les six premiers mois. Cette idée a été largement remise en cause : le cerveau reste capable de créer de nouveaux chemins pendant des années, et des progrès ont été observés très longtemps après la lésion, chez des personnes qui continuaient à s'entraîner régulièrement. Cela demande de la patience, et les progrès sont souvent lents et partiels, mais ce qui compte n'est pas le temps écoulé depuis la lésion : c'est ce que vous faites à partir d'aujourd'hui.",
  },
  {
    titre: "La sécurité avant tout",
    texte:
      "Votre équilibre étant très altéré, aucun exercice de cette application ne se fait debout. Tout est prévu assis avec le dos soutenu, ou allongé. Bloquez toujours les freins du fauteuil avant de commencer, gardez une amplitude modérée, et ne tentez jamais un transfert ou un redressement seul si vous n'en êtes pas certain.",
  },
  {
    titre: "Pourquoi masser avant de bouger",
    texte:
      "Le massage fait baisser le tonus des muscles spastiques, réchauffe les tissus et réveille les sensations. Un membre massé s'étire beaucoup mieux : c'est pour cela que chaque séance commence par là. Vous pouvez masser autant de fois par jour que vous le souhaitez, il n'y a aucun risque à en faire trop tant que c'est doux.",
  },
  {
    titre: "Le soir, l'aide d'une tierce personne change tout",
    texte:
      "Certaines mobilisations sont impossibles à faire seul : l'épaule, la hanche, l'étirement du mollet. Confiées le soir à une personne qui vous accompagne, elles entretiennent les articulations et prolongent leur effet pendant la nuit. La séance du soir de l'application est écrite pour être suivie par cette personne, consigne par consigne.",
  },
  {
    titre: "Soulager les appuis, tout au long de la journée",
    texte:
      "Rester assis longtemps met en tension les mêmes points d'appui. Prenez l'habitude de décharger vos appuis quelques secondes toutes les vingt à trente minutes, en vous penchant légèrement d'un côté puis de l'autre, mains sur les accoudoirs. C'est court, discret, et cela prévient les rougeurs et les escarres.",
  },
  {
    titre: "Fermer facile, ouvrir difficile : c'est classique",
    texte:
      "Après une lésion cérébrale, les muscles qui ferment la main restent forts et spastiques, tandis que ceux qui l'ouvrent sont affaiblis. L'ouverture se rééduque donc avec de l'aide : le poignet plié vers l'avant desserre naturellement les doigts, la main gauche termine le mouvement, et chaque intention d'ouvrir, même sans mouvement visible, entraîne le cerveau.",
  },
  {
    titre: "Comprendre la spasticité",
    texte:
      "Vos muscles droits sont trop toniques : ils se contractent seuls et résistent, surtout quand on les étire vite. Ce n'est pas de la mauvaise volonté de votre main, c'est un réflexe. La bonne nouvelle : la lenteur, le calme et les étirements prolongés le font baisser.",
  },
  {
    titre: "Lent, toujours plus lent",
    texte:
      "Un mouvement rapide ou forcé déclenche le réflexe spastique et la main se referme encore plus. Étirez très lentement, arrêtez-vous dès que ça résiste, respirez, et attendez que cela lâche de soi-même.",
  },
  {
    titre: "La chaleur détend",
    texte:
      "La spasticité diminue avec la chaleur : faites les exercices de la main après une douche chaude, ou passez la main droite quelques minutes sous l'eau chaude en testant la température avec la main gauche. Le froid, le stress et la fatigue, eux, l'augmentent.",
  },
  {
    titre: "Le cerveau apprend par la répétition",
    texte:
      "La neuroplasticité se nourrit de répétitions courtes et fréquentes. Quinze minutes chaque jour valent mieux qu'une heure une fois par semaine.",
  },
  {
    titre: "L'intention compte déjà",
    texte:
      "Même si le mouvement ne vient pas, le fait d'essayer, d'imaginer et de vouloir bouger active les bonnes zones du cerveau. Aucun essai n'est perdu.",
  },
  {
    titre: "Parlez de votre spasticité à vos soignants",
    texte:
      "Il existe des traitements spécifiques de la spasticité : kinésithérapie, médicaments, injections ciblées, attelles de posture. Ils complètent très bien ces exercices. Si la spasticité vous gêne beaucoup, c'est une vraie question à poser à votre médecin.",
  },
  {
    titre: "Les signaux pour s'arrêter",
    texte:
      "Douleur vive, vertige, essoufflement inhabituel, fatigue soudaine, rougeur qui ne s'efface pas sur un point d'appui : on s'arrête, on se repose, et on en parle à son médecin si cela se répète.",
  },
];

function Conseils() {
  return (
    <>
      <header className="entete">
        <h1>Conseils</h1>
        <p>Quelques repères pour une rééducation sereine.</p>
      </header>

      {CONSEILS.map(c => (
        <div className="carte" key={c.titre}>
          <h3>{c.titre}</h3>
          <p>{c.texte}</p>
        </div>
      ))}

      <div className="bandeau-securite">
        <Icone nom="info" taille={20} />
        <p>
          Ces exercices sont doux et classiques en rééducation, mais chaque
          situation est unique : faites-les valider par votre kinésithérapeute
          ou votre médecin, et signalez-leur toute douleur ou changement.
        </p>
      </div>
    </>
  );
}
