# raknet-node
[![Build Status](https://img.shields.io/github/workflow/status/b23r0/raknet-node/Rust)](https://github.com/b23r0/raknet-node/actions/workflows/rust.yml)
[![NPM version](https://img.shields.io/npm/v/raknet-node.svg)](http://npmjs.com/package/raknet-node)
[![ChatOnDiscord](https://img.shields.io/badge/chat-on%20discord-blue)](https://discord.gg/ZKtYMvDFN4)

Nodejs bindings to rust-raknet native library.

# Install

```
npm install raknet-node
```

Prebuilds are provided for 64-bit Windows 10, Linux. If a prebuild does not work, please create an issue.

# Build

```sh
napi build --release
```

# Usage

## class RaknetClient, RaknetServer

The RaknetClient and RaknetServer classes are JS wrappers for the internal RaknetSocket and RaknetListener classes implemented in Rust in src/.

All methods use asynchronous wrappers.

## Example

```js
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
```

# Lisence

MIT - https://github.com/b23r0/raknet-node/blob/main/LICENSE