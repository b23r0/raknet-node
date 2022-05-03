try{

  switch (process.platform) {
    case "win32":
      var { RaknetClient, RaknetServer } = require('./raknet-node-win.node')
      break;
    case "linux":
      var { RaknetClient, RaknetServer } = require('./raknet-node-linux.node')
      break;
  }
  
  module.exports = {
    RaknetServer : RaknetServer,
    RaknetClient : RaknetClient
  }
} catch (e) {
	console.log(e)
	console.log('[raknet] need to build')
}
