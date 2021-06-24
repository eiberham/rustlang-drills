const express = require('express')
const path = require('path')
const app = express()
const port = 4000

express.static.mime.types['wasm'] = 'application/wasm'

app.use(express.static('public'))

app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, '/index.html'));
})

app.listen(port, () => {
    console.log(`server listening at ${port}`)
})