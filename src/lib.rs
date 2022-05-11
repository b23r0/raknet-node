use std::net::SocketAddr;

use rust_raknet::*;
use napi_derive::*;
use napi::bindgen_prelude::*;

#[napi]
pub struct RaknetServer {
    server : RaknetListener
}

#[napi]
pub struct RaknetClient {
    client : RaknetSocket
}

#[napi]
pub fn enablelog(){
    enable_raknet_log(255);
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
        let client = match match tokio::time::timeout(std::time::Duration::from_secs(10) , RaknetSocket::connect(&address)).await{
            Ok(p) => p,
            Err(e) => {
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("{:?}", e),
                  ));
            }
        }{
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
    pub fn peeraddr(&mut self) -> Result<String> {
        Ok(self.client.peer_addr().unwrap().to_string())
    }

    #[napi]
    pub fn localaddr(&mut self) -> Result<String> {
        Ok(self.client.local_addr().unwrap().to_string())
    }

    #[napi]
    pub async fn close(&mut self) -> Result<()> {
        self.client.close().await.unwrap();
        Ok(())
    }

    #[napi]
    pub async fn ping(address : String) -> Result<String> {
        let (_ , motd) = match RaknetSocket::ping(&address.parse().unwrap()).await{
            Ok(p) => p,
            Err(e) => {
                return Err(Error::new(
                    Status::GenericFailure,
                    format!("{:?}", e),
                  ));
            }
        };
        Ok(motd)
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
    pub async fn localaddr(&mut self) -> Result<String> {
        Ok(self.server.local_addr().unwrap().to_string())
    }

    #[napi]
    pub fn close(&mut self) -> Result<()> {
        Ok(self.server.close().unwrap())
    }

    #[napi]
    pub fn setmotd(&mut self , motd : String){
        self.server.set_full_motd(motd).unwrap();
    }
}