

Bun.serve({
  fetch(request, server) {
      if (server.upgrade(request)) {
        console.log("bro")
        return
      }
      console.log("na ah")
      return new Response(void 0,{ status: 404 })
  },
  websocket: {
    open(ws) {
        console.log("Open: ")
    },
    message(ws, message) {
        console.log("Message: ",message)
    },
    close(ws, code, reason) {
        console.log("Clos: ",code,reason)
    },
  },
  port: 4040
})

