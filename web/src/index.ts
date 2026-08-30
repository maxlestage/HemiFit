import { serve } from "bun";
import index from "./index.html";

const server = serve({
  routes: {
    // Application monopage : index.html pour toutes les routes.
    "/*": index,
  },

  development: process.env.NODE_ENV !== "production" && {
    hmr: true,
    console: true,
  },
});

console.log(`💚 HemiFit disponible sur ${server.url}`);
