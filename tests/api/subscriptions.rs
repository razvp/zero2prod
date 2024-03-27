use crate::helpers::spawn_app;
use std::collections::HashMap;


#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("name", "le guin");
    params.insert("email", "ursula_le_guin@gmail.com");
    let response = client
        .post(&format!("{}/subscribe", test_app.address))
        .form(&params)
        .send()
        .await
        .expect("failed to execute request");
    assert_eq!(200, response.status());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("failed to fetch subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await;
    let app_address = test_app.address;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{app_address}/subscribe"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");
        assert_eq!(
            400,
            response.status(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status(),
            "The API did not return a 400 OK when the payload was {}.",
            description
        );
    }
}
