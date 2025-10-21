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
        get_agreements_latest_unaccepted,
        post_agreements_agreement_id_accept,
        post_auth_action,
        post_auth_action_init,
        get_auth_action_logs,
        get_auth_action_logs_id,
        get_auth_apps,
        get_auth_apps_app_id,
        get_auth_credentials,
        post_auth_credentials,
        put_auth_credentials_activate,
        post_auth_credentials_code,
        post_auth_credentials_code_init,
        post_auth_credentials_code_verify,
        put_auth_credentials_deactivate,
        post_auth_credentials_init,
        post_auth_login,
        post_auth_login_code,
        post_auth_login_delegated,
        post_auth_login_init,
        post_auth_login_social,
        post_auth_login_sso,
        post_auth_login_sso_init,
        put_auth_logout,
        get_auth_pats,
        post_auth_pats,
        get_auth_pats_token_id,
        put_auth_pats_token_id,
        delete_auth_pats_token_id,
        put_auth_pats_token_id_activate,
        put_auth_pats_token_id_deactivate,
        post_auth_recover_user,
        post_auth_recover_user_code,
        post_auth_recover_user_delegated,
        post_auth_recover_user_init,
        post_auth_registration,
        put_auth_registration_code,
        post_auth_registration_delegated,
        post_auth_registration_enduser,
        post_auth_registration_init,
        post_auth_registration_social,
        get_auth_service_accounts,
        post_auth_service_accounts,
        get_auth_service_accounts_service_account_id,
        put_auth_service_accounts_service_account_id,
        delete_auth_service_accounts_service_account_id,
        put_auth_service_accounts_service_account_id_activate,
        put_auth_service_accounts_service_account_id_deactivate,
        get_auth_users,
        post_auth_users,
        get_auth_users_user_id,
        put_auth_users_user_id,
        delete_auth_users_user_id,
        put_auth_users_user_id_activate,
        put_auth_users_user_id_deactivate,
        get_exchanges,
        post_exchanges,
        get_exchanges_exchange_id,
        delete_exchanges_exchange_id,
        get_exchanges_exchange_id_accounts,
        get_exchanges_exchange_id_accounts_account_id_assets,
        get_exchanges_exchange_id_accounts_account_id_assets_asset_withdrawal_networks,
        post_exchanges_exchange_id_accounts_account_id_deposits,
        post_exchanges_exchange_id_accounts_account_id_withdrawals,
        get_feesponsors,
        post_feesponsors,
        get_feesponsors_fee_sponsor_id,
        delete_feesponsors_fee_sponsor_id,
        put_feesponsors_fee_sponsor_id_activate,
        put_feesponsors_fee_sponsor_id_deactivate,
        get_feesponsors_fee_sponsor_id_fees,
        get_keystores,
        get_keys,
        post_keys,
        post_keys_import,
        get_keys_key_id,
        put_keys_key_id,
        delete_keys_key_id,
        post_keys_key_id_delegate,
        post_keys_key_id_derive,
        post_keys_key_id_export,
        get_keys_key_id_signatures,
        post_keys_key_id_signatures,
        get_keys_key_id_signatures_signature_id,
        get_networks_fees,
        post_networks_read_contract,
        get_networks_network_validators,
        post_networks_network_validators,
        get_networks_network_validators_validator_id,
        put_networks_network_validators_validator_id,
        delete_networks_network_validators_validator_id,
        get_permissions,
        post_permissions,
        get_permissions_permission_id,
        put_permissions_permission_id,
        put_permissions_permission_id_archive,
        get_permissions_permission_id_assignments,
        post_permissions_permission_id_assignments,
        delete_permissions_permission_id_assignments_assignment_id,
        get_signers,
        get_staking_stakes,
        post_staking_stakes,
        get_staking_stakes_stake_id,
        get_staking_stakes_stake_id_actions,
        post_staking_stakes_stake_id_actions,
        get_staking_stakes_stake_id_rewards,
        get_swaps,
        post_swaps,
        post_swaps_quotes,
        get_swaps_quotes_quote_id,
        get_swaps_swap_id,
        get_v2_policies,
        post_v2_policies,
        get_v2_policies_policy_id,
        put_v2_policies_policy_id,
        delete_v2_policies_policy_id,
        get_v2_policy_approvals,
        get_v2_policy_approvals_approval_id,
        post_v2_policy_approvals_approval_id_decisions,
        get_wallets,
        post_wallets,
        get_wallets_all_history,
        post_wallets_import,
        get_wallets_wallet_id,
        put_wallets_wallet_id,
        get_wallets_wallet_id_assets,
        post_wallets_wallet_id_delegate,
        post_wallets_wallet_id_export,
        get_wallets_wallet_id_history,
        get_wallets_wallet_id_nfts,
        get_wallets_wallet_id_offers,
        get_wallets_wallet_id_offers_offer_id,
        put_wallets_wallet_id_offers_offer_id_accept,
        put_wallets_wallet_id_offers_offer_id_reject,
        get_wallets_wallet_id_signatures,
        post_wallets_wallet_id_signatures,
        get_wallets_wallet_id_signatures_signature_id,
        put_wallets_wallet_id_tags,
        delete_wallets_wallet_id_tags,
        get_wallets_wallet_id_transactions,
        post_wallets_wallet_id_transactions,
        get_wallets_wallet_id_transactions_transaction_id,
        get_wallets_wallet_id_transfers,
        post_wallets_wallet_id_transfers,
        get_wallets_wallet_id_transfers_transfer_id,
        get_webhooks,
        post_webhooks,
        get_webhooks_webhook_id,
        put_webhooks_webhook_id,
        delete_webhooks_webhook_id,
        get_webhooks_webhook_id_events,
        get_webhooks_webhook_id_events_webhook_event_id,
        post_webhooks_webhook_id_ping,
        get_yields,
        post_yields,
        get_yields_yield_id,
        get_yields_yield_id_actions,
        post_yields_yield_id_actions
    )
)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn router() -> OpenApiRouter {
        OpenApiRouter::with_openapi(ApiDoc::openapi())
            .routes(routes!(get_agreements_latest_unaccepted))
            .routes(routes!(post_agreements_agreement_id_accept))
            .routes(routes!(post_auth_action))
            .routes(routes!(post_auth_action_init))
            .routes(routes!(get_auth_action_logs))
            .routes(routes!(get_auth_action_logs_id))
            .routes(routes!(get_auth_apps))
            .routes(routes!(get_auth_apps_app_id))
            .routes(routes!(get_auth_credentials))
            .routes(routes!(post_auth_credentials))
            .routes(routes!(put_auth_credentials_activate))
            .routes(routes!(post_auth_credentials_code))
            .routes(routes!(post_auth_credentials_code_init))
            .routes(routes!(post_auth_credentials_code_verify))
            .routes(routes!(put_auth_credentials_deactivate))
            .routes(routes!(post_auth_credentials_init))
            .routes(routes!(post_auth_login))
            .routes(routes!(post_auth_login_code))
            .routes(routes!(post_auth_login_delegated))
            .routes(routes!(post_auth_login_init))
            .routes(routes!(post_auth_login_social))
            .routes(routes!(post_auth_login_sso))
            .routes(routes!(post_auth_login_sso_init))
            .routes(routes!(put_auth_logout))
            .routes(routes!(get_auth_pats))
            .routes(routes!(post_auth_pats))
            .routes(routes!(get_auth_pats_token_id))
            .routes(routes!(put_auth_pats_token_id))
            .routes(routes!(delete_auth_pats_token_id))
            .routes(routes!(put_auth_pats_token_id_activate))
            .routes(routes!(put_auth_pats_token_id_deactivate))
            .routes(routes!(post_auth_recover_user))
            .routes(routes!(post_auth_recover_user_code))
            .routes(routes!(post_auth_recover_user_delegated))
            .routes(routes!(post_auth_recover_user_init))
            .routes(routes!(post_auth_registration))
            .routes(routes!(put_auth_registration_code))
            .routes(routes!(post_auth_registration_delegated))
            .routes(routes!(post_auth_registration_enduser))
            .routes(routes!(post_auth_registration_init))
            .routes(routes!(post_auth_registration_social))
            .routes(routes!(get_auth_service_accounts))
            .routes(routes!(post_auth_service_accounts))
            .routes(routes!(get_auth_service_accounts_service_account_id))
            .routes(routes!(put_auth_service_accounts_service_account_id))
            .routes(routes!(delete_auth_service_accounts_service_account_id))
            .routes(routes!(put_auth_service_accounts_service_account_id_activate))
            .routes(routes!(put_auth_service_accounts_service_account_id_deactivate))
            .routes(routes!(get_auth_users))
            .routes(routes!(post_auth_users))
            .routes(routes!(get_auth_users_user_id))
            .routes(routes!(put_auth_users_user_id))
            .routes(routes!(delete_auth_users_user_id))
            .routes(routes!(put_auth_users_user_id_activate))
            .routes(routes!(put_auth_users_user_id_deactivate))
            .routes(routes!(get_exchanges))
            .routes(routes!(post_exchanges))
            .routes(routes!(get_exchanges_exchange_id))
            .routes(routes!(delete_exchanges_exchange_id))
            .routes(routes!(get_exchanges_exchange_id_accounts))
            .routes(routes!(get_exchanges_exchange_id_accounts_account_id_assets))
            .routes(routes!(get_exchanges_exchange_id_accounts_account_id_assets_asset_withdrawal_networks))
            .routes(routes!(post_exchanges_exchange_id_accounts_account_id_deposits))
            .routes(routes!(post_exchanges_exchange_id_accounts_account_id_withdrawals))
            .routes(routes!(get_feesponsors))
            .routes(routes!(post_feesponsors))
            .routes(routes!(get_feesponsors_fee_sponsor_id))
            .routes(routes!(delete_feesponsors_fee_sponsor_id))
            .routes(routes!(put_feesponsors_fee_sponsor_id_activate))
            .routes(routes!(put_feesponsors_fee_sponsor_id_deactivate))
            .routes(routes!(get_feesponsors_fee_sponsor_id_fees))
            .routes(routes!(get_keystores))
            .routes(routes!(get_keys))
            .routes(routes!(post_keys))
            .routes(routes!(post_keys_import))
            .routes(routes!(get_keys_key_id))
            .routes(routes!(put_keys_key_id))
            .routes(routes!(delete_keys_key_id))
            .routes(routes!(post_keys_key_id_delegate))
            .routes(routes!(post_keys_key_id_derive))
            .routes(routes!(post_keys_key_id_export))
            .routes(routes!(get_keys_key_id_signatures))
            .routes(routes!(post_keys_key_id_signatures))
            .routes(routes!(get_keys_key_id_signatures_signature_id))
            .routes(routes!(get_networks_fees))
            .routes(routes!(post_networks_read_contract))
            .routes(routes!(get_networks_network_validators))
            .routes(routes!(post_networks_network_validators))
            .routes(routes!(get_networks_network_validators_validator_id))
            .routes(routes!(put_networks_network_validators_validator_id))
            .routes(routes!(delete_networks_network_validators_validator_id))
            .routes(routes!(get_permissions))
            .routes(routes!(post_permissions))
            .routes(routes!(get_permissions_permission_id))
            .routes(routes!(put_permissions_permission_id))
            .routes(routes!(put_permissions_permission_id_archive))
            .routes(routes!(get_permissions_permission_id_assignments))
            .routes(routes!(post_permissions_permission_id_assignments))
            .routes(routes!(delete_permissions_permission_id_assignments_assignment_id))
            .routes(routes!(get_signers))
            .routes(routes!(get_staking_stakes))
            .routes(routes!(post_staking_stakes))
            .routes(routes!(get_staking_stakes_stake_id))
            .routes(routes!(get_staking_stakes_stake_id_actions))
            .routes(routes!(post_staking_stakes_stake_id_actions))
            .routes(routes!(get_staking_stakes_stake_id_rewards))
            .routes(routes!(get_swaps))
            .routes(routes!(post_swaps))
            .routes(routes!(post_swaps_quotes))
            .routes(routes!(get_swaps_quotes_quote_id))
            .routes(routes!(get_swaps_swap_id))
            .routes(routes!(get_v2_policies))
            .routes(routes!(post_v2_policies))
            .routes(routes!(get_v2_policies_policy_id))
            .routes(routes!(put_v2_policies_policy_id))
            .routes(routes!(delete_v2_policies_policy_id))
            .routes(routes!(get_v2_policy_approvals))
            .routes(routes!(get_v2_policy_approvals_approval_id))
            .routes(routes!(post_v2_policy_approvals_approval_id_decisions))
            .routes(routes!(get_wallets))
            .routes(routes!(post_wallets))
            .routes(routes!(get_wallets_all_history))
            .routes(routes!(post_wallets_import))
            .routes(routes!(get_wallets_wallet_id))
            .routes(routes!(put_wallets_wallet_id))
            .routes(routes!(get_wallets_wallet_id_assets))
            .routes(routes!(post_wallets_wallet_id_delegate))
            .routes(routes!(post_wallets_wallet_id_export))
            .routes(routes!(get_wallets_wallet_id_history))
            .routes(routes!(get_wallets_wallet_id_nfts))
            .routes(routes!(get_wallets_wallet_id_offers))
            .routes(routes!(get_wallets_wallet_id_offers_offer_id))
            .routes(routes!(put_wallets_wallet_id_offers_offer_id_accept))
            .routes(routes!(put_wallets_wallet_id_offers_offer_id_reject))
            .routes(routes!(get_wallets_wallet_id_signatures))
            .routes(routes!(post_wallets_wallet_id_signatures))
            .routes(routes!(get_wallets_wallet_id_signatures_signature_id))
            .routes(routes!(put_wallets_wallet_id_tags))
            .routes(routes!(delete_wallets_wallet_id_tags))
            .routes(routes!(get_wallets_wallet_id_transactions))
            .routes(routes!(post_wallets_wallet_id_transactions))
            .routes(routes!(get_wallets_wallet_id_transactions_transaction_id))
            .routes(routes!(get_wallets_wallet_id_transfers))
            .routes(routes!(post_wallets_wallet_id_transfers))
            .routes(routes!(get_wallets_wallet_id_transfers_transfer_id))
            .routes(routes!(get_webhooks))
            .routes(routes!(post_webhooks))
            .routes(routes!(get_webhooks_webhook_id))
            .routes(routes!(put_webhooks_webhook_id))
            .routes(routes!(delete_webhooks_webhook_id))
            .routes(routes!(get_webhooks_webhook_id_events))
            .routes(routes!(get_webhooks_webhook_id_events_webhook_event_id))
            .routes(routes!(post_webhooks_webhook_id_ping))
            .routes(routes!(get_yields))
            .routes(routes!(post_yields))
            .routes(routes!(get_yields_yield_id))
            .routes(routes!(get_yields_yield_id_actions))
            .routes(routes!(post_yields_yield_id_actions))
    }
}

#[utoipa::path(
    get,
    path = "/agreements/latest-unaccepted",
    responses(
        (status = 200, body = generated::agreements::LatestUnacceptedGETResponse)
    )
)]
pub async fn get_agreements_latest_unaccepted(
) -> Json<generated::agreements::LatestUnacceptedGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::agreements::LatestUnacceptedGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/agreements/{agreementId}/accept",
    params(
        ("agreementId" = String, Path, description = "agreementId")
    ),
    responses(
        (status = 200, body = generated::agreements::AgreementIdAcceptPOSTResponse)
    )
)]
pub async fn post_agreements_agreement_id_accept(
    AxumPath(agreementId): AxumPath<String>,
) -> Json<generated::agreements::AgreementIdAcceptPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::agreements::AgreementIdAcceptPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/action",
    request_body = generated::auth::ActionPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ActionPOSTResponse)
    )
)]
pub async fn post_auth_action(
    Json(request): Json<generated::auth::ActionPOSTRequest>,
) -> Json<generated::auth::ActionPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ActionPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/action/init",
    request_body = generated::auth::ActionInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ActionInitPOSTResponse)
    )
)]
pub async fn post_auth_action_init(
    Json(request): Json<generated::auth::ActionInitPOSTRequest>,
) -> Json<generated::auth::ActionInitPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ActionInitPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/action/logs",
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_auth_action_logs(
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/auth/action/logs/{id}",
    params(
        ("id" = String, Path, description = "id")
    ),
    responses(
        (status = 200, body = generated::auth::ActionLogsIdGETResponse)
    )
)]
pub async fn get_auth_action_logs_id(
    AxumPath(id): AxumPath<String>,
) -> Json<generated::auth::ActionLogsIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ActionLogsIdGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/apps",
    responses(
        (status = 200, body = generated::auth::AppsGETResponse)
    )
)]
pub async fn get_auth_apps(
) -> Json<generated::auth::AppsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::AppsGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/apps/{appId}",
    params(
        ("appId" = String, Path, description = "appId")
    ),
    responses(
        (status = 200, body = generated::auth::AppsAppIdGETResponse)
    )
)]
pub async fn get_auth_apps_app_id(
    AxumPath(appId): AxumPath<String>,
) -> Json<generated::auth::AppsAppIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::AppsAppIdGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/credentials",
    responses(
        (status = 200, body = generated::auth::CredentialsGETResponse)
    )
)]
pub async fn get_auth_credentials(
) -> Json<generated::auth::CredentialsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/credentials",
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::auth::CredentialsPOSTResponse)
    )
)]
pub async fn post_auth_credentials(
    Json(request): Json<serde_json::Value>,
) -> Json<generated::auth::CredentialsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsPOSTResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/credentials/activate",
    request_body = generated::auth::CredentialsActivatePUTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsActivatePUTResponse)
    )
)]
pub async fn put_auth_credentials_activate(
    Json(request): Json<generated::auth::CredentialsActivatePUTRequest>,
) -> Json<generated::auth::CredentialsActivatePUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsActivatePUTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/credentials/code",
    request_body = generated::auth::CredentialsCodePOSTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsCodePOSTResponse)
    )
)]
pub async fn post_auth_credentials_code(
    Json(request): Json<generated::auth::CredentialsCodePOSTRequest>,
) -> Json<generated::auth::CredentialsCodePOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsCodePOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/credentials/code/init",
    request_body = generated::auth::CredentialsCodeInitPOSTRequest,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_auth_credentials_code_init(
    Json(request): Json<generated::auth::CredentialsCodeInitPOSTRequest>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    post,
    path = "/auth/credentials/code/verify",
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::auth::CredentialsCodeVerifyPOSTResponse)
    )
)]
pub async fn post_auth_credentials_code_verify(
    Json(request): Json<serde_json::Value>,
) -> Json<generated::auth::CredentialsCodeVerifyPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsCodeVerifyPOSTResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/credentials/deactivate",
    request_body = generated::auth::CredentialsDeactivatePUTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsDeactivatePUTResponse)
    )
)]
pub async fn put_auth_credentials_deactivate(
    Json(request): Json<generated::auth::CredentialsDeactivatePUTRequest>,
) -> Json<generated::auth::CredentialsDeactivatePUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsDeactivatePUTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/credentials/init",
    request_body = generated::auth::CredentialsInitPOSTRequest,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_auth_credentials_init(
    Json(request): Json<generated::auth::CredentialsInitPOSTRequest>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = generated::auth::LoginPOSTRequest,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_auth_login(
    Json(request): Json<generated::auth::LoginPOSTRequest>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    post,
    path = "/auth/login/code",
    request_body = generated::auth::LoginCodePOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginCodePOSTResponse)
    )
)]
pub async fn post_auth_login_code(
    Json(request): Json<generated::auth::LoginCodePOSTRequest>,
) -> Json<generated::auth::LoginCodePOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginCodePOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/delegated",
    request_body = generated::auth::LoginDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginDelegatedPOSTResponse)
    )
)]
pub async fn post_auth_login_delegated(
    Json(request): Json<generated::auth::LoginDelegatedPOSTRequest>,
) -> Json<generated::auth::LoginDelegatedPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginDelegatedPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/init",
    request_body = generated::auth::LoginInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginInitPOSTResponse)
    )
)]
pub async fn post_auth_login_init(
    Json(request): Json<generated::auth::LoginInitPOSTRequest>,
) -> Json<generated::auth::LoginInitPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginInitPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/social",
    request_body = generated::auth::LoginSocialPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSocialPOSTResponse)
    )
)]
pub async fn post_auth_login_social(
    Json(request): Json<generated::auth::LoginSocialPOSTRequest>,
) -> Json<generated::auth::LoginSocialPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginSocialPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/sso",
    request_body = generated::auth::LoginSsoPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSsoPOSTResponse)
    )
)]
pub async fn post_auth_login_sso(
    Json(request): Json<generated::auth::LoginSsoPOSTRequest>,
) -> Json<generated::auth::LoginSsoPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginSsoPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/sso/init",
    request_body = generated::auth::LoginSsoInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSsoInitPOSTResponse)
    )
)]
pub async fn post_auth_login_sso_init(
    Json(request): Json<generated::auth::LoginSsoInitPOSTRequest>,
) -> Json<generated::auth::LoginSsoInitPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginSsoInitPOSTResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/logout",
    request_body = generated::auth::LogoutPUTRequest,
    responses(
        (status = 200, body = generated::auth::LogoutPUTResponse)
    )
)]
pub async fn put_auth_logout(
    Json(request): Json<generated::auth::LogoutPUTRequest>,
) -> Json<generated::auth::LogoutPUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LogoutPUTResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/pats",
    responses(
        (status = 200, body = generated::auth::PatsGETResponse)
    )
)]
pub async fn get_auth_pats(
) -> Json<generated::auth::PatsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/pats",
    request_body = generated::auth::PatsPOSTRequest,
    responses(
        (status = 200, body = generated::auth::PatsPOSTResponse)
    )
)]
pub async fn post_auth_pats(
    Json(request): Json<generated::auth::PatsPOSTRequest>,
) -> Json<generated::auth::PatsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdGETResponse)
    )
)]
pub async fn get_auth_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdGETResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    request_body = generated::auth::PatsTokenIdPUTRequest,
    responses(
        (status = 200, body = generated::auth::PatsTokenIdPUTResponse)
    )
)]
pub async fn put_auth_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
    Json(request): Json<generated::auth::PatsTokenIdPUTRequest>,
) -> Json<generated::auth::PatsTokenIdPUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdPUTResponse::default())
}

#[utoipa::path(
    delete,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdDELETEResponse)
    )
)]
pub async fn delete_auth_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdDELETEResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdDELETEResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}/activate",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdActivatePUTResponse)
    )
)]
pub async fn put_auth_pats_token_id_activate(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdActivatePUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdActivatePUTResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}/deactivate",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdDeactivatePUTResponse)
    )
)]
pub async fn put_auth_pats_token_id_deactivate(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdDeactivatePUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdDeactivatePUTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/recover/user",
    request_body = generated::auth::RecoverUserPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserPOSTResponse)
    )
)]
pub async fn post_auth_recover_user(
    Json(request): Json<generated::auth::RecoverUserPOSTRequest>,
) -> Json<generated::auth::RecoverUserPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RecoverUserPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/code",
    request_body = generated::auth::RecoverUserCodePOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserCodePOSTResponse)
    )
)]
pub async fn post_auth_recover_user_code(
    Json(request): Json<generated::auth::RecoverUserCodePOSTRequest>,
) -> Json<generated::auth::RecoverUserCodePOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RecoverUserCodePOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/delegated",
    request_body = generated::auth::RecoverUserDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserDelegatedPOSTResponse)
    )
)]
pub async fn post_auth_recover_user_delegated(
    Json(request): Json<generated::auth::RecoverUserDelegatedPOSTRequest>,
) -> Json<generated::auth::RecoverUserDelegatedPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RecoverUserDelegatedPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/init",
    request_body = generated::auth::RecoverUserInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserInitPOSTResponse)
    )
)]
pub async fn post_auth_recover_user_init(
    Json(request): Json<generated::auth::RecoverUserInitPOSTRequest>,
) -> Json<generated::auth::RecoverUserInitPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RecoverUserInitPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration",
    request_body = generated::auth::RegistrationPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationPOSTResponse)
    )
)]
pub async fn post_auth_registration(
    Json(request): Json<generated::auth::RegistrationPOSTRequest>,
) -> Json<generated::auth::RegistrationPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationPOSTResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/registration/code",
    request_body = generated::auth::RegistrationCodePUTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationCodePUTResponse)
    )
)]
pub async fn put_auth_registration_code(
    Json(request): Json<generated::auth::RegistrationCodePUTRequest>,
) -> Json<generated::auth::RegistrationCodePUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationCodePUTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration/delegated",
    request_body = generated::auth::RegistrationDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationDelegatedPOSTResponse)
    )
)]
pub async fn post_auth_registration_delegated(
    Json(request): Json<generated::auth::RegistrationDelegatedPOSTRequest>,
) -> Json<generated::auth::RegistrationDelegatedPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationDelegatedPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration/enduser",
    request_body = generated::auth::RegistrationEnduserPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationEnduserPOSTResponse)
    )
)]
pub async fn post_auth_registration_enduser(
    Json(request): Json<generated::auth::RegistrationEnduserPOSTRequest>,
) -> Json<generated::auth::RegistrationEnduserPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationEnduserPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration/init",
    request_body = generated::auth::RegistrationInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationInitPOSTResponse)
    )
)]
pub async fn post_auth_registration_init(
    Json(request): Json<generated::auth::RegistrationInitPOSTRequest>,
) -> Json<generated::auth::RegistrationInitPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationInitPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration/social",
    request_body = generated::auth::RegistrationSocialPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationSocialPOSTResponse)
    )
)]
pub async fn post_auth_registration_social(
    Json(request): Json<generated::auth::RegistrationSocialPOSTRequest>,
) -> Json<generated::auth::RegistrationSocialPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationSocialPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/service-accounts",
    responses(
        (status = 200, body = generated::auth::ServiceAccountsGETResponse)
    )
)]
pub async fn get_auth_service_accounts(
) -> Json<generated::auth::ServiceAccountsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/service-accounts",
    request_body = generated::auth::ServiceAccountsPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ServiceAccountsPOSTResponse)
    )
)]
pub async fn post_auth_service_accounts(
    Json(request): Json<generated::auth::ServiceAccountsPOSTRequest>,
) -> Json<generated::auth::ServiceAccountsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdGETResponse)
    )
)]
pub async fn get_auth_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdGETResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    request_body = generated::auth::ServiceAccountsServiceAccountIdPUTRequest,
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdPUTResponse)
    )
)]
pub async fn put_auth_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
    Json(request): Json<generated::auth::ServiceAccountsServiceAccountIdPUTRequest>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdPUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdPUTResponse::default())
}

#[utoipa::path(
    delete,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdDELETEResponse)
    )
)]
pub async fn delete_auth_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdDELETEResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdDELETEResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}/activate",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse)
    )
)]
pub async fn put_auth_service_accounts_service_account_id_activate(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse::default())
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}/deactivate",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse)
    )
)]
pub async fn put_auth_service_accounts_service_account_id_deactivate(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse::default())
}

#[utoipa::path(
    get,
    path = "/auth/users",
    responses(
        (status = 200, body = generated::auth::UsersGETResponse)
    )
)]
pub async fn get_auth_users(
) -> Json<generated::auth::UsersGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::auth::UsersGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/auth/users",
    request_body = generated::auth::UsersPOSTRequest,
    responses(
        (status = 200, body = generated::User)
    )
)]
pub async fn post_auth_users(
    Json(request): Json<generated::auth::UsersPOSTRequest>,
) -> Json<generated::User> {
    // TODO: Replace with actual implementation
    Json(generated::User::default())
}

#[utoipa::path(
    get,
    path = "/auth/users/{userId}",
    params(
        ("userId" = String, Path, description = "userId")
    ),
    responses(
        (status = 200, body = generated::User)
    )
)]
pub async fn get_auth_users_user_id(
    AxumPath(userId): AxumPath<String>,
) -> Json<generated::User> {
    // TODO: Replace with actual implementation
    Json(generated::User::default())
}

#[utoipa::path(
    put,
    path = "/auth/users/{userId}",
    params(
        ("userId" = String, Path, description = "userId")
    ),
    request_body = generated::auth::UsersUserIdPUTRequest,
    responses(
        (status = 200, body = generated::User)
    )
)]
pub async fn put_auth_users_user_id(
    AxumPath(userId): AxumPath<String>,
    Json(request): Json<generated::auth::UsersUserIdPUTRequest>,
) -> Json<generated::User> {
    // TODO: Replace with actual implementation
    Json(generated::User::default())
}

#[utoipa::path(
    delete,
    path = "/auth/users/{userId}",
    params(
        ("userId" = String, Path, description = "userId")
    ),
    responses(
        (status = 200, body = generated::User)
    )
)]
pub async fn delete_auth_users_user_id(
    AxumPath(userId): AxumPath<String>,
) -> Json<generated::User> {
    // TODO: Replace with actual implementation
    Json(generated::User::default())
}

#[utoipa::path(
    put,
    path = "/auth/users/{userId}/activate",
    params(
        ("userId" = String, Path, description = "userId")
    ),
    responses(
        (status = 200, body = generated::User)
    )
)]
pub async fn put_auth_users_user_id_activate(
    AxumPath(userId): AxumPath<String>,
) -> Json<generated::User> {
    // TODO: Replace with actual implementation
    Json(generated::User::default())
}

#[utoipa::path(
    put,
    path = "/auth/users/{userId}/deactivate",
    params(
        ("userId" = String, Path, description = "userId")
    ),
    responses(
        (status = 200, body = generated::User)
    )
)]
pub async fn put_auth_users_user_id_deactivate(
    AxumPath(userId): AxumPath<String>,
) -> Json<generated::User> {
    // TODO: Replace with actual implementation
    Json(generated::User::default())
}

#[utoipa::path(
    get,
    path = "/exchanges",
    responses(
        (status = 200, body = generated::exchanges::ExchangesGETResponse)
    )
)]
pub async fn get_exchanges(
) -> Json<generated::exchanges::ExchangesGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangesGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/exchanges",
    request_body = generated::exchanges::ExchangesPOSTRequest,
    responses(
        (status = 200, body = generated::exchanges::ExchangesPOSTResponse)
    )
)]
pub async fn post_exchanges(
    Json(request): Json<generated::exchanges::ExchangesPOSTRequest>,
) -> Json<generated::exchanges::ExchangesPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangesPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdGETResponse)
    )
)]
pub async fn get_exchanges_exchange_id(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdGETResponse::default())
}

#[utoipa::path(
    delete,
    path = "/exchanges/{exchangeId}",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdDELETEResponse)
    )
)]
pub async fn delete_exchanges_exchange_id(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdDELETEResponse> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdDELETEResponse::default())
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}/accounts",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsGETResponse)
    )
)]
pub async fn get_exchanges_exchange_id_accounts(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdAccountsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdAccountsGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/assets",
    params(
        ("exchangeId" = String, Path, description = "exchangeId"),
        ("accountId" = String, Path, description = "accountId")
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse)
    )
)]
pub async fn get_exchanges_exchange_id_accounts_account_id_assets(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/assets/{asset}/withdrawal-networks",
    params(
        ("exchangeId" = String, Path, description = "exchangeId"),
        ("accountId" = String, Path, description = "accountId"),
        ("asset" = String, Path, description = "asset")
    ),
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_exchanges_exchange_id_accounts_account_id_assets_asset_withdrawal_networks(
    AxumPath((exchangeId, accountId, asset)): AxumPath<(String, String, String)>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    post,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/deposits",
    params(
        ("exchangeId" = String, Path, description = "exchangeId"),
        ("accountId" = String, Path, description = "accountId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse)
    )
)]
pub async fn post_exchanges_exchange_id_accounts_account_id_deposits(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/withdrawals",
    params(
        ("exchangeId" = String, Path, description = "exchangeId"),
        ("accountId" = String, Path, description = "accountId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse)
    )
)]
pub async fn post_exchanges_exchange_id_accounts_account_id_withdrawals(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/fee-sponsors",
    responses(
        (status = 200, body = generated::feesponsors::FeesponsorsGETResponse)
    )
)]
pub async fn get_feesponsors(
) -> Json<generated::feesponsors::FeesponsorsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::feesponsors::FeesponsorsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/fee-sponsors",
    request_body = generated::feesponsors::FeesponsorsPOSTRequest,
    responses(
        (status = 200, body = generated::feesponsors::FeesponsorsPOSTResponse)
    )
)]
pub async fn post_feesponsors(
    Json(request): Json<generated::feesponsors::FeesponsorsPOSTRequest>,
) -> Json<generated::feesponsors::FeesponsorsPOSTResponse> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx"
        }
    );
    let response: generated::feesponsors::FeesponsorsPOSTResponse = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    get,
    path = "/fee-sponsors/{feeSponsorId}",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdGETResponse)
    )
)]
pub async fn get_feesponsors_fee_sponsor_id(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdGETResponse> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx"
        }
    );
    let response: generated::feesponsors::FeeSponsorIdGETResponse = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    delete,
    path = "/fee-sponsors/{feeSponsorId}",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdDELETEResponse)
    )
)]
pub async fn delete_feesponsors_fee_sponsor_id(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdDELETEResponse> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx"
        }
    );
    let response: generated::feesponsors::FeeSponsorIdDELETEResponse = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    put,
    path = "/fee-sponsors/{feeSponsorId}/activate",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdActivatePUTResponse)
    )
)]
pub async fn put_feesponsors_fee_sponsor_id_activate(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdActivatePUTResponse> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx"
        }
    );
    let response: generated::feesponsors::FeeSponsorIdActivatePUTResponse = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    put,
    path = "/fee-sponsors/{feeSponsorId}/deactivate",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdDeactivatePUTResponse)
    )
)]
pub async fn put_feesponsors_fee_sponsor_id_deactivate(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdDeactivatePUTResponse> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx"
        }
    );
    let response: generated::feesponsors::FeeSponsorIdDeactivatePUTResponse = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    get,
    path = "/fee-sponsors/{feeSponsorId}/fees",
    params(
        ("feeSponsorId" = String, Path, description = "feeSponsorId")
    ),
    responses(
        (status = 200, body = generated::feesponsors::FeeSponsorIdFeesGETResponse)
    )
)]
pub async fn get_feesponsors_fee_sponsor_id_fees(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdFeesGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::feesponsors::FeeSponsorIdFeesGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/key-stores",
    responses(
        (status = 200, body = generated::keystores::KeystoresGETResponse)
    )
)]
pub async fn get_keystores(
) -> Json<generated::keystores::KeystoresGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keystores::KeystoresGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/keys",
    responses(
        (status = 200, body = generated::keys::KeysGETResponse)
    )
)]
pub async fn get_keys(
) -> Json<generated::keys::KeysGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeysGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/keys",
    request_body = generated::keys::KeysPOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeysPOSTResponse)
    )
)]
pub async fn post_keys(
    Json(request): Json<generated::keys::KeysPOSTRequest>,
) -> Json<generated::keys::KeysPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeysPOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/keys/import",
    request_body = generated::keys::ImportPOSTRequest,
    responses(
        (status = 200, body = generated::keys::ImportPOSTResponse)
    )
)]
pub async fn post_keys_import(
    Json(request): Json<generated::keys::ImportPOSTRequest>,
) -> Json<generated::keys::ImportPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::ImportPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdGETResponse)
    )
)]
pub async fn get_keys_key_id(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdGETResponse::default())
}

#[utoipa::path(
    put,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = generated::keys::KeyIdPUTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdPUTResponse)
    )
)]
pub async fn put_keys_key_id(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdPUTRequest>,
) -> Json<generated::keys::KeyIdPUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdPUTResponse::default())
}

#[utoipa::path(
    delete,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdDELETEResponse)
    )
)]
pub async fn delete_keys_key_id(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdDELETEResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdDELETEResponse::default())
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/delegate",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = generated::keys::KeyIdDelegatePOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdDelegatePOSTResponse)
    )
)]
pub async fn post_keys_key_id_delegate(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdDelegatePOSTRequest>,
) -> Json<generated::keys::KeyIdDelegatePOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdDelegatePOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/derive",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = generated::keys::KeyIdDerivePOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdDerivePOSTResponse)
    )
)]
pub async fn post_keys_key_id_derive(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdDerivePOSTRequest>,
) -> Json<generated::keys::KeyIdDerivePOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdDerivePOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/export",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = generated::keys::KeyIdExportPOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdExportPOSTResponse)
    )
)]
pub async fn post_keys_key_id_export(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdExportPOSTRequest>,
) -> Json<generated::keys::KeyIdExportPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdExportPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}/signatures",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesGETResponse)
    )
)]
pub async fn get_keys_key_id_signatures(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdSignaturesGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdSignaturesGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/signatures",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesPOSTResponse)
    )
)]
pub async fn post_keys_key_id_signatures(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::keys::KeyIdSignaturesPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdSignaturesPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}/signatures/{signatureId}",
    params(
        ("keyId" = String, Path, description = "keyId"),
        ("signatureId" = String, Path, description = "signatureId")
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesSignatureIdGETResponse)
    )
)]
pub async fn get_keys_key_id_signatures_signature_id(
    AxumPath((keyId, signatureId)): AxumPath<(String, String)>,
) -> Json<generated::keys::KeyIdSignaturesSignatureIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdSignaturesSignatureIdGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/networks/fees",
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_networks_fees(
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    post,
    path = "/networks/read-contract",
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::networks::ReadContractPOSTResponse)
    )
)]
pub async fn post_networks_read_contract(
    Json(request): Json<serde_json::Value>,
) -> Json<generated::networks::ReadContractPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::networks::ReadContractPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/networks/{network}/validators",
    params(
        ("network" = String, Path, description = "network")
    ),
    responses(
        (status = 200, body = generated::networks::NetworkValidatorsGETResponse)
    )
)]
pub async fn get_networks_network_validators(
    AxumPath(network): AxumPath<String>,
) -> Json<generated::networks::NetworkValidatorsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::networks::NetworkValidatorsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/networks/{network}/validators",
    params(
        ("network" = String, Path, description = "network")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::CantonValidator)
    )
)]
pub async fn post_networks_network_validators(
    AxumPath(network): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::CantonValidator> {
    // TODO: Replace with actual implementation
    Json(generated::CantonValidator::default())
}

#[utoipa::path(
    get,
    path = "/networks/{network}/validators/{validatorId}",
    params(
        ("network" = String, Path, description = "network"),
        ("validatorId" = String, Path, description = "validatorId")
    ),
    responses(
        (status = 200, body = generated::CantonValidator)
    )
)]
pub async fn get_networks_network_validators_validator_id(
    AxumPath((network, validatorId)): AxumPath<(String, String)>,
) -> Json<generated::CantonValidator> {
    // TODO: Replace with actual implementation
    Json(generated::CantonValidator::default())
}

#[utoipa::path(
    put,
    path = "/networks/{network}/validators/{validatorId}",
    params(
        ("network" = String, Path, description = "network"),
        ("validatorId" = String, Path, description = "validatorId")
    ),
    request_body = generated::networks::NetworkValidatorsValidatorIdPUTRequest,
    responses(
        (status = 200, body = generated::CantonValidator)
    )
)]
pub async fn put_networks_network_validators_validator_id(
    AxumPath((network, validatorId)): AxumPath<(String, String)>,
    Json(request): Json<generated::networks::NetworkValidatorsValidatorIdPUTRequest>,
) -> Json<generated::CantonValidator> {
    // TODO: Replace with actual implementation
    Json(generated::CantonValidator::default())
}

#[utoipa::path(
    delete,
    path = "/networks/{network}/validators/{validatorId}",
    params(
        ("network" = String, Path, description = "network"),
        ("validatorId" = String, Path, description = "validatorId")
    ),
    responses(
        (status = 200, body = generated::CantonValidator)
    )
)]
pub async fn delete_networks_network_validators_validator_id(
    AxumPath((network, validatorId)): AxumPath<(String, String)>,
) -> Json<generated::CantonValidator> {
    // TODO: Replace with actual implementation
    Json(generated::CantonValidator::default())
}

#[utoipa::path(
    get,
    path = "/permissions",
    responses(
        (status = 200, body = generated::permissions::PermissionsGETResponse)
    )
)]
pub async fn get_permissions(
) -> Json<generated::permissions::PermissionsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/permissions",
    request_body = generated::permissions::PermissionsPOSTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionsPOSTResponse)
    )
)]
pub async fn post_permissions(
    Json(request): Json<generated::permissions::PermissionsPOSTRequest>,
) -> Json<generated::permissions::PermissionsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionsPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/permissions/{permissionId}",
    params(
        ("permissionId" = String, Path, description = "permissionId")
    ),
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_permissions_permission_id(
    AxumPath(permissionId): AxumPath<String>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    put,
    path = "/permissions/{permissionId}",
    params(
        ("permissionId" = String, Path, description = "permissionId")
    ),
    request_body = generated::permissions::PermissionIdPUTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionIdPUTResponse)
    )
)]
pub async fn put_permissions_permission_id(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdPUTRequest>,
) -> Json<generated::permissions::PermissionIdPUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionIdPUTResponse::default())
}

#[utoipa::path(
    put,
    path = "/permissions/{permissionId}/archive",
    params(
        ("permissionId" = String, Path, description = "permissionId")
    ),
    request_body = generated::permissions::PermissionIdArchivePUTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionIdArchivePUTResponse)
    )
)]
pub async fn put_permissions_permission_id_archive(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdArchivePUTRequest>,
) -> Json<generated::permissions::PermissionIdArchivePUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionIdArchivePUTResponse::default())
}

#[utoipa::path(
    get,
    path = "/permissions/{permissionId}/assignments",
    params(
        ("permissionId" = String, Path, description = "permissionId")
    ),
    responses(
        (status = 200, body = generated::permissions::PermissionIdAssignmentsGETResponse)
    )
)]
pub async fn get_permissions_permission_id_assignments(
    AxumPath(permissionId): AxumPath<String>,
) -> Json<generated::permissions::PermissionIdAssignmentsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionIdAssignmentsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/permissions/{permissionId}/assignments",
    params(
        ("permissionId" = String, Path, description = "permissionId")
    ),
    request_body = generated::permissions::PermissionIdAssignmentsPOSTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionIdAssignmentsPOSTResponse)
    )
)]
pub async fn post_permissions_permission_id_assignments(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdAssignmentsPOSTRequest>,
) -> Json<generated::permissions::PermissionIdAssignmentsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionIdAssignmentsPOSTResponse::default())
}

#[utoipa::path(
    delete,
    path = "/permissions/{permissionId}/assignments/{assignmentId}",
    params(
        ("permissionId" = String, Path, description = "permissionId"),
        ("assignmentId" = String, Path, description = "assignmentId")
    ),
    responses(
        (status = 200)
    )
)]
pub async fn delete_permissions_permission_id_assignments_assignment_id(
    AxumPath((permissionId, assignmentId)): AxumPath<(String, String)>,
) -> StatusCode {
    // TODO: Replace with actual implementation
    StatusCode::OK
}

#[utoipa::path(
    get,
    path = "/signers",
    responses(
        (status = 200, body = generated::signers::SignersGETResponse)
    )
)]
pub async fn get_signers(
) -> Json<generated::signers::SignersGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::signers::SignersGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/staking/stakes",
    responses(
        (status = 200, body = generated::staking::StakesGETResponse)
    )
)]
pub async fn get_staking_stakes(
) -> Json<generated::staking::StakesGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::staking::StakesGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/staking/stakes",
    request_body = serde_json::Value,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_staking_stakes(
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/staking/stakes/{stakeId}",
    params(
        ("stakeId" = String, Path, description = "stakeId")
    ),
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_staking_stakes_stake_id(
    AxumPath(stakeId): AxumPath<String>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/staking/stakes/{stakeId}/actions",
    params(
        ("stakeId" = String, Path, description = "stakeId")
    ),
    responses(
        (status = 200, body = generated::staking::StakesStakeIdActionsGETResponse)
    )
)]
pub async fn get_staking_stakes_stake_id_actions(
    AxumPath(stakeId): AxumPath<String>,
) -> Json<generated::staking::StakesStakeIdActionsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::staking::StakesStakeIdActionsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/staking/stakes/{stakeId}/actions",
    params(
        ("stakeId" = String, Path, description = "stakeId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_staking_stakes_stake_id_actions(
    AxumPath(stakeId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/staking/stakes/{stakeId}/rewards",
    params(
        ("stakeId" = String, Path, description = "stakeId")
    ),
    responses(
        (status = 200, body = generated::staking::StakesStakeIdRewardsGETResponse)
    )
)]
pub async fn get_staking_stakes_stake_id_rewards(
    AxumPath(stakeId): AxumPath<String>,
) -> Json<generated::staking::StakesStakeIdRewardsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::staking::StakesStakeIdRewardsGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/swaps",
    responses(
        (status = 200, body = generated::swaps::SwapsGETResponse)
    )
)]
pub async fn get_swaps(
) -> Json<generated::swaps::SwapsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::swaps::SwapsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/swaps",
    request_body = generated::swaps::SwapsPOSTRequest,
    responses(
        (status = 200, body = generated::Swap)
    )
)]
pub async fn post_swaps(
    Json(request): Json<generated::swaps::SwapsPOSTRequest>,
) -> Json<generated::Swap> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2025-09-11T10:57:55.758Z",
          "id": "swap-6a3ku-bn8d7-8u5rs1oukojms7k8",
          "provider": "UniswapClassic",
          "quoteId": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
          "quotedSourceAsset": {
            "amount": "10000000000",
            "kind": "Native",
            "metadata": {
              "decimals": 18,
              "name": "Ethereum",
              "network": "EthereumSepolia",
              "symbol": "SepoliaETH",
              "tid": "native:eth"
            }
          },
          "quotedTargetAsset": {
            "amount": "6467571553831928182",
            "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
            "kind": "Erc20",
            "metadata": {
              "decimals": 18,
              "name": "Test",
              "network": "EthereumSepolia",
              "symbol": "Test",
              "tid": "erc20:0xda0be7efd234295395d4204d0df4358339b57b27"
            }
          },
          "reference": null,
          "requestBody": {
            "provider": "UniswapClassic",
            "quoteId": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
            "slippageBps": 100,
            "sourceAsset": {
              "amount": "10000000000",
              "kind": "Native"
            },
            "targetAsset": {
              "amount": "653003161",
              "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
              "kind": "Erc20"
            },
            "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv"
          },
          "requester": {
            "tokenId": "to-4etah-smoal-9n3rmhul4dpaueg5",
            "userId": "us-48r5q-eshfg-9pmr2lo6bmpr4i4i"
          },
          "slippageBps": 100,
          "status": "PendingPolicyApproval",
          "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv"
        }
    );
    let response: generated::Swap = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    post,
    path = "/swaps/quotes",
    request_body = generated::swaps::QuotesPOSTRequest,
    responses(
        (status = 200, body = generated::SwapQuote)
    )
)]
pub async fn post_swaps_quotes(
    Json(request): Json<generated::swaps::QuotesPOSTRequest>,
) -> Json<generated::SwapQuote> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2025-09-11T10:52:15.039Z",
          "id": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
          "provider": "UniswapClassic",
          "requestBody": {
            "provider": "UniswapClassic",
            "slippageBps": 100,
            "sourceAsset": {
              "amount": "10000000000",
              "kind": "Native"
            },
            "targetAsset": {
              "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
              "kind": "Erc20"
            },
            "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv"
          },
          "requester": {
            "tokenId": "to-4etah-smoal-9n3rmhul4dpaueg5",
            "userId": "us-48r5q-eshfg-9pmr2lo6bmpr4i4i"
          },
          "slippageBps": 100,
          "sourceAsset": {
            "amount": "10000000000",
            "kind": "Native",
            "metadata": {
              "decimals": 18,
              "name": "Ethereum",
              "network": "EthereumSepolia",
              "symbol": "SepoliaETH",
              "tid": "native:eth"
            }
          },
          "targetAsset": {
            "amount": "6467571553831928182",
            "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
            "kind": "Erc20",
            "metadata": {
              "decimals": 18,
              "name": "Test",
              "network": "EthereumSepolia",
              "symbol": "Test",
              "tid": "erc20:0xda0be7efd234295395d4204d0df4358339b57b27"
            }
          },
          "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv"
        }
    );
    let response: generated::SwapQuote = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    get,
    path = "/swaps/quotes/{quoteId}",
    params(
        ("quoteId" = String, Path, description = "quoteId")
    ),
    responses(
        (status = 200, body = generated::SwapQuote)
    )
)]
pub async fn get_swaps_quotes_quote_id(
    AxumPath(quoteId): AxumPath<String>,
) -> Json<generated::SwapQuote> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2025-09-11T10:52:15.039Z",
          "id": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
          "provider": "UniswapClassic",
          "requestBody": {
            "provider": "UniswapClassic",
            "slippageBps": 100,
            "sourceAsset": {
              "amount": "10000000000",
              "kind": "Native"
            },
            "targetAsset": {
              "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
              "kind": "Erc20"
            },
            "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv"
          },
          "requester": {
            "tokenId": "to-4etah-smoal-9n3rmhul4dpaueg5",
            "userId": "us-48r5q-eshfg-9pmr2lo6bmpr4i4i"
          },
          "slippageBps": 100,
          "sourceAsset": {
            "amount": "10000000000",
            "kind": "Native",
            "metadata": {
              "decimals": 18,
              "name": "Ethereum",
              "network": "EthereumSepolia",
              "symbol": "SepoliaETH",
              "tid": "native:eth"
            }
          },
          "targetAsset": {
            "amount": "6467571553831928182",
            "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
            "kind": "Erc20",
            "metadata": {
              "decimals": 18,
              "name": "Test",
              "network": "EthereumSepolia",
              "symbol": "Test",
              "tid": "erc20:0xda0be7efd234295395d4204d0df4358339b57b27"
            }
          },
          "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv"
        }
    );
    let response: generated::SwapQuote = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    get,
    path = "/swaps/{swapId}",
    params(
        ("swapId" = String, Path, description = "swapId")
    ),
    responses(
        (status = 200, body = generated::Swap)
    )
)]
pub async fn get_swaps_swap_id(
    AxumPath(swapId): AxumPath<String>,
) -> Json<generated::Swap> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "dateCreated": "2025-09-11T10:57:55.758Z",
          "id": "swap-6a3ku-bn8d7-8u5rs1oukojms7k8",
          "provider": "UniswapClassic",
          "quoteId": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
          "quotedSourceAsset": {
            "amount": "10000000000",
            "kind": "Native",
            "metadata": {
              "decimals": 18,
              "name": "Ethereum",
              "network": "EthereumSepolia",
              "symbol": "SepoliaETH",
              "tid": "native:eth"
            }
          },
          "quotedTargetAsset": {
            "amount": "6467571553831928182",
            "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
            "kind": "Erc20",
            "metadata": {
              "decimals": 18,
              "name": "Test",
              "network": "EthereumSepolia",
              "symbol": "Test",
              "tid": "erc20:0xda0be7efd234295395d4204d0df4358339b57b27"
            }
          },
          "reference": null,
          "requestBody": {
            "provider": "UniswapClassic",
            "quoteId": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
            "slippageBps": 100,
            "sourceAsset": {
              "amount": "10000000000",
              "kind": "Native"
            },
            "targetAsset": {
              "amount": "653003161",
              "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
              "kind": "Erc20"
            },
            "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv"
          },
          "requester": {
            "tokenId": "to-4etah-smoal-9n3rmhul4dpaueg5",
            "userId": "us-48r5q-eshfg-9pmr2lo6bmpr4i4i"
          },
          "slippageBps": 100,
          "status": "PendingPolicyApproval",
          "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv"
        }
    );
    let response: generated::Swap = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    get,
    path = "/v2/policies",
    responses(
        (status = 200, body = generated::v2::PoliciesGETResponse)
    )
)]
pub async fn get_v2_policies(
) -> Json<generated::v2::PoliciesGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::v2::PoliciesGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/v2/policies",
    request_body = serde_json::Value,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_v2_policies(
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/v2/policies/{policyId}",
    params(
        ("policyId" = String, Path, description = "policyId")
    ),
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_v2_policies_policy_id(
    AxumPath(policyId): AxumPath<String>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    put,
    path = "/v2/policies/{policyId}",
    params(
        ("policyId" = String, Path, description = "policyId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn put_v2_policies_policy_id(
    AxumPath(policyId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    delete,
    path = "/v2/policies/{policyId}",
    params(
        ("policyId" = String, Path, description = "policyId")
    ),
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn delete_v2_policies_policy_id(
    AxumPath(policyId): AxumPath<String>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/v2/policy-approvals",
    responses(
        (status = 200, body = generated::v2::PolicyApprovalsGETResponse)
    )
)]
pub async fn get_v2_policy_approvals(
) -> Json<generated::v2::PolicyApprovalsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::v2::PolicyApprovalsGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/v2/policy-approvals/{approvalId}",
    params(
        ("approvalId" = String, Path, description = "approvalId")
    ),
    responses(
        (status = 200, body = generated::v2::PolicyApprovalsApprovalIdGETResponse)
    )
)]
pub async fn get_v2_policy_approvals_approval_id(
    AxumPath(approvalId): AxumPath<String>,
) -> Json<generated::v2::PolicyApprovalsApprovalIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::v2::PolicyApprovalsApprovalIdGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/v2/policy-approvals/{approvalId}/decisions",
    params(
        ("approvalId" = String, Path, description = "approvalId")
    ),
    request_body = generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTRequest,
    responses(
        (status = 200, body = generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse)
    )
)]
pub async fn post_v2_policy_approvals_approval_id_decisions(
    AxumPath(approvalId): AxumPath<String>,
    Json(request): Json<generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTRequest>,
) -> Json<generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/wallets",
    responses(
        (status = 200, body = generated::wallets::WalletsGETResponse)
    )
)]
pub async fn get_wallets(
) -> Json<generated::wallets::WalletsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/wallets",
    request_body = generated::wallets::WalletsPOSTRequest,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_wallets(
    Json(request): Json<generated::wallets::WalletsPOSTRequest>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/wallets/all/history",
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_wallets_all_history(
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    post,
    path = "/wallets/import",
    request_body = generated::wallets::ImportPOSTRequest,
    responses(
        (status = 200, body = generated::Wallet)
    )
)]
pub async fn post_wallets_import(
    Json(request): Json<generated::wallets::ImportPOSTRequest>,
) -> Json<generated::Wallet> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "address": "0x00e3495cf6af59008f22ffaf32d4c92ac33dac47",
          "custodial": true,
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "id": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "name": "trading hot wallet",
          "network": "Ethereum",
          "signingKey": {
            "curve": "secp256k1",
            "id": "key-6ece3-9l565-xxxxxxxxxxxxxxxx",
            "publicKey": "e2375c8c9e87bfcd0be8f29d76c818cabacd51584f72cb2222d49a13b036d84d3d",
            "scheme": "ECDSA"
          },
          "status": "Active",
          "tags": []
        }
    );
    let response: generated::Wallet = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::Wallet)
    )
)]
pub async fn get_wallets_wallet_id(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::Wallet> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "address": "0x00e3495cf6af59008f22ffaf32d4c92ac33dac47",
          "custodial": true,
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "id": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "name": "trading hot wallet",
          "network": "Ethereum",
          "signingKey": {
            "curve": "secp256k1",
            "id": "key-6ece3-9l565-xxxxxxxxxxxxxxxx",
            "publicKey": "e2375c8c9e87bfcd0be8f29d76c818cabacd51584f72cb2222d49a13b036d84d3d",
            "scheme": "ECDSA"
          },
          "status": "Active",
          "tags": []
        }
    );
    let response: generated::Wallet = serde_json::from_value(example_json)
        .expect("Failed to deserialize example into response type");
    Json(response)
}

#[utoipa::path(
    put,
    path = "/wallets/{walletId}",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = generated::wallets::WalletIdPUTRequest,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn put_wallets_wallet_id(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdPUTRequest>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/assets",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdAssetsGETResponse)
    )
)]
pub async fn get_wallets_wallet_id_assets(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdAssetsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdAssetsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/delegate",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = generated::wallets::WalletIdDelegatePOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdDelegatePOSTResponse)
    )
)]
pub async fn post_wallets_wallet_id_delegate(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdDelegatePOSTRequest>,
) -> Json<generated::wallets::WalletIdDelegatePOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdDelegatePOSTResponse::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/export",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = generated::wallets::WalletIdExportPOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdExportPOSTResponse)
    )
)]
pub async fn post_wallets_wallet_id_export(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdExportPOSTRequest>,
) -> Json<generated::wallets::WalletIdExportPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdExportPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/history",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_wallets_wallet_id_history(
    AxumPath(walletId): AxumPath<String>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/nfts",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdNftsGETResponse)
    )
)]
pub async fn get_wallets_wallet_id_nfts(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdNftsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdNftsGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/offers",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdOffersGETResponse)
    )
)]
pub async fn get_wallets_wallet_id_offers(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdOffersGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdOffersGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/offers/{offerId}",
    params(
        ("walletId" = String, Path, description = "walletId"),
        ("offerId" = String, Path, description = "offerId")
    ),
    responses(
        (status = 200, body = generated::Offer)
    )
)]
pub async fn get_wallets_wallet_id_offers_offer_id(
    AxumPath((walletId, offerId)): AxumPath<(String, String)>,
) -> Json<generated::Offer> {
    // TODO: Replace with actual implementation
    Json(generated::Offer::default())
}

#[utoipa::path(
    put,
    path = "/wallets/{walletId}/offers/{offerId}/accept",
    params(
        ("walletId" = String, Path, description = "walletId"),
        ("offerId" = String, Path, description = "offerId")
    ),
    responses(
        (status = 200, body = generated::Offer)
    )
)]
pub async fn put_wallets_wallet_id_offers_offer_id_accept(
    AxumPath((walletId, offerId)): AxumPath<(String, String)>,
) -> Json<generated::Offer> {
    // TODO: Replace with actual implementation
    Json(generated::Offer::default())
}

#[utoipa::path(
    put,
    path = "/wallets/{walletId}/offers/{offerId}/reject",
    params(
        ("walletId" = String, Path, description = "walletId"),
        ("offerId" = String, Path, description = "offerId")
    ),
    responses(
        (status = 200, body = generated::Offer)
    )
)]
pub async fn put_wallets_wallet_id_offers_offer_id_reject(
    AxumPath((walletId, offerId)): AxumPath<(String, String)>,
) -> Json<generated::Offer> {
    // TODO: Replace with actual implementation
    Json(generated::Offer::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/signatures",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdSignaturesGETResponse)
    )
)]
pub async fn get_wallets_wallet_id_signatures(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdSignaturesGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdSignaturesGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/signatures",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::wallets::WalletIdSignaturesPOSTResponse)
    )
)]
pub async fn post_wallets_wallet_id_signatures(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::wallets::WalletIdSignaturesPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdSignaturesPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/signatures/{signatureId}",
    params(
        ("walletId" = String, Path, description = "walletId"),
        ("signatureId" = String, Path, description = "signatureId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdSignaturesSignatureIdGETResponse)
    )
)]
pub async fn get_wallets_wallet_id_signatures_signature_id(
    AxumPath((walletId, signatureId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdSignaturesSignatureIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdSignaturesSignatureIdGETResponse::default())
}

#[utoipa::path(
    put,
    path = "/wallets/{walletId}/tags",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = generated::wallets::WalletIdTagsPUTRequest,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn put_wallets_wallet_id_tags(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdTagsPUTRequest>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    delete,
    path = "/wallets/{walletId}/tags",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = generated::wallets::WalletIdTagsDELETERequest,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn delete_wallets_wallet_id_tags(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdTagsDELETERequest>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transactions",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransactionsGETResponse)
    )
)]
pub async fn get_wallets_wallet_id_transactions(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdTransactionsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdTransactionsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/transactions",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::wallets::WalletIdTransactionsPOSTResponse)
    )
)]
pub async fn post_wallets_wallet_id_transactions(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::wallets::WalletIdTransactionsPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdTransactionsPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transactions/{transactionId}",
    params(
        ("walletId" = String, Path, description = "walletId"),
        ("transactionId" = String, Path, description = "transactionId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransactionsTransactionIdGETResponse)
    )
)]
pub async fn get_wallets_wallet_id_transactions_transaction_id(
    AxumPath((walletId, transactionId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdTransactionsTransactionIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdTransactionsTransactionIdGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transfers",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransfersGETResponse)
    )
)]
pub async fn get_wallets_wallet_id_transfers(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdTransfersGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdTransfersGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/transfers",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::TransferRequest)
    )
)]
pub async fn post_wallets_wallet_id_transfers(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::TransferRequest> {
    // TODO: Replace with actual implementation
    Json(generated::TransferRequest::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transfers/{transferId}",
    params(
        ("walletId" = String, Path, description = "walletId"),
        ("transferId" = String, Path, description = "transferId")
    ),
    responses(
        (status = 200, body = generated::TransferRequest)
    )
)]
pub async fn get_wallets_wallet_id_transfers_transfer_id(
    AxumPath((walletId, transferId)): AxumPath<(String, String)>,
) -> Json<generated::TransferRequest> {
    // TODO: Replace with actual implementation
    Json(generated::TransferRequest::default())
}

#[utoipa::path(
    get,
    path = "/webhooks",
    responses(
        (status = 200, body = generated::webhooks::WebhooksGETResponse)
    )
)]
pub async fn get_webhooks(
) -> Json<generated::webhooks::WebhooksGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhooksGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/webhooks",
    request_body = generated::webhooks::WebhooksPOSTRequest,
    responses(
        (status = 200, body = generated::webhooks::WebhooksPOSTResponse)
    )
)]
pub async fn post_webhooks(
    Json(request): Json<generated::webhooks::WebhooksPOSTRequest>,
) -> Json<generated::webhooks::WebhooksPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhooksPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdGETResponse)
    )
)]
pub async fn get_webhooks_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdGETResponse::default())
}

#[utoipa::path(
    put,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    request_body = generated::webhooks::WebhookIdPUTRequest,
    responses(
        (status = 200, body = generated::webhooks::WebhookIdPUTResponse)
    )
)]
pub async fn put_webhooks_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
    Json(request): Json<generated::webhooks::WebhookIdPUTRequest>,
) -> Json<generated::webhooks::WebhookIdPUTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdPUTResponse::default())
}

#[utoipa::path(
    delete,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdDELETEResponse)
    )
)]
pub async fn delete_webhooks_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdDELETEResponse> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdDELETEResponse::default())
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}/events",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdEventsGETResponse)
    )
)]
pub async fn get_webhooks_webhook_id_events(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdEventsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdEventsGETResponse::default())
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}/events/{webhookEventId}",
    params(
        ("webhookId" = String, Path, description = "webhookId"),
        ("webhookEventId" = String, Path, description = "webhookEventId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse)
    )
)]
pub async fn get_webhooks_webhook_id_events_webhook_event_id(
    AxumPath((webhookId, webhookEventId)): AxumPath<(String, String)>,
) -> Json<generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/webhooks/{webhookId}/ping",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdPingPOSTResponse)
    )
)]
pub async fn post_webhooks_webhook_id_ping(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdPingPOSTResponse> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdPingPOSTResponse::default())
}

#[utoipa::path(
    get,
    path = "/yields",
    responses(
        (status = 200, body = generated::yields::YieldsGETResponse)
    )
)]
pub async fn get_yields(
) -> Json<generated::yields::YieldsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::yields::YieldsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/yields",
    request_body = serde_json::Value,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_yields(
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/yields/{yieldId}",
    params(
        ("yieldId" = String, Path, description = "yieldId")
    ),
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn get_yields_yield_id(
    AxumPath(yieldId): AxumPath<String>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

#[utoipa::path(
    get,
    path = "/yields/{yieldId}/actions",
    params(
        ("yieldId" = String, Path, description = "yieldId")
    ),
    responses(
        (status = 200, body = generated::yields::YieldIdActionsGETResponse)
    )
)]
pub async fn get_yields_yield_id_actions(
    AxumPath(yieldId): AxumPath<String>,
) -> Json<generated::yields::YieldIdActionsGETResponse> {
    // TODO: Replace with actual implementation
    Json(generated::yields::YieldIdActionsGETResponse::default())
}

#[utoipa::path(
    post,
    path = "/yields/{yieldId}/actions",
    params(
        ("yieldId" = String, Path, description = "yieldId")
    ),
    request_body = generated::yields::YieldIdActionsPOSTRequest,
    responses(
        (status = 200, body = serde_json::Value)
    )
)]
pub async fn post_yields_yield_id_actions(
    AxumPath(yieldId): AxumPath<String>,
    Json(request): Json<generated::yields::YieldIdActionsPOSTRequest>,
) -> Json<serde_json::Value> {
    // TODO: Replace with actual implementation
    Json(serde_json::json!({}))
}

