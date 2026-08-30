/** Suivi de progression, sauvegardé localement sur l'appareil. */

export interface SeanceTerminee {
  /** Date au format AAAA-MM-JJ (heure locale). */
  date: string;
  titre: string;
  minutes: number;
  exercicesFaits: number;
  /** Ressenti après la séance : 1 = difficile, 2 = correct, 3 = bien. */
  ressenti?: 1 | 2 | 3;
}

const CLE = "hemifit.progression.v1";

export function dateISO(d: Date = new Date()): string {
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const j = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${j}`;
}

export function chargerHistorique(): SeanceTerminee[] {
  try {
    const brut = localStorage.getItem(CLE);
    if (!brut) return [];
    const donnees = JSON.parse(brut);
    return Array.isArray(donnees) ? donnees : [];
  } catch {
    return [];
  }
}

export function enregistrerSeance(seance: SeanceTerminee): SeanceTerminee[] {
  const historique = chargerHistorique();
  historique.push(seance);
  try {
    localStorage.setItem(CLE, JSON.stringify(historique));
  } catch {
    // Stockage indisponible (navigation privée…) : l'app continue sans sauvegarde.
  }
  return historique;
}

export function seanceFaiteAujourdhui(historique: SeanceTerminee[]): boolean {
  const aujourdhui = dateISO();
  return historique.some(s => s.date === aujourdhui);
}

/** Nombre de jours consécutifs avec au moins une séance, en comptant aujourd'hui ou hier. */
export function serieEnCours(historique: SeanceTerminee[]): number {
  const jours = new Set(historique.map(s => s.date));
  let serie = 0;
  const curseur = new Date();
  // La série n'est pas cassée si la séance du jour n'est pas encore faite.
  if (!jours.has(dateISO(curseur))) {
    curseur.setDate(curseur.getDate() - 1);
  }
  while (jours.has(dateISO(curseur))) {
    serie += 1;
    curseur.setDate(curseur.getDate() - 1);
  }
  return serie;
}

/**
 * Meilleure série jamais atteinte. Elle n'est jamais perdue : une
 * interruption, même longue, n'efface pas ce qui a été accompli.
 */
export function meilleureSerie(historique: SeanceTerminee[]): number {
  const jours = [...new Set(historique.map(s => s.date))].sort();
  let meilleure = 0;
  let courante = 0;
  let precedent: string | null = null;

  for (const jour of jours) {
    if (precedent && estLeLendemain(precedent, jour)) {
      courante += 1;
    } else {
      courante = 1;
    }
    meilleure = Math.max(meilleure, courante);
    precedent = jour;
  }
  return meilleure;
}

function estLeLendemain(veille: string, jour: string): boolean {
  const d = new Date(`${veille}T12:00:00`);
  d.setDate(d.getDate() + 1);
  return dateISO(d) === jour;
}

/**
 * Nombre de jours depuis la dernière séance ; `null` si aucune séance
 * n'a jamais été faite.
 */
export function joursDepuisDerniereSeance(
  historique: SeanceTerminee[],
): number | null {
  if (historique.length === 0) return null;
  const derniere = historique
    .map(s => s.date)
    .sort()
    .at(-1)!;
  const d = new Date(`${derniere}T12:00:00`);
  const aujourdhui = new Date();
  aujourdhui.setHours(12, 0, 0, 0);
  const ms = aujourdhui.getTime() - d.getTime();
  return Math.max(0, Math.round(ms / 86_400_000));
}

export function seancesSur7Jours(historique: SeanceTerminee[]): number {
  const jours = new Set<string>();
  const curseur = new Date();
  for (let i = 0; i < 7; i++) {
    jours.add(dateISO(curseur));
    curseur.setDate(curseur.getDate() - 1);
  }
  return historique.filter(s => jours.has(s.date)).length;
}

export function minutesTotales(historique: SeanceTerminee[]): number {
  return historique.reduce((t, s) => t + s.minutes, 0);
}
