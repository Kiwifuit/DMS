import path from "path";
import { defineConfig } from "vite";

export default defineConfig({
  resolve: {
    alias: {
      "~/backend": path.resolve(__dirname, "src/backend/index.d.ts"),
    },
  },
  optimizeDeps: {
    exclude: ["~/backend"], // Exclude it from dependency pre-bundling
  },
});
