use crate::state::AppState;
use crate::infra::mongo::repositories::InferenceServerRepository;
use crate::domain::repositories::InferenceServerRepository as DomainInferenceServerRepository;
use shared::requests::GetInferenceServerDocsPath;
use std::collections::hash_map::HashMap;
use actix_web::{
    web,
    get,
    HttpRequest as ActixHttpRequest,
    HttpResponse,
    Responder as ActixResponder
};

#[get("/inference-api/inference-servers/{inference_server_name}/docs")]
async fn get_inference_server_docs(
    _req: ActixHttpRequest,
    _path: web::Path<GetInferenceServerDocsPath>,
    _query: web::Query<HashMap<String, String>>,
    _body: web::Bytes,
    data: web::Data<AppState>
) -> impl ActixResponder {
    let repo = InferenceServerRepository::new(data.db.clone());
    let _test = repo.list_all().await;
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
            <meta charset="UTF-8">
            <title>Swagger UI</title>
            <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist/swagger-ui.css">
            </head>
            <body>
                <div id="swagger-ui"></div>

                <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
                <script>
                    SwaggerUIBundle({
                        url: "https://raw.githubusercontent.com/tapis-project/openapi-systems/prod/SystemsAPI.yaml",
                        dom_id: "\#swagger-ui",
                    });
                </script>
            </body>
        </html>
    "#;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}
