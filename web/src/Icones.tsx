/**
 * Jeu d'icônes vectorielles de HemiFit.
 *
 * Tracé uniforme (24×24, épaisseur 1.75, extrémités arrondies) pour un
 * rendu sobre et cohérent. Les icônes héritent de la couleur du texte
 * via `currentColor` et sont purement décoratives : chaque icône est
 * toujours accompagnée d'un libellé lisible.
 */

export type NomIcone =
  | "accueil"
  | "exercices"
  | "progres"
  | "conseils"
  | "massage"
  | "sensoriel"
  | "main"
  | "bras"
  | "tronc"
  | "jambe"
  | "soir"
  | "autonome"
  | "aide"
  | "serie"
  | "record"
  | "valide"
  | "lecture"
  | "pause"
  | "horloge"
  | "info"
  | "pousse";

const TRACES: Record<NomIcone, React.ReactNode> = {
  // ——— Navigation ———
  accueil: (
    <>
      <path d="M3.5 10.2 12 3.6l8.5 6.6" />
      <path d="M5.8 9v10.4a1 1 0 0 0 1 1h10.4a1 1 0 0 0 1-1V9" />
      <path d="M9.8 20.4v-5.6h4.4v5.6" />
    </>
  ),
  exercices: (
    <>
      <path d="M4 6.5h.01M4 12h.01M4 17.5h.01" />
      <path d="M8.5 6.5H20M8.5 12H20M8.5 17.5H20" />
    </>
  ),
  progres: (
    <>
      <path d="M3.5 3.5v15a2 2 0 0 0 2 2h15" />
      <path d="M7 15.5l3.8-4.4 3.1 2.6L20 7" />
    </>
  ),
  conseils: (
    <>
      <path d="M9.2 17.5h5.6" />
      <path d="M10 20.5h4" />
      <path d="M12 3.5a5.6 5.6 0 0 0-3.3 10.1c.5.4.8 1 .8 1.6v.3h5v-.3c0-.6.3-1.2.8-1.6A5.6 5.6 0 0 0 12 3.5Z" />
    </>
  ),

  // ——— Familles d'exercices ———
  // Ondes de détente au-dessus d'une main : le massage.
  massage: (
    <>
      <path d="M3.4 8.2c1.9-2 3.3-2 5.2 0s3.3 2 5.2 0 3.3-2 5.2 0" />
      <path d="M3.4 13c1.9-2 3.3-2 5.2 0s3.3 2 5.2 0 3.3-2 5.2 0" />
      <path d="M3.4 17.8c1.9-2 3.3-2 5.2 0s3.3 2 5.2 0 3.3-2 5.2 0" />
    </>
  ),
  // Point de contact et ondes : la sensation.
  sensoriel: (
    <>
      <circle cx="12" cy="12" r="2.2" />
      <path d="M7.8 16.2a6 6 0 0 1 0-8.4" />
      <path d="M16.2 7.8a6 6 0 0 1 0 8.4" />
      <path d="M4.9 19.1a10 10 0 0 1 0-14.2" />
      <path d="M19.1 4.9a10 10 0 0 1 0 14.2" />
    </>
  ),
  main: (
    <>
      <path d="M9.5 11V5.4a1.5 1.5 0 0 1 3 0V11" />
      <path d="M12.5 11V6.6a1.5 1.5 0 0 1 3 0V11" />
      <path d="M15.5 11.4V8.8a1.5 1.5 0 0 1 3 0v5.4a6.4 6.4 0 0 1-6.4 6.4h-.7a5.9 5.9 0 0 1-4.2-1.7L4 15.6a1.6 1.6 0 0 1 2.3-2.3l1.7 1.7" />
      <path d="M9.5 11V9.4a1.5 1.5 0 0 0-3 0V15" />
    </>
  ),
  // Segments et articulations : le bras.
  bras: (
    <>
      <circle cx="5.9" cy="5.9" r="2.3" />
      <path d="M7.6 7.5 12 11.9" />
      <circle cx="13.6" cy="13.5" r="2.3" />
      <path d="M15.2 15.2 18.6 18.6" />
      <circle cx="20" cy="20" r="1.5" />
    </>
  ),
  // Buste assis, dos soutenu : le tronc.
  tronc: (
    <>
      <circle cx="9.4" cy="5.2" r="2.5" />
      <path d="M9.4 7.9v6.9h6.9" />
      <path d="M16.3 14.8v5.4" />
      <path d="M4.6 8.6v11.6" />
    </>
  ),
  // Segments jambe assise (hanche, genou, pied).
  jambe: (
    <>
      <circle cx="5.6" cy="6.4" r="2.3" />
      <path d="M7.5 7.6h8.2" />
      <circle cx="17.6" cy="8.2" r="2.3" />
      <path d="M17.6 10.5v6.4" />
      <path d="M15.6 17.4h4.6" />
    </>
  ),

  // ——— Modes de réalisation ———
  autonome: (
    <>
      <circle cx="12" cy="7" r="3.1" />
      <path d="M5.6 20.4a6.4 6.4 0 0 1 12.8 0" />
    </>
  ),
  aide: (
    <>
      <circle cx="8.6" cy="7.4" r="2.8" />
      <path d="M3 20.2a5.6 5.6 0 0 1 11.2 0" />
      <path d="M16.2 5.1a2.8 2.8 0 0 1 0 5.4" />
      <path d="M17.4 14.9a5.6 5.6 0 0 1 3.6 5.3" />
    </>
  ),
  soir: (
    <>
      <path d="M20.2 14.4A8.4 8.4 0 0 1 9.6 3.8a8.4 8.4 0 1 0 10.6 10.6Z" />
    </>
  ),

  // ——— Suivi ———
  serie: (
    <>
      <path d="M12 3.2c2.9 2.7 4.6 5.2 4.6 8.1a4.6 4.6 0 1 1-9.2 0c0-1.5.6-2.8 1.6-4 .4 1.1 1.1 1.8 2 2.1-.4-2.4.2-4.5 1-6.2Z" />
    </>
  ),
  record: (
    <>
      <path d="m12 3.6 2.6 5.3 5.8.8-4.2 4.1 1 5.8-5.2-2.7-5.2 2.7 1-5.8L3.6 9.7l5.8-.8Z" />
    </>
  ),
  valide: <path d="m4.8 12.6 4.8 4.8L19.2 7.2" />,
  horloge: (
    <>
      <circle cx="12" cy="12" r="8.6" />
      <path d="M12 7.2V12l3.2 2" />
    </>
  ),
  info: (
    <>
      <circle cx="12" cy="12" r="8.6" />
      <path d="M12 11v5.4" />
      <path d="M12 7.8h.01" />
    </>
  ),

  // ——— Commandes ———
  lecture: <path d="M8.4 5.4 19 12 8.4 18.6Z" />,
  pause: (
    <>
      <path d="M9.4 5.6v12.8" />
      <path d="M14.6 5.6v12.8" />
    </>
  ),
  pousse: <path d="m6 9.5 6 6 6-6" />,
};

interface Props {
  nom: NomIcone;
  /** Taille en pixels (carré). */
  taille?: number;
  /** Épaisseur du tracé ; augmentée pour les petites tailles. */
  epaisseur?: number;
  /** Remplir plutôt que tracer (flamme, étoile, lecture…). */
  plein?: boolean;
  className?: string;
}

export function Icone({
  nom,
  taille = 24,
  epaisseur = 1.75,
  plein = false,
  className,
}: Props) {
  return (
    <svg
      className={className}
      width={taille}
      height={taille}
      viewBox="0 0 24 24"
      fill={plein ? "currentColor" : "none"}
      stroke="currentColor"
      strokeWidth={plein ? 0 : epaisseur}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
      focusable="false"
    >
      {TRACES[nom]}
    </svg>
  );
}
