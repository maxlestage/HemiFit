# HemiFit pour iPhone

Application personnelle de rééducation en douceur, écrite en **Swift 6.3 / SwiftUI**, avec :

- **SwiftData** pour enregistrer les séances (tout reste sur l'iPhone, rien ne part sur internet) ;
- **Notifications locales** pour le rappel quotidien (activable dans l'onglet Conseils) ;
- une interface pensée pour être utilisée **d'une seule main (la gauche)** : gros boutons, navigation en bas d'écran, texte large compatible Dynamic Type.

## Ouvrir le projet dans Xcode

Le projet est versionné : **ouvrez simplement `ios/HemiFit.xcodeproj`**. Rien à installer, aucune commande à lancer.

Le projet utilise un *groupe synchronisé* sur le dossier `HemiFit/` : Xcode y détecte les fichiers tout seul. Ajouter un fichier Swift ne demande donc jamais de modifier le projet, et deux personnes qui ajoutent des fichiers en parallèle ne créent pas de conflit.

Un schéma partagé `HemiFit` est également versionné, ce qui permet de compiler depuis un service d'intégration continue sans configuration supplémentaire.

## Configuration requise

- **Xcode 26.6** ou plus récent (fournit **Swift 6.3** et le SDK iOS 26.5) — prenez toujours la dernière version disponible sur le Mac App Store.
- **iOS 26** minimum sur l'iPhone.

> ⚠️ **Si votre iPhone tourne sur une version d'iOS plus ancienne**, l'application refusera de s'installer. C'est réglable dans Xcode, sans toucher au code : sélectionnez le projet **HemiFit**, onglet **General**, puis abaissez **Minimum Deployments** (par exemple à iOS 18).

## Organisation du code

| Fichier | Rôle |
|---|---|
| `HemiFitApp.swift` | Point d'entrée, conteneur SwiftData |
| `Modeles.swift` | Journal des séances + statistiques (série, minutes…) |
| `Exercices.swift` | Catalogue des exercices et programme de la semaine |
| `Rappels.swift` | Rappel quotidien par notification locale |
| `Vues/ContentView.swift` | Navigation par onglets + styles communs |
| `Vues/AccueilView.swift` | Séance du jour, série en cours |
| `Vues/SeanceGuideeView.swift` | Séance guidée : minuteur, étapes, ressenti |
| `Vues/ExercicesListeView.swift` | Catalogue complet, exercice à la carte |
| `Vues/ProgresView.swift` | Statistiques et historique |
| `Vues/ConseilsView.swift` | Conseils de rééducation + réglage du rappel |

## Le fichier `project.yml`

Il est conservé comme mémo lisible des réglages, mais **le projet Xcode fait foi**. Lancer `xcodegen generate` écraserait le projet versionné : à ne faire qu'en connaissance de cause.

## Adapter le programme

Le programme de la semaine se règle dans `Exercices.swift` (`Catalogue.seanceDuJour`) : lundi la main et l'ouverture, mardi/vendredi le bras et l'épaule, mercredi/samedi le tronc et les jambes, jeudi la main et le poignet, dimanche massage et détente. La séance du soir avec une tierce personne est définie juste en dessous (`Catalogue.seanceDuSoir`). Les exercices eux-mêmes (consignes, durées) sont dans le même fichier — n'hésitez pas à les ajuster avec votre kinésithérapeute.

> Ce catalogue est le miroir de `web/src/data/exercises.ts` : toute modification de l'un doit être reportée à l'identique dans l'autre.

> ⚕️ HemiFit accompagne la rééducation mais ne remplace ni kinésithérapeute ni médecin. Montrez-leur les exercices et arrêtez tout mouvement douloureux.
