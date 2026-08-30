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
