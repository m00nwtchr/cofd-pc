import path from "path";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueI18n from "@intlify/vite-plugin-vue-i18n";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [
		vue(),
		vueI18n({
			include: path.resolve(__dirname, "./src/locales/**"),
			// runtimeOnly: false
		})
	],
	base: "./",
	// define: { "process.env": process.env }
});
