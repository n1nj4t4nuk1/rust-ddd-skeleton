use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use config_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_204_when_entry_is_deleted() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "to-delete", "value": "bye" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let delete_req = test::TestRequest::delete()
        .uri("/config/to-delete")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, delete_req).await;
    assert_eq!(resp.status(), 204);
}

#[tokio::test]
async fn it_is_no_longer_retrievable_after_deletion() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "to-delete", "value": "bye" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let delete_req = test::TestRequest::delete()
        .uri("/config/to-delete")
        .to_request();
    let _: ServiceResponse = test::call_service(&app, delete_req).await;

    let get_req = test::TestRequest::get()
        .uri("/config/to-delete")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_404_when_deleting_nonexistent_entry() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/config/ghost")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_404_when_deleting_the_same_entry_twice() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let post_req = test::TestRequest::post()
        .uri("/config")
        .set_json(json!({ "key": "once", "value": "only" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let first = test::TestRequest::delete()
        .uri("/config/once")
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::delete()
        .uri("/config/once")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 404);
}
