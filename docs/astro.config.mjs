// @ts-check

import mdx from "@astrojs/mdx";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";
import icon from "astro-icon";

// https://astro.build/config
export default defineConfig({
  site: "https://pantheon-org.github.io",
  base: "/tekhne",
  integrations: [mdx(), icon()],
  vite: {
    plugins: [tailwindcss()],
  },
});
