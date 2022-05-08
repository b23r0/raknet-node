try{

  switch (process.platform) {
    case "win32":
      var { RaknetClient, RaknetServer } = require('./raknet-node-win.node')
      break;
    case "linux":
      var { RaknetClient, RaknetServer } = require('./raknet-node-linux.node')
      break;
  }
} catch (e) {
	console.log(e)
	console.log('[raknet] need to build')
	process.exit(1)
}

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

const { EventEmitter } = require('events')

class Client extends EventEmitter {
  constructor (hostname, port, options = {}) {
    super()
    connectcb(hostname + ":" + port, (client) => {
		this.client = client
		this.startListening()
	})

  }

  startListening () {
	
	address = this.client.peeraddr()
	
	this.emit('connect', { address })
	
	for (;;){
		recvcb(this.client , (buf) => {
			if (buf == null){
				this.emit('disconnect', { address })
				break
			}
			
			this.emit('encapsulated', { buffer: buf, address })
		})
	}
  }

  send (message) {
    this.client.send(message)
  }
}

class Server extends EventEmitter {
  constructor (hostname, port) {
    super()
    listen(hostname + ":" + port, (client) => {
		this.emit('openConnection', client)
		recvcb(client , (buf) => {
			if (buf == null){
				this.emit('closeConnection', { client.peeraddr() })
				break
			}
			this.emit('encapsulated', { buffer: buf, client.peeraddr()})
		})
	})
  }
}

module.exports = { Server, Client }