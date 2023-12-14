export type AutoConfig<Keys = keyof Add> = Keys extends keyof Add ? { type: Keys } & Add[Keys] & BaseServer : never

export type Config = {
  servers?: AutoConfig[]
}

export type BaseServer = {
  port: number
  domain?: {
    name: string
    tlsKey?: string
    tlsCert?: string
  }
}

export type Actions = { [x in keyof Add]: Add[x] & BaseServer }
export type Action<T extends keyof Add = keyof Add> = { type: T } & Add[T]  & BaseServer

export type Add = {
  static: {
    /**
      * root directory to map with request url pathname
      * @example "/var/www"
      */
    root: string
  },
  proxy: {
    pass: string
  }
}


/** 
  * helper function if your tls file stored in a pattern
  * @example
  * ```typescript
  * // tls is in `/etc/letsencrypt/<domain-name>/....pem`
  * const domain = tlsRoot("/etc/letsencrypt")
  *
  * export default defineConfig({
  *   servers: [
  *     proxy({ domain: domain("deuzo.me") })
  *   ]
  * })
  * ```
  */
export const tlsRoot = <
  T extends Readonly<string>,
  Key extends string = "privkey.pem",
  Cert extends string = "fullchain.pem"
>(
  /** @example `/etc/letsencrypt` */
  root: T,
  /**
    * @default 
    * ```json
    * {
    *   key: "privkey.pem",
    *   cert: "fullchain.pem",
    * }
    * ```
    */
  files?: {
    /** @default "privkey.pem" */
    key?: Key
    /** @default "fullchain.pem" */
    cert?: Cert
  }
) => <Domain extends string>(domain: Domain) => {
  const o = Object.assign({ key: "privkey.pem", cert: "fullchain.pem" },files)
  return {
    name: domain,
    tlsKey: `${root}/${domain}/${o.key}` as `${T}/${Domain}/${Key}`,
    tlsCert: `${root}/${domain}/${o.cert}` as `${T}/${Domain}/${Cert}`,
  }
}

export const server: {
  [x in keyof Add]: (
    config: Add[x] & BaseServer
  ) => { type: x } & Add[x] & BaseServer
} = new Proxy({}, { get: (_,p) => (conf: any) => ({ type: p, ...conf }) }) as any;

/**
  * @example
  * ```typescript
  * import { defineConfig, tlsRoot, server } from "./lib/config"
  * 
  * const tls = tlsRoot("/etc/letsencrypt")
  * const { static: serve, proxy } = server
  *
  * export default defineConfig({
  *   servers: [
  *     serve({
  *       port: 80,
  *       root: "/var/me",
  *     }),
  *     proxy({
  *       port: 443,
  *       pass: "http://localhost:3000",
  *       domain: tls("deuzo.me")
  *     }),
  *   ]
  * })
  * ```
  */
export function defineConfig(config: Config) {
  return config
}

