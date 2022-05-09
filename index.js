try{

  switch (process.platform) {
    case "win32":
      var { RaknetClient, RaknetServer } = require('./raknet-node-win.node')
      break;
    case "linux":
      var { RaknetClient, RaknetServer } = require('./raknet-node-linux.node')
      break;
	default :
	  console.log('[raknet] need to build')
	  process.exit(1)
  }
} catch (e) {
	console.log(e)
	console.log('[raknet] need to build')
	process.exit(1)
}

function listen(address , cb) {

	RaknetServer.bind(address).then((server) => {
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
		if (cb(buf)){
			recvcb(client , cb)
		}
	}).catch((e) => {
		cb(null)
	})
}

function connectcb(address , cb){
	RaknetClient.connect(address).then((client) =>{
		cb(client)
	})
}

const { EventEmitter } = require('events')

class Client extends EventEmitter {
  constructor (hostname, port, options = {}) {
    super()
    connectcb(hostname + ":" + port, (client) => {
		this.client = client
		var address = this.client.peeraddr()
	
		this.emit('connect', { address })
		recvcb(this.client , (buf) => {
			if (buf == null){
				this.emit('disconnect', { address })
				return false
			}
			
			this.emit('encapsulated', { buffer: buf, address })
			return true
		})
	})

  }

  send (message) {
    this.client.send(message)
  }
}

class Server extends EventEmitter {
  constructor (hostname, port) {
    super()
    listen(hostname + ":" + port, (client) => {
		var address = client.peeraddr()
		this.emit('openConnection', client)
		recvcb(client , (buf) => {
			if (buf == null){
				this.emit('closeConnection', { address })
				return false
			} else {
				this.emit('encapsulated', { buffer: buf, address})
				return true
			}
		})
	})
  }
}

module.exports = { Server, Client , RaknetClient, RaknetServer }