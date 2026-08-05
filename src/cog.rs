use cognite::AuthenticatorConfig;
use cognite::CogniteClient;

pub fn get_client() -> CogniteClient {
    let auth_config = AuthenticatorConfig {
        client_id: std::env::var("COGNITE_CLIENT_ID").unwrap(),
        secret: std::env::var("COGNITE_CLIENT_SECRET").unwrap(),
        token_url: std::env::var("COGNITE_TOKEN_URL").unwrap(),
        resource: None,
        audience: None,
        scopes: None,
        default_expires_in: None,
    };
    CogniteClient::new_from_oidc(
        "https://greenfield.cognitedata.com",
        auth_config,
        "arild-lab",
        "amsreadings",
        None,
    )
    .expect("Could not instantiate Cognite client")
}
