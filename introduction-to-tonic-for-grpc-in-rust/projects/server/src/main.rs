use tonic::{transport::Server, Request, Response, Status};

use protos::{
    basic_service_server::{BasicService, BasicServiceServer},
    {GreetResponse, GreetRequest}
};

pub mod protos {
    tonic::include_proto!("example.protos");
}

#[derive(Debug, Default)]
pub struct BasicServiceImpl {}

#[tonic::async_trait]
impl BasicService for BasicServiceImpl {
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        let reply = GreetResponse {
            greeting: format!(" Hi {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let service = BasicServiceImpl::default();

    Server::builder()
        .add_service(BasicServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
