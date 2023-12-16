import { defineConfig } from "./config/config.lib"

export default defineConfig({
  servers: {
    "localhost:3000": {
      proxy: {
        target: "localhost:4040",
        // target: "localhost:8000",
      }
    }
  },
  port: 3000,
  tls: {
    dir: "/etc/letsencrypt",
  }
})
