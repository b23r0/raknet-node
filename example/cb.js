const raknet = require('./raknet-node')
const assert = require('assert');

data = Buffer.from([0xfe ,1, 2, 3])

//raknet.enablelog()

function listen(address , cb) {

	raknet.RaknetServer.bind(address).then((server) => {
		const doSomething = async () => {
			for (;;){
				cb(await server.accept())
			}
		}
		doSomething().then(null)
	})
}

function recvcb(client , cb) {
	client.recv().then((buf) => {
		cb(buf)
	})
}

function connectcb(address , cb){
	raknet.RaknetClient.connect(address).then((client) =>{
		cb(client)
	})
}

address = "127.0.0.1:19132"

listen(address , (client) => {
	client.send(data).then(null)
	recvcb(client, (buf) => {
		console.log("finished1")
		assert(buf.compare(data) == 0)
	})
})

connectcb(address , (client) => {
	client.send(data).then(null)
	recvcb(client, (buf) => {
		console.log("finished2")
		assert(buf.compare(data) == 0)
	})
})

connectcb(address , (client) => {
	client.send(data).then(null)
	recvcb(client, (buf) => {
		console.log("finished3")
		assert(buf.compare(data) == 0)
	})
})