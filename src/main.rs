use tonic::transport::Server;
use anyhow::Result;
use std::sync::Arc;

use grpc_db_server::proto::users::user_guide_server::UserGuideServer;

use grpc_db_server::driver::{
    grpc::handler::UserGuideService,
    modules::Modules
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "172.21.0.4:50051".parse()?;

    let modules = Arc::new(Modules::new().await);

    let user_guide_service = UserGuideService::new(modules);

    let svc = UserGuideServer::new(user_guide_service);

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
