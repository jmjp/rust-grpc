use tonic::{transport::Server, Request, Response, Status};

use api::echo_service_server::{EchoService, EchoServiceServer};
use api::{EchoRequest, EchoResponse};

use ::clap::{Parser};

pub mod api {
    tonic::include_proto!("api");
}


#[derive(Parser)]
#[command(author, version)]
#[command(about = "echo-server - a simple echo microservice", long_about = None)]
pub struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "0.0.0.0")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "9090")]
    port: u16,
}


#[derive(Debug, Default)]
pub struct Echo {}

#[tonic::async_trait]
impl EchoService for Echo {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let data = request.into_inner().clone();
        let reply = EchoResponse {
            name: format!("{}", data.name),
            message: format!("Olá, {}", data.name),
        };

        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let echo = Echo::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}