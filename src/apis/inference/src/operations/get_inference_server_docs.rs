use actix_web::{web, get, HttpResponse, Responder};
use shared::requests::GetInferenceServerDocsPath;
use std::collections::hash_map::HashMap;

#[get("/inference-api/inference-servers/{inference_server_name}/docs")]
async fn get_inference_server_docs(
    req: ActixHttpRequest,
    path: web::Path<GetInferenceServerDocsPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Bytes,
) -> impl Responder {
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
