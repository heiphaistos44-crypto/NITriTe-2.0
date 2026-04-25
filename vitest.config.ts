import { defineConfig } from "vitest/config";
import vue from "@vitejs/plugin-vue";
import { resolve } from "path";

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: "happy-dom",
    globals: true,
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
      include: ["src/composables/**", "src/stores/**", "src/utils/**"],
      exclude: ["src/**/*.vue"],
    },
  },
  resolve: {
    alias: { "@": resolve(__dirname, "src") },
  },
  define: {
    __APP_VERSION__: JSON.stringify("test"),
  },
});
