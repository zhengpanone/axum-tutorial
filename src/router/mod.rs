use axum::Router;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable as _};
use utoipa_scalar::{Scalar, Servable as _};
use utoipa_swagger_ui::SwaggerUi;

use crate::{docs::ApiDoc, state::AppState};

pub mod hello_router;
pub mod user_router;

pub fn api_router() -> Router<AppState> {
    Router::<AppState>::new()
        .merge(hello_router::router()) // Router<()> 可并入任意 Router<S>
        .nest("/users", user_router::routes())
        // 四种 API 文档 UI，共用同一份 ApiDoc：
        // Swagger UI: /swagger-ui，OpenAPI JSON: /api-docs/openapi.json
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        // Redoc: /redoc（三栏阅读式文档，适合对外发布）
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // Scalar: /scalar（现代美观，交互体验好）
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        // RapiDoc: /rapidoc（轻量，支持 Try it out）
        // 用 new 而非 with_openapi：后者会自己注册 spec 路由，与上面的
        // /api-docs/openapi.json 重叠导致 panic；new 只引用已有地址
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}
