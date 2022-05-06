use std::net::SocketAddr;

use rust_raknet::*;
use napi_derive::*;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction , ThreadsafeFunctionCallMode};
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

    #[napi]
    pub async fn peer_addr(&mut self) -> Result<String> {
        Ok(self.client.peer_addr().unwrap().to_string())
    }

    #[napi]
    pub async fn local_addr(&mut self) -> Result<String> {
        Ok(self.client.local_addr().unwrap().to_string())
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
        let mut server = RaknetListener::bind(&address).await.unwrap();

        server.listen().await;

        Ok(RaknetServer{
            server : server
        })
    }

    #[napi]
    pub fn listen(address : String  , callback: JsFunction) {
        let tsfn: ThreadsafeFunction<RaknetClient, ErrorStrategy::CalleeHandled> = callback.create_threadsafe_function(0, |ctx| {
            Ok(vec![ctx.value])
        }).unwrap();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async move {
                loop{
                    let mut server = RaknetListener::bind(&address.parse().unwrap()).await.unwrap();
                    server.listen().await;
                    let client = server.accept().await.unwrap();
                    tsfn.call(
                        Ok(RaknetClient{
                            client : client
                        }) , 
                        ThreadsafeFunctionCallMode::Blocking
                    );
                }
            });
        });
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

    #[napi]
    pub async fn local_addr(&mut self) -> Result<String> {
        Ok(self.server.local_addr().unwrap().to_string())
    }
}