import { defineConfig } from "./config.lib"

export default defineConfig({
  servers: {
    "deuzo.me": {
      proxy: {
        target: "localhost:4040",
      }
    }
  },
  port: 3000,
  tls: {
    dir: "/etc/letsencrypt",
  }
})
