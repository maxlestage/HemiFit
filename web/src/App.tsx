import { useMemo, useState } from "react";
import {
  CATEGORIES,
  EXERCICES,
  dureeTotaleMin,
  seanceDuJour,
  type Categorie,
  type Exercice,
  type Seance,
} from "./data/exercises";
import {
  chargerHistorique,
  minutesTotales,
  seanceFaiteAujourdhui,
  seancesSur7Jours,
  serieEnCours,
  type SeanceTerminee,
} from "./lib/progression";
import { LecteurSeance } from "./LecteurSeance";

type Onglet = "accueil" | "exercices" | "progres" | "conseils";

export function App() {
  const [onglet, setOnglet] = useState<Onglet>("accueil");
  const [seanceEnCours, setSeanceEnCours] = useState<Seance | null>(null);
  const [historique, setHistorique] = useState<SeanceTerminee[]>(() =>
    chargerHistorique(),
  );

  const seance = useMemo(() => seanceDuJour(), []);

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
            historique={historique}
            onDemarrer={() => setSeanceEnCours(seance)}
          />
        )}
        {onglet === "exercices" && (
          <Exercices onSeanceLibre={ex => setSeanceEnCours(ex)} />
        )}
        {onglet === "progres" && <Progres historique={historique} />}
        {onglet === "conseils" && <Conseils />}
      </main>

      <nav className="nav-basse" aria-label="Navigation principale">
        <BoutonOnglet
          actif={onglet === "accueil"}
          icone="🏠"
          libelle="Accueil"
          onClick={() => setOnglet("accueil")}
        />
        <BoutonOnglet
          actif={onglet === "exercices"}
          icone="📋"
          libelle="Exercices"
          onClick={() => setOnglet("exercices")}
        />
        <BoutonOnglet
          actif={onglet === "progres"}
          icone="📈"
          libelle="Progrès"
          onClick={() => setOnglet("progres")}
        />
        <BoutonOnglet
          actif={onglet === "conseils"}
          icone="💡"
          libelle="Conseils"
          onClick={() => setOnglet("conseils")}
        />
      </nav>
    </>
  );
}

function BoutonOnglet(props: {
  actif: boolean;
  icone: string;
  libelle: string;
  onClick: () => void;
}) {
  return (
    <button
      className={props.actif ? "actif" : ""}
      onClick={props.onClick}
      aria-current={props.actif ? "page" : undefined}
    >
      <span className="icone" aria-hidden="true">
        {props.icone}
      </span>
      {props.libelle}
    </button>
  );
}

/* ————————————————— Accueil ————————————————— */

function Accueil(props: {
  seance: Seance;
  historique: SeanceTerminee[];
  onDemarrer: () => void;
}) {
  const faiteAujourdhui = seanceFaiteAujourdhui(props.historique);
  const serie = serieEnCours(props.historique);
  const minutes = dureeTotaleMin(props.seance);

  const salutation =
    new Date().getHours() < 12
      ? "Bonjour"
      : new Date().getHours() < 18
        ? "Bon après-midi"
        : "Bonsoir";

  return (
    <>
      <header className="entete">
        <h1>{salutation} 👋</h1>
        <p>Chaque petit mouvement compte. On y va en douceur.</p>
      </header>

      {serie > 0 && (
        <div className="carte">
          <span className="grand-chiffre">🔥 {serie}</span>
          <p>
            {serie === 1
              ? "jour de suite — bien joué, la régularité commence ici !"
              : "jours de suite — la régularité, c'est votre superpouvoir."}
          </p>
        </div>
      )}

      <div className="carte">
        <span className="badge">Séance du jour · {minutes} min environ</span>
        <h2>{props.seance.titre}</h2>
        <p>{props.seance.description}</p>
        <p>
          {props.seance.exercices.length} exercices, tous assis ou allongé,
          avec l'aide de votre main gauche.
        </p>
        <button className="btn btn-principal" onClick={props.onDemarrer}>
          {faiteAujourdhui ? "Refaire une séance ✨" : "Commencer la séance ▶"}
        </button>
      </div>

      {faiteAujourdhui && (
        <div className="carte">
          <h3>✅ Séance du jour déjà faite</h3>
          <p>
            Bravo ! Reposez-vous, l'important c'est la régularité, pas la
            quantité.
          </p>
        </div>
      )}

      <div className="bandeau-securite">
        ⚕️ HemiFit accompagne votre rééducation mais ne remplace pas votre
        kinésithérapeute ni votre médecin. Montrez-leur ces exercices, et
        arrêtez tout mouvement qui fait mal.
      </div>
    </>
  );
}

/* ————————————————— Exercices ————————————————— */

function Exercices(props: { onSeanceLibre: (s: Seance) => void }) {
  const [ouvert, setOuvert] = useState<string | null>(null);

  const ordre: Categorie[] = ["sensoriel", "main", "bras", "jambe"];

  return (
    <>
      <header className="entete">
        <h1>Tous les exercices</h1>
        <p>
          Touchez un exercice pour voir les consignes, ou lancez-le seul quand
          vous en avez envie.
        </p>
      </header>

      {ordre.map(cat => (
        <section key={cat}>
          <h2 style={{ margin: "20px 0 12px", fontSize: "1.2rem" }}>
            {CATEGORIES[cat].emoji} {CATEGORIES[cat].titre}
          </h2>
          {EXERCICES.filter(e => e.categorie === cat).map(ex => (
            <CarteExercice
              key={ex.id}
              exercice={ex}
              ouvert={ouvert === ex.id}
              onToggle={() => setOuvert(ouvert === ex.id ? null : ex.id)}
              onLancer={() =>
                props.onSeanceLibre({
                  titre: ex.nom,
                  description: "Exercice à la carte",
                  exercices: [ex],
                })
              }
            />
          ))}
        </section>
      ))}
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
        <span className="emoji" aria-hidden="true">
          {CATEGORIES[ex.categorie].emoji}
        </span>
        <span className="infos">
          <strong>{ex.nom}</strong>
          <span>
            {ex.dosage} · {ex.position}
          </span>
        </span>
        <span aria-hidden="true">{props.ouvert ? "▴" : "▾"}</span>
      </button>

      {props.ouvert && (
        <div className="carte">
          <p>
            <em>{ex.objectif}</em>
          </p>
          <ul className="etapes">
            {ex.etapes.map((etape, i) => (
              <li key={i}>{etape}</li>
            ))}
          </ul>
          <button className="btn btn-secondaire" onClick={props.onLancer}>
            Faire cet exercice ▶
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

  return (
    <>
      <header className="entete">
        <h1>Mes progrès</h1>
        <p>La régularité compte plus que la performance.</p>
      </header>

      <div className="grille-stats">
        <div className="carte">
          <span className="grand-chiffre">{serieEnCours(h)}</span>
          <p>jours de suite</p>
        </div>
        <div className="carte">
          <span className="grand-chiffre">{seancesSur7Jours(h)}</span>
          <p>séances sur 7 jours</p>
        </div>
        <div className="carte">
          <span className="grand-chiffre">{h.length}</span>
          <p>séances au total</p>
        </div>
        <div className="carte">
          <span className="grand-chiffre">{minutesTotales(h)}</span>
          <p>minutes de rééducation</p>
        </div>
      </div>

      <div className="carte">
        <h2>Dernières séances</h2>
        {recentes.length === 0 && (
          <p>
            Aucune séance pour l'instant. La première est la plus importante —
            elle vous attend sur l'accueil. 💚
          </p>
        )}
        {recentes.map((s, i) => (
          <div className="historique-ligne" key={`${s.date}-${i}`}>
            <div>
              <strong>{s.titre}</strong>
              <div className="date">{formaterDate(s.date)}</div>
            </div>
            <div style={{ textAlign: "right", whiteSpace: "nowrap" }}>
              {s.minutes} min{" "}
              {s.ressenti === 3 ? "😊" : s.ressenti === 2 ? "🙂" : s.ressenti === 1 ? "😮‍💨" : ""}
            </div>
          </div>
        ))}
      </div>
    </>
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

function Conseils() {
  return (
    <>
      <header className="entete">
        <h1>Conseils</h1>
        <p>Quelques repères pour une rééducation sereine.</p>
      </header>

      <div className="carte">
        <h3>👐 Fermer facile, ouvrir difficile : c'est classique</h3>
        <p>
          Après une lésion cérébrale, les muscles qui ferment la main restent
          forts (et spastiques), tandis que ceux qui l'ouvrent sont
          affaiblis. L'ouverture se rééduque donc avec de l'aide : le poignet
          plié vers l'avant desserre naturellement les doigts, la main gauche
          termine le mouvement, et chaque intention d'ouvrir — même sans
          mouvement visible — entraîne le cerveau.
        </p>
      </div>

      <div className="carte">
        <h3>🌊 Comprendre la spasticité</h3>
        <p>
          Vos muscles droits sont trop « toniques » : ils se contractent tout
          seuls et résistent, surtout quand on les étire vite. Ce n'est pas de
          la mauvaise volonté de votre main — c'est un réflexe. La bonne
          nouvelle : la lenteur, le calme et les étirements prolongés la font
          baisser.
        </p>
      </div>

      <div className="carte">
        <h3>🐢 Lent, toujours plus lent</h3>
        <p>
          Un mouvement rapide ou forcé déclenche le réflexe spastique : la
          main se referme encore plus. Étirez très lentement, arrêtez-vous dès
          que ça résiste, respirez… et attendez que ça lâche tout seul. Ça
          vient toujours.
        </p>
      </div>

      <div className="carte">
        <h3>🛁 La chaleur détend</h3>
        <p>
          La spasticité diminue avec la chaleur : faites les exercices de la
          main après une douche chaude, ou passez la main droite quelques
          minutes sous l'eau chaude (testez la température avec la main
          gauche). Le froid, le stress et la fatigue, eux, l'augmentent.
        </p>
      </div>

      <div className="carte">
        <h3>🧠 Le cerveau apprend par la répétition</h3>
        <p>
          Après une lésion cérébrale, le cerveau peut créer de nouveaux
          chemins : c'est la neuroplasticité. Elle se nourrit de répétitions
          courtes et fréquentes — 15 minutes par jour valent mieux qu'une
          heure une fois par semaine.
        </p>
      </div>

      <div className="carte">
        <h3>👀 Regardez votre côté droit</h3>
        <p>
          Pendant les exercices, regardez votre main ou votre jambe droite
          bouger, même quand c'est la main gauche qui aide. Voir le mouvement
          aide le cerveau à le réapprendre.
        </p>
      </div>

      <div className="carte">
        <h3>✋ L'intention compte déjà</h3>
        <p>
          Même si le mouvement ne vient pas, le fait d'essayer, d'imaginer et
          de vouloir bouger active les bonnes zones du cerveau. Aucun essai
          n'est perdu.
        </p>
      </div>

      <div className="carte">
        <h3>💬 Parlez de votre spasticité à vos soignants</h3>
        <p>
          Il existe des traitements spécifiques de la spasticité
          (kinésithérapie, médicaments, injections ciblées, attelles) qui
          complètent très bien ces exercices. Si la spasticité vous gêne
          beaucoup, c'est une vraie question à poser à votre médecin.
        </p>
      </div>

      <div className="carte">
        <h3>🛑 Les signaux pour s'arrêter</h3>
        <p>
          Douleur vive, vertige, essoufflement inhabituel, fatigue soudaine :
          on s'arrête, on se repose, et on en parle à son médecin si ça se
          répète.
        </p>
      </div>

      <div className="bandeau-securite">
        ⚕️ Ces exercices sont doux et classiques en rééducation, mais chaque
        situation est unique : faites-les valider par votre kinésithérapeute
        ou votre médecin, et signalez-leur toute douleur ou changement.
      </div>
    </>
  );
}
