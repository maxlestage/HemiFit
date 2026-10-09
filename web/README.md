# HemiFit — site web

Site **mobile-first** de rééducation en douceur, écrit en **Rust** avec **Yew** et compilé en **WebAssembly** :

- **Rust 1.99** (édition 2024), cible `wasm32-unknown-unknown` ;
- **Yew 0.23** (composants `#[component]`, rendu côté navigateur) ;
- **wasm-bindgen 0.2.129** et **web-sys 0.3.106** pour parler au navigateur ;
- **Trunk 0.21.14** pour construire le site, avec **binaryen 133** (`wasm-opt`).

Toutes les données (séances, progrès) restent dans le navigateur (`localStorage`, clé `hemifit.progression.v1`, même format que l'ancienne version React) : rien ne part sur internet, et l'historique déjà enregistré est repris tel quel.

## Préparer la machine

```bash
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install --locked trunk     # ou le binaire publié sur GitHub
```

## Démarrer

```bash
cd web
trunk serve            # http://127.0.0.1:8080, rechargement automatique
```

## Autres commandes

```bash
cargo test                                   # calculs de dates, séries, catalogue
cargo clippy --target wasm32-unknown-unknown # vérifications
trunk build --release                        # site de production dans dist/
```

`dist/` est **versionné exprès** : Heroku le sert tel quel avec `server.js`, sans rien construire. Après toute modification, reconstruisez puis committez `dist/`.

## Organisation du code

| Fichier | Rôle |
|---|---|
| `index.html` | Page unique ; contient le rideau d'ouverture, affiché pendant le chargement |
| `styles.css` | Design « papier » et animations |
| `Trunk.toml` | Construction ; fixe les versions de wasm-bindgen et de binaryen |
| `src/main.rs` | Point d'entrée : monte l'application dans `#app` |
| `src/app.rs` | Onglets Accueil / Exercices / Progrès / Conseils |
| `src/pages/` | Les quatre écrans |
| `src/lecteur.rs` | Séance guidée : minuteur, étapes, ressenti |
| `src/exercices.rs` | Catalogue d'exercices + programme de la semaine (dupliqué dans `ios/HemiFit/Exercices.swift`) |
| `src/progression.rs` | Sauvegarde locale et statistiques |
| `src/dates.rs` | Dates du calendrier local, testables hors navigateur |
| `src/icones.rs` | Jeu d'icônes vectorielles |
| `src/composants.rs` | Titre en fente, compteur, bandeau de principes, manifeste |
| `src/mouvement.rs` | Animations : rideau, révélations au défilement, magnétisme |

## Les animations

Elles sont inspirées de [zamocorp.com](https://zamocorp.com), adaptées à une application de rééducation utilisée d'une main :

- **rideau d'ouverture**, une fois par session : l'anneau de la marque se trace, un compteur monte jusqu'à 100, puis le rideau se lève (un appui le lève aussitôt) ;
- **grain** très léger sur tout l'écran, animé seulement à la souris ;
- **titres en fente** : chaque mot monte, l'un après l'autre ;
- **révélations au défilement** : les cartes montent en cascade, les filets des sections se déroulent, l'anneau en filigrane de la séance du jour se trace ;
- **compteurs** qui montent jusqu'à leur valeur, **bandeau de principes** qui défile (un appui l'arrête) et **manifeste** qui s'allume mot à mot ;
- à la souris seulement : **boutons magnétiques** et **anneau** qui suit le pointeur.

Garde-fous : avec « réduire les animations », rien ne bouge et rien n'est masqué ; seul le contenu encore sous l'écran attend d'être révélé ; `src/mouvement.rs` ne touche jamais aux nœuds gérés par Yew, il pose seulement des classes sur `<html>` et des attributs `data-*`.

## L'utiliser sur le téléphone

Ouvrez l'adresse du site dans Safari ou Chrome puis **« Ajouter à l'écran d'accueil »** : le site se comporte alors comme une application.

> HemiFit accompagne la rééducation mais ne remplace ni kinésithérapeute ni médecin.
