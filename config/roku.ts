


Bun.serve({
  async fetch(req: Request) {
    let py = 'MOVE' + await req.text() + process.env.PORT
    console.log(py)
    return new Response(py)
  }
})
