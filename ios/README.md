# HemiFit pour iPhone 📱

Application personnelle de rééducation en douceur, écrite en **Swift 6 / SwiftUI**, avec :

- **SwiftData** pour enregistrer les séances (tout reste sur l'iPhone, rien ne part sur internet) ;
- **Notifications locales** pour le rappel quotidien (activable dans l'onglet Conseils) ;
- une interface pensée pour être utilisée **d'une seule main (la gauche)** : gros boutons, navigation en bas d'écran, texte large compatible Dynamic Type.

## Ouvrir le projet dans Xcode

Le plus simple est de générer le projet avec [XcodeGen](https://github.com/yonaskolb/XcodeGen) :

```bash
brew install xcodegen
cd ios
xcodegen generate
open HemiFit.xcodeproj
```

### Sans XcodeGen

1. Dans Xcode : **File → New → Project → iOS → App**.
2. Nom : `HemiFit`, interface **SwiftUI**, langage **Swift**.
3. Supprimez les fichiers générés (`ContentView.swift`, etc.) et glissez le dossier `HemiFit/` de ce dépôt dans le projet.
4. Lancez sur votre iPhone (⌘R).

## Configuration requise

- Xcode 16 ou plus récent (Swift 6) — prenez toujours la dernière version disponible.
- iOS 18 minimum sur l'iPhone (l'app profite des API SwiftUI récentes ; montez la cible de déploiement si votre iOS est plus récent).

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

## Adapter le programme

Le programme de la semaine se règle dans `Exercices.swift` (`Catalogue.seanceDuJour`) : lundi/jeudi la main, mardi/vendredi le bras, mercredi/samedi la jambe, dimanche une séance courte. Les exercices eux-mêmes (consignes, durées) sont dans le même fichier — n'hésitez pas à les ajuster avec votre kinésithérapeute.

> ⚕️ HemiFit accompagne la rééducation mais ne remplace ni kinésithérapeute ni médecin. Montrez-leur les exercices et arrêtez tout mouvement douloureux.
