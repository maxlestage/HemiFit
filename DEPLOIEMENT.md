# Mettre HemiFit en ligne sur Heroku 📱

**Tout se fait depuis le téléphone**, dans le navigateur (Safari ou Chrome). Aucune ligne de commande, aucun ordinateur.

> 💰 **À savoir avant de commencer** : Heroku n'a plus d'offre gratuite. Le plus petit forfait (« Eco ») coûte **5 $/mois** et suffit largement pour HemiFit. Il faudra une carte bancaire lors de l'inscription.

## Étape 1 — Fusionner la pull request ✅ (déjà fait)

Le code (site + fichiers Heroku) est déjà sur la branche principale **`master`**. Il n'y a rien à faire pour cette étape.

> ⚠️ **Important** : dans Heroku, déployez toujours la branche **`master`**. C'est elle qui contient `package.json`, `Procfile` et `server.js`.

## Étape 2 — Créer le compte Heroku (5 minutes)

1. Allez sur **https://signup.heroku.com** et créez un compte (e-mail, nom, pays…).
2. Confirmez votre adresse e-mail via le lien reçu.
3. Dans **Account Settings → Billing**, ajoutez votre carte bancaire.
4. Dans **Billing**, souscrivez au forfait **Eco dynos** (5 $/mois).

## Étape 3 — Créer l'application (5 minutes)

1. Sur **https://dashboard.heroku.com**, touchez **New → Create new app**.
2. Nom de l'app : par exemple `hemifit` (ou `hemifit-max` si le nom est pris). Région : **Europe**. Touchez **Create app**.
3. Dans l'onglet **Deploy** de l'app :
   - Section *Deployment method* : touchez **GitHub**, puis **Connect to GitHub** et autorisez Heroku à accéder à votre compte GitHub.
   - Cherchez `HemiFit`, touchez **Connect** à côté du dépôt.
4. Section *Automatic deploys* : choisissez la branche **master** puis touchez **Enable Automatic Deploys**.
5. Section *Manual deploy* : branche **master**, touchez **Deploy Branch** pour le tout premier déploiement.
6. Attendez ~1 minute, puis touchez **View** (ou **Open app** en haut) : HemiFit est en ligne ! 🎉

Votre site aura une adresse du type **https://hemifit-xxxx.herokuapp.com**.

## Étape 4 — L'installer comme une application (1 minute)

Sur votre iPhone, ouvrez l'adresse dans **Safari** :

1. Touchez le bouton **Partager** (le carré avec la flèche).
2. Touchez **« Sur l'écran d'accueil »**, puis **Ajouter**.

HemiFit apparaît sur votre écran d'accueil et s'ouvre en plein écran, comme une vraie application. Vos progrès sont enregistrés dans le téléphone.

## Et après ?

- **Mises à jour automatiques** : grâce aux *Automatic deploys*, chaque amélioration fusionnée sur `master` est mise en ligne toute seule, sans rien faire.
- **Comment ça marche** : le site est déjà construit dans `web/dist` (versionné exprès) ; Heroku le sert avec `server.js` (Node, zéro dépendance) via le buildpack standard `heroku/nodejs`. Fiable et sans surprise.
- **En cas de souci** : dans le dashboard Heroku, onglet **Activity** pour voir les déploiements, **More → View logs** pour les journaux.

## Si le déploiement échoue

### « No default language could be detected for this app »

Heroku n'a pas trouvé de `package.json` à la racine : c'est qu'il déploie **la mauvaise branche**.

Dans l'onglet **Deploy** de l'app, vérifiez que la branche choisie est bien **`master`** — à la fois dans *Automatic deploys* et dans *Manual deploy* — puis relancez **Deploy Branch**.

### « Application error » à l'ouverture du site

Le site est déployé mais le serveur ne démarre pas. Touchez **More → View logs** en haut à droite du dashboard et regardez les dernières lignes. Vous devriez y voir `💚 HemiFit en écoute sur le port …` si tout va bien.

### Le site s'ouvre mais reste blanc

Le dossier `web/dist` (le site construit) manque. Vérifiez sur GitHub que le dossier **web/dist** existe bien sur la branche `master` : https://github.com/maxlestage/HemiFit/tree/master/web/dist
