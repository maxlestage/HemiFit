import { useEffect, useRef, useState } from "react";
import { CATEGORIES, type Seance } from "./data/exercises";
import {
  dateISO,
  enregistrerSeance,
  type SeanceTerminee,
} from "./lib/progression";

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
    return (
      <FinDeSeance
        seance={seance}
        onTerminee={onTerminee}
      />
    );
  }

  return (
    <div className="seance-plein-ecran" role="region" aria-label="Séance en cours">
      <div className="seance-progression" aria-hidden="true">
        {seance.exercices.map((_, i) => (
          <span key={i} className={i <= indice ? "fait" : ""} />
        ))}
      </div>

      <p style={{ margin: "0 0 4px", color: "var(--encre-douce)" }}>
        Exercice {indice + 1} sur {seance.exercices.length}
      </p>

      <span className="badge">
        {CATEGORIES[exercice.categorie].emoji}{" "}
        {CATEGORIES[exercice.categorie].titre} · {exercice.position}
      </span>

      <h1 style={{ margin: "0 0 6px", fontSize: "1.5rem" }}>{exercice.nom}</h1>
      <p style={{ margin: "0 0 8px", color: "var(--encre-douce)" }}>
        {exercice.objectif}
      </p>
      <p style={{ margin: "0 0 4px", fontWeight: 650 }}>{exercice.dosage}</p>

      <Minuteur
        cle={`${indice}-${exercice.id}`}
        dureeSec={exercice.dureeSec}
      />

      <ul className="etapes">
        {exercice.etapes.map((etape, i) => (
          <li key={i}>{etape}</li>
        ))}
      </ul>

      <div className="zone-boutons-bas">
        <button className="btn btn-principal" onClick={suivant}>
          {dernierExercice ? "Terminer la séance ✅" : "Exercice suivant →"}
        </button>
        <button className="btn btn-discret" onClick={onQuitter}>
          Arrêter la séance
        </button>
      </div>
    </div>
  );
}

/** Minuteur avec pause, remis à zéro à chaque exercice via `cle`. */
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

  return (
    <div style={{ textAlign: "center" }}>
      <div className="minuteur" aria-live="polite">
        {restant > 0 ? `${min}:${sec}` : "Bien joué 💚"}
      </div>
      {restant > 0 && (
        <button
          className="btn btn-secondaire"
          onClick={() => setEnPause(p => !p)}
          style={{ minHeight: 56 }}
        >
          {enPause ? "Reprendre ▶" : "Pause ⏸"}
        </button>
      )}
    </div>
  );
}

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
        <span className="emoji" aria-hidden="true">
          🎉
        </span>
        <h1 style={{ margin: "0 0 8px" }}>Séance terminée !</h1>
        <p style={{ color: "var(--encre-douce)" }}>
          Chaque séance renforce les nouveaux chemins de votre cerveau. Soyez
          fier de vous.
        </p>
      </div>

      <h2 style={{ fontSize: "1.15rem", margin: "0 0 4px" }}>
        Comment vous sentez-vous ?
      </h2>
      <div className="choix-ressenti">
        <BoutonRessenti
          emoji="😮‍💨"
          libelle="Difficile"
          choisi={ressenti === 1}
          onClick={() => setRessenti(1)}
        />
        <BoutonRessenti
          emoji="🙂"
          libelle="Correct"
          choisi={ressenti === 2}
          onClick={() => setRessenti(2)}
        />
        <BoutonRessenti
          emoji="😊"
          libelle="Bien"
          choisi={ressenti === 3}
          onClick={() => setRessenti(3)}
        />
      </div>

      <div className="zone-boutons-bas">
        <button className="btn btn-principal" onClick={enregistrer}>
          Enregistrer ma séance 💾
        </button>
      </div>
    </div>
  );
}

function BoutonRessenti(props: {
  emoji: string;
  libelle: string;
  choisi: boolean;
  onClick: () => void;
}) {
  return (
    <button
      className={props.choisi ? "choisi" : ""}
      onClick={props.onClick}
      aria-pressed={props.choisi}
    >
      <span className="emoji" aria-hidden="true">
        {props.emoji}
      </span>
      {props.libelle}
    </button>
  );
}
