# HemiFit

**Ma rééducation en douceur, un petit mouvement à la fois.**

HemiFit est une application personnelle de rééducation après une lésion cérébrale, pensée pour une **hémiparésie droite avec forte spasticité**, un **déplacement en fauteuil roulant** et un **équilibre très altéré** : le côté gauche est sain et vient assister le côté droit. Tous les exercices se font **assis avec le dos soutenu, ou allongé** — jamais debout.

Face à la spasticité, l'application applique les bons réflexes : **jamais de mouvement rapide ni forcé** (ça déclenche le réflexe spastique), des **étirements lents et prolongés** (30 s), de la **détente et de la respiration avant chaque séance**, de l'**appui sur la paume ouverte**, et l'entraînement du **relâchement** plutôt que du serrage.

## Ce que fait HemiFit

- **Une séance guidée chaque jour** (~15 à 20 minutes), réalisable seul : chaque séance commence par la détente (qui fait baisser le tonus spastique), puis le programme alterne au fil de la semaine — main et ouverture, renforcement du haut du corps, tronc et jambes, main et poignet, bras et épaule, renforcement du tronc et des jambes. Le dimanche est entièrement consacré au massage.
- **Une séance du soir avec une tierce personne** (~30 minutes) : massages et mobilisations passives impossibles à faire seul — épaule, coude, doigts, hanche, cheville, drainage des jambes, puis installation pour la nuit. Les consignes s'adressent directement à la personne qui accompagne.
- **Une section massage complète**, réalisable d'une seule main : avant-bras, main, espace du pouce, drainage vers l'épaule et nuque.
- **Des exercices adaptés au fauteuil** : soulagement des appuis (prévention des escarres), bascule du bassin, redressement du buste, mobilité de cheville et drainage.
- **Une section renforcement musculaire**, pour reconstruire après une longue immobilité : le côté gauche qui porte tout, le tronc qui sécurise les transferts, les cuisses en isométrie, l'endurance — et la protection de l'épaule gauche, trop souvent oubliée.
- **Une section électrodes** : pose et séances d'électrostimulation pour la main et le pied.
- **Un catalogue filtrable** : une pastille par famille (massage, sensoriel, main, bras, tronc, jambes, muscles, électrodes) et un filtre « seul / avec de l'aide ». Le catalogue est long : on atteint n'importe quelle famille d'un seul appui, sans faire défiler.
- **Un lecteur de séance** : un exercice à la fois, un grand minuteur, des consignes pas à pas.
- **Un suivi motivant et jamais punitif** : série de jours consécutifs, **meilleure série (jamais perdue)**, minutes cumulées, ressenti après chaque séance. Après une longue interruption, l'application accueille le retour plutôt que de sanctionner l'absence — on reprend là où on en est, jamais à zéro.
- **Un rappel quotidien** (application iPhone) pour garder la régularité.

L'interface est conçue pour être utilisée **d'une seule main (la gauche)** : gros boutons (64 px minimum), navigation en bas d'écran, texte large et contrasté, mode sombre automatique, et un jeu d'icônes vectorielles sobre — sans aucun emoji.

## Deux applications

| Dossier | Description |
|---|---|
| [`web/`](web/) | Site mobile-first — **React 19.2 + Bun 1.4 + TypeScript 7** |
| [`ios/`](ios/) | Application iPhone — **Swift 6.3 + SwiftUI + SwiftData** (Xcode 26.6, iOS 26) |

### Versions utilisées

| Élément | Version | Remarque |
|---|---|---|
| Bun | 1.4.0 | Développement et construction du site |
| React | 19.2.8 | |
| TypeScript | 7.0.2 | Réécriture native ; `baseUrl` supprimé |
| Node (Heroku) | 24.x | LTS active, recommandée par Heroku en production |
| Pile Heroku | heroku-26 | Ubuntu 26.04 LTS |
| Swift | 6.3 | Fourni par Xcode 26.6 |
| iOS | 26 | Cible de déploiement, abaissable dans `ios/project.yml` |

Les deux partagent le même catalogue d'exercices et le même programme hebdomadaire. Les données restent sur l'appareil : rien ne part sur internet.

## Mettre en ligne (depuis un téléphone 📱)

Le site se déploie sur **Heroku** sans ordinateur ni ligne de commande : tout se fait dans le navigateur du téléphone. Suivez le guide pas à pas : **[DEPLOIEMENT.md](DEPLOIEMENT.md)**.

En résumé : le site déjà construit (`web/dist`, versionné exprès) est servi par `server.js` (Node, zéro dépendance) — Heroku se connecte au dépôt GitHub et se redéploie automatiquement à chaque mise à jour de `master`.

[![Déployer sur Heroku](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/maxlestage/HemiFit)

## Démarrage rapide

```bash
# Site web
cd web && bun install && bun dev

# Application iPhone
open "ios/HemiFit ‣.xcodeproj"   # projet versionné, rien à installer
```

## Important

HemiFit **accompagne** la rééducation mais **ne remplace pas** un kinésithérapeute ni un médecin. Les exercices proposés sont doux et classiques en rééducation de l'hémiparésie, mais chaque situation est unique : faites-les valider par vos soignants, et arrêtez immédiatement tout mouvement douloureux.
