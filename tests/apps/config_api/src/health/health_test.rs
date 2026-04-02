use actix_web::{dev::ServiceResponse, test, App};

use config_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_200_on_health_check() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}
