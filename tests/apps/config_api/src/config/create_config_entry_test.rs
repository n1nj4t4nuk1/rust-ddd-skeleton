use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use config_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_201_when_entry_is_created() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "my-key", "value": "my-value" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn it_persists_the_entry_and_can_be_retrieved() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "greeting", "value": "hello" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let get_req = test::TestRequest::get()
        .uri("/config/greeting")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["key"], "greeting");
    assert_eq!(body["value"], "hello");
}

#[tokio::test]
async fn it_returns_409_when_creating_a_duplicate_key() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let body = json!({ "key": "dup-key", "value": "some-value" });

    let first = test::TestRequest::post()
        .uri("/config")
        .set_json(&body)
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post()
        .uri("/config")
        .set_json(&body)
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);
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

#[tokio::test]
async fn it_stores_the_value_exactly_as_provided() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "exact-key", "value": "exact value with spaces" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let get_req = test::TestRequest::get()
        .uri("/config/exact-key")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["value"], "exact value with spaces");
}

#[tokio::test]
async fn it_isolates_state_between_tests() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/config/greeting")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
