use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, header, Response},
    response::IntoResponse,
    routing::get_service,
};
use tower::util::MapResponseLayer;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable as _};
use utoipa_scalar::{Scalar, Servable as _};
use utoipa_swagger_ui::SwaggerUi;

use crate::{docs::ApiDoc, handler::not_found::not_found, state::AppState};

pub mod hello_router;
pub mod user_router;

pub fn api_router() -> Router<AppState> {
    Router::<AppState>::new()
        .merge(hello_router::router()) // Router<()> 可并入任意 Router<S>
        .nest("/users", user_router::routes())
        // 四种 API 文档 UI，共用同一份 ApiDoc：
        // Swagger UI: /swagger-ui，OpenAPI JSON: /api-docs/openapi.json
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Redoc: /redoc（三栏阅读式文档，适合对外发布）
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // Scalar: /scalar（现代美观，交互体验好）
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        // RapiDoc: /rapidoc（轻量，支持 Try it out）
        // 用 new 而非 with_openapi：后者会自己注册 spec 路由，与上面的
        // /api-docs/openapi.json 重叠导致 panic；new 只引用已有地址
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}

/// ServeDir 设置的 Content-Type（如 text/html）不带 charset，
/// 中文 Windows 浏览器会退回 GBK 解码导致乱码，这里统一补上 charset=utf-8
fn utf8_charset<B>(mut res: Response<B>) -> Response<B> {
    let ct = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    if let Some(ct) = ct {
        if ct.starts_with("text/") && !ct.contains("charset") {
            if let Ok(v) = HeaderValue::from_str(&format!("{ct}; charset=utf-8")) {
                res.headers_mut().insert(header::CONTENT_TYPE, v);
            }
        }
    }
    res
}

pub fn static_router() -> Router<AppState> {
    Router::<AppState>::new().fallback_service(get_service(
        ServeDir::new("./").not_found_service(tower::service_fn(|_req: Request| async {
            Ok::<_, std::convert::Infallible>(not_found().await.into_response())
        })),
    )
    .layer(MapResponseLayer::new(utf8_charset)))
}
