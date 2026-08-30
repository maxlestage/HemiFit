# HemiFit — consignes du projet

## Contexte utilisateur (important)

Le propriétaire du dépôt travaille **uniquement depuis un téléphone**, sans ordinateur, et vit avec une **hémiparésie droite avec forte spasticité** après une lésion cérébrale. Toute manipulation manuelle a un coût réel pour lui.

En conséquence :

- **Fusionner systématiquement les pull requests soi-même**, sans les laisser en attente et sans demander confirmation. Après un `push`, on ouvre la pull request puis on la fusionne dans la foulée une fois les vérifications passées.
- **Ne jamais proposer d'étape en ligne de commande** : toute action de sa part doit être réalisable au doigt dans un navigateur mobile.
- **Tout écrire en français**, y compris le code (noms de variables, commentaires), l'interface et les messages de commit.

## Structure

| Dossier | Contenu |
|---|---|
| `web/` | Site mobile-first — React 19 + Bun + TypeScript |
| `ios/` | Application iPhone — Swift 6 + SwiftUI + SwiftData |
| `server.js`, `Procfile`, `app.json` | Déploiement Heroku |

Le catalogue d'exercices est **dupliqué volontairement** entre `web/src/data/exercises.ts` et `ios/HemiFit/Exercices.swift` : toute modification de l'un doit être reportée à l'identique dans l'autre.

## ⚠️ Reconstruire le site après chaque modification du web

Heroku ne construit rien : il sert le dossier **`web/dist`, qui est versionné exprès**. Après toute modification dans `web/src`, il faut impérativement reconstruire et committer le résultat, sinon le site en ligne reste inchangé :

```bash
cd web && bunx tsc --noEmit && bun run build   # met à jour web/dist
```

Vérifications avant de pousser : `bunx tsc --noEmit` puis `bun run build` doivent passer.

## Versions

Le propriétaire souhaite que tout reste à jour. **Ne jamais se fier à sa mémoire pour les numéros de version** : les interroger en direct.

```bash
curl -s https://registry.npmjs.org/<paquet>/latest   # npm (react, typescript, bun…)
curl -s https://nodejs.org/dist/index.json           # versions Node et statut LTS
cd web && bun update --latest                        # met à jour les dépendances du site
```

Deux règles de jugement :

- **Node (Heroku) reste sur la LTS active**, pas sur la version « Current » : Heroku recommande explicitement les LTS en production. Actuellement **24.x**.
- **Changer de version majeure demande une vérification**, pas une simple substitution de numéro. Exemple vécu : TypeScript 7 a supprimé `baseUrl`, ce qui cassait le `tsconfig.json`.

## Contenu des exercices

Les exercices visent une **hémiparésie droite spastique**, avec ces principes non négociables :

- tout se fait **assis ou allongé** (la marche n'est pas acquise) ;
- la **main gauche (saine) assiste** le côté droit ;
- **jamais de mouvement rapide ni forcé** : cela déclenche le réflexe spastique ;
- privilégier les **étirements lents et prolongés** (15–30 s) et la détente préalable ;
- entraîner le **relâchement et l'ouverture** de la main, jamais le serrage (les fléchisseurs sont déjà trop forts, les extenseurs affaiblis) ;
- l'**intention de mouvement compte**, même sans mouvement visible ;
- **masser avant de mobiliser** : le massage abaisse le tonus, chaque séance commence donc par la détente puis un massage.

## Ton de l'application

Le propriétaire veut continuer à récupérer **même longtemps après la lésion**, et c'est une attente légitime : l'idée d'un plateau définitif à six mois est largement remise en cause, la neuroplasticité se poursuivant des années durant. L'application doit donc :

- **encourager honnêtement** — progrès possibles à long terme, mais souvent lents et partiels : ni fatalisme, ni fausses promesses de guérison ;
- **ne jamais punir une interruption** : la meilleure série est conservée, un retour après plusieurs semaines est accueilli chaleureusement, et rien n'est jamais remis à zéro.

Conserver systématiquement les avertissements médicaux (ne remplace ni kinésithérapeute ni médecin ; arrêter en cas de douleur).
