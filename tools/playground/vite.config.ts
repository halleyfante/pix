import { defineConfig } from "vite";

export default defineConfig({
  root: "source",
  build: {
    outDir: "../distribution",
    emptyOutDir: true,
  },
});
