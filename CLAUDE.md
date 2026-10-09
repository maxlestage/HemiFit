# HemiFit — consignes du projet

## Contexte utilisateur (important)

Le propriétaire du dépôt travaille **uniquement depuis un téléphone**, sans ordinateur. Il vit avec une **hémiparésie droite avec forte spasticité** après une lésion cérébrale, **se déplace en fauteuil roulant** et a un **équilibre très altéré**. Toute manipulation manuelle a un coût réel pour lui.

En conséquence :

- **Fusionner systématiquement les pull requests soi-même**, sans les laisser en attente et sans demander confirmation. Après un `push`, on ouvre la pull request puis on la fusionne dans la foulée une fois les vérifications passées.
- **Ne jamais proposer d'étape en ligne de commande** : toute action de sa part doit être réalisable au doigt dans un navigateur mobile.
- **Tout écrire en français**, y compris le code (noms de variables, commentaires), l'interface et les messages de commit.

## Structure

| Dossier | Contenu |
|---|---|
| `web/` | Site mobile-first — Rust + Yew, compilé en WebAssembly (Trunk) |
| `ios/` | Application iPhone — Swift 6 + SwiftUI + SwiftData |
| `server.js`, `Procfile`, `app.json` | Déploiement Heroku |

Le catalogue d'exercices est **dupliqué volontairement** entre `web/src/exercices.rs` et `ios/HemiFit/Exercices.swift` : toute modification de l'un doit être reportée à l'identique dans l'autre.

L'historique du site est stocké dans le navigateur sous la clé `hemifit.progression.v1`, au format JSON hérité de l'ancienne version React (`date`, `titre`, `minutes`, `exercicesFaits`, `ressenti`). **Ne jamais changer cette clé ni ce format** sans migration : ce serait effacer les progrès du propriétaire.

## ⚠️ Reconstruire le site après chaque modification du web

Heroku ne construit rien : il sert le dossier **`web/dist`, qui est versionné exprès**. Après toute modification dans `web/` (`src/`, `styles.css`, `index.html`), il faut impérativement reconstruire et committer le résultat, sinon le site en ligne reste inchangé :

```bash
rustup target add wasm32-unknown-unknown       # une fois
cd web && cargo test && trunk build --release   # met à jour web/dist
```

Vérifications avant de pousser : `cargo fmt --check`, `cargo clippy --target wasm32-unknown-unknown` (sans avertissement), `cargo test` puis `trunk build --release` doivent passer. Vérifier aussi le rendu dans Chromium (Playwright est installé) au format téléphone, en clair, en sombre et avec `reducedMotion: "reduce"`.

## Versions

Le propriétaire souhaite que tout reste à jour. **Ne jamais se fier à sa mémoire pour les numéros de version** : les interroger en direct.

```bash
curl -s https://static.rust-lang.org/dist/channel-rust-stable.toml | grep -m1 '^version'  # Rust stable
curl -s -H "User-Agent: hemifit" https://crates.io/api/v1/crates/<crate>                 # yew, trunk, wasm-bindgen…
curl -s https://nodejs.org/dist/index.json           # versions Node et statut LTS
rustup update stable                                 # met à jour Rust
cd web && cargo update                               # met à jour les dépendances du site
```

Trunk télécharge lui-même `wasm-bindgen` et `wasm-opt` (binaryen) : leurs versions sont fixées dans `web/Trunk.toml`. Garder `wasm_bindgen` égal à la version de la crate `wasm-bindgen`, et `wasm_opt` sur la dernière version de binaryen (trouver la plus récente en testant `https://github.com/WebAssembly/binaryen/releases/download/version_<N>/binaryen-version_<N>-x86_64-linux.tar.gz`).

Deux règles de jugement :

- **Node (Heroku) reste sur la LTS active**, pas sur la version « Current » : Heroku recommande explicitement les LTS en production. Actuellement **24.x**.
- **Changer de version majeure demande une vérification**, pas une simple substitution de numéro. Exemples vécus : TypeScript 7 a supprimé `baseUrl`, ce qui cassait le `tsconfig.json` ; Rust 1.87+ produit du WebAssembly « bulk memory » que le binaryen 123 proposé par Trunk refuse, d'où la version fixée et les options `data-wasm-opt-params` dans `web/index.html`.

## Contenu des exercices

Les exercices visent une **hémiparésie droite spastique**, avec ces principes non négociables :

- tout se fait **assis avec le dos soutenu, ou allongé** : jamais debout, jamais de transfert non sécurisé, l'équilibre étant très altéré ;
- ne jamais présenter la marche comme l'objectif : parler de confort, d'autonomie, de transferts et de prévention des complications ;
- la **main gauche (saine) assiste** le côté droit ;
- **jamais de mouvement rapide ni forcé** : cela déclenche le réflexe spastique ;
- privilégier les **étirements lents et prolongés** (15–30 s) et la détente préalable ;
- entraîner le **relâchement et l'ouverture** de la main, jamais le serrage (les fléchisseurs sont déjà trop forts, les extenseurs affaiblis) ;
- l'**intention de mouvement compte**, même sans mouvement visible ;
- **masser avant de mobiliser** : le massage abaisse le tonus, chaque séance commence donc par la détente puis un massage ;
- **prévenir les complications du fauteuil** : soulagement des appuis (escarres), mobilité de cheville (pied en pointe), drainage (gonflement) ;
- **renforcer aussi le côté gauche** : il n'est pas épargné, il se désentraîne en fauteuil alors qu'il assure les transferts. Son épaule est la première à s'user : toute séance de renforcement du haut du corps doit inclure le travail des rotateurs ;
- **à droite, entretenir sans forcer** : contractions isométriques et mouvements guidés, jamais d'effort qui réveille la spasticité ;
- **respiration jamais bloquée** pendant un effort, et **quarante-huit heures de repos** entre deux séances sollicitant les mêmes muscles.

Chaque exercice porte un champ `realisation` : `autonome` (réalisable seul) ou `tierce-personne` (mobilisations passives faites le soir par un accompagnant, dont les consignes s'adressent à cette personne). L'interface doit toujours distinguer les deux clairement.

## Interface

**Aucun emoji dans l'interface ni dans le contenu** : le propriétaire les trouve peu professionnels. On utilise un jeu d'icônes vectorielles homogène (`web/src/icones.rs`) côté web et des symboles SF côté iOS. Les emoji restent proscrits dans les titres, les libellés et les textes de conseils.

### Animations

Le site s'anime dans l'esprit de **zamocorp.com**, choix du propriétaire : rideau d'ouverture, grain, titres en fente, cartes en cascade, filets qui se déroulent (`web/src/mouvement.rs` et la fin de `web/styles.css`). Règles à garder :

- **respecter « réduire les animations »** : rien ne bouge et rien n'est masqué ;
- **ne jamais retarder une action** : un appui lève le rideau, les boutons répondent tout de suite, seul le contenu sous l'écran attend d'être révélé ;
- **rien de rapide ni de clignotant** ; ce qui défile en continu doit pouvoir s'arrêter d'un appui ;
- les effets de souris (magnétisme, anneau) ne s'activent jamais au toucher ;
- `mouvement.rs` ne crée, ne déplace ni ne supprime aucun nœud géré par Yew : il pose des classes sur `<html>` et des attributs `data-*`.

## Ton de l'application

Le propriétaire veut continuer à récupérer **même longtemps après la lésion**, et c'est une attente légitime : l'idée d'un plateau définitif à six mois est largement remise en cause, la neuroplasticité se poursuivant des années durant. L'application doit donc :

- **encourager honnêtement** — progrès possibles à long terme, mais souvent lents et partiels : ni fatalisme, ni fausses promesses de guérison ;
- **ne jamais punir une interruption** : la meilleure série est conservée, un retour après plusieurs semaines est accueilli chaleureusement, et rien n'est jamais remis à zéro.

Conserver systématiquement les avertissements médicaux (ne remplace ni kinésithérapeute ni médecin ; arrêter en cas de douleur).
