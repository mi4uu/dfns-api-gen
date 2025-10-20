// Auto-generated API endpoints
// Do not edit manually

use axum::{extract::Path as AxumPath, http::StatusCode, Json};
use crate::generated;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi(
    info(title = "Dfns API", version = "1.0.0"),
    paths(
        get_latest_unaccepted,
        post_agreement_id_accept,
        post_action,
        post_action_init,
        get_action_logs,
        get_action_logs_id,
        get_apps,
        get_apps_app_id,
        get_credentials,
        post_credentials,
        put_credentials_activate,
        post_credentials_code,
        post_credentials_code_init,
        post_credentials_code_verify,
        put_credentials_deactivate,
        post_credentials_init,
        post_login,
        post_login_code,
        post_login_delegated,
        post_login_init,
        post_login_social,
        post_login_sso,
        post_login_sso_init,
        put_logout,
        get_pats,
        post_pats,
        get_pats_token_id,
        put_pats_token_id,
        delete_pats_token_id,
        put_pats_token_id_activate,
        put_pats_token_id_deactivate,
        post_recover_user,
        post_recover_user_code,
        post_recover_user_delegated,
        post_recover_user_init,
        post_registration,
        put_registration_code,
        post_registration_delegated,
        post_registration_enduser,
        post_registration_init,
        post_registration_social,
        get_service_accounts,
        post_service_accounts,
        get_service_accounts_service_account_id,
        put_service_accounts_service_account_id,
        delete_service_accounts_service_account_id,
        put_service_accounts_service_account_id_activate,
        put_service_accounts_service_account_id_deactivate,
        get_users,
        post_users,
        get_users_user_id,
        put_users_user_id,
        delete_users_user_id,
        put_users_user_id_activate,
        put_users_user_id_deactivate,
        get_,
        post_,
        get_exchange_id,
        delete_exchange_id,
        get_exchange_id_accounts,
        get_exchange_id_accounts_account_id_assets,
        get_exchange_id_accounts_account_id_assets_asset_withdrawal_networks,
        post_exchange_id_accounts_account_id_deposits,
        post_exchange_id_accounts_account_id_withdrawals,
        get_,
        post_,
        get_fee_sponsor_id,
        delete_fee_sponsor_id,
        put_fee_sponsor_id_activate,
        put_fee_sponsor_id_deactivate,
        get_fee_sponsor_id_fees,
        get_,
        get_,
        post_,
        post_import,
        get_key_id,
        put_key_id,
        delete_key_id,
        post_key_id_delegate,
        post_key_id_derive,
        post_key_id_export,
        get_key_id_signatures,
        post_key_id_signatures,
        get_key_id_signatures_signature_id,
        get_fees,
        post_read_contract,
        get_network_validators,
        post_network_validators,
        get_network_validators_validator_id,
        put_network_validators_validator_id,
        delete_network_validators_validator_id,
        get_,
        post_,
        get_permission_id,
        put_permission_id,
        put_permission_id_archive,
        get_permission_id_assignments,
        post_permission_id_assignments,
        delete_permission_id_assignments_assignment_id,
        get_,
        get_stakes,
        post_stakes,
        get_stakes_stake_id,
        get_stakes_stake_id_actions,
        post_stakes_stake_id_actions,
        get_stakes_stake_id_rewards,
        get_,
        post_,
        post_quotes,
        get_quotes_quote_id,
        get_swap_id,
        get_policies,
        post_policies,
        get_policies_policy_id,
        put_policies_policy_id,
        delete_policies_policy_id,
        get_policy_approvals,
        get_policy_approvals_approval_id,
        post_policy_approvals_approval_id_decisions,
        get_,
        post_,
        get_all_history,
        post_import,
        get_wallet_id,
        put_wallet_id,
        get_wallet_id_assets,
        post_wallet_id_delegate,
        post_wallet_id_export,
        get_wallet_id_history,
        get_wallet_id_nfts,
        get_wallet_id_offers,
        get_wallet_id_offers_offer_id,
        put_wallet_id_offers_offer_id_accept,
        put_wallet_id_offers_offer_id_reject,
        get_wallet_id_signatures,
        post_wallet_id_signatures,
        get_wallet_id_signatures_signature_id,
        put_wallet_id_tags,
        delete_wallet_id_tags,
        get_wallet_id_transactions,
        post_wallet_id_transactions,
        get_wallet_id_transactions_transaction_id,
        get_wallet_id_transfers,
        post_wallet_id_transfers,
        get_wallet_id_transfers_transfer_id,
        get_,
        post_,
        get_webhook_id,
        put_webhook_id,
        delete_webhook_id,
        get_webhook_id_events,
        get_webhook_id_events_webhook_event_id,
        post_webhook_id_ping,
        get_,
        post_,
        get_yield_id,
        get_yield_id_actions,
        post_yield_id_actions
    )
)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn router() -> OpenApiRouter {
        OpenApiRouter::with_openapi(ApiDoc::openapi())
            .routes(routes!(get_latest_unaccepted))
            .routes(routes!(post_agreement_id_accept))
            .routes(routes!(post_action))
            .routes(routes!(post_action_init))
            .routes(routes!(get_action_logs))
            .routes(routes!(get_action_logs_id))
            .routes(routes!(get_apps))
            .routes(routes!(get_apps_app_id))
            .routes(routes!(get_credentials))
            .routes(routes!(post_credentials))
            .routes(routes!(put_credentials_activate))
            .routes(routes!(post_credentials_code))
            .routes(routes!(post_credentials_code_init))
            .routes(routes!(post_credentials_code_verify))
            .routes(routes!(put_credentials_deactivate))
            .routes(routes!(post_credentials_init))
            .routes(routes!(post_login))
            .routes(routes!(post_login_code))
            .routes(routes!(post_login_delegated))
            .routes(routes!(post_login_init))
            .routes(routes!(post_login_social))
            .routes(routes!(post_login_sso))
            .routes(routes!(post_login_sso_init))
            .routes(routes!(put_logout))
            .routes(routes!(get_pats))
            .routes(routes!(post_pats))
            .routes(routes!(get_pats_token_id))
            .routes(routes!(put_pats_token_id))
            .routes(routes!(delete_pats_token_id))
            .routes(routes!(put_pats_token_id_activate))
            .routes(routes!(put_pats_token_id_deactivate))
            .routes(routes!(post_recover_user))
            .routes(routes!(post_recover_user_code))
            .routes(routes!(post_recover_user_delegated))
            .routes(routes!(post_recover_user_init))
            .routes(routes!(post_registration))
            .routes(routes!(put_registration_code))
            .routes(routes!(post_registration_delegated))
            .routes(routes!(post_registration_enduser))
            .routes(routes!(post_registration_init))
            .routes(routes!(post_registration_social))
            .routes(routes!(get_service_accounts))
            .routes(routes!(post_service_accounts))
            .routes(routes!(get_service_accounts_service_account_id))
            .routes(routes!(put_service_accounts_service_account_id))
            .routes(routes!(delete_service_accounts_service_account_id))
            .routes(routes!(put_service_accounts_service_account_id_activate))
            .routes(routes!(put_service_accounts_service_account_id_deactivate))
            .routes(routes!(get_users))
            .routes(routes!(post_users))
            .routes(routes!(get_users_user_id))
            .routes(routes!(put_users_user_id))
            .routes(routes!(delete_users_user_id))
            .routes(routes!(put_users_user_id_activate))
            .routes(routes!(put_users_user_id_deactivate))
            .routes(routes!(get_))
            .routes(routes!(post_))
            .routes(routes!(get_exchange_id))
            .routes(routes!(delete_exchange_id))
            .routes(routes!(get_exchange_id_accounts))
            .routes(routes!(get_exchange_id_accounts_account_id_assets))
            .routes(routes!(get_exchange_id_accounts_account_id_assets_asset_withdrawal_networks))
            .routes(routes!(post_exchange_id_accounts_account_id_deposits))
            .routes(routes!(post_exchange_id_accounts_account_id_withdrawals))
            .routes(routes!(get_))
            .routes(routes!(post_))
            .routes(routes!(get_fee_sponsor_id))
            .routes(routes!(delete_fee_sponsor_id))
            .routes(routes!(put_fee_sponsor_id_activate))
            .routes(routes!(put_fee_sponsor_id_deactivate))
            .routes(routes!(get_fee_sponsor_id_fees))
            .routes(routes!(get_))
            .routes(routes!(get_))
            .routes(routes!(post_))
            .routes(routes!(post_import))
            .routes(routes!(get_key_id))
            .routes(routes!(put_key_id))
            .routes(routes!(delete_key_id))
            .routes(routes!(post_key_id_delegate))
            .routes(routes!(post_key_id_derive))
            .routes(routes!(post_key_id_export))
            .routes(routes!(get_key_id_signatures))
            .routes(routes!(post_key_id_signatures))
            .routes(routes!(get_key_id_signatures_signature_id))
            .routes(routes!(get_fees))
            .routes(routes!(post_read_contract))
            .routes(routes!(get_network_validators))
            .routes(routes!(post_network_validators))
            .routes(routes!(get_network_validators_validator_id))
            .routes(routes!(put_network_validators_validator_id))
            .routes(routes!(delete_network_validators_validator_id))
            .routes(routes!(get_))
            .routes(routes!(post_))
            .routes(routes!(get_permission_id))
            .routes(routes!(put_permission_id))
            .routes(routes!(put_permission_id_archive))
            .routes(routes!(get_permission_id_assignments))
            .routes(routes!(post_permission_id_assignments))
            .routes(routes!(delete_permission_id_assignments_assignment_id))
            .routes(routes!(get_))
            .routes(routes!(get_stakes))
            .routes(routes!(post_stakes))
            .routes(routes!(get_stakes_stake_id))
            .routes(routes!(get_stakes_stake_id_actions))
            .routes(routes!(post_stakes_stake_id_actions))
            .routes(routes!(get_stakes_stake_id_rewards))
            .routes(routes!(get_))
            .routes(routes!(post_))
            .routes(routes!(post_quotes))
            .routes(routes!(get_quotes_quote_id))
            .routes(routes!(get_swap_id))
            .routes(routes!(get_policies))
            .routes(routes!(post_policies))
            .routes(routes!(get_policies_policy_id))
            .routes(routes!(put_policies_policy_id))
            .routes(routes!(delete_policies_policy_id))
            .routes(routes!(get_policy_approvals))
            .routes(routes!(get_policy_approvals_approval_id))
            .routes(routes!(post_policy_approvals_approval_id_decisions))
            .routes(routes!(get_))
            .routes(routes!(post_))
            .routes(routes!(get_all_history))
            .routes(routes!(post_import))
            .routes(routes!(get_wallet_id))
            .routes(routes!(put_wallet_id))
            .routes(routes!(get_wallet_id_assets))
            .routes(routes!(post_wallet_id_delegate))
            .routes(routes!(post_wallet_id_export))
            .routes(routes!(get_wallet_id_history))
            .routes(routes!(get_wallet_id_nfts))
            .routes(routes!(get_wallet_id_offers))
            .routes(routes!(get_wallet_id_offers_offer_id))
            .routes(routes!(put_wallet_id_offers_offer_id_accept))
            .routes(routes!(put_wallet_id_offers_offer_id_reject))
            .routes(routes!(get_wallet_id_signatures))
            .routes(routes!(post_wallet_id_signatures))
            .routes(routes!(get_wallet_id_signatures_signature_id))
            .routes(routes!(put_wallet_id_tags))
            .routes(routes!(delete_wallet_id_tags))
            .routes(routes!(get_wallet_id_transactions))
            .routes(routes!(post_wallet_id_transactions))
            .routes(routes!(get_wallet_id_transactions_transaction_id))
            .routes(routes!(get_wallet_id_transfers))
            .routes(routes!(post_wallet_id_transfers))
            .routes(routes!(get_wallet_id_transfers_transfer_id))
            .routes(routes!(get_))
            .routes(routes!(post_))
            .routes(routes!(get_webhook_id))
            .routes(routes!(put_webhook_id))
            .routes(routes!(delete_webhook_id))
            .routes(routes!(get_webhook_id_events))
            .routes(routes!(get_webhook_id_events_webhook_event_id))
            .routes(routes!(post_webhook_id_ping))
            .routes(routes!(get_))
            .routes(routes!(post_))
            .routes(routes!(get_yield_id))
            .routes(routes!(get_yield_id_actions))
            .routes(routes!(post_yield_id_actions))
    }
}

#[utoipa::path(
    get,
    path = "/agreements/latest-unaccepted",
    responses(
        (status = 200, body = generated::agreements::LatestUnacceptedGETResponse200)
    )
)]
pub async fn get_latest_unaccepted(
) -> Json<generated::agreements::LatestUnacceptedGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::agreements::LatestUnacceptedGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/agreements/{agreementId}/accept",
    params(
        ("agreementId" = String, Path, description = "agreementId")),
    ),
    responses(
        (status = 200, body = generated::agreements::AgreementIdAcceptPOSTResponse200)
    )
)]
pub async fn post_agreement_id_accept(
    AxumPath(agreementId): AxumPath<String>,
) -> Json<generated::agreements::AgreementIdAcceptPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::agreements::AgreementIdAcceptPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/action",
    request_body = generated::auth::ActionPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ActionPOSTResponse200)
    )
)]
pub async fn post_action(
    Json(request): Json<generated::auth::ActionPOSTRequest>,
) -> Json<generated::auth::ActionPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ActionPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/action/init",
    request_body = generated::auth::ActionInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ActionInitPOSTResponse200)
    )
)]
pub async fn post_action_init(
    Json(request): Json<generated::auth::ActionInitPOSTRequest>,
) -> Json<generated::auth::ActionInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ActionInitPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/action/logs",
    responses(
        (status = 200, body = generated::auth::ActionLogsGETResponse200)
    )
)]
pub async fn get_action_logs(
) -> Json<generated::auth::ActionLogsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ActionLogsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/action/logs/{id}",
    params(
        ("id" = String, Path, description = "id")),
    ),
    responses(
        (status = 200, body = generated::auth::ActionLogsIdGETResponse200)
    )
)]
pub async fn get_action_logs_id(
    AxumPath(id): AxumPath<String>,
) -> Json<generated::auth::ActionLogsIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ActionLogsIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/apps",
    responses(
        (status = 200, body = generated::auth::AppsGETResponse200)
    )
)]
pub async fn get_apps(
) -> Json<generated::auth::AppsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::AppsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/apps/{appId}",
    params(
        ("appId" = String, Path, description = "appId")),
    ),
    responses(
        (status = 200, body = generated::auth::AppsAppIdGETResponse200)
    )
)]
pub async fn get_apps_app_id(
    AxumPath(appId): AxumPath<String>,
) -> Json<generated::auth::AppsAppIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::AppsAppIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/credentials",
    responses(
        (status = 200, body = generated::auth::CredentialsGETResponse200)
    )
)]
pub async fn get_credentials(
) -> Json<generated::auth::CredentialsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::CredentialsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/credentials",
    request_body = generated::auth::CredentialsPOSTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsPOSTResponse200)
    )
)]
pub async fn post_credentials(
    Json(request): Json<generated::auth::CredentialsPOSTRequest>,
) -> Json<generated::auth::CredentialsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::CredentialsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/credentials/activate",
    request_body = generated::auth::CredentialsActivatePUTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsActivatePUTResponse200)
    )
)]
pub async fn put_credentials_activate(
    Json(request): Json<generated::auth::CredentialsActivatePUTRequest>,
) -> Json<generated::auth::CredentialsActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::CredentialsActivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/credentials/code",
    request_body = generated::auth::CredentialsCodePOSTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsCodePOSTResponse200)
    )
)]
pub async fn post_credentials_code(
    Json(request): Json<generated::auth::CredentialsCodePOSTRequest>,
) -> Json<generated::auth::CredentialsCodePOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::CredentialsCodePOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/credentials/code/init",
    request_body = generated::auth::CredentialsCodeInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsCodeInitPOSTResponse200)
    )
)]
pub async fn post_credentials_code_init(
    Json(request): Json<generated::auth::CredentialsCodeInitPOSTRequest>,
) -> Json<generated::auth::CredentialsCodeInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::CredentialsCodeInitPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/credentials/code/verify",
    request_body = generated::auth::CredentialsCodeVerifyPOSTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsCodeVerifyPOSTResponse200)
    )
)]
pub async fn post_credentials_code_verify(
    Json(request): Json<generated::auth::CredentialsCodeVerifyPOSTRequest>,
) -> Json<generated::auth::CredentialsCodeVerifyPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::CredentialsCodeVerifyPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/credentials/deactivate",
    request_body = generated::auth::CredentialsDeactivatePUTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsDeactivatePUTResponse200)
    )
)]
pub async fn put_credentials_deactivate(
    Json(request): Json<generated::auth::CredentialsDeactivatePUTRequest>,
) -> Json<generated::auth::CredentialsDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::CredentialsDeactivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/credentials/init",
    request_body = generated::auth::CredentialsInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsInitPOSTResponse200)
    )
)]
pub async fn post_credentials_init(
    Json(request): Json<generated::auth::CredentialsInitPOSTRequest>,
) -> Json<generated::auth::CredentialsInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::CredentialsInitPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = generated::auth::LoginPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginPOSTResponse200)
    )
)]
pub async fn post_login(
    Json(request): Json<generated::auth::LoginPOSTRequest>,
) -> Json<generated::auth::LoginPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::LoginPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/login/code",
    request_body = generated::auth::LoginCodePOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginCodePOSTResponse200)
    )
)]
pub async fn post_login_code(
    Json(request): Json<generated::auth::LoginCodePOSTRequest>,
) -> Json<generated::auth::LoginCodePOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::LoginCodePOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/login/delegated",
    request_body = generated::auth::LoginDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginDelegatedPOSTResponse200)
    )
)]
pub async fn post_login_delegated(
    Json(request): Json<generated::auth::LoginDelegatedPOSTRequest>,
) -> Json<generated::auth::LoginDelegatedPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::LoginDelegatedPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/login/init",
    request_body = generated::auth::LoginInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginInitPOSTResponse200)
    )
)]
pub async fn post_login_init(
    Json(request): Json<generated::auth::LoginInitPOSTRequest>,
) -> Json<generated::auth::LoginInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::LoginInitPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/login/social",
    request_body = generated::auth::LoginSocialPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSocialPOSTResponse200)
    )
)]
pub async fn post_login_social(
    Json(request): Json<generated::auth::LoginSocialPOSTRequest>,
) -> Json<generated::auth::LoginSocialPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::LoginSocialPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/login/sso",
    request_body = generated::auth::LoginSsoPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSsoPOSTResponse200)
    )
)]
pub async fn post_login_sso(
    Json(request): Json<generated::auth::LoginSsoPOSTRequest>,
) -> Json<generated::auth::LoginSsoPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::LoginSsoPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/login/sso/init",
    request_body = generated::auth::LoginSsoInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSsoInitPOSTResponse200)
    )
)]
pub async fn post_login_sso_init(
    Json(request): Json<generated::auth::LoginSsoInitPOSTRequest>,
) -> Json<generated::auth::LoginSsoInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::LoginSsoInitPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/logout",
    request_body = generated::auth::LogoutPUTRequest,
    responses(
        (status = 200, body = generated::auth::LogoutPUTResponse200)
    )
)]
pub async fn put_logout(
    Json(request): Json<generated::auth::LogoutPUTRequest>,
) -> Json<generated::auth::LogoutPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::LogoutPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/pats",
    responses(
        (status = 200, body = generated::auth::PatsGETResponse200)
    )
)]
pub async fn get_pats(
) -> Json<generated::auth::PatsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::PatsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/pats",
    request_body = generated::auth::PatsPOSTRequest,
    responses(
        (status = 200, body = generated::auth::PatsPOSTResponse200)
    )
)]
pub async fn post_pats(
    Json(request): Json<generated::auth::PatsPOSTRequest>,
) -> Json<generated::auth::PatsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::PatsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")),
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdGETResponse200)
    )
)]
pub async fn get_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::PatsTokenIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")),
    ),
    request_body = generated::auth::PatsTokenIdPUTRequest,
    responses(
        (status = 200, body = generated::auth::PatsTokenIdPUTResponse200)
    )
)]
pub async fn put_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
    Json(request): Json<generated::auth::PatsTokenIdPUTRequest>,
) -> Json<generated::auth::PatsTokenIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::PatsTokenIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")),
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdDELETEResponse200)
    )
)]
pub async fn delete_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::PatsTokenIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}/activate",
    params(
        ("tokenId" = String, Path, description = "tokenId")),
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdActivatePUTResponse200)
    )
)]
pub async fn put_pats_token_id_activate(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::PatsTokenIdActivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}/deactivate",
    params(
        ("tokenId" = String, Path, description = "tokenId")),
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdDeactivatePUTResponse200)
    )
)]
pub async fn put_pats_token_id_deactivate(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::PatsTokenIdDeactivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/recover/user",
    request_body = generated::auth::RecoverUserPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserPOSTResponse200)
    )
)]
pub async fn post_recover_user(
    Json(request): Json<generated::auth::RecoverUserPOSTRequest>,
) -> Json<generated::auth::RecoverUserPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RecoverUserPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/code",
    request_body = generated::auth::RecoverUserCodePOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserCodePOSTResponse200)
    )
)]
pub async fn post_recover_user_code(
    Json(request): Json<generated::auth::RecoverUserCodePOSTRequest>,
) -> Json<generated::auth::RecoverUserCodePOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RecoverUserCodePOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/delegated",
    request_body = generated::auth::RecoverUserDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserDelegatedPOSTResponse200)
    )
)]
pub async fn post_recover_user_delegated(
    Json(request): Json<generated::auth::RecoverUserDelegatedPOSTRequest>,
) -> Json<generated::auth::RecoverUserDelegatedPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RecoverUserDelegatedPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/init",
    request_body = generated::auth::RecoverUserInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserInitPOSTResponse200)
    )
)]
pub async fn post_recover_user_init(
    Json(request): Json<generated::auth::RecoverUserInitPOSTRequest>,
) -> Json<generated::auth::RecoverUserInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RecoverUserInitPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/registration",
    request_body = generated::auth::RegistrationPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationPOSTResponse200)
    )
)]
pub async fn post_registration(
    Json(request): Json<generated::auth::RegistrationPOSTRequest>,
) -> Json<generated::auth::RegistrationPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RegistrationPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/registration/code",
    request_body = generated::auth::RegistrationCodePUTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationCodePUTResponse200)
    )
)]
pub async fn put_registration_code(
    Json(request): Json<generated::auth::RegistrationCodePUTRequest>,
) -> Json<generated::auth::RegistrationCodePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RegistrationCodePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/registration/delegated",
    request_body = generated::auth::RegistrationDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationDelegatedPOSTResponse200)
    )
)]
pub async fn post_registration_delegated(
    Json(request): Json<generated::auth::RegistrationDelegatedPOSTRequest>,
) -> Json<generated::auth::RegistrationDelegatedPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RegistrationDelegatedPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/registration/enduser",
    request_body = generated::auth::RegistrationEnduserPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationEnduserPOSTResponse200)
    )
)]
pub async fn post_registration_enduser(
    Json(request): Json<generated::auth::RegistrationEnduserPOSTRequest>,
) -> Json<generated::auth::RegistrationEnduserPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RegistrationEnduserPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/registration/init",
    request_body = generated::auth::RegistrationInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationInitPOSTResponse200)
    )
)]
pub async fn post_registration_init(
    Json(request): Json<generated::auth::RegistrationInitPOSTRequest>,
) -> Json<generated::auth::RegistrationInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RegistrationInitPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/registration/social",
    request_body = generated::auth::RegistrationSocialPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationSocialPOSTResponse200)
    )
)]
pub async fn post_registration_social(
    Json(request): Json<generated::auth::RegistrationSocialPOSTRequest>,
) -> Json<generated::auth::RegistrationSocialPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::RegistrationSocialPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/service-accounts",
    responses(
        (status = 200, body = generated::auth::ServiceAccountsGETResponse200)
    )
)]
pub async fn get_service_accounts(
) -> Json<generated::auth::ServiceAccountsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ServiceAccountsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/service-accounts",
    request_body = generated::auth::ServiceAccountsPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ServiceAccountsPOSTResponse200)
    )
)]
pub async fn post_service_accounts(
    Json(request): Json<generated::auth::ServiceAccountsPOSTRequest>,
) -> Json<generated::auth::ServiceAccountsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ServiceAccountsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")),
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdGETResponse200)
    )
)]
pub async fn get_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ServiceAccountsServiceAccountIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")),
    ),
    request_body = generated::auth::ServiceAccountsServiceAccountIdPUTRequest,
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdPUTResponse200)
    )
)]
pub async fn put_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
    Json(request): Json<generated::auth::ServiceAccountsServiceAccountIdPUTRequest>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ServiceAccountsServiceAccountIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")),
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdDELETEResponse200)
    )
)]
pub async fn delete_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ServiceAccountsServiceAccountIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}/activate",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")),
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse200)
    )
)]
pub async fn put_service_accounts_service_account_id_activate(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}/deactivate",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")),
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse200)
    )
)]
pub async fn put_service_accounts_service_account_id_deactivate(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/users",
    responses(
        (status = 200, body = generated::auth::UsersGETResponse200)
    )
)]
pub async fn get_users(
) -> Json<generated::auth::UsersGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::UsersGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/auth/users",
    request_body = generated::auth::UsersPOSTRequest,
    responses(
        (status = 200, body = generated::auth::UsersPOSTResponse200)
    )
)]
pub async fn post_users(
    Json(request): Json<generated::auth::UsersPOSTRequest>,
) -> Json<generated::auth::UsersPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::UsersPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/auth/users/{userId}",
    params(
        ("userId" = String, Path, description = "userId")),
    ),
    responses(
        (status = 200, body = generated::auth::UsersUserIdGETResponse200)
    )
)]
pub async fn get_users_user_id(
    AxumPath(userId): AxumPath<String>,
) -> Json<generated::auth::UsersUserIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::UsersUserIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/users/{userId}",
    params(
        ("userId" = String, Path, description = "userId")),
    ),
    request_body = generated::auth::UsersUserIdPUTRequest,
    responses(
        (status = 200, body = generated::auth::UsersUserIdPUTResponse200)
    )
)]
pub async fn put_users_user_id(
    AxumPath(userId): AxumPath<String>,
    Json(request): Json<generated::auth::UsersUserIdPUTRequest>,
) -> Json<generated::auth::UsersUserIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::UsersUserIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/auth/users/{userId}",
    params(
        ("userId" = String, Path, description = "userId")),
    ),
    responses(
        (status = 200, body = generated::auth::UsersUserIdDELETEResponse200)
    )
)]
pub async fn delete_users_user_id(
    AxumPath(userId): AxumPath<String>,
) -> Json<generated::auth::UsersUserIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::UsersUserIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/users/{userId}/activate",
    params(
        ("userId" = String, Path, description = "userId")),
    ),
    responses(
        (status = 200, body = generated::auth::UsersUserIdActivatePUTResponse200)
    )
)]
pub async fn put_users_user_id_activate(
    AxumPath(userId): AxumPath<String>,
) -> Json<generated::auth::UsersUserIdActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::UsersUserIdActivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/auth/users/{userId}/deactivate",
    params(
        ("userId" = String, Path, description = "userId")),
    ),
    responses(
        (status = 200, body = generated::auth::UsersUserIdDeactivatePUTResponse200)
    )
)]
pub async fn put_users_user_id_deactivate(
    AxumPath(userId): AxumPath<String>,
) -> Json<generated::auth::UsersUserIdDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::auth::UsersUserIdDeactivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/exchanges",
    responses(
        (status = 200, body = generated::exchanges::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::exchanges::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/exchanges",
    request_body = generated::exchanges::POSTRequest,
    responses(
        (status = 200, body = generated::exchanges::POSTResponse200)
    )
)]
pub async fn post_(
    Json(request): Json<generated::exchanges::POSTRequest>,
) -> Json<generated::exchanges::POSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::POSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")),
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdGETResponse200)
    )
)]
pub async fn get_exchange_id(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::ExchangeIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/exchanges/{exchangeId}",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")),
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdDELETEResponse200)
    )
)]
pub async fn delete_exchange_id(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::ExchangeIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}/accounts",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")),
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsGETResponse200)
    )
)]
pub async fn get_exchange_id_accounts(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdAccountsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::ExchangeIdAccountsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/assets",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")),
        ("accountId" = String, Path, description = "accountId")),
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse200)
    )
)]
pub async fn get_exchange_id_accounts_account_id_assets(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/assets/{asset}/withdrawal-networks",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")),
        ("accountId" = String, Path, description = "accountId")),
        ("asset" = String, Path, description = "asset")),
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdAssetsAssetWithdrawalNetworksGETResponse200)
    )
)]
pub async fn get_exchange_id_accounts_account_id_assets_asset_withdrawal_networks(
    AxumPath((exchangeId, accountId, asset)): AxumPath<(String, String, String)>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdAssetsAssetWithdrawalNetworksGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::ExchangeIdAccountsAccountIdAssetsAssetWithdrawalNetworksGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/deposits",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")),
        ("accountId" = String, Path, description = "accountId")),
    ),
    request_body = generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTRequest,
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse200)
    )
)]
pub async fn post_exchange_id_accounts_account_id_deposits(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
    Json(request): Json<generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTRequest>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/withdrawals",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")),
        ("accountId" = String, Path, description = "accountId")),
    ),
    request_body = generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTRequest,
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200)
    )
)]
pub async fn post_exchange_id_accounts_account_id_withdrawals(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
    Json(request): Json<generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTRequest>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/fee-sponsors",
    responses(
        (status = 200, body = generated::feesponsors::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::feesponsors::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::feesponsors::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/fee-sponsors",
    request_body = generated::feesponsors::POSTRequest,
    responses(
        (status = 200, body = generated::feesponsors::POSTResponse200)
    )
)]
pub async fn post_(
    Json(request): Json<generated::feesponsors::POSTRequest>,
) -> Json<generated::feesponsors::POSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::feesponsors::POSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/fee-sponsors/{feeSponsorId}",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")),
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdGETResponse200)
    )
)]
pub async fn get_fee_sponsor_id(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::feesponsors::FeeSponsorIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/fee-sponsors/{feeSponsorId}",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")),
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdDELETEResponse200)
    )
)]
pub async fn delete_fee_sponsor_id(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::feesponsors::FeeSponsorIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/fee-sponsors/{feeSponsorId}/activate",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")),
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdActivatePUTResponse200)
    )
)]
pub async fn put_fee_sponsor_id_activate(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::feesponsors::FeeSponsorIdActivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/fee-sponsors/{feeSponsorId}/deactivate",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")),
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdDeactivatePUTResponse200)
    )
)]
pub async fn put_fee_sponsor_id_deactivate(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::feesponsors::FeeSponsorIdDeactivatePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/fee-sponsors/{feeSponsorId}/fees",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")),
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdFeesGETResponse200)
    )
)]
pub async fn get_fee_sponsor_id_fees(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdFeesGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::feesponsors::FeeSponsorIdFeesGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/key-stores",
    responses(
        (status = 200, body = generated::keystores::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::keystores::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keystores::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/keys",
    responses(
        (status = 200, body = generated::keys::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::keys::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/keys",
    request_body = generated::keys::POSTRequest,
    responses(
        (status = 200, body = generated::keys::POSTResponse200)
    )
)]
pub async fn post_(
    Json(request): Json<generated::keys::POSTRequest>,
) -> Json<generated::keys::POSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::POSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/keys/import",
    request_body = generated::keys::ImportPOSTRequest,
    responses(
        (status = 200, body = generated::keys::ImportPOSTResponse200)
    )
)]
pub async fn post_import(
    Json(request): Json<generated::keys::ImportPOSTRequest>,
) -> Json<generated::keys::ImportPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::ImportPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")),
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdGETResponse200)
    )
)]
pub async fn get_key_id(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")),
    ),
    request_body = generated::keys::KeyIdPUTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdPUTResponse200)
    )
)]
pub async fn put_key_id(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdPUTRequest>,
) -> Json<generated::keys::KeyIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")),
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdDELETEResponse200)
    )
)]
pub async fn delete_key_id(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/delegate",
    params(
        ("keyId" = String, Path, description = "keyId")),
    ),
    request_body = generated::keys::KeyIdDelegatePOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdDelegatePOSTResponse200)
    )
)]
pub async fn post_key_id_delegate(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdDelegatePOSTRequest>,
) -> Json<generated::keys::KeyIdDelegatePOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdDelegatePOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/derive",
    params(
        ("keyId" = String, Path, description = "keyId")),
    ),
    request_body = generated::keys::KeyIdDerivePOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdDerivePOSTResponse200)
    )
)]
pub async fn post_key_id_derive(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdDerivePOSTRequest>,
) -> Json<generated::keys::KeyIdDerivePOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdDerivePOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/export",
    params(
        ("keyId" = String, Path, description = "keyId")),
    ),
    request_body = generated::keys::KeyIdExportPOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdExportPOSTResponse200)
    )
)]
pub async fn post_key_id_export(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdExportPOSTRequest>,
) -> Json<generated::keys::KeyIdExportPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdExportPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}/signatures",
    params(
        ("keyId" = String, Path, description = "keyId")),
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesGETResponse200)
    )
)]
pub async fn get_key_id_signatures(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdSignaturesGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdSignaturesGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/signatures",
    params(
        ("keyId" = String, Path, description = "keyId")),
    ),
    request_body = generated::keys::KeyIdSignaturesPOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesPOSTResponse200)
    )
)]
pub async fn post_key_id_signatures(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdSignaturesPOSTRequest>,
) -> Json<generated::keys::KeyIdSignaturesPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdSignaturesPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}/signatures/{signatureId}",
    params(
        ("keyId" = String, Path, description = "keyId")),
        ("signatureId" = String, Path, description = "signatureId")),
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesSignatureIdGETResponse200)
    )
)]
pub async fn get_key_id_signatures_signature_id(
    AxumPath((keyId, signatureId)): AxumPath<(String, String)>,
) -> Json<generated::keys::KeyIdSignaturesSignatureIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::keys::KeyIdSignaturesSignatureIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/networks/fees",
    responses(
        (status = 200, body = generated::networks::FeesGETResponse200)
    )
)]
pub async fn get_fees(
) -> Json<generated::networks::FeesGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::networks::FeesGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/networks/read-contract",
    request_body = generated::networks::ReadContractPOSTRequest,
    responses(
        (status = 200, body = generated::networks::ReadContractPOSTResponse200)
    )
)]
pub async fn post_read_contract(
    Json(request): Json<generated::networks::ReadContractPOSTRequest>,
) -> Json<generated::networks::ReadContractPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::networks::ReadContractPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/networks/{network}/validators",
    params(
        ("network" = String, Path, description = "network")),
    ),
    responses(
        (status = 200, body = generated::networks::NetworkValidatorsGETResponse200)
    )
)]
pub async fn get_network_validators(
    AxumPath(network): AxumPath<String>,
) -> Json<generated::networks::NetworkValidatorsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::networks::NetworkValidatorsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/networks/{network}/validators",
    params(
        ("network" = String, Path, description = "network")),
    ),
    request_body = generated::networks::NetworkValidatorsPOSTRequest,
    responses(
        (status = 200, body = generated::networks::NetworkValidatorsPOSTResponse200)
    )
)]
pub async fn post_network_validators(
    AxumPath(network): AxumPath<String>,
    Json(request): Json<generated::networks::NetworkValidatorsPOSTRequest>,
) -> Json<generated::networks::NetworkValidatorsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::networks::NetworkValidatorsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/networks/{network}/validators/{validatorId}",
    params(
        ("network" = String, Path, description = "network")),
        ("validatorId" = String, Path, description = "validatorId")),
    ),
    responses(
        (status = 200, body = generated::networks::NetworkValidatorsValidatorIdGETResponse200)
    )
)]
pub async fn get_network_validators_validator_id(
    AxumPath((network, validatorId)): AxumPath<(String, String)>,
) -> Json<generated::networks::NetworkValidatorsValidatorIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::networks::NetworkValidatorsValidatorIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/networks/{network}/validators/{validatorId}",
    params(
        ("network" = String, Path, description = "network")),
        ("validatorId" = String, Path, description = "validatorId")),
    ),
    request_body = generated::networks::NetworkValidatorsValidatorIdPUTRequest,
    responses(
        (status = 200, body = generated::networks::NetworkValidatorsValidatorIdPUTResponse200)
    )
)]
pub async fn put_network_validators_validator_id(
    AxumPath((network, validatorId)): AxumPath<(String, String)>,
    Json(request): Json<generated::networks::NetworkValidatorsValidatorIdPUTRequest>,
) -> Json<generated::networks::NetworkValidatorsValidatorIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::networks::NetworkValidatorsValidatorIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/networks/{network}/validators/{validatorId}",
    params(
        ("network" = String, Path, description = "network")),
        ("validatorId" = String, Path, description = "validatorId")),
    ),
    responses(
        (status = 200, body = generated::networks::NetworkValidatorsValidatorIdDELETEResponse200)
    )
)]
pub async fn delete_network_validators_validator_id(
    AxumPath((network, validatorId)): AxumPath<(String, String)>,
) -> Json<generated::networks::NetworkValidatorsValidatorIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::networks::NetworkValidatorsValidatorIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/permissions",
    responses(
        (status = 200, body = generated::permissions::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::permissions::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::permissions::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/permissions",
    request_body = generated::permissions::POSTRequest,
    responses(
        (status = 200, body = generated::permissions::POSTResponse200)
    )
)]
pub async fn post_(
    Json(request): Json<generated::permissions::POSTRequest>,
) -> Json<generated::permissions::POSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::permissions::POSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/permissions/{permissionId}",
    params(
        ("permissionId" = String, Path, description = "permissionId")),
    ),
    responses(
        (status = 200, body = generated::permissions::PermissionIdGETResponse200)
    )
)]
pub async fn get_permission_id(
    AxumPath(permissionId): AxumPath<String>,
) -> Json<generated::permissions::PermissionIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::permissions::PermissionIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/permissions/{permissionId}",
    params(
        ("permissionId" = String, Path, description = "permissionId")),
    ),
    request_body = generated::permissions::PermissionIdPUTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionIdPUTResponse200)
    )
)]
pub async fn put_permission_id(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdPUTRequest>,
) -> Json<generated::permissions::PermissionIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::permissions::PermissionIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/permissions/{permissionId}/archive",
    params(
        ("permissionId" = String, Path, description = "permissionId")),
    ),
    request_body = generated::permissions::PermissionIdArchivePUTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionIdArchivePUTResponse200)
    )
)]
pub async fn put_permission_id_archive(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdArchivePUTRequest>,
) -> Json<generated::permissions::PermissionIdArchivePUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::permissions::PermissionIdArchivePUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/permissions/{permissionId}/assignments",
    params(
        ("permissionId" = String, Path, description = "permissionId")),
    ),
    responses(
        (status = 200, body = generated::permissions::PermissionIdAssignmentsGETResponse200)
    )
)]
pub async fn get_permission_id_assignments(
    AxumPath(permissionId): AxumPath<String>,
) -> Json<generated::permissions::PermissionIdAssignmentsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::permissions::PermissionIdAssignmentsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/permissions/{permissionId}/assignments",
    params(
        ("permissionId" = String, Path, description = "permissionId")),
    ),
    request_body = generated::permissions::PermissionIdAssignmentsPOSTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionIdAssignmentsPOSTResponse200)
    )
)]
pub async fn post_permission_id_assignments(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdAssignmentsPOSTRequest>,
) -> Json<generated::permissions::PermissionIdAssignmentsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::permissions::PermissionIdAssignmentsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/permissions/{permissionId}/assignments/{assignmentId}",
    params(
        ("permissionId" = String, Path, description = "permissionId")),
        ("assignmentId" = String, Path, description = "assignmentId")),
    ),
    responses(
        (status = 200)
    )
)]
pub async fn delete_permission_id_assignments_assignment_id(
    AxumPath((permissionId, assignmentId)): AxumPath<(String, String)>,
) -> StatusCode {
    // TODO: Replace with actual implementation
    StatusCode::OK
}

#[utoipa::path(
    get,
    path = "/signers",
    responses(
        (status = 200, body = generated::signers::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::signers::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::signers::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/staking/stakes",
    responses(
        (status = 200, body = generated::staking::StakesGETResponse200)
    )
)]
pub async fn get_stakes(
) -> Json<generated::staking::StakesGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::staking::StakesGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/staking/stakes",
    request_body = generated::staking::StakesPOSTRequest,
    responses(
        (status = 200, body = generated::staking::StakesPOSTResponse200)
    )
)]
pub async fn post_stakes(
    Json(request): Json<generated::staking::StakesPOSTRequest>,
) -> Json<generated::staking::StakesPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::staking::StakesPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/staking/stakes/{stakeId}",
    params(
        ("stakeId" = String, Path, description = "stakeId")),
    ),
    responses(
        (status = 200, body = generated::staking::StakesStakeIdGETResponse200)
    )
)]
pub async fn get_stakes_stake_id(
    AxumPath(stakeId): AxumPath<String>,
) -> Json<generated::staking::StakesStakeIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::staking::StakesStakeIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/staking/stakes/{stakeId}/actions",
    params(
        ("stakeId" = String, Path, description = "stakeId")),
    ),
    responses(
        (status = 200, body = generated::staking::StakesStakeIdActionsGETResponse200)
    )
)]
pub async fn get_stakes_stake_id_actions(
    AxumPath(stakeId): AxumPath<String>,
) -> Json<generated::staking::StakesStakeIdActionsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::staking::StakesStakeIdActionsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/staking/stakes/{stakeId}/actions",
    params(
        ("stakeId" = String, Path, description = "stakeId")),
    ),
    request_body = generated::staking::StakesStakeIdActionsPOSTRequest,
    responses(
        (status = 200, body = generated::staking::StakesStakeIdActionsPOSTResponse200)
    )
)]
pub async fn post_stakes_stake_id_actions(
    AxumPath(stakeId): AxumPath<String>,
    Json(request): Json<generated::staking::StakesStakeIdActionsPOSTRequest>,
) -> Json<generated::staking::StakesStakeIdActionsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::staking::StakesStakeIdActionsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/staking/stakes/{stakeId}/rewards",
    params(
        ("stakeId" = String, Path, description = "stakeId")),
    ),
    responses(
        (status = 200, body = generated::staking::StakesStakeIdRewardsGETResponse200)
    )
)]
pub async fn get_stakes_stake_id_rewards(
    AxumPath(stakeId): AxumPath<String>,
) -> Json<generated::staking::StakesStakeIdRewardsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::staking::StakesStakeIdRewardsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/swaps",
    responses(
        (status = 200, body = generated::swaps::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::swaps::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::swaps::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/swaps",
    request_body = generated::swaps::POSTRequest,
    responses(
        (status = 200, body = generated::swaps::POSTResponse200)
    )
)]
pub async fn post_(
    Json(request): Json<generated::swaps::POSTRequest>,
) -> Json<generated::swaps::POSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::swaps::POSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/swaps/quotes",
    request_body = generated::swaps::QuotesPOSTRequest,
    responses(
        (status = 200, body = generated::swaps::QuotesPOSTResponse200)
    )
)]
pub async fn post_quotes(
    Json(request): Json<generated::swaps::QuotesPOSTRequest>,
) -> Json<generated::swaps::QuotesPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::swaps::QuotesPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/swaps/quotes/{quoteId}",
    params(
        ("quoteId" = String, Path, description = "quoteId")),
    ),
    responses(
        (status = 200, body = generated::swaps::QuotesQuoteIdGETResponse200)
    )
)]
pub async fn get_quotes_quote_id(
    AxumPath(quoteId): AxumPath<String>,
) -> Json<generated::swaps::QuotesQuoteIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::swaps::QuotesQuoteIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/swaps/{swapId}",
    params(
        ("swapId" = String, Path, description = "swapId")),
    ),
    responses(
        (status = 200, body = generated::swaps::SwapIdGETResponse200)
    )
)]
pub async fn get_swap_id(
    AxumPath(swapId): AxumPath<String>,
) -> Json<generated::swaps::SwapIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::swaps::SwapIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/v2/policies",
    responses(
        (status = 200, body = generated::v2::PoliciesGETResponse200)
    )
)]
pub async fn get_policies(
) -> Json<generated::v2::PoliciesGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::v2::PoliciesGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/v2/policies",
    request_body = generated::v2::PoliciesPOSTRequest,
    responses(
        (status = 200, body = generated::v2::PoliciesPOSTResponse200)
    )
)]
pub async fn post_policies(
    Json(request): Json<generated::v2::PoliciesPOSTRequest>,
) -> Json<generated::v2::PoliciesPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::v2::PoliciesPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/v2/policies/{policyId}",
    params(
        ("policyId" = String, Path, description = "policyId")),
    ),
    responses(
        (status = 200, body = generated::v2::PoliciesPolicyIdGETResponse200)
    )
)]
pub async fn get_policies_policy_id(
    AxumPath(policyId): AxumPath<String>,
) -> Json<generated::v2::PoliciesPolicyIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::v2::PoliciesPolicyIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/v2/policies/{policyId}",
    params(
        ("policyId" = String, Path, description = "policyId")),
    ),
    request_body = generated::v2::PoliciesPolicyIdPUTRequest,
    responses(
        (status = 200, body = generated::v2::PoliciesPolicyIdPUTResponse200)
    )
)]
pub async fn put_policies_policy_id(
    AxumPath(policyId): AxumPath<String>,
    Json(request): Json<generated::v2::PoliciesPolicyIdPUTRequest>,
) -> Json<generated::v2::PoliciesPolicyIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::v2::PoliciesPolicyIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/v2/policies/{policyId}",
    params(
        ("policyId" = String, Path, description = "policyId")),
    ),
    responses(
        (status = 200, body = generated::v2::PoliciesPolicyIdDELETEResponse200)
    )
)]
pub async fn delete_policies_policy_id(
    AxumPath(policyId): AxumPath<String>,
) -> Json<generated::v2::PoliciesPolicyIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::v2::PoliciesPolicyIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/v2/policy-approvals",
    responses(
        (status = 200, body = generated::v2::PolicyApprovalsGETResponse200)
    )
)]
pub async fn get_policy_approvals(
) -> Json<generated::v2::PolicyApprovalsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::v2::PolicyApprovalsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/v2/policy-approvals/{approvalId}",
    params(
        ("approvalId" = String, Path, description = "approvalId")),
    ),
    responses(
        (status = 200, body = generated::v2::PolicyApprovalsApprovalIdGETResponse200)
    )
)]
pub async fn get_policy_approvals_approval_id(
    AxumPath(approvalId): AxumPath<String>,
) -> Json<generated::v2::PolicyApprovalsApprovalIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::v2::PolicyApprovalsApprovalIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/v2/policy-approvals/{approvalId}/decisions",
    params(
        ("approvalId" = String, Path, description = "approvalId")),
    ),
    request_body = generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTRequest,
    responses(
        (status = 200, body = generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse200)
    )
)]
pub async fn post_policy_approvals_approval_id_decisions(
    AxumPath(approvalId): AxumPath<String>,
    Json(request): Json<generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTRequest>,
) -> Json<generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets",
    responses(
        (status = 200, body = generated::wallets::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::wallets::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/wallets",
    request_body = generated::wallets::POSTRequest,
    responses(
        (status = 200, body = generated::wallets::POSTResponse200)
    )
)]
pub async fn post_(
    Json(request): Json<generated::wallets::POSTRequest>,
) -> Json<generated::wallets::POSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::POSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/all/history",
    responses(
        (status = 200, body = generated::wallets::AllHistoryGETResponse200)
    )
)]
pub async fn get_all_history(
) -> Json<generated::wallets::AllHistoryGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::AllHistoryGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/wallets/import",
    request_body = generated::wallets::ImportPOSTRequest,
    responses(
        (status = 200, body = generated::wallets::ImportPOSTResponse200)
    )
)]
pub async fn post_import(
    Json(request): Json<generated::wallets::ImportPOSTRequest>,
) -> Json<generated::wallets::ImportPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::ImportPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdGETResponse200)
    )
)]
pub async fn get_wallet_id(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/wallets/{walletId}",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    request_body = generated::wallets::WalletIdPUTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdPUTResponse200)
    )
)]
pub async fn put_wallet_id(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdPUTRequest>,
) -> Json<generated::wallets::WalletIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/assets",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdAssetsGETResponse200)
    )
)]
pub async fn get_wallet_id_assets(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdAssetsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdAssetsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/delegate",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    request_body = generated::wallets::WalletIdDelegatePOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdDelegatePOSTResponse200)
    )
)]
pub async fn post_wallet_id_delegate(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdDelegatePOSTRequest>,
) -> Json<generated::wallets::WalletIdDelegatePOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdDelegatePOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/export",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    request_body = generated::wallets::WalletIdExportPOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdExportPOSTResponse200)
    )
)]
pub async fn post_wallet_id_export(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdExportPOSTRequest>,
) -> Json<generated::wallets::WalletIdExportPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdExportPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/history",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdHistoryGETResponse200)
    )
)]
pub async fn get_wallet_id_history(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdHistoryGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdHistoryGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/nfts",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdNftsGETResponse200)
    )
)]
pub async fn get_wallet_id_nfts(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdNftsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdNftsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/offers",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdOffersGETResponse200)
    )
)]
pub async fn get_wallet_id_offers(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdOffersGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdOffersGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/offers/{offerId}",
    params(
        ("walletId" = String, Path, description = "walletId")),
        ("offerId" = String, Path, description = "offerId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdOffersOfferIdGETResponse200)
    )
)]
pub async fn get_wallet_id_offers_offer_id(
    AxumPath((walletId, offerId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdOffersOfferIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdOffersOfferIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/wallets/{walletId}/offers/{offerId}/accept",
    params(
        ("walletId" = String, Path, description = "walletId")),
        ("offerId" = String, Path, description = "offerId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdOffersOfferIdAcceptPUTResponse200)
    )
)]
pub async fn put_wallet_id_offers_offer_id_accept(
    AxumPath((walletId, offerId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdOffersOfferIdAcceptPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdOffersOfferIdAcceptPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/wallets/{walletId}/offers/{offerId}/reject",
    params(
        ("walletId" = String, Path, description = "walletId")),
        ("offerId" = String, Path, description = "offerId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdOffersOfferIdRejectPUTResponse200)
    )
)]
pub async fn put_wallet_id_offers_offer_id_reject(
    AxumPath((walletId, offerId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdOffersOfferIdRejectPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdOffersOfferIdRejectPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/signatures",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdSignaturesGETResponse200)
    )
)]
pub async fn get_wallet_id_signatures(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdSignaturesGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdSignaturesGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/signatures",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    request_body = generated::wallets::WalletIdSignaturesPOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdSignaturesPOSTResponse200)
    )
)]
pub async fn post_wallet_id_signatures(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdSignaturesPOSTRequest>,
) -> Json<generated::wallets::WalletIdSignaturesPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdSignaturesPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/signatures/{signatureId}",
    params(
        ("walletId" = String, Path, description = "walletId")),
        ("signatureId" = String, Path, description = "signatureId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdSignaturesSignatureIdGETResponse200)
    )
)]
pub async fn get_wallet_id_signatures_signature_id(
    AxumPath((walletId, signatureId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdSignaturesSignatureIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdSignaturesSignatureIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/wallets/{walletId}/tags",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    request_body = generated::wallets::WalletIdTagsPUTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdTagsPUTResponse200)
    )
)]
pub async fn put_wallet_id_tags(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdTagsPUTRequest>,
) -> Json<generated::wallets::WalletIdTagsPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdTagsPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/wallets/{walletId}/tags",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    request_body = generated::wallets::WalletIdTagsDELETERequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdTagsDELETEResponse200)
    )
)]
pub async fn delete_wallet_id_tags(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdTagsDELETERequest>,
) -> Json<generated::wallets::WalletIdTagsDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdTagsDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transactions",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransactionsGETResponse200)
    )
)]
pub async fn get_wallet_id_transactions(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdTransactionsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdTransactionsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/transactions",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    request_body = generated::wallets::WalletIdTransactionsPOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdTransactionsPOSTResponse200)
    )
)]
pub async fn post_wallet_id_transactions(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdTransactionsPOSTRequest>,
) -> Json<generated::wallets::WalletIdTransactionsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdTransactionsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transactions/{transactionId}",
    params(
        ("walletId" = String, Path, description = "walletId")),
        ("transactionId" = String, Path, description = "transactionId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransactionsTransactionIdGETResponse200)
    )
)]
pub async fn get_wallet_id_transactions_transaction_id(
    AxumPath((walletId, transactionId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdTransactionsTransactionIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdTransactionsTransactionIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transfers",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransfersGETResponse200)
    )
)]
pub async fn get_wallet_id_transfers(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdTransfersGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdTransfersGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/transfers",
    params(
        ("walletId" = String, Path, description = "walletId")),
    ),
    request_body = generated::wallets::WalletIdTransfersPOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdTransfersPOSTResponse200)
    )
)]
pub async fn post_wallet_id_transfers(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdTransfersPOSTRequest>,
) -> Json<generated::wallets::WalletIdTransfersPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdTransfersPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transfers/{transferId}",
    params(
        ("walletId" = String, Path, description = "walletId")),
        ("transferId" = String, Path, description = "transferId")),
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransfersTransferIdGETResponse200)
    )
)]
pub async fn get_wallet_id_transfers_transfer_id(
    AxumPath((walletId, transferId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdTransfersTransferIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::wallets::WalletIdTransfersTransferIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/webhooks",
    responses(
        (status = 200, body = generated::webhooks::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::webhooks::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::webhooks::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/webhooks",
    request_body = generated::webhooks::POSTRequest,
    responses(
        (status = 200, body = generated::webhooks::POSTResponse200)
    )
)]
pub async fn post_(
    Json(request): Json<generated::webhooks::POSTRequest>,
) -> Json<generated::webhooks::POSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::webhooks::POSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")),
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdGETResponse200)
    )
)]
pub async fn get_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::webhooks::WebhookIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    put,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")),
    ),
    request_body = generated::webhooks::WebhookIdPUTRequest,
    responses(
        (status = 200, body = generated::webhooks::WebhookIdPUTResponse200)
    )
)]
pub async fn put_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
    Json(request): Json<generated::webhooks::WebhookIdPUTRequest>,
) -> Json<generated::webhooks::WebhookIdPUTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::webhooks::WebhookIdPUTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")),
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdDELETEResponse200)
    )
)]
pub async fn delete_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::webhooks::WebhookIdDELETEResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}/events",
    params(
        ("webhookId" = String, Path, description = "webhookId")),
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdEventsGETResponse200)
    )
)]
pub async fn get_webhook_id_events(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdEventsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::webhooks::WebhookIdEventsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}/events/{webhookEventId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")),
        ("webhookEventId" = String, Path, description = "webhookEventId")),
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse200)
    )
)]
pub async fn get_webhook_id_events_webhook_event_id(
    AxumPath((webhookId, webhookEventId)): AxumPath<(String, String)>,
) -> Json<generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/webhooks/{webhookId}/ping",
    params(
        ("webhookId" = String, Path, description = "webhookId")),
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdPingPOSTResponse200)
    )
)]
pub async fn post_webhook_id_ping(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdPingPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::webhooks::WebhookIdPingPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/yields",
    responses(
        (status = 200, body = generated::yields::GETResponse200)
    )
)]
pub async fn get_(
) -> Json<generated::yields::GETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::yields::GETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/yields",
    request_body = generated::yields::POSTRequest,
    responses(
        (status = 200, body = generated::yields::POSTResponse200)
    )
)]
pub async fn post_(
    Json(request): Json<generated::yields::POSTRequest>,
) -> Json<generated::yields::POSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::yields::POSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/yields/{yieldId}",
    params(
        ("yieldId" = String, Path, description = "yieldId")),
    ),
    responses(
        (status = 200, body = generated::yields::YieldIdGETResponse200)
    )
)]
pub async fn get_yield_id(
    AxumPath(yieldId): AxumPath<String>,
) -> Json<generated::yields::YieldIdGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::yields::YieldIdGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    get,
    path = "/yields/{yieldId}/actions",
    params(
        ("yieldId" = String, Path, description = "yieldId")),
    ),
    responses(
        (status = 200, body = generated::yields::YieldIdActionsGETResponse200)
    )
)]
pub async fn get_yield_id_actions(
    AxumPath(yieldId): AxumPath<String>,
) -> Json<generated::yields::YieldIdActionsGETResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::yields::YieldIdActionsGETResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

#[utoipa::path(
    post,
    path = "/yields/{yieldId}/actions",
    params(
        ("yieldId" = String, Path, description = "yieldId")),
    ),
    request_body = generated::yields::YieldIdActionsPOSTRequest,
    responses(
        (status = 200, body = generated::yields::YieldIdActionsPOSTResponse200)
    )
)]
pub async fn post_yield_id_actions(
    AxumPath(yieldId): AxumPath<String>,
    Json(request): Json<generated::yields::YieldIdActionsPOSTRequest>,
) -> Json<generated::yields::YieldIdActionsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let response: generated::yields::YieldIdActionsPOSTResponse200 = serde_json::from_value(serde_json::json!({})).unwrap();
    Json(response)
}

