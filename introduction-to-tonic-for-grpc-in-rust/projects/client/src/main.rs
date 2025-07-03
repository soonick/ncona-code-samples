use tonic::Request;

use protos::{
    basic_service_client::BasicServiceClient,
    GreetRequest
};

pub mod protos {
    tonic::include_proto!("example.protos");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BasicServiceClient::connect("http://tonic-server:50051").await?;

    let request = Request::new(GreetRequest {
        name: "Carlos".to_string()
    });

    let response = client.greet(request).await?;

    println!("The response message is: {:?}", response.get_ref());
    println!("The response metadata is: {:?}", response.metadata());
    println!("The response greeting is: {}", response.get_ref().greeting);

    Ok(())
}
