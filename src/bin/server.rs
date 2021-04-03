use kvstore::rpc_server::{Rpc, RpcServer};
use kvstore::Request as kv_request;
use kvstore::Response as kv_response;
use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Status};

// why push error
pub mod kvstore {
    tonic::include_proto!("rpc");
}

struct KvStoreImpl {
    value: HashMap<String, String>,
}

impl KvStoreImpl {
    pub fn init(&mut self) {
        for i in 0..10 {
            self.value
                .insert(format!("key{}", i), format!("value{}", i));
        }
    }
}

impl Default for KvStoreImpl {
    fn default() -> Self {
        let tep: HashMap<String, String> = HashMap::default();
        KvStoreImpl { value: tep }
    }
}

#[tonic::async_trait]
impl Rpc for KvStoreImpl {
    async fn get_values(
        &self,
        request: Request<kv_request>,
    ) -> Result<Response<kv_response>, Status> {
        if self.value.contains_key(&request.get_ref().key) {
            return Ok(Response::new(kv_response {
                value: self.value[&request.get_ref().key].clone(),
            }));
        }
        Err(Status::not_found("not found"))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut server_impl = KvStoreImpl::default();
    server_impl.init();

    let addr = "0.0.0.0:50051".parse().unwrap();

    println!("Server start and run!");
    let _server = Server::builder()
    .add_service(RpcServer::new(server_impl))
    .serve(addr).await?;
    
    Ok(())
}
