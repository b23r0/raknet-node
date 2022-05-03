const raknet = require('raknet-node')
const assert = require('assert');

async function main(){
	address = "127.0.0.1:19132"
	server = await raknet.RaknetServer.bind(address)
	console.log("bind to : " + address)
	
	client1 = await raknet.RaknetClient.connect(address)

	console.log("connect to : " + address + " success")

	client2 = await server.accept()

	await client1.send(Buffer.from([0xfe ,1, 2, 3]))
	await client2.send(Buffer.from([0xfe ,1, 2, 3]))

	buf1 = await client1.recv()
	buf2 = await client2.recv()

	assert(buf1.compare(buf2) == 0)
	
	return "finished"
}

main()
 .then(console.log)
 .catch(console.error)