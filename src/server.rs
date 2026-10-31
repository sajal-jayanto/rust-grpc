use tonic::{transport::Server, Request, Response, Status};

use hello::hello_server::{ Hello , HelloServer};
use hello::{ HelloWorldRequest , HelloWorldResponse };

pub mod hello { 
  tonic::include_proto!("hello"); 
}

#[derive(Debug, Default)]
pub struct HelloService {} 

#[tonic::async_trait]
impl Hello for HelloService {
  async fn hello_world(&self , request: Request<HelloWorldRequest>) -> Result<Response<HelloWorldResponse> , Status> {
    let req_body = request.into_inner();
    let ans = HelloWorldResponse {
      message: format!("Hello World from {}" , req_body.name).into(),
    };
    Ok(Response::new(ans))
  }
}

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:9000".parse()?;

  let hello_world_service = HelloService::default();

  Server::builder()
    .add_service(HelloServer::new(hello_world_service))
    .serve(addr)
    .await?;

  Ok(())
}