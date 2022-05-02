try{
  const { RaknetClient, RaknetServer } = require('./raknet-node.node')
  module.exports = {
    RaknetServer : RaknetServer,
    RaknetClient : RaknetClient
  }
} catch (e) {
	console.log(e)
	console.log('[raknet] need to build')
}
