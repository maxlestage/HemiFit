import { useEffect, useRef, useState } from "react";
import { CATEGORIES, REALISATIONS, type Seance } from "./data/exercises";
import {
  dateISO,
  enregistrerSeance,
  type SeanceTerminee,
} from "./lib/progression";
import { Icone } from "./Icones";

interface Props {
  seance: Seance;
  onQuitter: () => void;
  onTerminee: (historique: SeanceTerminee[]) => void;
}

/**
 * Lecteur de séance guidée : un exercice à la fois, un grand minuteur,
 * et des boutons larges en bas d'écran, accessibles au pouce gauche.
 */
export function LecteurSeance({ seance, onQuitter, onTerminee }: Props) {
  const [indice, setIndice] = useState(0);
  const [phase, setPhase] = useState<"exercice" | "fin">("exercice");

  const exercice = seance.exercices[indice]!;
  const dernierExercice = indice === seance.exercices.length - 1;

  function suivant() {
    if (dernierExercice) {
      setPhase("fin");
    } else {
      setIndice(indice + 1);
    }
  }

  if (phase === "fin") {
    return <FinDeSeance seance={seance} onTerminee={onTerminee} />;
  }

  return (
    <div className="seance-plein-ecran" role="region" aria-label="Séance en cours">
      <div className="seance-progression" aria-hidden="true">
        {seance.exercices.map((_, i) => (
          <span key={i} className={i <= indice ? "fait" : ""} />
        ))}
      </div>

      <p className="compteur-exercice">
        Exercice {indice + 1} sur {seance.exercices.length}
      </p>

      <div className="entete-carte">
        <span className="badge">
          <Icone nom={CATEGORIES[exercice.categorie].icone} taille={16} />
          {CATEGORIES[exercice.categorie].titre} · {exercice.position}
        </span>
        {exercice.realisation === "tierce-personne" && (
          <span className="pastille-mode pastille-aide">
            <Icone nom="aide" taille={15} epaisseur={2} />
            {REALISATIONS["tierce-personne"].court}
          </span>
        )}
      </div>

      <h1 className="titre-exercice">{exercice.nom}</h1>
      <p className="objectif">{exercice.objectif}</p>
      <p className="dosage">{exercice.dosage}</p>

      <Minuteur cle={`${indice}-${exercice.id}`} dureeSec={exercice.dureeSec} />

      <ul className="etapes">
        {exercice.etapes.map((etape, i) => (
          <li key={i}>{etape}</li>
        ))}
      </ul>

      <div className="zone-boutons-bas">
        <button className="btn btn-principal" onClick={suivant}>
          {dernierExercice ? (
            <>
              <Icone nom="valide" taille={20} epaisseur={2.4} />
              Terminer la séance
            </>
          ) : (
            <>
              <Icone nom="lecture" taille={19} plein />
              Exercice suivant
            </>
          )}
        </button>
        <button className="btn btn-discret" onClick={onQuitter}>
          Arrêter la séance
        </button>
      </div>
    </div>
  );
}

/** Rayon et circonférence de l'anneau du minuteur. */
const RAYON = 94;
const CIRCONFERENCE = 2 * Math.PI * RAYON;

/** Minuteur circulaire avec pause, remis à zéro à chaque exercice via `cle`. */
function Minuteur({ cle, dureeSec }: { cle: string; dureeSec: number }) {
  const [restant, setRestant] = useState(dureeSec);
  const [enPause, setEnPause] = useState(false);
  const cleRef = useRef(cle);

  if (cleRef.current !== cle) {
    cleRef.current = cle;
    setRestant(dureeSec);
    setEnPause(false);
  }

  useEffect(() => {
    if (enPause || restant <= 0) return;
    const t = setInterval(() => setRestant(r => Math.max(0, r - 1)), 1000);
    return () => clearInterval(t);
  }, [enPause, restant > 0, cle]);

  const min = Math.floor(restant / 60);
  const sec = String(restant % 60).padStart(2, "0");
  const proportion = dureeSec > 0 ? restant / dureeSec : 0;

  return (
    <div className="bloc-minuteur">
      <div className="minuteur-anneau">
        <svg viewBox="0 0 208 208" aria-hidden="true">
          <defs>
            <linearGradient id="degradeMinuteur" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stopColor="var(--accent-vif)" />
              <stop offset="100%" stopColor="var(--accent)" />
            </linearGradient>
          </defs>
          <circle className="piste" cx="104" cy="104" r={RAYON} />
          <circle
            className="jauge"
            cx="104"
            cy="104"
            r={RAYON}
            strokeDasharray={CIRCONFERENCE}
            strokeDashoffset={CIRCONFERENCE * (1 - proportion)}
          />
        </svg>
        {restant > 0 ? (
          <span className="minuteur-texte" aria-live="polite">
            {min}:{sec}
          </span>
        ) : (
          <span className="minuteur-fini" aria-live="polite">
            <Icone nom="valide" taille={40} epaisseur={2.2} />
            Terminé
          </span>
        )}
      </div>

      {restant > 0 && (
        <button
          className="btn btn-secondaire btn-pause"
          onClick={() => setEnPause(p => !p)}
        >
          <Icone nom={enPause ? "lecture" : "pause"} taille={18} plein={enPause} />
          {enPause ? "Reprendre" : "Pause"}
        </button>
      )}
    </div>
  );
}

const RESSENTIS = [
  { valeur: 1 as const, libelle: "Difficile" },
  { valeur: 2 as const, libelle: "Correct" },
  { valeur: 3 as const, libelle: "Bien" },
];

/** Écran de fin : ressenti puis enregistrement de la séance. */
function FinDeSeance({
  seance,
  onTerminee,
}: {
  seance: Seance;
  onTerminee: (historique: SeanceTerminee[]) => void;
}) {
  const [ressenti, setRessenti] = useState<1 | 2 | 3 | undefined>();

  function enregistrer() {
    const minutes = Math.max(
      1,
      Math.round(seance.exercices.reduce((t, e) => t + e.dureeSec, 0) / 60),
    );
    const historique = enregistrerSeance({
      date: dateISO(),
      titre: seance.titre,
      minutes,
      exercicesFaits: seance.exercices.length,
      ressenti,
    });
    onTerminee(historique);
  }

  return (
    <div className="seance-plein-ecran">
      <div className="felicitations">
        <span className="sceau-fin">
          <Icone nom="valide" taille={44} epaisseur={2.2} />
        </span>
        <h1>Séance terminée</h1>
        <p>
          Chaque séance renforce les nouveaux chemins de votre cerveau. Vous
          pouvez en être fier.
        </p>
      </div>

      <h2 className="question-ressenti">Comment vous sentez-vous ?</h2>
      <div className="choix-ressenti">
        {RESSENTIS.map(r => (
          <button
            key={r.valeur}
            className={ressenti === r.valeur ? "choisi" : ""}
            onClick={() => setRessenti(r.valeur)}
            aria-pressed={ressenti === r.valeur}
          >
            <span className="jauge-ressenti" aria-hidden="true">
              {[1, 2, 3].map(n => (
                <i key={n} className={n <= r.valeur ? "remplie" : ""} />
              ))}
            </span>
            {r.libelle}
          </button>
        ))}
      </div>

      <div className="zone-boutons-bas">
        <button className="btn btn-principal" onClick={enregistrer}>
          <Icone nom="valide" taille={20} epaisseur={2.4} />
          Enregistrer la séance
        </button>
      </div>
    </div>
  );
}
