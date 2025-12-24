use tonic::{transport::Server, Request, Response, Status};

pub mod rkv {
    tonic::include_proto!("rkv");
}

use rkv::kv_store_server::{KvStore, KvStoreServer};
use rkv::{Empty, Pong};

#[derive(Default)]
pub struct MyKVServer {}

#[tonic::async_trait]
impl KvStore for MyKVServer {
    async fn ping(&self, _request: Request<Empty>) -> Result<Response<Pong>, Status> {
        println!("Received a ping request!");

        let reply = Pong {
            message: "Hello from RKV Server!".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let kv_service = MyKVServer::default();

    println!("RKV Server listening on {}", addr);

    Server::builder()
        .add_service(KvStoreServer::new(kv_service))
        .serve(addr)
        .await?;

    Ok(())
}
