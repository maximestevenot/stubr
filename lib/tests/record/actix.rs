use actix_web::{
    test::{call_service, init_service, TestRequest},
    web, App, HttpResponse,
};
use asserhttp::*;
use serde_json::json;

use stubr::{ActixRecord, RecordConfig};

use crate::utils::*;

#[actix_web::test]
async fn should_record_from_actix_integration_test() {
    let cfg = RecordConfig {
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(relaxed_resp_headers()),
        ..Default::default()
    };
    let uri = "/record-client/actix";
    let app = App::new()
        .route(uri, web::get().to(|| async { HttpResponse::Ok().await }))
        .wrap(ActixRecord(cfg));
    call_service(&init_service(app).await, TestRequest::get().uri(uri).to_request())
        .await
        .expect_status_ok();
    assert_recorded_stub_eq(
        "record-client-actix-11162265122061890704",
        json!({
            "request": {
                "method": "GET",
                "urlPath": uri
            },
            "response": {
                "status": 200
            }
        }),
    )
}

#[actix_web::test]
async fn should_record_from_actix_failing_integration_test() {
    let cfg = RecordConfig {
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(relaxed_resp_headers()),
        ..Default::default()
    };
    let uri = "/record-client/actix/ko";
    let app = App::new()
        .route(uri, web::get().to(|| async { HttpResponse::InternalServerError().await }))
        .wrap(ActixRecord(cfg));
    call_service(&init_service(app).await, TestRequest::get().uri(uri).to_request())
        .await
        .expect_status_internal_server_error();
    assert_recorded_stub_eq(
        "record-client-actix-ko-7499948008303883645",
        json!({
            "request": {
                "method": "GET",
                "urlPath": uri
            },
            "response": {
                "status": 500
            }
        }),
    )
}
