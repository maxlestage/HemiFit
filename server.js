// Serveur de production pour Heroku : sert le site déjà construit
// dans web/dist avec Node, sans aucune dépendance.
import { createServer } from "node:http";
import { existsSync, readFileSync, statSync } from "node:fs";
import { extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { brotliCompressSync, constants, gzipSync } from "node:zlib";

const RACINE = join(fileURLToPath(new URL(".", import.meta.url)), "web", "dist");
const PORT = process.env.PORT || 3000;

const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  // Indispensable : sans ce type, le navigateur ne peut pas compiler le
  // WebAssembly pendant son téléchargement.
  ".wasm": "application/wasm",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".ico": "image/x-icon",
  ".json": "application/json",
  ".map": "application/json",
  ".webmanifest": "application/manifest+json",
  ".txt": "text/plain; charset=utf-8",
};

// Types qui gagnent à être compressés (les images PNG le sont déjà).
const COMPRESSIBLES = new Set([
  ".html",
  ".js",
  ".css",
  ".wasm",
  ".svg",
  ".json",
  ".webmanifest",
  ".txt",
]);

// Les fichiers construits portent une empreinte dans leur nom
// (ex. hemifit-3bf7d54de3f5f13a_bg.wasm) : ils ne changent jamais et
// se gardent un an. Les autres (index.html, icônes, manifeste) restent frais.
const EMPREINTE = /-[0-9a-f]{10,16}(_bg)?\.[a-z]+$/;

// Le site est petit et ne change qu'au déploiement : chaque fichier est
// lu et compressé une seule fois, puis gardé en mémoire.
const cache = new Map();

function lire(fichier, encodage) {
  const cle = `${encodage}:${fichier}`;
  if (!cache.has(cle)) {
    const brut = readFileSync(fichier);
    const corps =
      encodage === "br"
        ? brotliCompressSync(brut, {
            params: { [constants.BROTLI_PARAM_QUALITY]: 11 },
          })
        : encodage === "gzip"
          ? gzipSync(brut, { level: 9 })
          : brut;
    cache.set(cle, corps);
  }
  return cache.get(cle);
}

createServer((req, res) => {
  let chemin;
  try {
    chemin = decodeURIComponent(new URL(req.url, "http://x").pathname);
  } catch {
    res.writeHead(400);
    res.end();
    return;
  }
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
  const accepte = req.headers["accept-encoding"] || "";
  const encodage = !COMPRESSIBLES.has(ext)
    ? null
    : /\bbr\b/.test(accepte)
      ? "br"
      : /\bgzip\b/.test(accepte)
        ? "gzip"
        : null;
  const corps = lire(fichier, encodage);

  res.writeHead(200, {
    "content-type": TYPES[ext] || "application/octet-stream",
    "content-length": corps.length,
    "cache-control": EMPREINTE.test(fichier)
      ? "public, max-age=31536000, immutable"
      : "no-cache",
    vary: "accept-encoding",
    ...(encodage && { "content-encoding": encodage }),
  });
  res.end(req.method === "HEAD" ? undefined : corps);
}).listen(PORT, () => {
  console.log(`HemiFit en écoute sur le port ${PORT}`);
});
