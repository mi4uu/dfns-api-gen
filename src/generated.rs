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

pub mod agreements {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LatestUnacceptedGetResponse200 {
        #[serde(rename = "latestAgreement")]
        pub latest_agreement: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct AgreementIdAcceptPostResponse200 {
        #[serde(rename = "agreementId")]
        pub agreement_id: String,
        #[serde(rename = "dateAccepted")]
        pub date_accepted: String,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

}

pub mod auth {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ActionPostRequest {
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
    pub struct ActionPostResponse200 {
        #[serde(rename = "userAction")]
        pub user_action: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ActionInitPostRequest {
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
    pub struct ActionInitPostResponse200 {
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
    pub struct ActionLogsIdGetResponse200 {
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
    pub struct AppsGetResponse200 {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct AppsAppIdGetResponse200 {
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
    pub struct CredentialsGetResponse200 {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CredentialsPostResponse200 {
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
    pub struct CredentialsActivatePutRequest {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CredentialsActivatePutResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CredentialsCodePostRequest {
        /// Code expiration, as an ISO-8601 datetime string or a unix timestamp
        pub expiration: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CredentialsCodePostResponse200 {
        pub code: String,
        pub expiration: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CredentialsCodeInitPostRequest {
        pub code: String,
        #[serde(rename = "credentialKind")]
        pub credential_kind: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CredentialsCodeVerifyPostResponse200 {
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
    pub struct CredentialsDeactivatePutRequest {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CredentialsDeactivatePutResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CredentialsInitPostRequest {
        pub kind: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginPostRequest {
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
    pub struct LoginCodePostRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginCodePostResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginDelegatedPostRequest {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginDelegatedPostResponse200 {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginInitPostRequest {
        #[serde(rename = "loginCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub login_code: Option<String>,
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginInitPostResponse200 {
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
    pub struct LoginSocialPostRequest {
        #[serde(rename = "idToken")]
        pub id_token: String,
        #[serde(rename = "orgId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub org_id: Option<String>,
        #[serde(rename = "socialLoginProviderKind")]
        pub social_login_provider_kind: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginSocialPostResponse200 {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginSsoPostRequest {
        /// Authorization code obtained from the IdP
        pub code: String,
        /// State forwarded by the IdP
        pub state: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginSsoPostResponse200 {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LoginSsoInitPostRequest {
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
    pub struct LoginSsoInitPostResponse200 {
        /// The URL to redirect the user to authenticate with the IdP
        #[serde(rename = "ssoRedirectUrl")]
        pub sso_redirect_url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LogoutPutRequest {
        #[serde(rename = "allSessions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub all_sessions: Option<bool>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LogoutPutResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PatsGetResponse200 {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PatsPostRequest {
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
    pub struct PatsPostResponse200 {
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
    pub struct PatsTokenIdGetResponse200 {
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
    pub struct PatsTokenIdPutRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PatsTokenIdPutResponse200 {
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
    pub struct PatsTokenIdDeleteResponse200 {
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
    pub struct PatsTokenIdActivatePutResponse200 {
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
    pub struct PatsTokenIdDeactivatePutResponse200 {
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
    pub struct RecoverUserPostRequest {
        #[serde(rename = "newCredentials")]
        pub new_credentials: serde_json::Value,
        pub recovery: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RecoverUserPostResponse200 {
        pub credential: serde_json::Value,
        pub user: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RecoverUserCodePostRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RecoverUserCodePostResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RecoverUserDelegatedPostRequest {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RecoverUserDelegatedPostResponse200 {
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
    pub struct RecoverUserInitPostRequest {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
        #[serde(rename = "verificationCode")]
        pub verification_code: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RecoverUserInitPostResponse200 {
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
    pub struct RegistrationPostRequest {
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
    pub struct RegistrationPostResponse200 {
        pub credential: serde_json::Value,
        pub user: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RegistrationCodePutRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RegistrationCodePutResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RegistrationDelegatedPostRequest {
        pub email: String,
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        pub kind: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RegistrationDelegatedPostResponse200 {
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
    pub struct RegistrationEnduserPostRequest {
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
    pub struct RegistrationEnduserPostResponse200 {
        pub authentication: serde_json::Value,
        pub credential: serde_json::Value,
        pub user: serde_json::Value,
        pub wallets: Vec<Wallet>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RegistrationInitPostRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(rename = "registrationCode")]
        pub registration_code: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RegistrationInitPostResponse200 {
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
    pub struct RegistrationSocialPostRequest {
        #[serde(rename = "idToken")]
        pub id_token: String,
        #[serde(rename = "orgId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub org_id: Option<String>,
        #[serde(rename = "socialLoginProviderKind")]
        pub social_login_provider_kind: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RegistrationSocialPostResponse200 {
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
    pub struct ServiceAccountsGetResponse200 {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ServiceAccountsPostRequest {
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
    pub struct ServiceAccountsPostResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ServiceAccountsServiceAccountIdGetResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ServiceAccountsServiceAccountIdPutRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ServiceAccountsServiceAccountIdPutResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ServiceAccountsServiceAccountIdDeleteResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ServiceAccountsServiceAccountIdActivatePutResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ServiceAccountsServiceAccountIdDeactivatePutResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct UsersGetResponse200 {
        pub items: Vec<User>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct UsersPostRequest {
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
    pub struct UsersUserIdPutRequest {
        #[serde(rename = "isSSORequired")]
        pub is_ssorequired: bool,
    }

}

pub mod exchanges {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct GetResponse200 {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PostRequest {
        pub kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "readConfiguration")]
        pub read_configuration: serde_json::Value,
        #[serde(rename = "writeConfiguration")]
        pub write_configuration: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PostResponse200 {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ExchangeIdGetResponse200 {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ExchangeIdDeleteResponse200 {
        pub deleted: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ExchangeIdAccountsGetResponse200 {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ExchangeIdAccountsAccountIdAssetsGetResponse200 {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ExchangeIdAccountsAccountIdDepositsPostResponse200 {
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
    pub struct ExchangeIdAccountsAccountIdWithdrawalsPostResponse200 {
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

}

pub mod feesponsors {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct FeeSponsorIdGetResponse200 {
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
    pub struct FeeSponsorIdDeleteResponse200 {
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
    pub struct FeeSponsorIdActivatePutResponse200 {
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
    pub struct FeeSponsorIdDeactivatePutResponse200 {
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
    pub struct FeeSponsorIdFeesGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

}

pub mod keys {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ImportPostRequest {
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
    pub struct ImportPostResponse200 {
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
    pub struct KeyIdGetResponse200 {
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
    pub struct KeyIdPutRequest {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct KeyIdPutResponse200 {
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
    pub struct KeyIdDeleteResponse200 {
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
    pub struct KeyIdDelegatePostRequest {
        #[serde(rename = "delegateTo")]
        pub delegate_to: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct KeyIdDelegatePostResponse200 {
        #[serde(rename = "keyId")]
        pub key_id: String,
        pub status: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct KeyIdDerivePostRequest {
        pub domain: String,
        pub seed: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct KeyIdDerivePostResponse200 {
        pub output: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct KeyIdExportPostRequest {
        #[serde(rename = "encryptionKey")]
        pub encryption_key: String,
        #[serde(rename = "supportedSchemes")]
        pub supported_schemes: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct KeyIdExportPostResponse200 {
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
    pub struct KeyIdSignaturesGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "keyId")]
        pub key_id: String,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct KeyIdSignaturesPostResponse200 {
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
    pub struct KeyIdSignaturesSignatureIdGetResponse200 {
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

}

pub mod keystores {
    use super::*;

}

pub mod networks {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ReadContractPostResponse200 {
        pub data: String,
        pub kind: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NetworkValidatorsGetResponse200 {
        /// Current page items.
        pub items: Vec<CantonValidator>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NetworkValidatorsValidatorIdPutRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ledger: Option<serde_json::Value>,
        /// Nickname for this validator.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validator: Option<serde_json::Value>,
    }

}

pub mod permissions {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PermissionIdPutRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operations: Option<Vec<serde_json::Value>>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PermissionIdPutResponse200 {
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
    pub struct PermissionIdArchivePutRequest {
        #[serde(rename = "isArchived")]
        pub is_archived: bool,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PermissionIdArchivePutResponse200 {
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
    pub struct PermissionIdAssignmentsGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PermissionIdAssignmentsPostRequest {
        #[serde(rename = "identityId")]
        pub identity_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PermissionIdAssignmentsPostResponse200 {
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

}

pub mod signers {
    use super::*;

}

pub mod staking {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct StakesGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct StakesStakeIdActionsGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct StakesStakeIdRewardsGetResponse200 {
        pub balance: String,
        pub symbol: String,
    }

}

pub mod swaps {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct QuotesPostRequest {
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

}

pub mod v2 {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PoliciesGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PolicyApprovalsGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PolicyApprovalsApprovalIdGetResponse200 {
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
    pub struct PolicyApprovalsApprovalIdDecisionsPostRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        pub value: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PolicyApprovalsApprovalIdDecisionsPostResponse200 {
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

}

pub mod wallets {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdPutRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdAssetsGetResponse200 {
        pub assets: Vec<serde_json::Value>,
        #[serde(rename = "netWorth")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub net_worth: Option<serde_json::Value>,
        pub network: Network,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdDelegatePostRequest {
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdDelegatePostResponse200 {
        pub status: String,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdExportPostRequest {
        #[serde(rename = "encryptionKey")]
        pub encryption_key: String,
        #[serde(rename = "supportedSchemes")]
        pub supported_schemes: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdExportPostResponse200 {
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
    pub struct WalletIdNftsGetResponse200 {
        pub network: Network,
        pub nfts: Vec<serde_json::Value>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdOffersGetResponse200 {
        /// Current page items.
        pub items: Vec<Offer>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdSignaturesGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "keyId")]
        pub key_id: String,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdSignaturesPostResponse200 {
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
    pub struct WalletIdSignaturesSignatureIdGetResponse200 {
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
    pub struct WalletIdTagsPutRequest {
        pub tags: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdTagsDeleteRequest {
        /// List of tags.
        pub tags: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdTransactionsGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WalletIdTransactionsPostResponse200 {
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
    pub struct WalletIdTransactionsTransactionIdGetResponse200 {
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
    pub struct WalletIdTransfersGetResponse200 {
        pub items: Vec<TransferRequest>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

}

pub mod webhooks {
    use super::*;

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WebhookIdGetResponse200 {
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
    pub struct WebhookIdPutRequest {
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
    pub struct WebhookIdPutResponse200 {
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
    pub struct WebhookIdDeleteResponse200 {
        pub deleted: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WebhookIdEventsGetResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// WebhookEvent
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WebhookIdEventsWebhookEventIdGetResponse200 {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WebhookIdPingPostResponse200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        pub status: String,
    }

}

pub mod yields {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct YieldIdActionsGetResponse200 {
        /// Current page items.
        pub items: Vec<YieldAction>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Request body for creating a yield action. Different protocols may have different requirements.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct YieldIdActionsPostRequest {
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

}

