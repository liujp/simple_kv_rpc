use kv_client::rpc_client::RpcClient;
use kv_client::{Request};

pub mod kv_client {
    tonic::include_proto!("rpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RpcClient::connect("http://0.0.0.0:50051").await?;
    for i in 0..10 {
        let request = tonic::Request::new(Request{key: format!("key{}",i)});
        let response = client.get_values(request).await?;
        println!("RESPONSE={:?}", response.into_inner());
    }

    Ok(())
}