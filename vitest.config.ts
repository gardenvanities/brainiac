import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vitest/config";

// Reaproveita o plugin svelte já presente no projeto (via SvelteKit) para
// compilar módulos `.svelte.ts` com runes nos testes — sem dependência nova.
export default defineConfig({
  plugins: [svelte()],
  test: {
    include: ["tests/**/*.test.ts"],
  },
});
