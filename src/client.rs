pub mod hello {
  tonic::include_proto!("hello");
}
use hello::HelloWorldRequest;
use hello::hello_client::HelloClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut client = HelloClient::connect("http://[::1]:9000").await?;

  let request = tonic::Request::new(HelloWorldRequest {
    name: "Jayanto".to_owned(),
  });

  let res = client.hello_world(request).await?;
  println!("{:#?}", res.into_inner());
  Ok(())
}
