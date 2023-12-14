import { defineConfig, tlsRoot, server } from "./config.lib"


const tls = tlsRoot("/etc/letsencrypt")
const { static: serve, proxy } = server

export default defineConfig({
  servers: [
    serve({
      port: 80,
      root: "/var/me",
    }),
    proxy({
      port: 443,
      pass: "http://localhost:3000",
      domain: tls("deuzo.me")
    }),
  ]
})
