use tonic::{Request, Response, Status};
use derive_new::new;
use std::sync::Arc;
use crate::proto::users::user_guide_server::UserGuide;
use crate::proto::users::{GetUserRequest, GetUserResponse};

use crate::driver::modules::Modules;
use crate::app::model::users::FindUserName;

#[derive(new)]
pub struct UserGuideService {
    pub modules: Arc<Modules>,
}

#[tonic::async_trait]
impl UserGuide for UserGuideService {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> {
        // GetUserRequest → FindUserNameに変換
        let user_name = request.into_inner().into();

        let user = self.modules.users_use_case.find_user(user_name).await;

        match user {
            Ok(user) => {
                if let Some(u) = user {
                    let res = GetUserResponse {
                        id: u.id,
                        name: u.name,
                    };

                    Ok(Response::new(res))
                } else {
                    Err(Status::internal("Internal server error"))
                }
            },
            Err(e) => Err(Status::internal(format!("Internal server error: {}", e)))
        }
    }
}

impl From<GetUserRequest> for FindUserName {
    fn from(gur: GetUserRequest) -> Self {
        FindUserName {
            name: gur.name,
        }
    }
}