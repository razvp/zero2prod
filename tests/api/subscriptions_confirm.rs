use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    dbg!(&app.address);
    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .expect("failed to execute request");

    assert_eq!(response.status(), 400);
}
