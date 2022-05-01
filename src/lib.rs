use std::net::SocketAddr;

use rust_raknet::*;
use napi_derive::*;
use napi::bindgen_prelude::*;
/// module registration is done by the runtime, no need to explicitly do it now.
#[napi]
pub struct RaknetServer {
    server : RaknetListener
}

#[napi]
pub struct RaknetClient {
    client : RaknetSocket
}

#[napi]
impl RaknetClient{
    #[napi]
    pub async fn connect(address : String ) -> Result<RaknetClient> {
        let address : SocketAddr = match address.parse(){
            Ok(p) => p,
            Err(e) => {
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("{}", e),
                  ));
            }
        };
        let client = match RaknetSocket::connect(&address).await{
            Ok(p) => p,
            Err(e) => {
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("{:?}", e),
                  ));
            }
        };

        Ok(RaknetClient{
            client : client
        })
    }

    #[napi]
    pub async fn send(&mut self ,buf : Buffer) -> Result<()> {
        let buf = Vec::<u8>::from(buf);
        match self.client.send(&buf , Reliability::ReliableOrdered).await{
            Ok(p) => Ok(p),
            Err(e) => {
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("{:?}", e),
                  ));
            }
        }
    }

    #[napi]
    pub async fn recv(&mut self) -> Result<Buffer>{
        let buf = match self.client.recv().await{
            Ok(p) => p,
            Err(e) => {
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("{:?}", e),
                  ));
            }
        };

        Ok(buf.into())
    }
}

#[napi]
impl RaknetServer {
    #[napi]
    pub async fn bind(address : String ) -> Result<RaknetServer>{
        let address : SocketAddr = match address.parse(){
            Ok(p) => p,
            Err(e) => {
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("{}", e),
                  ));
            }
        };
        let mut server = RaknetListener::bind(address).await.unwrap();

        server.listen().await;

        Ok(RaknetServer{
            server : server
        })
    }

    #[napi]
    pub async fn accept(&mut self) -> Result<RaknetClient> {
        let client = match self.server.accept().await{
            Ok(p) => p,
            Err(e) => {
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("{:?}", e),
                  ));
            }
        };
        Ok(RaknetClient{
            client : client
        })
    }
}