# HemiFit — site web 🌐

Site **mobile-first** de rééducation en douceur, construit avec :

- **Bun** (serveur fullstack natif avec rechargement à chaud — aucun autre bundler nécessaire) ;
- **React 19** ;
- **TypeScript** (mode strict).

Toutes les données (séances, progrès) restent dans le navigateur (`localStorage`) : rien ne part sur internet.

## Démarrer

```bash
cd web
bun install
bun dev          # http://localhost:3000, rechargement à chaud
```

## Autres commandes

```bash
bun run typecheck   # vérification TypeScript
bun run build       # build de production dans dist/
bun start           # servir le build de production
```

## Organisation du code

| Fichier | Rôle |
|---|---|
| `src/index.ts` | Serveur Bun (`Bun.serve` + import de `index.html`) |
| `src/index.html` | Page unique, mobile-first |
| `src/styles.css` | Design : gros boutons, contrastes, mode sombre auto |
| `src/App.tsx` | Onglets Accueil / Exercices / Progrès / Conseils |
| `src/LecteurSeance.tsx` | Séance guidée : minuteur, étapes, ressenti |
| `src/data/exercises.ts` | Catalogue d'exercices + programme de la semaine |
| `src/lib/progression.ts` | Sauvegarde locale et statistiques |

## L'utiliser sur le téléphone

Une fois servi (par exemple `bun start` sur un petit serveur, ou le dossier `dist/` déposé sur n'importe quel hébergement statique), ouvrez l'adresse dans Safari ou Chrome puis **« Ajouter à l'écran d'accueil »** : le site se comporte alors comme une application.

> ⚕️ HemiFit accompagne la rééducation mais ne remplace ni kinésithérapeute ni médecin.
