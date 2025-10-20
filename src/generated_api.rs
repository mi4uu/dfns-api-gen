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
        (status = 200, body = generated::agreements::LatestUnacceptedGETResponse200)
    )
)]
pub async fn get_agreements_latest_unaccepted(
) -> Json<generated::agreements::LatestUnacceptedGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::agreements::LatestUnacceptedGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/agreements/{agreementId}/accept",
    params(
        ("agreementId" = String, Path, description = "agreementId")
    ),
    responses(
        (status = 200, body = generated::agreements::AgreementIdAcceptPOSTResponse200)
    )
)]
pub async fn post_agreements_agreement_id_accept(
    AxumPath(agreementId): AxumPath<String>,
) -> Json<generated::agreements::AgreementIdAcceptPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::agreements::AgreementIdAcceptPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/action",
    request_body = generated::auth::ActionPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ActionPOSTResponse200)
    )
)]
pub async fn post_auth_action(
    Json(request): Json<generated::auth::ActionPOSTRequest>,
) -> Json<generated::auth::ActionPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ActionPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/action/init",
    request_body = generated::auth::ActionInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ActionInitPOSTResponse200)
    )
)]
pub async fn post_auth_action_init(
    Json(request): Json<generated::auth::ActionInitPOSTRequest>,
) -> Json<generated::auth::ActionInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ActionInitPOSTResponse200::default())
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
        (status = 200, body = generated::auth::ActionLogsIdGETResponse200)
    )
)]
pub async fn get_auth_action_logs_id(
    AxumPath(id): AxumPath<String>,
) -> Json<generated::auth::ActionLogsIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ActionLogsIdGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/auth/apps",
    responses(
        (status = 200, body = generated::auth::AppsGETResponse200)
    )
)]
pub async fn get_auth_apps(
) -> Json<generated::auth::AppsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::AppsGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/auth/apps/{appId}",
    params(
        ("appId" = String, Path, description = "appId")
    ),
    responses(
        (status = 200, body = generated::auth::AppsAppIdGETResponse200)
    )
)]
pub async fn get_auth_apps_app_id(
    AxumPath(appId): AxumPath<String>,
) -> Json<generated::auth::AppsAppIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::AppsAppIdGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/auth/credentials",
    responses(
        (status = 200, body = generated::auth::CredentialsGETResponse200)
    )
)]
pub async fn get_auth_credentials(
) -> Json<generated::auth::CredentialsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/credentials",
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::auth::CredentialsPOSTResponse200)
    )
)]
pub async fn post_auth_credentials(
    Json(request): Json<serde_json::Value>,
) -> Json<generated::auth::CredentialsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsPOSTResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/credentials/activate",
    request_body = generated::auth::CredentialsActivatePUTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsActivatePUTResponse200)
    )
)]
pub async fn put_auth_credentials_activate(
    Json(request): Json<generated::auth::CredentialsActivatePUTRequest>,
) -> Json<generated::auth::CredentialsActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsActivatePUTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/credentials/code",
    request_body = generated::auth::CredentialsCodePOSTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsCodePOSTResponse200)
    )
)]
pub async fn post_auth_credentials_code(
    Json(request): Json<generated::auth::CredentialsCodePOSTRequest>,
) -> Json<generated::auth::CredentialsCodePOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsCodePOSTResponse200::default())
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
        (status = 200, body = generated::auth::CredentialsCodeVerifyPOSTResponse200)
    )
)]
pub async fn post_auth_credentials_code_verify(
    Json(request): Json<serde_json::Value>,
) -> Json<generated::auth::CredentialsCodeVerifyPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsCodeVerifyPOSTResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/credentials/deactivate",
    request_body = generated::auth::CredentialsDeactivatePUTRequest,
    responses(
        (status = 200, body = generated::auth::CredentialsDeactivatePUTResponse200)
    )
)]
pub async fn put_auth_credentials_deactivate(
    Json(request): Json<generated::auth::CredentialsDeactivatePUTRequest>,
) -> Json<generated::auth::CredentialsDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::CredentialsDeactivatePUTResponse200::default())
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
        (status = 200, body = generated::auth::LoginCodePOSTResponse200)
    )
)]
pub async fn post_auth_login_code(
    Json(request): Json<generated::auth::LoginCodePOSTRequest>,
) -> Json<generated::auth::LoginCodePOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginCodePOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/delegated",
    request_body = generated::auth::LoginDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginDelegatedPOSTResponse200)
    )
)]
pub async fn post_auth_login_delegated(
    Json(request): Json<generated::auth::LoginDelegatedPOSTRequest>,
) -> Json<generated::auth::LoginDelegatedPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginDelegatedPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/init",
    request_body = generated::auth::LoginInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginInitPOSTResponse200)
    )
)]
pub async fn post_auth_login_init(
    Json(request): Json<generated::auth::LoginInitPOSTRequest>,
) -> Json<generated::auth::LoginInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginInitPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/social",
    request_body = generated::auth::LoginSocialPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSocialPOSTResponse200)
    )
)]
pub async fn post_auth_login_social(
    Json(request): Json<generated::auth::LoginSocialPOSTRequest>,
) -> Json<generated::auth::LoginSocialPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginSocialPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/sso",
    request_body = generated::auth::LoginSsoPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSsoPOSTResponse200)
    )
)]
pub async fn post_auth_login_sso(
    Json(request): Json<generated::auth::LoginSsoPOSTRequest>,
) -> Json<generated::auth::LoginSsoPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginSsoPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/login/sso/init",
    request_body = generated::auth::LoginSsoInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::LoginSsoInitPOSTResponse200)
    )
)]
pub async fn post_auth_login_sso_init(
    Json(request): Json<generated::auth::LoginSsoInitPOSTRequest>,
) -> Json<generated::auth::LoginSsoInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LoginSsoInitPOSTResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/logout",
    request_body = generated::auth::LogoutPUTRequest,
    responses(
        (status = 200, body = generated::auth::LogoutPUTResponse200)
    )
)]
pub async fn put_auth_logout(
    Json(request): Json<generated::auth::LogoutPUTRequest>,
) -> Json<generated::auth::LogoutPUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::LogoutPUTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/auth/pats",
    responses(
        (status = 200, body = generated::auth::PatsGETResponse200)
    )
)]
pub async fn get_auth_pats(
) -> Json<generated::auth::PatsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/pats",
    request_body = generated::auth::PatsPOSTRequest,
    responses(
        (status = 200, body = generated::auth::PatsPOSTResponse200)
    )
)]
pub async fn post_auth_pats(
    Json(request): Json<generated::auth::PatsPOSTRequest>,
) -> Json<generated::auth::PatsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdGETResponse200)
    )
)]
pub async fn get_auth_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdGETResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    request_body = generated::auth::PatsTokenIdPUTRequest,
    responses(
        (status = 200, body = generated::auth::PatsTokenIdPUTResponse200)
    )
)]
pub async fn put_auth_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
    Json(request): Json<generated::auth::PatsTokenIdPUTRequest>,
) -> Json<generated::auth::PatsTokenIdPUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdPUTResponse200::default())
}

#[utoipa::path(
    delete,
    path = "/auth/pats/{tokenId}",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdDELETEResponse200)
    )
)]
pub async fn delete_auth_pats_token_id(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdDELETEResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}/activate",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdActivatePUTResponse200)
    )
)]
pub async fn put_auth_pats_token_id_activate(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdActivatePUTResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/pats/{tokenId}/deactivate",
    params(
        ("tokenId" = String, Path, description = "tokenId")
    ),
    responses(
        (status = 200, body = generated::auth::PatsTokenIdDeactivatePUTResponse200)
    )
)]
pub async fn put_auth_pats_token_id_deactivate(
    AxumPath(tokenId): AxumPath<String>,
) -> Json<generated::auth::PatsTokenIdDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::PatsTokenIdDeactivatePUTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/recover/user",
    request_body = generated::auth::RecoverUserPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserPOSTResponse200)
    )
)]
pub async fn post_auth_recover_user(
    Json(request): Json<generated::auth::RecoverUserPOSTRequest>,
) -> Json<generated::auth::RecoverUserPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RecoverUserPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/code",
    request_body = generated::auth::RecoverUserCodePOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserCodePOSTResponse200)
    )
)]
pub async fn post_auth_recover_user_code(
    Json(request): Json<generated::auth::RecoverUserCodePOSTRequest>,
) -> Json<generated::auth::RecoverUserCodePOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RecoverUserCodePOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/delegated",
    request_body = generated::auth::RecoverUserDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserDelegatedPOSTResponse200)
    )
)]
pub async fn post_auth_recover_user_delegated(
    Json(request): Json<generated::auth::RecoverUserDelegatedPOSTRequest>,
) -> Json<generated::auth::RecoverUserDelegatedPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RecoverUserDelegatedPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/recover/user/init",
    request_body = generated::auth::RecoverUserInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RecoverUserInitPOSTResponse200)
    )
)]
pub async fn post_auth_recover_user_init(
    Json(request): Json<generated::auth::RecoverUserInitPOSTRequest>,
) -> Json<generated::auth::RecoverUserInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RecoverUserInitPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration",
    request_body = generated::auth::RegistrationPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationPOSTResponse200)
    )
)]
pub async fn post_auth_registration(
    Json(request): Json<generated::auth::RegistrationPOSTRequest>,
) -> Json<generated::auth::RegistrationPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationPOSTResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/registration/code",
    request_body = generated::auth::RegistrationCodePUTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationCodePUTResponse200)
    )
)]
pub async fn put_auth_registration_code(
    Json(request): Json<generated::auth::RegistrationCodePUTRequest>,
) -> Json<generated::auth::RegistrationCodePUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationCodePUTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration/delegated",
    request_body = generated::auth::RegistrationDelegatedPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationDelegatedPOSTResponse200)
    )
)]
pub async fn post_auth_registration_delegated(
    Json(request): Json<generated::auth::RegistrationDelegatedPOSTRequest>,
) -> Json<generated::auth::RegistrationDelegatedPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationDelegatedPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration/enduser",
    request_body = generated::auth::RegistrationEnduserPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationEnduserPOSTResponse200)
    )
)]
pub async fn post_auth_registration_enduser(
    Json(request): Json<generated::auth::RegistrationEnduserPOSTRequest>,
) -> Json<generated::auth::RegistrationEnduserPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationEnduserPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration/init",
    request_body = generated::auth::RegistrationInitPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationInitPOSTResponse200)
    )
)]
pub async fn post_auth_registration_init(
    Json(request): Json<generated::auth::RegistrationInitPOSTRequest>,
) -> Json<generated::auth::RegistrationInitPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationInitPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/registration/social",
    request_body = generated::auth::RegistrationSocialPOSTRequest,
    responses(
        (status = 200, body = generated::auth::RegistrationSocialPOSTResponse200)
    )
)]
pub async fn post_auth_registration_social(
    Json(request): Json<generated::auth::RegistrationSocialPOSTRequest>,
) -> Json<generated::auth::RegistrationSocialPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::RegistrationSocialPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/auth/service-accounts",
    responses(
        (status = 200, body = generated::auth::ServiceAccountsGETResponse200)
    )
)]
pub async fn get_auth_service_accounts(
) -> Json<generated::auth::ServiceAccountsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/auth/service-accounts",
    request_body = generated::auth::ServiceAccountsPOSTRequest,
    responses(
        (status = 200, body = generated::auth::ServiceAccountsPOSTResponse200)
    )
)]
pub async fn post_auth_service_accounts(
    Json(request): Json<generated::auth::ServiceAccountsPOSTRequest>,
) -> Json<generated::auth::ServiceAccountsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdGETResponse200)
    )
)]
pub async fn get_auth_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdGETResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    request_body = generated::auth::ServiceAccountsServiceAccountIdPUTRequest,
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdPUTResponse200)
    )
)]
pub async fn put_auth_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
    Json(request): Json<generated::auth::ServiceAccountsServiceAccountIdPUTRequest>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdPUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdPUTResponse200::default())
}

#[utoipa::path(
    delete,
    path = "/auth/service-accounts/{serviceAccountId}",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdDELETEResponse200)
    )
)]
pub async fn delete_auth_service_accounts_service_account_id(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdDELETEResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}/activate",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse200)
    )
)]
pub async fn put_auth_service_accounts_service_account_id_activate(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdActivatePUTResponse200::default())
}

#[utoipa::path(
    put,
    path = "/auth/service-accounts/{serviceAccountId}/deactivate",
    params(
        ("serviceAccountId" = String, Path, description = "serviceAccountId")
    ),
    responses(
        (status = 200, body = generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse200)
    )
)]
pub async fn put_auth_service_accounts_service_account_id_deactivate(
    AxumPath(serviceAccountId): AxumPath<String>,
) -> Json<generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::ServiceAccountsServiceAccountIdDeactivatePUTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/auth/users",
    responses(
        (status = 200, body = generated::auth::UsersGETResponse200)
    )
)]
pub async fn get_auth_users(
) -> Json<generated::auth::UsersGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::auth::UsersGETResponse200::default())
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
        (status = 200, body = generated::exchanges::ExchangesGETResponse200)
    )
)]
pub async fn get_exchanges(
) -> Json<generated::exchanges::ExchangesGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangesGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/exchanges",
    request_body = generated::exchanges::ExchangesPOSTRequest,
    responses(
        (status = 200, body = generated::exchanges::ExchangesPOSTResponse200)
    )
)]
pub async fn post_exchanges(
    Json(request): Json<generated::exchanges::ExchangesPOSTRequest>,
) -> Json<generated::exchanges::ExchangesPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangesPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdGETResponse200)
    )
)]
pub async fn get_exchanges_exchange_id(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdGETResponse200::default())
}

#[utoipa::path(
    delete,
    path = "/exchanges/{exchangeId}",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdDELETEResponse200)
    )
)]
pub async fn delete_exchanges_exchange_id(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdDELETEResponse200::default())
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}/accounts",
    params(
        ("exchangeId" = String, Path, description = "exchangeId")
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsGETResponse200)
    )
)]
pub async fn get_exchanges_exchange_id_accounts(
    AxumPath(exchangeId): AxumPath<String>,
) -> Json<generated::exchanges::ExchangeIdAccountsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdAccountsGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/exchanges/{exchangeId}/accounts/{accountId}/assets",
    params(
        ("exchangeId" = String, Path, description = "exchangeId"),
        ("accountId" = String, Path, description = "accountId")
    ),
    responses(
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse200)
    )
)]
pub async fn get_exchanges_exchange_id_accounts_account_id_assets(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdAccountsAccountIdAssetsGETResponse200::default())
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
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse200)
    )
)]
pub async fn post_exchanges_exchange_id_accounts_account_id_deposits(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdAccountsAccountIdDepositsPOSTResponse200::default())
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
        (status = 200, body = generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200)
    )
)]
pub async fn post_exchanges_exchange_id_accounts_account_id_withdrawals(
    AxumPath((exchangeId, accountId)): AxumPath<(String, String)>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::exchanges::ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/fee-sponsors",
    responses(
        (status = 200, body = generated::feesponsors::FeesponsorsGETResponse200)
    )
)]
pub async fn get_feesponsors(
) -> Json<generated::feesponsors::FeesponsorsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::feesponsors::FeesponsorsGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/fee-sponsors",
    request_body = generated::feesponsors::FeesponsorsPOSTRequest,
    responses(
        (status = 200, body = generated::feesponsors::FeesponsorsPOSTResponse200)
    )
)]
pub async fn post_feesponsors(
    Json(request): Json<generated::feesponsors::FeesponsorsPOSTRequest>,
) -> Json<generated::feesponsors::FeesponsorsPOSTResponse200> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "dateCreated": "2023-04-14T20:41:28.715Z"
        }
    );
    let response: generated::feesponsors::FeesponsorsPOSTResponse200 = serde_json::from_value(example_json)
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
        (status = 200, body = generated::feesponsors::FeeSponsorIdGETResponse200)
    )
)]
pub async fn get_feesponsors_fee_sponsor_id(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdGETResponse200> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "dateCreated": "2023-04-14T20:41:28.715Z"
        }
    );
    let response: generated::feesponsors::FeeSponsorIdGETResponse200 = serde_json::from_value(example_json)
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
        (status = 200, body = generated::feesponsors::FeeSponsorIdDELETEResponse200)
    )
)]
pub async fn delete_feesponsors_fee_sponsor_id(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "dateCreated": "2023-04-14T20:41:28.715Z"
        }
    );
    let response: generated::feesponsors::FeeSponsorIdDELETEResponse200 = serde_json::from_value(example_json)
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
        (status = 200, body = generated::feesponsors::FeeSponsorIdActivatePUTResponse200)
    )
)]
pub async fn put_feesponsors_fee_sponsor_id_activate(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdActivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "dateCreated": "2023-04-14T20:41:28.715Z"
        }
    );
    let response: generated::feesponsors::FeeSponsorIdActivatePUTResponse200 = serde_json::from_value(example_json)
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
        (status = 200, body = generated::feesponsors::FeeSponsorIdDeactivatePUTResponse200)
    )
)]
pub async fn put_feesponsors_fee_sponsor_id_deactivate(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdDeactivatePUTResponse200> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "id": "fs-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "walletId": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Solana",
          "status": "Active",
          "dateCreated": "2023-04-14T20:41:28.715Z"
        }
    );
    let response: generated::feesponsors::FeeSponsorIdDeactivatePUTResponse200 = serde_json::from_value(example_json)
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
        (status = 200, body = generated::feesponsors::FeeSponsorIdFeesGETResponse200)
    )
)]
pub async fn get_feesponsors_fee_sponsor_id_fees(
    AxumPath(feeSponsorId): AxumPath<String>,
) -> Json<generated::feesponsors::FeeSponsorIdFeesGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::feesponsors::FeeSponsorIdFeesGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/key-stores",
    responses(
        (status = 200, body = generated::keystores::KeystoresGETResponse200)
    )
)]
pub async fn get_keystores(
) -> Json<generated::keystores::KeystoresGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keystores::KeystoresGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/keys",
    responses(
        (status = 200, body = generated::keys::KeysGETResponse200)
    )
)]
pub async fn get_keys(
) -> Json<generated::keys::KeysGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeysGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/keys",
    request_body = generated::keys::KeysPOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeysPOSTResponse200)
    )
)]
pub async fn post_keys(
    Json(request): Json<generated::keys::KeysPOSTRequest>,
) -> Json<generated::keys::KeysPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeysPOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/keys/import",
    request_body = generated::keys::ImportPOSTRequest,
    responses(
        (status = 200, body = generated::keys::ImportPOSTResponse200)
    )
)]
pub async fn post_keys_import(
    Json(request): Json<generated::keys::ImportPOSTRequest>,
) -> Json<generated::keys::ImportPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::ImportPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdGETResponse200)
    )
)]
pub async fn get_keys_key_id(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdGETResponse200::default())
}

#[utoipa::path(
    put,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = generated::keys::KeyIdPUTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdPUTResponse200)
    )
)]
pub async fn put_keys_key_id(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdPUTRequest>,
) -> Json<generated::keys::KeyIdPUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdPUTResponse200::default())
}

#[utoipa::path(
    delete,
    path = "/keys/{keyId}",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdDELETEResponse200)
    )
)]
pub async fn delete_keys_key_id(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdDELETEResponse200::default())
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/delegate",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = generated::keys::KeyIdDelegatePOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdDelegatePOSTResponse200)
    )
)]
pub async fn post_keys_key_id_delegate(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdDelegatePOSTRequest>,
) -> Json<generated::keys::KeyIdDelegatePOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdDelegatePOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/derive",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = generated::keys::KeyIdDerivePOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdDerivePOSTResponse200)
    )
)]
pub async fn post_keys_key_id_derive(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdDerivePOSTRequest>,
) -> Json<generated::keys::KeyIdDerivePOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdDerivePOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/export",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = generated::keys::KeyIdExportPOSTRequest,
    responses(
        (status = 200, body = generated::keys::KeyIdExportPOSTResponse200)
    )
)]
pub async fn post_keys_key_id_export(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<generated::keys::KeyIdExportPOSTRequest>,
) -> Json<generated::keys::KeyIdExportPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdExportPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}/signatures",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesGETResponse200)
    )
)]
pub async fn get_keys_key_id_signatures(
    AxumPath(keyId): AxumPath<String>,
) -> Json<generated::keys::KeyIdSignaturesGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdSignaturesGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/keys/{keyId}/signatures",
    params(
        ("keyId" = String, Path, description = "keyId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesPOSTResponse200)
    )
)]
pub async fn post_keys_key_id_signatures(
    AxumPath(keyId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::keys::KeyIdSignaturesPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdSignaturesPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/keys/{keyId}/signatures/{signatureId}",
    params(
        ("keyId" = String, Path, description = "keyId"),
        ("signatureId" = String, Path, description = "signatureId")
    ),
    responses(
        (status = 200, body = generated::keys::KeyIdSignaturesSignatureIdGETResponse200)
    )
)]
pub async fn get_keys_key_id_signatures_signature_id(
    AxumPath((keyId, signatureId)): AxumPath<(String, String)>,
) -> Json<generated::keys::KeyIdSignaturesSignatureIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::keys::KeyIdSignaturesSignatureIdGETResponse200::default())
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
        (status = 200, body = generated::networks::ReadContractPOSTResponse200)
    )
)]
pub async fn post_networks_read_contract(
    Json(request): Json<serde_json::Value>,
) -> Json<generated::networks::ReadContractPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::networks::ReadContractPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/networks/{network}/validators",
    params(
        ("network" = String, Path, description = "network")
    ),
    responses(
        (status = 200, body = generated::networks::NetworkValidatorsGETResponse200)
    )
)]
pub async fn get_networks_network_validators(
    AxumPath(network): AxumPath<String>,
) -> Json<generated::networks::NetworkValidatorsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::networks::NetworkValidatorsGETResponse200::default())
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
        (status = 200, body = generated::permissions::PermissionsGETResponse200)
    )
)]
pub async fn get_permissions(
) -> Json<generated::permissions::PermissionsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionsGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/permissions",
    request_body = generated::permissions::PermissionsPOSTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionsPOSTResponse200)
    )
)]
pub async fn post_permissions(
    Json(request): Json<generated::permissions::PermissionsPOSTRequest>,
) -> Json<generated::permissions::PermissionsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionsPOSTResponse200::default())
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
        (status = 200, body = generated::permissions::PermissionIdPUTResponse200)
    )
)]
pub async fn put_permissions_permission_id(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdPUTRequest>,
) -> Json<generated::permissions::PermissionIdPUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionIdPUTResponse200::default())
}

#[utoipa::path(
    put,
    path = "/permissions/{permissionId}/archive",
    params(
        ("permissionId" = String, Path, description = "permissionId")
    ),
    request_body = generated::permissions::PermissionIdArchivePUTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionIdArchivePUTResponse200)
    )
)]
pub async fn put_permissions_permission_id_archive(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdArchivePUTRequest>,
) -> Json<generated::permissions::PermissionIdArchivePUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionIdArchivePUTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/permissions/{permissionId}/assignments",
    params(
        ("permissionId" = String, Path, description = "permissionId")
    ),
    responses(
        (status = 200, body = generated::permissions::PermissionIdAssignmentsGETResponse200)
    )
)]
pub async fn get_permissions_permission_id_assignments(
    AxumPath(permissionId): AxumPath<String>,
) -> Json<generated::permissions::PermissionIdAssignmentsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionIdAssignmentsGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/permissions/{permissionId}/assignments",
    params(
        ("permissionId" = String, Path, description = "permissionId")
    ),
    request_body = generated::permissions::PermissionIdAssignmentsPOSTRequest,
    responses(
        (status = 200, body = generated::permissions::PermissionIdAssignmentsPOSTResponse200)
    )
)]
pub async fn post_permissions_permission_id_assignments(
    AxumPath(permissionId): AxumPath<String>,
    Json(request): Json<generated::permissions::PermissionIdAssignmentsPOSTRequest>,
) -> Json<generated::permissions::PermissionIdAssignmentsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::permissions::PermissionIdAssignmentsPOSTResponse200::default())
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
        (status = 200, body = generated::signers::SignersGETResponse200)
    )
)]
pub async fn get_signers(
) -> Json<generated::signers::SignersGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::signers::SignersGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/staking/stakes",
    responses(
        (status = 200, body = generated::staking::StakesGETResponse200)
    )
)]
pub async fn get_staking_stakes(
) -> Json<generated::staking::StakesGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::staking::StakesGETResponse200::default())
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
        (status = 200, body = generated::staking::StakesStakeIdActionsGETResponse200)
    )
)]
pub async fn get_staking_stakes_stake_id_actions(
    AxumPath(stakeId): AxumPath<String>,
) -> Json<generated::staking::StakesStakeIdActionsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::staking::StakesStakeIdActionsGETResponse200::default())
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
        (status = 200, body = generated::staking::StakesStakeIdRewardsGETResponse200)
    )
)]
pub async fn get_staking_stakes_stake_id_rewards(
    AxumPath(stakeId): AxumPath<String>,
) -> Json<generated::staking::StakesStakeIdRewardsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::staking::StakesStakeIdRewardsGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/swaps",
    responses(
        (status = 200, body = generated::swaps::SwapsGETResponse200)
    )
)]
pub async fn get_swaps(
) -> Json<generated::swaps::SwapsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::swaps::SwapsGETResponse200::default())
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
          "id": "swap-6a3ku-bn8d7-8u5rs1oukojms7k8",
          "quoteId": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
          "reference": null,
          "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "status": "PendingPolicyApproval",
          "provider": "UniswapClassic",
          "quotedSourceAsset": {
            "kind": "Native",
            "amount": "10000000000",
            "metadata": {
              "network": "EthereumSepolia",
              "name": "Ethereum",
              "symbol": "SepoliaETH",
              "decimals": 18,
              "tid": "native:eth"
            }
          },
          "quotedTargetAsset": {
            "kind": "Erc20",
            "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
            "amount": "6467571553831928182",
            "metadata": {
              "network": "EthereumSepolia",
              "name": "Test",
              "symbol": "Test",
              "decimals": 18,
              "tid": "erc20:0xda0be7efd234295395d4204d0df4358339b57b27"
            }
          },
          "slippageBps": 100,
          "dateCreated": "2025-09-11T10:57:55.758Z",
          "requestBody": {
            "quoteId": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
            "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "provider": "UniswapClassic",
            "slippageBps": 100,
            "sourceAsset": {
              "kind": "Native",
              "amount": "10000000000"
            },
            "targetAsset": {
              "kind": "Erc20",
              "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
              "amount": "653003161"
            }
          },
          "requester": {
            "userId": "us-48r5q-eshfg-9pmr2lo6bmpr4i4i",
            "tokenId": "to-4etah-smoal-9n3rmhul4dpaueg5"
          }
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
          "id": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
          "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "provider": "UniswapClassic",
          "sourceAsset": {
            "kind": "Native",
            "amount": "10000000000",
            "metadata": {
              "network": "EthereumSepolia",
              "name": "Ethereum",
              "symbol": "SepoliaETH",
              "decimals": 18,
              "tid": "native:eth"
            }
          },
          "targetAsset": {
            "kind": "Erc20",
            "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
            "amount": "6467571553831928182",
            "metadata": {
              "network": "EthereumSepolia",
              "name": "Test",
              "symbol": "Test",
              "decimals": 18,
              "tid": "erc20:0xda0be7efd234295395d4204d0df4358339b57b27"
            }
          },
          "slippageBps": 100,
          "dateCreated": "2025-09-11T10:52:15.039Z",
          "requestBody": {
            "provider": "UniswapClassic",
            "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "sourceAsset": {
              "kind": "Native",
              "amount": "10000000000"
            },
            "targetAsset": {
              "kind": "Erc20",
              "contract": "0xda0be7efd234295395d4204d0df4358339b57b27"
            },
            "slippageBps": 100
          },
          "requester": {
            "userId": "us-48r5q-eshfg-9pmr2lo6bmpr4i4i",
            "tokenId": "to-4etah-smoal-9n3rmhul4dpaueg5"
          }
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
          "id": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
          "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "provider": "UniswapClassic",
          "sourceAsset": {
            "kind": "Native",
            "amount": "10000000000",
            "metadata": {
              "network": "EthereumSepolia",
              "name": "Ethereum",
              "symbol": "SepoliaETH",
              "decimals": 18,
              "tid": "native:eth"
            }
          },
          "targetAsset": {
            "kind": "Erc20",
            "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
            "amount": "6467571553831928182",
            "metadata": {
              "network": "EthereumSepolia",
              "name": "Test",
              "symbol": "Test",
              "decimals": 18,
              "tid": "erc20:0xda0be7efd234295395d4204d0df4358339b57b27"
            }
          },
          "slippageBps": 100,
          "dateCreated": "2025-09-11T10:52:15.039Z",
          "requestBody": {
            "provider": "UniswapClassic",
            "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "sourceAsset": {
              "kind": "Native",
              "amount": "10000000000"
            },
            "targetAsset": {
              "kind": "Erc20",
              "contract": "0xda0be7efd234295395d4204d0df4358339b57b27"
            },
            "slippageBps": 100
          },
          "requester": {
            "userId": "us-48r5q-eshfg-9pmr2lo6bmpr4i4i",
            "tokenId": "to-4etah-smoal-9n3rmhul4dpaueg5"
          }
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
          "id": "swap-6a3ku-bn8d7-8u5rs1oukojms7k8",
          "quoteId": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
          "reference": null,
          "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
          "status": "PendingPolicyApproval",
          "provider": "UniswapClassic",
          "quotedSourceAsset": {
            "kind": "Native",
            "amount": "10000000000",
            "metadata": {
              "network": "EthereumSepolia",
              "name": "Ethereum",
              "symbol": "SepoliaETH",
              "decimals": 18,
              "tid": "native:eth"
            }
          },
          "quotedTargetAsset": {
            "kind": "Erc20",
            "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
            "amount": "6467571553831928182",
            "metadata": {
              "network": "EthereumSepolia",
              "name": "Test",
              "symbol": "Test",
              "decimals": 18,
              "tid": "erc20:0xda0be7efd234295395d4204d0df4358339b57b27"
            }
          },
          "slippageBps": 100,
          "dateCreated": "2025-09-11T10:57:55.758Z",
          "requestBody": {
            "quoteId": "swapQuote-3hgv4-q8tbf-8v2ajmrbmg6m4i9t",
            "walletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "targetWalletId": "wa-3l3cj-l8mq7-8q78f9nopq7f1qjv",
            "provider": "UniswapClassic",
            "slippageBps": 100,
            "sourceAsset": {
              "kind": "Native",
              "amount": "10000000000"
            },
            "targetAsset": {
              "kind": "Erc20",
              "contract": "0xda0be7efd234295395d4204d0df4358339b57b27",
              "amount": "653003161"
            }
          },
          "requester": {
            "userId": "us-48r5q-eshfg-9pmr2lo6bmpr4i4i",
            "tokenId": "to-4etah-smoal-9n3rmhul4dpaueg5"
          }
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
        (status = 200, body = generated::v2::PoliciesGETResponse200)
    )
)]
pub async fn get_v2_policies(
) -> Json<generated::v2::PoliciesGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::v2::PoliciesGETResponse200::default())
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
        (status = 200, body = generated::v2::PolicyApprovalsGETResponse200)
    )
)]
pub async fn get_v2_policy_approvals(
) -> Json<generated::v2::PolicyApprovalsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::v2::PolicyApprovalsGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/v2/policy-approvals/{approvalId}",
    params(
        ("approvalId" = String, Path, description = "approvalId")
    ),
    responses(
        (status = 200, body = generated::v2::PolicyApprovalsApprovalIdGETResponse200)
    )
)]
pub async fn get_v2_policy_approvals_approval_id(
    AxumPath(approvalId): AxumPath<String>,
) -> Json<generated::v2::PolicyApprovalsApprovalIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::v2::PolicyApprovalsApprovalIdGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/v2/policy-approvals/{approvalId}/decisions",
    params(
        ("approvalId" = String, Path, description = "approvalId")
    ),
    request_body = generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTRequest,
    responses(
        (status = 200, body = generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse200)
    )
)]
pub async fn post_v2_policy_approvals_approval_id_decisions(
    AxumPath(approvalId): AxumPath<String>,
    Json(request): Json<generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTRequest>,
) -> Json<generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::v2::PolicyApprovalsApprovalIdDecisionsPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/wallets",
    responses(
        (status = 200, body = generated::wallets::WalletsGETResponse200)
    )
)]
pub async fn get_wallets(
) -> Json<generated::wallets::WalletsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletsGETResponse200::default())
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
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::Wallet)
    )
)]
pub async fn post_wallets_import(
    Json(request): Json<serde_json::Value>,
) -> Json<generated::Wallet> {
    // TODO: Replace with actual implementation
    let example_json = serde_json::json!(
        {
          "id": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Ethereum",
          "address": "0x00e3495cf6af59008f22ffaf32d4c92ac33dac47",
          "name": "trading hot wallet",
          "signingKey": {
            "id": "key-6ece3-9l565-xxxxxxxxxxxxxxxx",
            "scheme": "ECDSA",
            "curve": "secp256k1",
            "publicKey": "e2375c8c9e87bfcd0be8f29d76c818cabacd51584f72cb2222d49a13b036d84d3d"
          },
          "status": "Active",
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "custodial": true,
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
          "id": "wa-1f04s-lqc9q-xxxxxxxxxxxxxxxx",
          "network": "Ethereum",
          "address": "0x00e3495cf6af59008f22ffaf32d4c92ac33dac47",
          "name": "trading hot wallet",
          "signingKey": {
            "id": "key-6ece3-9l565-xxxxxxxxxxxxxxxx",
            "scheme": "ECDSA",
            "curve": "secp256k1",
            "publicKey": "e2375c8c9e87bfcd0be8f29d76c818cabacd51584f72cb2222d49a13b036d84d3d"
          },
          "status": "Active",
          "dateCreated": "2023-04-14T20:41:28.715Z",
          "custodial": true,
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
        (status = 200, body = generated::wallets::WalletIdAssetsGETResponse200)
    )
)]
pub async fn get_wallets_wallet_id_assets(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdAssetsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdAssetsGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/delegate",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = generated::wallets::WalletIdDelegatePOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdDelegatePOSTResponse200)
    )
)]
pub async fn post_wallets_wallet_id_delegate(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdDelegatePOSTRequest>,
) -> Json<generated::wallets::WalletIdDelegatePOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdDelegatePOSTResponse200::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/export",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = generated::wallets::WalletIdExportPOSTRequest,
    responses(
        (status = 200, body = generated::wallets::WalletIdExportPOSTResponse200)
    )
)]
pub async fn post_wallets_wallet_id_export(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<generated::wallets::WalletIdExportPOSTRequest>,
) -> Json<generated::wallets::WalletIdExportPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdExportPOSTResponse200::default())
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
        (status = 200, body = generated::wallets::WalletIdNftsGETResponse200)
    )
)]
pub async fn get_wallets_wallet_id_nfts(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdNftsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdNftsGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/offers",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdOffersGETResponse200)
    )
)]
pub async fn get_wallets_wallet_id_offers(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdOffersGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdOffersGETResponse200::default())
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
        (status = 200, body = generated::wallets::WalletIdSignaturesGETResponse200)
    )
)]
pub async fn get_wallets_wallet_id_signatures(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdSignaturesGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdSignaturesGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/signatures",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::wallets::WalletIdSignaturesPOSTResponse200)
    )
)]
pub async fn post_wallets_wallet_id_signatures(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::wallets::WalletIdSignaturesPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdSignaturesPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/signatures/{signatureId}",
    params(
        ("walletId" = String, Path, description = "walletId"),
        ("signatureId" = String, Path, description = "signatureId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdSignaturesSignatureIdGETResponse200)
    )
)]
pub async fn get_wallets_wallet_id_signatures_signature_id(
    AxumPath((walletId, signatureId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdSignaturesSignatureIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdSignaturesSignatureIdGETResponse200::default())
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
        (status = 200, body = generated::wallets::WalletIdTransactionsGETResponse200)
    )
)]
pub async fn get_wallets_wallet_id_transactions(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdTransactionsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdTransactionsGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/wallets/{walletId}/transactions",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, body = generated::wallets::WalletIdTransactionsPOSTResponse200)
    )
)]
pub async fn post_wallets_wallet_id_transactions(
    AxumPath(walletId): AxumPath<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<generated::wallets::WalletIdTransactionsPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdTransactionsPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transactions/{transactionId}",
    params(
        ("walletId" = String, Path, description = "walletId"),
        ("transactionId" = String, Path, description = "transactionId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransactionsTransactionIdGETResponse200)
    )
)]
pub async fn get_wallets_wallet_id_transactions_transaction_id(
    AxumPath((walletId, transactionId)): AxumPath<(String, String)>,
) -> Json<generated::wallets::WalletIdTransactionsTransactionIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdTransactionsTransactionIdGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/wallets/{walletId}/transfers",
    params(
        ("walletId" = String, Path, description = "walletId")
    ),
    responses(
        (status = 200, body = generated::wallets::WalletIdTransfersGETResponse200)
    )
)]
pub async fn get_wallets_wallet_id_transfers(
    AxumPath(walletId): AxumPath<String>,
) -> Json<generated::wallets::WalletIdTransfersGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::wallets::WalletIdTransfersGETResponse200::default())
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
        (status = 200, body = generated::webhooks::WebhooksGETResponse200)
    )
)]
pub async fn get_webhooks(
) -> Json<generated::webhooks::WebhooksGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhooksGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/webhooks",
    request_body = generated::webhooks::WebhooksPOSTRequest,
    responses(
        (status = 200, body = generated::webhooks::WebhooksPOSTResponse200)
    )
)]
pub async fn post_webhooks(
    Json(request): Json<generated::webhooks::WebhooksPOSTRequest>,
) -> Json<generated::webhooks::WebhooksPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhooksPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdGETResponse200)
    )
)]
pub async fn get_webhooks_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdGETResponse200::default())
}

#[utoipa::path(
    put,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    request_body = generated::webhooks::WebhookIdPUTRequest,
    responses(
        (status = 200, body = generated::webhooks::WebhookIdPUTResponse200)
    )
)]
pub async fn put_webhooks_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
    Json(request): Json<generated::webhooks::WebhookIdPUTRequest>,
) -> Json<generated::webhooks::WebhookIdPUTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdPUTResponse200::default())
}

#[utoipa::path(
    delete,
    path = "/webhooks/{webhookId}",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdDELETEResponse200)
    )
)]
pub async fn delete_webhooks_webhook_id(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdDELETEResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdDELETEResponse200::default())
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}/events",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdEventsGETResponse200)
    )
)]
pub async fn get_webhooks_webhook_id_events(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdEventsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdEventsGETResponse200::default())
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhookId}/events/{webhookEventId}",
    params(
        ("webhookId" = String, Path, description = "webhookId"),
        ("webhookEventId" = String, Path, description = "webhookEventId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse200)
    )
)]
pub async fn get_webhooks_webhook_id_events_webhook_event_id(
    AxumPath((webhookId, webhookEventId)): AxumPath<(String, String)>,
) -> Json<generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdEventsWebhookEventIdGETResponse200::default())
}

#[utoipa::path(
    post,
    path = "/webhooks/{webhookId}/ping",
    params(
        ("webhookId" = String, Path, description = "webhookId")
    ),
    responses(
        (status = 200, body = generated::webhooks::WebhookIdPingPOSTResponse200)
    )
)]
pub async fn post_webhooks_webhook_id_ping(
    AxumPath(webhookId): AxumPath<String>,
) -> Json<generated::webhooks::WebhookIdPingPOSTResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::webhooks::WebhookIdPingPOSTResponse200::default())
}

#[utoipa::path(
    get,
    path = "/yields",
    responses(
        (status = 200, body = generated::yields::YieldsGETResponse200)
    )
)]
pub async fn get_yields(
) -> Json<generated::yields::YieldsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::yields::YieldsGETResponse200::default())
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
        (status = 200, body = generated::yields::YieldIdActionsGETResponse200)
    )
)]
pub async fn get_yields_yield_id_actions(
    AxumPath(yieldId): AxumPath<String>,
) -> Json<generated::yields::YieldIdActionsGETResponse200> {
    // TODO: Replace with actual implementation
    Json(generated::yields::YieldIdActionsGETResponse200::default())
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

