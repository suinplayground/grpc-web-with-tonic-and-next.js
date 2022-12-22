use greeting::greeter_server::{Greeter, GreeterServer};
use greeting::{GreetingMessage, Person};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

mod greeting {
    // greeting.protoから生成したRustコードを展開するマクロ
    tonic::include_proto!("greeting");
}

#[derive(Default)]
struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<Person>,
    ) -> Result<Response<GreetingMessage>, Status> {
        // gRPCリクエストから入力値を参照する
        let name = request.into_inner().name;
        println!("Creating a greeting message for {:?}", name);
        // レスポンスの内容を作成する
        let greeting_message = GreetingMessage {
            text: format!("Hello {}!", name),
        };
        // gRPCレスポンスを作成する
        let response = Response::new(greeting_message);
        // gRPCレスポンスを返す
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    let allow_cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any);
    println!("GreeterServer listening on {}", addr);
    Server::builder()
        .accept_http1(true) // gRPC-webに対応するために必要
        .layer(allow_cors) // CORSに対応するために必要
        .layer(GrpcWebLayer::new()) // gRPC-webに対応するために必要
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
