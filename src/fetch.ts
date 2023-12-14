


Bun.serve({
  async fetch(req: Request) {
    return new Response('MOVE' + await req.text())
  }
})
