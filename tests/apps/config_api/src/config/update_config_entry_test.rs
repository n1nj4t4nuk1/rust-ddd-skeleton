use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use config_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_200_when_entry_is_updated() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "theme", "value": "light" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let put_req = test::TestRequest::put()
        .uri("/config/theme")
        .set_json(json!({ "value": "dark" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, put_req).await;
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn it_persists_the_updated_value() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "theme", "value": "light" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let put_req = test::TestRequest::put()
        .uri("/config/theme")
        .set_json(json!({ "value": "dark" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, put_req).await;

    let get_req = test::TestRequest::get()
        .uri("/config/theme")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["value"], "dark");
}

#[tokio::test]
async fn it_returns_404_when_updating_nonexistent_entry() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::put()
        .uri("/config/ghost")
        .set_json(json!({ "value": "new-value" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
