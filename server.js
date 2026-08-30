// Serveur de production pour Heroku : sert le site déjà construit
// dans web/dist avec Node, sans aucune dépendance.
import { createServer } from "node:http";
import { createReadStream, existsSync, statSync } from "node:fs";
import { extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

const RACINE = join(fileURLToPath(new URL(".", import.meta.url)), "web", "dist");
const PORT = process.env.PORT || 3000;

const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".ico": "image/x-icon",
  ".json": "application/json",
  ".map": "application/json",
  ".webmanifest": "application/manifest+json",
  ".txt": "text/plain; charset=utf-8",
};

createServer((req, res) => {
  const chemin = decodeURIComponent(new URL(req.url, "http://x").pathname);
  let fichier = normalize(join(RACINE, chemin));

  if (!fichier.startsWith(RACINE)) {
    res.writeHead(403);
    res.end();
    return;
  }

  // Application monopage : toute route inconnue renvoie index.html.
  if (!existsSync(fichier) || statSync(fichier).isDirectory()) {
    fichier = join(RACINE, "index.html");
  }

  const ext = extname(fichier);
  res.writeHead(200, {
    "content-type": TYPES[ext] || "application/octet-stream",
    // Les fichiers construits portent un hash dans leur nom : cache long ;
    // index.html doit rester frais pour suivre les mises à jour.
    "cache-control":
      ext === ".html" ? "no-cache" : "public, max-age=31536000, immutable",
  });
  createReadStream(fichier).pipe(res);
}).listen(PORT, () => {
  console.log(`💚 HemiFit en écoute sur le port ${PORT}`);
});
