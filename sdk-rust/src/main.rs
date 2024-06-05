use bitwarden::{
    auth::login::AccessTokenLoginRequest, client::client_settings::ClientSettings, error::Result,
    secrets_manager::secrets::SecretCreateRequest, Client,
};
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identity_url = env::var("BW_IDENTITY_URL").unwrap();
    let api_url = env::var("BW_API_URL").unwrap();
    let access_token = env::var("BW_ACCESS_TOKEN").unwrap();
    let organization_id =
        Uuid::parse_str(env::var("BW_ORGANIZATION_ID").unwrap().as_str()).unwrap();
    let project_id = Uuid::parse_str(env::var("BW_PROJECT_ID").unwrap().as_str()).unwrap();

    let mut client = Client::new(Some(ClientSettings {
        identity_url: identity_url,
        api_url: api_url,
        ..Default::default()
    }));

    client
        .auth()
        .login_access_token(&AccessTokenLoginRequest {
            access_token: access_token,
            state_file: None,
        })
        .await
        .unwrap();

    let secret = client
        .secrets()
        .create(&SecretCreateRequest {
            organization_id,
            key: "Secret Key".to_string(),
            value: "Secret Value".to_string(),
            note: "Secret Note".to_string(),
            project_ids: Some(vec![project_id]),
        })
        .await
        .unwrap();

    print!(
        "Secret Created!\n\n  Key: {}\nValue: {}\n Note: {}\n",
        secret.key, secret.value, secret.note
    );

    Ok(())
}
