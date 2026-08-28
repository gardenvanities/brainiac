import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vitest/config";

// Reaproveita o plugin svelte já presente no projeto (via SvelteKit) para
// compilar módulos `.svelte.ts` com runes nos testes — sem dependência nova.
export default defineConfig({
  plugins: [
    svelte({
      // Sob Vitest o ambiente é consumer 'server' e o plugin compilaria TODOS
      // os `.svelte.ts` com generate:'server' — onde `$effect` é no-op e os
      // sinais ficariam num runtime DIFERENTE do dos efeitos dos testes
      // (stores server ↔ efeitos client nunca se enxergam). A app sempre
      // roda o runtime client; forçamos client aqui para que stores e
      // testes compartilhem o MESMO runtime reativo.
      dynamicCompileOptions() {
        return { generate: "client" };
      },
    }),
  ],
  test: {
    include: ["tests/**/*.test.ts"],
  },
});
