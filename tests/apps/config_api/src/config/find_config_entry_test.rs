use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use config_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_200_with_the_entry_when_found() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "lang", "value": "rust" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let get_req = test::TestRequest::get()
        .uri("/config/lang")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn it_returns_the_correct_key_and_value() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "color", "value": "blue" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let get_req = test::TestRequest::get()
        .uri("/config/color")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["key"], "color");
    assert_eq!(body["value"], "blue");
}

#[tokio::test]
async fn it_returns_404_when_entry_does_not_exist() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/config/nonexistent")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
