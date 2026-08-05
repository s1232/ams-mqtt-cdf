use crate::config::CogniteConfig;
use cognite::AuthenticatorConfig;
use cognite::CogniteClient;

pub fn get_client(config: &CogniteConfig) -> CogniteClient {
    let auth_config = AuthenticatorConfig {
        client_id: config.client_id.clone(),
        secret: config.client_secret.clone(),
        token_url: config.token_url.clone(),
        resource: None,
        audience: None,
        scopes: None,
        default_expires_in: None,
    };
    CogniteClient::new_from_oidc(
        &config.base_url,
        auth_config,
        &config.project,
        "amsreadings",
        None,
    )
    .expect("Could not instantiate Cognite client")
}
