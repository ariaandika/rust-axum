





const ws = new WebSocket("ws://localhost:3000",{
  headers: {
    host: "deuzo.me"
  }
})

ws.onerror = err=>console.log({err})

ws.addEventListener("open", e => {
  console.log("Okei")
  ws.send("nice oof")
  ws.close()
})

ws.addEventListener('error',er=> console.log({er}))
ws.addEventListener('close',cols=> console.log({cols}))
