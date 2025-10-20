// Auto-generated from OpenAPI schema
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockchainKind {
    Algorand,
    Aptos,
    Bitcoin,
    BitcoinCash,
    Canton,
    Cardano,
    Cosmos,
    Evm,
    Hedera,
    Icp,
    Iota,
    Kadena,
    Kaspa,
    Near,
    Polymesh,
    Solana,
    Stellar,
    Substrate,
    Sui,
    Tezos,
    Ton,
    Tron,
    Xrpl,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CantonValidator {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    pub id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub network: String,
    /// Organization id.
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "partyHint")]
    pub party_hint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Network {
    Algorand,
    AlgorandTestnet,
    Aptos,
    AptosTestnet,
    ArbitrumOne,
    ArbitrumSepolia,
    AvalancheC,
    AvalancheCFuji,
    BabylonGenesis,
    BabylonTestnet5,
    Base,
    BaseSepolia,
    Berachain,
    BerachainBepolia,
    Bitcoin,
    BitcoinSignet,
    BitcoinTestnet3,
    BitcoinCash,
    Bob,
    BobSepolia,
    Bsc,
    BscTestnet,
    Canton,
    CantonTestnet,
    Cardano,
    CardanoPreprod,
    Celo,
    CeloAlfajores,
    Codex,
    CodexSepolia,
    CosmosHub4,
    CosmosIcsTestnet,
    Dogecoin,
    Ethereum,
    EthereumGoerli,
    EthereumSepolia,
    EthereumHolesky,
    EthereumHoodi,
    FantomOpera,
    FantomTestnet,
    FlareC,
    FlareCCoston2,
    Hedera,
    HederaTestnet,
    Ink,
    InkSepolia,
    InternetComputer,
    Ion,
    IonTestnet,
    Iota,
    IotaTestnet,
    KadenaTestnet4,
    Kadena,
    Kaspa,
    Kusama,
    Litecoin,
    Near,
    NearTestnet,
    Optimism,
    OptimismSepolia,
    Origyn,
    Plume,
    PlumeSepolia,
    Polkadot,
    Polygon,
    PolygonAmoy,
    Polymesh,
    PolymeshTestnet,
    Race,
    RaceSepolia,
    SeiAtlantic2,
    SeiPacific1,
    Solana,
    SolanaDevnet,
    Stellar,
    StellarTestnet,
    Sui,
    SuiTestnet,
    Tsc,
    TscTestnet1,
    Tezos,
    TezosGhostnet,
    Ton,
    TonTestnet,
    Tron,
    TronNile,
    Westend,
    XrpLedger,
    XrpLedgerTestnet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub from: String,
    /// Offer id.
    pub id: String,
    pub kind: String,
    pub metadata: serde_json::Value,
    pub network: Network,
    /// Organization id.
    #[serde(rename = "orgId")]
    pub org_id: String,
    pub status: String,
    pub timestamp: String,
    pub to: String,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub value: String,
    /// Wallet id.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Swap {
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the swap was initiated.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Swap id.
    pub id: String,
    /// Swap provider.
    pub provider: String,
    /// Id of the quote this swap is based on.
    #[serde(rename = "quoteId")]
    pub quote_id: String,
    /// The source asset for this swap transaction.
    #[serde(rename = "quotedSourceAsset")]
    pub quoted_source_asset: serde_json::Value,
    /// The target asset for this swap transaction.
    #[serde(rename = "quotedTargetAsset")]
    pub quoted_target_asset: serde_json::Value,
    /// Optional user-defined reference for this Swap.
    pub reference: String,
    /// The full request used for initiating this swap.
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you are willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%.
    #[serde(rename = "slippageBps")]
    pub slippage_bps: f64,
    /// Swap status.
    pub status: String,
    /// Id of the Dfns wallet receiving the target asset. Currently this value must be the same as the `walletId`.
    #[serde(rename = "targetWalletId")]
    pub target_wallet_id: String,
    /// Id of the Dfns wallet spending the sourceAsset.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwapQuote {
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the quote was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// ID of the Swap Quote.
    pub id: String,
    /// Swap provider.
    pub provider: String,
    /// The full request used for obtaining this quote.
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you're willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%.
    #[serde(rename = "slippageBps")]
    pub slippage_bps: f64,
    /// The source asset that will be spent on the swap transaction.
    #[serde(rename = "sourceAsset")]
    pub source_asset: serde_json::Value,
    /// The target asset that will be received with the swap transaction.
    #[serde(rename = "targetAsset")]
    pub target_asset: serde_json::Value,
    /// If not provided, the walletId is used as the target wallet. If provided, this field is currently required to be the same as walletId
    #[serde(rename = "targetWalletId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_wallet_id: Option<String>,
    /// Id of the Dfns wallet spending the sourceAsset.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferRequest {
    #[serde(rename = "approvalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(rename = "dateBroadcasted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_broadcasted: Option<String>,
    #[serde(rename = "dateConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_confirmed: Option<String>,
    #[serde(rename = "datePolicyResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_policy_resolved: Option<String>,
    #[serde(rename = "dateRequested")]
    pub date_requested: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "feeSponsorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_sponsor_id: Option<String>,
    pub id: String,
    pub metadata: serde_json::Value,
    pub network: Network,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    pub status: String,
    #[serde(rename = "txHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "credentialUuid")]
    pub credential_uuid: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isRegistered")]
    pub is_registered: bool,
    #[serde(rename = "isSSORequired")]
    pub is_ssorequired: bool,
    #[serde(rename = "isServiceAccount")]
    pub is_service_account: bool,
    /// User kind.
    pub kind: String,
    pub name: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "permissionAssignments")]
    pub permission_assignments: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// User id.
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    /// Wallet address on its corresponding network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Whether the wallet is owned by an end user (non-custodial), or by your organization (custodial).
    pub custodial: bool,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date string when wallet was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date string when wallet was deleted.
    #[serde(rename = "dateDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_deleted: Option<String>,
    /// User-defined value that can be used to correlate the entity with an external system.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// ID of the wallet.
    pub id: String,
    /// Wallet nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Network this wallet is bound to.
    pub network: String,
    /// Details about the key underlying the wallet.
    #[serde(rename = "signingKey")]
    pub signing_key: serde_json::Value,
    /// Wallet status.
    pub status: String,
    /// List of tags.
    pub tags: Vec<String>,
    /// Id of the validator on which the wallet is created for Canton networks
    #[serde(rename = "validatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<String>,
}

/// A yield investment representing funds deposited to earn interest from a DeFi protocol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Yield {
    /// The total amount currently invested in this yield.
    pub amount: serde_json::Value,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date. When the yield was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Unique identifier for the yield investment.
    pub id: String,
    /// The DeFi protocol used for yield generation. Currently supports OFNS protocol
    pub protocol: String,
    /// The total interest earned so far in this yield.
    pub rewards: serde_json::Value,
    /// Wallet id.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

/// A specific action performed on a yield investment, such as depositing or withdrawing funds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YieldAction {
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date. When the yield action was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// An optional external identifier provided by the client to ensure idempotency and prevent duplicate operations.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Unique identifier for the yield action.
    pub id: String,
    /// The type of action being performed on the yield investment: Deposit to add funds or Withdraw to remove funds.
    pub kind: String,
    /// The full request used for initiating this yield action.
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    /// Status of the yield action. Once initiated, the status will be InProgress, after processing it will be Completed or Failed.
    pub status: String,
    /// Unique identifier for the yield investment.
    #[serde(rename = "yieldId")]
    pub yield_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgreementsAgreementIdAcceptPostResponse200 {
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "dateAccepted")]
    pub date_accepted: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgreementsLatestUnacceptedGetResponse200 {
    #[serde(rename = "latestAgreement")]
    pub latest_agreement: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthActionInitPostRequest {
    #[serde(rename = "userActionHttpMethod")]
    pub user_action_http_method: String,
    #[serde(rename = "userActionHttpPath")]
    pub user_action_http_path: String,
    #[serde(rename = "userActionPayload")]
    pub user_action_payload: String,
    #[serde(rename = "userActionServerKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_action_server_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthActionInitPostResponse200 {
    #[serde(rename = "allowCredentials")]
    pub allow_credentials: serde_json::Value,
    pub attestation: String,
    pub challenge: String,
    #[serde(rename = "challengeIdentifier")]
    pub challenge_identifier: String,
    #[serde(rename = "externalAuthenticationUrl")]
    pub external_authentication_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp: Option<serde_json::Value>,
    #[serde(rename = "supportedCredentialKinds")]
    pub supported_credential_kinds: Vec<serde_json::Value>,
    #[serde(rename = "userVerification")]
    pub user_verification: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthActionLogsIdGetResponse200 {
    pub action: String,
    #[serde(rename = "actionToken")]
    pub action_token: String,
    #[serde(rename = "datePerformed")]
    pub date_performed: String,
    #[serde(rename = "firstFactorCredential")]
    pub first_factor_credential: serde_json::Value,
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthActionPostRequest {
    /// Temporary authentication token returned by the [Create User Action Signature Challenge](https://docs.dfns.co/api-reference/auth/create-user-action-challenge)
    #[serde(rename = "challengeIdentifier")]
    pub challenge_identifier: String,
    /// First factor credential used to sign the user action
    #[serde(rename = "firstFactor")]
    pub first_factor: serde_json::Value,
    /// Second factor credential used to authenticate a user
    #[serde(rename = "secondFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_factor: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthActionPostResponse200 {
    #[serde(rename = "userAction")]
    pub user_action: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthAppsAppIdGetResponse200 {
    #[serde(rename = "accessTokens")]
    pub access_tokens: Vec<serde_json::Value>,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "expectedOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_origin: Option<String>,
    #[serde(rename = "expectedRpId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_rp_id: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub kind: String,
    pub name: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "permissionAssignments")]
    pub permission_assignments: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthAppsGetResponse200 {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsActivatePutRequest {
    #[serde(rename = "credentialUuid")]
    pub credential_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsActivatePutResponse200 {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsCodeInitPostRequest {
    pub code: String,
    #[serde(rename = "credentialKind")]
    pub credential_kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsCodePostRequest {
    /// Code expiration, as an ISO-8601 datetime string or a unix timestamp
    pub expiration: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsCodePostResponse200 {
    pub code: String,
    pub expiration: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsCodeVerifyPostResponse200 {
    #[serde(rename = "credentialId")]
    pub credential_id: String,
    #[serde(rename = "credentialUuid")]
    pub credential_uuid: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub kind: String,
    pub name: String,
    pub origin: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "relyingPartyId")]
    pub relying_party_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsDeactivatePutRequest {
    #[serde(rename = "credentialUuid")]
    pub credential_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsDeactivatePutResponse200 {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsGetResponse200 {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsInitPostRequest {
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentialsPostResponse200 {
    #[serde(rename = "credentialId")]
    pub credential_id: String,
    #[serde(rename = "credentialUuid")]
    pub credential_uuid: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub kind: String,
    pub name: String,
    pub origin: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "relyingPartyId")]
    pub relying_party_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginCodePostRequest {
    #[serde(rename = "orgId")]
    pub org_id: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginCodePostResponse200 {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginDelegatedPostRequest {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginDelegatedPostResponse200 {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginInitPostRequest {
    #[serde(rename = "loginCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_code: Option<String>,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginInitPostResponse200 {
    #[serde(rename = "allowCredentials")]
    pub allow_credentials: serde_json::Value,
    pub attestation: String,
    pub challenge: String,
    #[serde(rename = "challengeIdentifier")]
    pub challenge_identifier: String,
    #[serde(rename = "externalAuthenticationUrl")]
    pub external_authentication_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp: Option<serde_json::Value>,
    #[serde(rename = "supportedCredentialKinds")]
    pub supported_credential_kinds: Vec<serde_json::Value>,
    #[serde(rename = "userVerification")]
    pub user_verification: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginPostRequest {
    /// Temporary authentication token returned by the [Create User Action Signature Challenge](https://docs.dfns.co/api-reference/auth/create-user-action-challenge)
    #[serde(rename = "challengeIdentifier")]
    pub challenge_identifier: String,
    /// First factor credential used to sign the user action
    #[serde(rename = "firstFactor")]
    pub first_factor: serde_json::Value,
    /// Second factor credential used to authenticate a user
    #[serde(rename = "secondFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_factor: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginSocialPostRequest {
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "socialLoginProviderKind")]
    pub social_login_provider_kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginSocialPostResponse200 {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginSsoInitPostRequest {
    /// Client Id obtained from the IdP
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Organization id.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// Redirect URI used for the authentication flow
    #[serde(rename = "redirectUri")]
    pub redirect_uri: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginSsoInitPostResponse200 {
    /// The URL to redirect the user to authenticate with the IdP
    #[serde(rename = "ssoRedirectUrl")]
    pub sso_redirect_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginSsoPostRequest {
    /// Authorization code obtained from the IdP
    pub code: String,
    /// State forwarded by the IdP
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLoginSsoPostResponse200 {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLogoutPutRequest {
    #[serde(rename = "allSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_sessions: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthLogoutPutResponse200 {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsGetResponse200 {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsPostRequest {
    #[serde(rename = "daysValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_valid: Option<i64>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub name: String,
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_id: Option<String>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "secondsValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_valid: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsPostResponse200 {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "credId")]
    pub cred_id: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub kind: String,
    #[serde(rename = "linkedAppId")]
    pub linked_app_id: String,
    #[serde(rename = "linkedUserId")]
    pub linked_user_id: String,
    pub name: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "permissionAssignments")]
    pub permission_assignments: Vec<serde_json::Value>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsTokenIdActivatePutResponse200 {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "credId")]
    pub cred_id: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Access token kind.
    pub kind: String,
    #[serde(rename = "linkedAppId")]
    pub linked_app_id: String,
    #[serde(rename = "linkedUserId")]
    pub linked_user_id: String,
    pub name: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "permissionAssignments")]
    pub permission_assignments: Vec<serde_json::Value>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsTokenIdDeactivatePutResponse200 {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "credId")]
    pub cred_id: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Access token kind.
    pub kind: String,
    #[serde(rename = "linkedAppId")]
    pub linked_app_id: String,
    #[serde(rename = "linkedUserId")]
    pub linked_user_id: String,
    pub name: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "permissionAssignments")]
    pub permission_assignments: Vec<serde_json::Value>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsTokenIdDeleteResponse200 {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "credId")]
    pub cred_id: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Access token kind.
    pub kind: String,
    #[serde(rename = "linkedAppId")]
    pub linked_app_id: String,
    #[serde(rename = "linkedUserId")]
    pub linked_user_id: String,
    pub name: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "permissionAssignments")]
    pub permission_assignments: Vec<serde_json::Value>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsTokenIdGetResponse200 {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "credId")]
    pub cred_id: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Access token kind.
    pub kind: String,
    #[serde(rename = "linkedAppId")]
    pub linked_app_id: String,
    #[serde(rename = "linkedUserId")]
    pub linked_user_id: String,
    pub name: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "permissionAssignments")]
    pub permission_assignments: Vec<serde_json::Value>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsTokenIdPutRequest {
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPatsTokenIdPutResponse200 {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "credId")]
    pub cred_id: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Access token kind.
    pub kind: String,
    #[serde(rename = "linkedAppId")]
    pub linked_app_id: String,
    #[serde(rename = "linkedUserId")]
    pub linked_user_id: String,
    pub name: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "permissionAssignments")]
    pub permission_assignments: Vec<serde_json::Value>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRecoverUserCodePostRequest {
    #[serde(rename = "orgId")]
    pub org_id: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRecoverUserCodePostResponse200 {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRecoverUserDelegatedPostRequest {
    #[serde(rename = "credentialId")]
    pub credential_id: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRecoverUserDelegatedPostResponse200 {
    #[serde(rename = "allowedRecoveryCredentials")]
    pub allowed_recovery_credentials: Vec<serde_json::Value>,
    pub attestation: String,
    #[serde(rename = "authenticatorSelection")]
    pub authenticator_selection: serde_json::Value,
    pub challenge: String,
    #[serde(rename = "excludeCredentials")]
    pub exclude_credentials: Vec<serde_json::Value>,
    #[serde(rename = "otpUrl")]
    pub otp_url: String,
    #[serde(rename = "pubKeyCredParams")]
    pub pub_key_cred_params: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp: Option<serde_json::Value>,
    #[serde(rename = "supportedCredentialKinds")]
    pub supported_credential_kinds: serde_json::Value,
    #[serde(rename = "temporaryAuthenticationToken")]
    pub temporary_authentication_token: String,
    pub user: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRecoverUserInitPostRequest {
    #[serde(rename = "credentialId")]
    pub credential_id: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    pub username: String,
    #[serde(rename = "verificationCode")]
    pub verification_code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRecoverUserInitPostResponse200 {
    #[serde(rename = "allowedRecoveryCredentials")]
    pub allowed_recovery_credentials: Vec<serde_json::Value>,
    pub attestation: String,
    #[serde(rename = "authenticatorSelection")]
    pub authenticator_selection: serde_json::Value,
    pub challenge: String,
    #[serde(rename = "excludeCredentials")]
    pub exclude_credentials: Vec<serde_json::Value>,
    #[serde(rename = "otpUrl")]
    pub otp_url: String,
    #[serde(rename = "pubKeyCredParams")]
    pub pub_key_cred_params: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp: Option<serde_json::Value>,
    #[serde(rename = "supportedCredentialKinds")]
    pub supported_credential_kinds: serde_json::Value,
    #[serde(rename = "temporaryAuthenticationToken")]
    pub temporary_authentication_token: String,
    pub user: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRecoverUserPostRequest {
    #[serde(rename = "newCredentials")]
    pub new_credentials: serde_json::Value,
    pub recovery: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRecoverUserPostResponse200 {
    pub credential: serde_json::Value,
    pub user: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationCodePutRequest {
    #[serde(rename = "orgId")]
    pub org_id: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationCodePutResponse200 {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationDelegatedPostRequest {
    pub email: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationDelegatedPostResponse200 {
    pub attestation: String,
    #[serde(rename = "authenticatorSelection")]
    pub authenticator_selection: serde_json::Value,
    pub challenge: String,
    #[serde(rename = "excludeCredentials")]
    pub exclude_credentials: Vec<serde_json::Value>,
    #[serde(rename = "otpUrl")]
    pub otp_url: String,
    #[serde(rename = "pubKeyCredParams")]
    pub pub_key_cred_params: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp: Option<serde_json::Value>,
    #[serde(rename = "supportedCredentialKinds")]
    pub supported_credential_kinds: serde_json::Value,
    #[serde(rename = "temporaryAuthenticationToken")]
    pub temporary_authentication_token: String,
    pub user: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationEnduserPostRequest {
    #[serde(rename = "firstFactorCredential")]
    pub first_factor_credential: serde_json::Value,
    /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
    #[serde(rename = "recoveryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_credential: Option<serde_json::Value>,
    #[serde(rename = "secondFactorCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_factor_credential: Option<serde_json::Value>,
    pub wallets: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationEnduserPostResponse200 {
    pub authentication: serde_json::Value,
    pub credential: serde_json::Value,
    pub user: serde_json::Value,
    pub wallets: Vec<Wallet>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationInitPostRequest {
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "registrationCode")]
    pub registration_code: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationInitPostResponse200 {
    pub attestation: String,
    #[serde(rename = "authenticatorSelection")]
    pub authenticator_selection: serde_json::Value,
    pub challenge: String,
    #[serde(rename = "excludeCredentials")]
    pub exclude_credentials: Vec<serde_json::Value>,
    #[serde(rename = "otpUrl")]
    pub otp_url: String,
    #[serde(rename = "pubKeyCredParams")]
    pub pub_key_cred_params: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp: Option<serde_json::Value>,
    #[serde(rename = "supportedCredentialKinds")]
    pub supported_credential_kinds: serde_json::Value,
    #[serde(rename = "temporaryAuthenticationToken")]
    pub temporary_authentication_token: String,
    pub user: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationPostRequest {
    #[serde(rename = "firstFactorCredential")]
    pub first_factor_credential: serde_json::Value,
    /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
    #[serde(rename = "recoveryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_credential: Option<serde_json::Value>,
    #[serde(rename = "secondFactorCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_factor_credential: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationPostResponse200 {
    pub credential: serde_json::Value,
    pub user: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationSocialPostRequest {
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "socialLoginProviderKind")]
    pub social_login_provider_kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthRegistrationSocialPostResponse200 {
    pub attestation: String,
    #[serde(rename = "authenticatorSelection")]
    pub authenticator_selection: serde_json::Value,
    pub challenge: String,
    #[serde(rename = "excludeCredentials")]
    pub exclude_credentials: Vec<serde_json::Value>,
    #[serde(rename = "otpUrl")]
    pub otp_url: String,
    #[serde(rename = "pubKeyCredParams")]
    pub pub_key_cred_params: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp: Option<serde_json::Value>,
    #[serde(rename = "supportedCredentialKinds")]
    pub supported_credential_kinds: serde_json::Value,
    #[serde(rename = "temporaryAuthenticationToken")]
    pub temporary_authentication_token: String,
    pub user: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsGetResponse200 {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsPostRequest {
    #[serde(rename = "daysValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_valid: Option<i64>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub name: String,
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_id: Option<String>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsPostResponse200 {
    #[serde(rename = "accessTokens")]
    pub access_tokens: Vec<serde_json::Value>,
    #[serde(rename = "userInfo")]
    pub user_info: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsServiceAccountIdActivatePutResponse200 {
    #[serde(rename = "accessTokens")]
    pub access_tokens: Vec<serde_json::Value>,
    #[serde(rename = "userInfo")]
    pub user_info: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsServiceAccountIdDeactivatePutResponse200 {
    #[serde(rename = "accessTokens")]
    pub access_tokens: Vec<serde_json::Value>,
    #[serde(rename = "userInfo")]
    pub user_info: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsServiceAccountIdDeleteResponse200 {
    #[serde(rename = "accessTokens")]
    pub access_tokens: Vec<serde_json::Value>,
    #[serde(rename = "userInfo")]
    pub user_info: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsServiceAccountIdGetResponse200 {
    #[serde(rename = "accessTokens")]
    pub access_tokens: Vec<serde_json::Value>,
    #[serde(rename = "userInfo")]
    pub user_info: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsServiceAccountIdPutRequest {
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthServiceAccountsServiceAccountIdPutResponse200 {
    #[serde(rename = "accessTokens")]
    pub access_tokens: Vec<serde_json::Value>,
    #[serde(rename = "userInfo")]
    pub user_info: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUsersGetResponse200 {
    pub items: Vec<User>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUsersPostRequest {
    /// The email address of the new user.
    pub email: String,
    /// Value that can be used to correlate the entity with an external system.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// If set to true, the user will have to authenticate via SSO
    #[serde(rename = "isSSORequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ssorequired: Option<bool>,
    /// The kind of user being created. 
    ///       In this endpoint it can only be "`CustomerEmployee`" (creating an "`EndUser`" is done through the [Delegated Registration](https://docs.dfns.co/api-reference/auth/registration-flows#delegated-users-registration-flow) endpoint)
    pub kind: String,
    #[serde(rename = "publicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUsersUserIdPutRequest {
    #[serde(rename = "isSSORequired")]
    pub is_ssorequired: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesExchangeIdAccountsAccountIdAssetsGetResponse200 {
    /// Current page items.
    pub items: Vec<serde_json::Value>,
    /// token to use as `paginationToken` to request the next page.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesExchangeIdAccountsAccountIdDepositsPostResponse200 {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "exchangeId")]
    pub exchange_id: String,
    #[serde(rename = "exchangeReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_reference: Option<String>,
    pub id: String,
    pub kind: String,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    #[serde(rename = "transferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesExchangeIdAccountsAccountIdWithdrawalsPostResponse200 {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "exchangeId")]
    pub exchange_id: String,
    #[serde(rename = "exchangeReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_reference: Option<String>,
    pub id: String,
    pub kind: String,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    #[serde(rename = "transferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesExchangeIdAccountsGetResponse200 {
    /// Current page items.
    pub items: Vec<serde_json::Value>,
    /// token to use as `paginationToken` to request the next page.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesExchangeIdDeleteResponse200 {
    pub deleted: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesExchangeIdGetResponse200 {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    pub id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesGetResponse200 {
    /// Current page items.
    pub items: Vec<serde_json::Value>,
    /// token to use as `paginationToken` to request the next page.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesPostRequest {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "readConfiguration")]
    pub read_configuration: serde_json::Value,
    #[serde(rename = "writeConfiguration")]
    pub write_configuration: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesPostResponse200 {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    pub id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeSponsorsFeeSponsorIdActivatePutResponse200 {
    /// Defines whether EndUsers and their delegated wallets can use this Fee Sponsor.
    #[serde(rename = "allowEndUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_end_user: Option<bool>,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the Fee Sponsor was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Fee Sponsor id.
    pub id: String,
    /// Nickname for the Fee Sponsor. This is displayed on the transfer modal in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub network: serde_json::Value,
    /// Fee sponsor status.
    pub status: String,
    /// Id of the wallet that is used to sponsor the fee for other wallets
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeSponsorsFeeSponsorIdDeactivatePutResponse200 {
    /// Defines whether EndUsers and their delegated wallets can use this Fee Sponsor.
    #[serde(rename = "allowEndUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_end_user: Option<bool>,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the Fee Sponsor was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Fee Sponsor id.
    pub id: String,
    /// Nickname for the Fee Sponsor. This is displayed on the transfer modal in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub network: serde_json::Value,
    /// Fee sponsor status.
    pub status: String,
    /// Id of the wallet that is used to sponsor the fee for other wallets
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeSponsorsFeeSponsorIdDeleteResponse200 {
    /// Defines whether EndUsers and their delegated wallets can use this Fee Sponsor.
    #[serde(rename = "allowEndUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_end_user: Option<bool>,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the Fee Sponsor was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Fee Sponsor id.
    pub id: String,
    /// Nickname for the Fee Sponsor. This is displayed on the transfer modal in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub network: serde_json::Value,
    /// Fee sponsor status.
    pub status: String,
    /// Id of the wallet that is used to sponsor the fee for other wallets
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeSponsorsFeeSponsorIdFeesGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeSponsorsFeeSponsorIdGetResponse200 {
    /// Defines whether EndUsers and their delegated wallets can use this Fee Sponsor.
    #[serde(rename = "allowEndUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_end_user: Option<bool>,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the Fee Sponsor was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Fee Sponsor id.
    pub id: String,
    /// Nickname for the Fee Sponsor. This is displayed on the transfer modal in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub network: serde_json::Value,
    /// Fee sponsor status.
    pub status: String,
    /// Id of the wallet that is used to sponsor the fee for other wallets
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeSponsorsGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeSponsorsPostRequest {
    /// Defines whether EndUsers and their delegated wallets can use this Fee Sponsor.
    #[serde(rename = "allowEndUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_end_user: Option<bool>,
    /// Nickname for the Fee Sponsor. This will be displayed on the transfer modal in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Id of the wallet that will be used to sponsor the fee for other wallets.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeSponsorsPostResponse200 {
    /// Defines whether EndUsers and their delegated wallets can use this Fee Sponsor.
    #[serde(rename = "allowEndUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_end_user: Option<bool>,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the Fee Sponsor was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Fee Sponsor id.
    pub id: String,
    /// Nickname for the Fee Sponsor. This is displayed on the transfer modal in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub network: serde_json::Value,
    /// Fee sponsor status.
    pub status: String,
    /// Id of the wallet that is used to sponsor the fee for other wallets
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyStoresGetResponse200 {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysImportPostRequest {
    pub curve: String,
    #[serde(rename = "encryptedKeyShares")]
    pub encrypted_key_shares: Vec<serde_json::Value>,
    #[serde(rename = "minSigners")]
    pub min_signers: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub protocol: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysImportPostResponse200 {
    pub curve: String,
    pub custodial: bool,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_deleted: Option<String>,
    #[serde(rename = "dateExported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_exported: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub scheme: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdDelegatePostRequest {
    #[serde(rename = "delegateTo")]
    pub delegate_to: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdDelegatePostResponse200 {
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdDeleteResponse200 {
    pub curve: String,
    pub custodial: bool,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_deleted: Option<String>,
    #[serde(rename = "dateExported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_exported: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub scheme: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdDerivePostRequest {
    pub domain: String,
    pub seed: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdDerivePostResponse200 {
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdExportPostRequest {
    #[serde(rename = "encryptionKey")]
    pub encryption_key: String,
    #[serde(rename = "supportedSchemes")]
    pub supported_schemes: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdExportPostResponse200 {
    pub curve: String,
    /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
    #[serde(rename = "encryptedKeyShares")]
    pub encrypted_key_shares: Vec<serde_json::Value>,
    /// The TSS threshold of the wallet private signing key shares
    #[serde(rename = "minSigners")]
    pub min_signers: f64,
    pub protocol: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdGetResponse200 {
    pub curve: String,
    pub custodial: bool,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_deleted: Option<String>,
    #[serde(rename = "dateExported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_exported: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub scheme: String,
    pub status: String,
    pub store: serde_json::Value,
    pub wallets: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdPutRequest {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdPutResponse200 {
    pub curve: String,
    pub custodial: bool,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_deleted: Option<String>,
    #[serde(rename = "dateExported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_exported: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub scheme: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdSignaturesGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdSignaturesPostResponse200 {
    #[serde(rename = "approvalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(rename = "dateConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_confirmed: Option<String>,
    #[serde(rename = "datePolicyResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_policy_resolved: Option<String>,
    #[serde(rename = "dateRequested")]
    pub date_requested: String,
    #[serde(rename = "dateSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_signed: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    pub id: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<serde_json::Value>>,
    #[serde(rename = "signedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_data: Option<String>,
    pub status: String,
    #[serde(rename = "txHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysKeyIdSignaturesSignatureIdGetResponse200 {
    #[serde(rename = "approvalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(rename = "dateConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_confirmed: Option<String>,
    #[serde(rename = "datePolicyResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_policy_resolved: Option<String>,
    #[serde(rename = "dateRequested")]
    pub date_requested: String,
    #[serde(rename = "dateSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_signed: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    pub id: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<serde_json::Value>>,
    #[serde(rename = "signedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_data: Option<String>,
    pub status: String,
    #[serde(rename = "txHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysPostRequest {
    pub curve: String,
    #[serde(rename = "delayDelegation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_delegation: Option<bool>,
    #[serde(rename = "delegateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub scheme: String,
    #[serde(rename = "storeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysPostResponse200 {
    pub curve: String,
    pub custodial: bool,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_deleted: Option<String>,
    #[serde(rename = "dateExported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_exported: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub scheme: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworksNetworkValidatorsGetResponse200 {
    /// Current page items.
    pub items: Vec<CantonValidator>,
    /// token to use as `paginationToken` to request the next page.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworksNetworkValidatorsValidatorIdPutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger: Option<serde_json::Value>,
    /// Nickname for this validator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworksReadContractPostResponse200 {
    pub data: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPermissionIdArchivePutRequest {
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPermissionIdArchivePutResponse200 {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    pub id: String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "isImmutable")]
    pub is_immutable: bool,
    pub name: String,
    pub operations: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPermissionIdAssignmentsGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPermissionIdAssignmentsPostRequest {
    #[serde(rename = "identityId")]
    pub identity_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPermissionIdAssignmentsPostResponse200 {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    pub id: String,
    #[serde(rename = "identityId")]
    pub identity_id: String,
    #[serde(rename = "isImmutable")]
    pub is_immutable: bool,
    #[serde(rename = "permissionId")]
    pub permission_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPermissionIdPutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPermissionIdPutResponse200 {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    pub id: String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "isImmutable")]
    pub is_immutable: bool,
    pub name: String,
    pub operations: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPostRequest {
    pub name: String,
    pub operations: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPostResponse200 {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    pub id: String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "isImmutable")]
    pub is_immutable: bool,
    pub name: String,
    pub operations: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignersGetResponse200 {
    pub clusters: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakingStakesGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakingStakesStakeIdActionsGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakingStakesStakeIdRewardsGetResponse200 {
    pub balance: String,
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwapsGetResponse200 {
    /// Current page items.
    pub items: Vec<Swap>,
    /// token to use as `paginationToken` to request the next page.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwapsPostRequest {
    /// Provided for this swap. Used for attesting that the swap is being created with the same parameters as the quote.
    pub provider: String,
    /// Quote to use for this swap.
    #[serde(rename = "quoteId")]
    pub quote_id: String,
    /// An optional reference for this Swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you are willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%. Used for attesting that the swap is being created with the same parameters as the quote. 
    #[serde(rename = "slippageBps")]
    pub slippage_bps: f64,
    /// The source asset that will be spent on the Swap transaction. Used for attesting that the swap is being created with the same parameters as the quote.
    #[serde(rename = "sourceAsset")]
    pub source_asset: serde_json::Value,
    /// The target asset that will be received with the Swap transaction. Used for attesting that the swap is being created with the same parameters as the quote.
    #[serde(rename = "targetAsset")]
    pub target_asset: serde_json::Value,
    /// Id of the Dfns wallet receiving the target asset. Currently this value must be the same as the `walletId`. Used for attesting that the swap is being created with the same parameters as the quote.
    #[serde(rename = "targetWalletId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_wallet_id: Option<String>,
    /// Id of the Dfns wallet spending the sourceAsset. Used for attesting that the swap is being created with the same parameters as the quote.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwapsQuotesPostRequest {
    /// Swap provider.
    pub provider: String,
    /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you're willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%.
    #[serde(rename = "slippageBps")]
    pub slippage_bps: f64,
    /// The source asset that will be spent on the Swap transaction, following the same stucture as the [transfer API](https://docs.dfns.co/api-reference/wallets/transfer-asset).
    #[serde(rename = "sourceAsset")]
    pub source_asset: serde_json::Value,
    /// The target asset that will be received with the Swap transaction, follows the same structure as sourceAsset, but doesn't include the amount.
    #[serde(rename = "targetAsset")]
    pub target_asset: serde_json::Value,
    /// Id of the Dfns wallet receiving the target asset. Currently this value must be the same as the `walletId`.
    #[serde(rename = "targetWalletId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_wallet_id: Option<String>,
    /// Id of the Dfns wallet spending the sourceAsset.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2PoliciesGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2PolicyApprovalsApprovalIdDecisionsPostRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2PolicyApprovalsApprovalIdDecisionsPostResponse200 {
    pub activity: serde_json::Value,
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "dateResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_resolved: Option<String>,
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    pub decisions: Vec<serde_json::Value>,
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    pub id: String,
    #[serde(rename = "initiatorId")]
    pub initiator_id: String,
    #[serde(rename = "policyEvaluations")]
    pub policy_evaluations: Vec<serde_json::Value>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2PolicyApprovalsApprovalIdGetResponse200 {
    pub activity: serde_json::Value,
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "dateResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_resolved: Option<String>,
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    pub decisions: Vec<serde_json::Value>,
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    pub id: String,
    #[serde(rename = "initiatorId")]
    pub initiator_id: String,
    #[serde(rename = "policyEvaluations")]
    pub policy_evaluations: Vec<serde_json::Value>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2PolicyApprovalsGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsGetResponse200 {
    pub items: Vec<Wallet>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsImportPostRequest {
    pub curve: String,
    #[serde(rename = "encryptedKeyShares")]
    pub encrypted_key_shares: Vec<serde_json::Value>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "minSigners")]
    pub min_signers: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub network: String,
    pub protocol: serde_json::Value,
    /// Id of the validator on which the wallet is created
    #[serde(rename = "validatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsPostRequest {
    /// Specify if you want to create the wallet from a service account and later [delegate it](/api-reference/keys/delegate-key) to an end user.
    #[serde(rename = "delayDelegation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_delegation: Option<bool>,
    /// ID of the end user to delegate this wallet to. The wallet will only be usable by the end user. More info [here](https://docs.dfns.co/advanced/delegated-signing).
    #[serde(rename = "delegateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_to: Option<String>,
    /// User-defined value that can be used to correlate the entity with an external system
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Wallet nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Network used for the wallet.
    pub network: String,
    /// Options for the wallet's underlying key
    #[serde(rename = "signingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<serde_json::Value>,
    /// List of tags to be created for this wallet. If specified, requires the `Wallets:Tags:Add` permission, like the [Tag Wallet](https://docs.dfns.co/api-reference/wallets/tag-wallet) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Id of the validator on which the wallet is created for Canton networks
    #[serde(rename = "validatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdAssetsGetResponse200 {
    pub assets: Vec<serde_json::Value>,
    #[serde(rename = "netWorth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_worth: Option<serde_json::Value>,
    pub network: Network,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdDelegatePostRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdDelegatePostResponse200 {
    pub status: String,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdExportPostRequest {
    #[serde(rename = "encryptionKey")]
    pub encryption_key: String,
    #[serde(rename = "supportedSchemes")]
    pub supported_schemes: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdExportPostResponse200 {
    pub curve: String,
    /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
    #[serde(rename = "encryptedKeyShares")]
    pub encrypted_key_shares: Vec<serde_json::Value>,
    /// The TSS threshold of the wallet private signing key shares
    #[serde(rename = "minSigners")]
    pub min_signers: f64,
    pub protocol: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdNftsGetResponse200 {
    pub network: Network,
    pub nfts: Vec<serde_json::Value>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdOffersGetResponse200 {
    /// Current page items.
    pub items: Vec<Offer>,
    /// token to use as `paginationToken` to request the next page.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdPutRequest {
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdSignaturesGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdSignaturesPostResponse200 {
    #[serde(rename = "approvalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(rename = "dateConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_confirmed: Option<String>,
    #[serde(rename = "datePolicyResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_policy_resolved: Option<String>,
    #[serde(rename = "dateRequested")]
    pub date_requested: String,
    #[serde(rename = "dateSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_signed: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    pub id: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<serde_json::Value>>,
    #[serde(rename = "signedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_data: Option<String>,
    pub status: String,
    #[serde(rename = "txHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdSignaturesSignatureIdGetResponse200 {
    #[serde(rename = "approvalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(rename = "dateConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_confirmed: Option<String>,
    #[serde(rename = "datePolicyResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_policy_resolved: Option<String>,
    #[serde(rename = "dateRequested")]
    pub date_requested: String,
    #[serde(rename = "dateSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_signed: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    pub id: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<serde_json::Value>>,
    #[serde(rename = "signedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_data: Option<String>,
    pub status: String,
    #[serde(rename = "txHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdTagsDeleteRequest {
    /// List of tags.
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdTagsPutRequest {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdTransactionsGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdTransactionsPostResponse200 {
    #[serde(rename = "approvalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(rename = "dateBroadcasted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_broadcasted: Option<String>,
    #[serde(rename = "dateConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_confirmed: Option<String>,
    #[serde(rename = "datePolicyResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_policy_resolved: Option<String>,
    #[serde(rename = "dateRequested")]
    pub date_requested: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    pub id: String,
    pub network: Network,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    pub status: String,
    #[serde(rename = "txHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdTransactionsTransactionIdGetResponse200 {
    #[serde(rename = "approvalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(rename = "dateBroadcasted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_broadcasted: Option<String>,
    #[serde(rename = "dateConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_confirmed: Option<String>,
    #[serde(rename = "datePolicyResolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_policy_resolved: Option<String>,
    #[serde(rename = "dateRequested")]
    pub date_requested: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    pub id: String,
    pub network: Network,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: serde_json::Value,
    pub status: String,
    #[serde(rename = "txHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletsWalletIdTransfersGetResponse200 {
    pub items: Vec<TransferRequest>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksPostRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All events this webhook is subscribed to.
    pub events: Vec<serde_json::Value>,
    /// Webhook status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Webhook url
    pub url: String,
}

/// Webhook
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksPostResponse200 {
    /// Date when webhook was created
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Date when webhook was last updated
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    /// Short description this webhook's purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All events this webhook is subscribed to.
    pub events: Vec<serde_json::Value>,
    /// Webhook ID
    pub id: String,
    /// The secret associated with this webhook, with which webhook requests will be signed.
    pub secret: String,
    /// Webhook status
    pub status: String,
    /// Webhook url
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksWebhookIdDeleteResponse200 {
    pub deleted: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksWebhookIdEventsGetResponse200 {
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// WebhookEvent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksWebhookIdEventsWebhookEventIdGetResponse200 {
    pub data: serde_json::Value,
    /// ISO date string when event was raised
    pub date: String,
    /// Error message if any error happened during the webhook request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// WebhookEvent ID
    pub id: String,
    /// Webhook event
    pub kind: String,
    /// Status code of the webhook request
    pub status: String,
    /// Unix timestamp when the event was forwarded to the webhook url by our servers.
    #[serde(rename = "timestampSent")]
    pub timestamp_sent: i64,
}

/// Webhook
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksWebhookIdGetResponse200 {
    /// Date when webhook was created
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Date when webhook was last updated
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    /// Short description this webhook's purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All events this webhook is subscribed to.
    pub events: Vec<serde_json::Value>,
    /// Webhook ID
    pub id: String,
    /// Webhook status
    pub status: String,
    /// Webhook url
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksWebhookIdPingPostResponse200 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksWebhookIdPutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All events this webhook is subscribed to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
    /// Webhook status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Webhook url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Webhook
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhooksWebhookIdPutResponse200 {
    /// Date when webhook was created
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Date when webhook was last updated
    #[serde(rename = "dateUpdated")]
    pub date_updated: String,
    /// Short description this webhook's purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All events this webhook is subscribed to.
    pub events: Vec<serde_json::Value>,
    /// Webhook ID
    pub id: String,
    /// Webhook status
    pub status: String,
    /// Webhook url
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YieldsGetResponse200 {
    /// Current page items.
    pub items: Vec<Yield>,
    /// token to use as `paginationToken` to request the next page.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YieldsYieldIdActionsGetResponse200 {
    /// Current page items.
    pub items: Vec<YieldAction>,
    /// token to use as `paginationToken` to request the next page.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Request body for creating a yield action. Different protocols may have different requirements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YieldsYieldIdActionsPostRequest {
    /// An optional external identifier provided by the client to ensure idempotency and prevent duplicate operations.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The type of action being performed on the yield investment: Deposit to add funds or Withdraw to remove funds.
    pub kind: String,
    /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you're willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%.
    #[serde(rename = "slippageBps")]
    pub slippage_bps: f64,
    #[serde(rename = "sourceAsset")]
    pub source_asset: serde_json::Value,
    #[serde(rename = "targetAsset")]
    pub target_asset: serde_json::Value,
}

