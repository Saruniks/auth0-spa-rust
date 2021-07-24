use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub given_name: String,
    pub family_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Claim {
    pub __raw: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct ConfigOptions {
    pub domain: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub useRefreshTokens: bool,
    pub cacheLocation: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct GetTokenSilentlyOptions {
    pub audience: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct AuthLogoutOptions {
    pub returnTo: String,
}
