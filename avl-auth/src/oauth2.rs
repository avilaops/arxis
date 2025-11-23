//! OAuth2 and OpenID Connect implementation

use crate::error::{AuthError, Result};
use crate::models::OAuth2Provider;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct OAuth2Manager {
    providers: Arc<RwLock<HashMap<String, ProviderClient>>>,
}

struct ProviderClient {
    client: BasicClient,
    scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub url: String,
    pub state: String,
    pub pkce_verifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenExchange {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub id_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub provider: String,
}

impl OAuth2Manager {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_provider(&self, provider: OAuth2Provider) -> Result<()> {
        let client = BasicClient::new(
            ClientId::new(provider.client_id.clone()),
            Some(ClientSecret::new(provider.client_secret.clone())),
            AuthUrl::new(provider.auth_url.clone())
                .map_err(|e| AuthError::ConfigError(e.to_string()))?,
            Some(
                TokenUrl::new(provider.token_url.clone())
                    .map_err(|e| AuthError::ConfigError(e.to_string()))?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(provider.redirect_url.clone())
                .map_err(|e| AuthError::ConfigError(e.to_string()))?,
        );

        let provider_client = ProviderClient {
            client,
            scopes: provider.scopes.clone(),
        };

        let mut providers = self.providers.write().await;
        providers.insert(provider.name.clone(), provider_client);

        tracing::info!("Registered OAuth2 provider: {}", provider.name);
        Ok(())
    }

    pub async fn authorize_url(
        &self,
        provider_name: &str,
        use_pkce: bool,
    ) -> Result<AuthorizationRequest> {
        let providers = self.providers.read().await;
        let provider = providers
            .get(provider_name)
            .ok_or_else(|| AuthError::OAuth2Error(format!("Provider not found: {}", provider_name)))?;

        let mut auth_request = provider.client.authorize_url(CsrfToken::new_random);

        for scope in &provider.scopes {
            auth_request = auth_request.add_scope(Scope::new(scope.clone()));
        }

        let (url, state, pkce_verifier) = if use_pkce {
            let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
            let (url, state) = auth_request.set_pkce_challenge(challenge).url();
            (url, state, Some(verifier.secret().to_string()))
        } else {
            let (url, state) = auth_request.url();
            (url, state, None)
        };

        Ok(AuthorizationRequest {
            url: url.to_string(),
            state: state.secret().to_string(),
            pkce_verifier,
        })
    }

    pub async fn exchange_code(
        &self,
        provider_name: &str,
        code: &str,
        pkce_verifier: Option<&str>,
    ) -> Result<TokenExchange> {
        let providers = self.providers.read().await;
        let provider = providers
            .get(provider_name)
            .ok_or_else(|| AuthError::OAuth2Error(format!("Provider not found: {}", provider_name)))?;

        let mut token_request = provider
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()));

        if let Some(verifier) = pkce_verifier {
            token_request = token_request.set_pkce_verifier(PkceCodeVerifier::new(verifier.to_string()));
        }

        let token_response = token_request
            .request_async(async_http_client)
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        Ok(TokenExchange {
            access_token: token_response.access_token().secret().to_string(),
            refresh_token: token_response.refresh_token().map(|t| t.secret().to_string()),
            expires_in: token_response.expires_in().map(|d| d.as_secs()),
            id_token: None, // OAuth2 crate doesn't provide id_token directly
        })
    }

    pub async fn get_user_info(
        &self,
        provider_name: &str,
        access_token: &str,
    ) -> Result<UserInfo> {
        match provider_name {
            "google" => self.get_google_user_info(access_token).await,
            "github" => self.get_github_user_info(access_token).await,
            "microsoft" => self.get_microsoft_user_info(access_token).await,
            _ => Err(AuthError::OAuth2Error(format!(
                "User info not supported for provider: {}",
                provider_name
            ))),
        }
    }

    async fn get_google_user_info(&self, access_token: &str) -> Result<UserInfo> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        #[derive(Deserialize)]
        struct GoogleUserInfo {
            id: String,
            email: String,
            verified_email: bool,
            name: Option<String>,
            picture: Option<String>,
        }

        let user: GoogleUserInfo = response
            .json()
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        Ok(UserInfo {
            id: user.id,
            email: user.email,
            email_verified: user.verified_email,
            name: user.name,
            picture: user.picture,
            provider: "google".to_string(),
        })
    }

    async fn get_github_user_info(&self, access_token: &str) -> Result<UserInfo> {
        let client = reqwest::Client::new();

        // Get user info
        let user_response = client
            .get("https://api.github.com/user")
            .bearer_auth(access_token)
            .header("User-Agent", "AVL-Auth")
            .send()
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        #[derive(Deserialize)]
        struct GitHubUser {
            id: u64,
            _login: String,
            name: Option<String>,
            avatar_url: Option<String>,
        }

        let user: GitHubUser = user_response
            .json()
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        // Get primary email
        let email_response = client
            .get("https://api.github.com/user/emails")
            .bearer_auth(access_token)
            .header("User-Agent", "AVL-Auth")
            .send()
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        #[derive(Deserialize)]
        struct GitHubEmail {
            email: String,
            primary: bool,
            verified: bool,
        }

        let emails: Vec<GitHubEmail> = email_response
            .json()
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        let primary_email = emails
            .into_iter()
            .find(|e| e.primary)
            .ok_or_else(|| AuthError::OAuth2Error("No primary email found".to_string()))?;

        Ok(UserInfo {
            id: user.id.to_string(),
            email: primary_email.email,
            email_verified: primary_email.verified,
            name: user.name,
            picture: user.avatar_url,
            provider: "github".to_string(),
        })
    }

    async fn get_microsoft_user_info(&self, access_token: &str) -> Result<UserInfo> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://graph.microsoft.com/v1.0/me")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        #[derive(Deserialize)]
        struct MicrosoftUserInfo {
            id: String,
            #[serde(rename = "userPrincipalName")]
            user_principal_name: String,
            #[serde(rename = "displayName")]
            display_name: Option<String>,
        }

        let user: MicrosoftUserInfo = response
            .json()
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;

        Ok(UserInfo {
            id: user.id,
            email: user.user_principal_name,
            email_verified: true, // Microsoft accounts are verified
            name: user.display_name,
            picture: None,
            provider: "microsoft".to_string(),
        })
    }
}

impl Default for OAuth2Manager {
    fn default() -> Self {
        Self::new()
    }
}
