use utoipa::OpenApi;

use crate::handler::{hello_handler, user_handler};

/// OpenAPI 文档定义，由 utoipa 宏从 Handler 注解自动收集生成
#[derive(OpenApi)]
#[openapi(
    info(
        title = "axum-tutorial API",
        version = "0.1.0",
        description = "基于 axum 的学习项目：路由、Handler、状态管理与用户 CRUD 示例"
    ),
    paths(
        hello_handler::hello1,
        hello_handler::hello2,
        hello_handler::hello_query,
        user_handler::list_users,
        user_handler::create_user,
        user_handler::get_user,
        user_handler::update_user,
        user_handler::delete_user,
    ),
    components(
        schemas(
            user_handler::User,
            user_handler::CreateUser,
            user_handler::UpdateUser,
            hello_handler::HelloResp,
        )
    ),
    tags(
        (name = "hello", description = "Hello 演示接口"),
        (name = "users", description = "用户 CRUD 接口"),
    )
)]
pub struct ApiDoc;
