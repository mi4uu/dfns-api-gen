// Auto-generated from OpenAPI schema
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum CantonValidatorKind {
    Shared,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum CantonValidatorNetwork {
    Canton,
    CantonDevnet,
    CantonTestnet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CantonValidator {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    pub id: String,
    pub kind: CantonValidatorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub network: CantonValidatorNetwork,
    /// Organization id.
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "partyHint")]
    pub party_hint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum OfferKind {
    Native,
    Aip21,
    Asa,
    Coin,
    Erc20,
    Erc721,
    Asset,
    Hip17,
    Hts,
    Sep41,
    Spl,
    Spl2022,
    Tep74,
    Trc10,
    Trc20,
    Trc721,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct OfferMetadataAssetQuotes {
    #[serde(rename = "EUR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<f64>,
    #[serde(rename = "USD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct OfferMetadataAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotes: Option<OfferMetadataAssetQuotes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct OfferMetadata {
    pub asset: OfferMetadataAsset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum OfferStatus {
    Pending,
    Accepted,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Offer {
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub from: String,
    /// Offer id.
    pub id: String,
    pub kind: OfferKind,
    pub metadata: OfferMetadata,
    pub network: Network,
    /// Organization id.
    #[serde(rename = "orgId")]
    pub org_id: String,
    pub status: OfferStatus,
    pub timestamp: String,
    pub to: String,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub value: String,
    /// Wallet id.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

/// Swap provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum SwapProvider {
    UniswapX,
    UniswapClassic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SwapRequester {
    /// Service Account token or Personal Access token used when requesting the resource.
    #[serde(rename = "tokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// User (could be a service account) who requested the resource.
    #[serde(rename = "userId")]
    pub user_id: String,
}

/// Swap status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum SwapStatus {
    PendingPolicyApproval,
    InProgress,
    Completed,
    Failed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Swap {
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the swap was initiated.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Swap id.
    pub id: String,
    pub provider: SwapProvider,
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
    pub requester: SwapRequester,
    /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you are willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%.
    #[serde(rename = "slippageBps")]
    pub slippage_bps: f64,
    pub status: SwapStatus,
    /// Id of the Dfns wallet receiving the target asset. Currently this value must be the same as the `walletId`.
    #[serde(rename = "targetWalletId")]
    pub target_wallet_id: String,
    /// Id of the Dfns wallet spending the sourceAsset.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

/// Swap provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum SwapQuoteProvider {
    UniswapX,
    UniswapClassic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SwapQuoteRequester {
    /// Service Account token or Personal Access token used when requesting the quote.
    #[serde(rename = "tokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// User (could be a service account) who requested the quote.
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SwapQuote {
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). When the quote was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// ID of the Swap Quote.
    pub id: String,
    pub provider: SwapQuoteProvider,
    /// The full request used for obtaining this quote.
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: SwapQuoteRequester,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TransferRequestMetadataAssetQuotes {
    #[serde(rename = "EUR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<f64>,
    #[serde(rename = "USD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TransferRequestMetadataAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotes: Option<TransferRequestMetadataAssetQuotes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TransferRequestMetadata {
    pub asset: TransferRequestMetadataAsset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TransferRequestRequester {
    #[serde(rename = "tokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum TransferRequestStatus {
    Pending,
    Executing,
    Broadcasted,
    Confirmed,
    Failed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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
    pub metadata: TransferRequestMetadata,
    pub network: Network,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: TransferRequestRequester,
    pub status: TransferRequestStatus,
    #[serde(rename = "txHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

/// User kind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum UserKind {
    CustomerEmployee,
    EndUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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
    pub kind: UserKind,
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

/// Network this wallet is bound to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum WalletNetwork {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum WalletSigningKeyCurve {
    #[serde(rename = "ed25519")]
    Ed25519,
    #[serde(rename = "secp256k1")]
    Secp256k1,
    #[serde(rename = "stark")]
    Stark,
}

/// Key scheme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum WalletSigningKeyScheme {
    ECDSA,
    EdDSA,
    Schnorr,
}

/// Details about the key underlying the wallet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct WalletSigningKey {
    pub curve: WalletSigningKeyCurve,
    /// The end user ID the key (and wallet) is delegated to.
    #[serde(rename = "delegatedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_to: Option<String>,
    /// Key id.
    pub id: String,
    /// Hex-encoded value of the public key.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub scheme: WalletSigningKeyScheme,
}

/// Wallet status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum WalletStatus {
    Active,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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
    pub network: WalletNetwork,
    /// Details about the key underlying the wallet.
    #[serde(rename = "signingKey")]
    pub signing_key: WalletSigningKey,
    pub status: WalletStatus,
    /// List of tags.
    pub tags: Vec<String>,
    /// Id of the validator on which the wallet is created for Canton networks
    #[serde(rename = "validatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<String>,
}

/// The DeFi protocol used for yield generation. Currently supports OFNS protocol
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum YieldProtocol {
    #[serde(rename = "0fns")]
    _0fns,
}

/// A yield investment representing funds deposited to earn interest from a DeFi protocol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Yield {
    /// The total amount currently invested in this yield.
    pub amount: serde_json::Value,
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date (must be UTC). [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date. When the yield was created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// Unique identifier for the yield investment.
    pub id: String,
    pub protocol: YieldProtocol,
    /// The total interest earned so far in this yield.
    pub rewards: serde_json::Value,
    /// Wallet id.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

/// The type of action being performed on the yield investment: Deposit to add funds or Withdraw to remove funds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum YieldActionKind {
    Deposit,
    Withdraw,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct YieldActionRequester {
    /// Service Account token or Personal Access token used when requesting the resource.
    #[serde(rename = "tokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// User (could be a service account) who requested the resource.
    #[serde(rename = "userId")]
    pub user_id: String,
}

/// Status of the yield action. Once initiated, the status will be InProgress, after processing it will be Completed or Failed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum YieldActionStatus {
    PendingPolicyApproval,
    InProgress,
    Completed,
    Failed,
    Rejected,
}

/// A specific action performed on a yield investment, such as depositing or withdrawing funds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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
    pub kind: YieldActionKind,
    /// The full request used for initiating this yield action.
    #[serde(rename = "requestBody")]
    pub request_body: serde_json::Value,
    pub requester: YieldActionRequester,
    pub status: YieldActionStatus,
    /// Unique identifier for the yield investment.
    #[serde(rename = "yieldId")]
    pub yield_id: String,
}

pub mod agreements {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum LatestUnacceptedGetResponseLatestAgreementAgreementType {
        PrivacyPolicy,
        TermsAndConditions,
        UniswapTermsOfService,
        UniswapPrivacyPolicy,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LatestUnacceptedGetResponseLatestAgreement {
        #[serde(rename = "agreementType")]
        pub agreement_type: LatestUnacceptedGetResponseLatestAgreementAgreementType,
        #[serde(rename = "agreementUrl")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub agreement_url: Option<String>,
        pub details: String,
        pub id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LatestUnacceptedGetResponse {
        #[serde(rename = "latestAgreement")]
        pub latest_agreement: LatestUnacceptedGetResponseLatestAgreement,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct AgreementIdAcceptPostResponse {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ActionPostResponse {
        #[serde(rename = "userAction")]
        pub user_action: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ActionInitPostRequestUserActionServerKind {
        Api,
        Staff,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ActionInitPostRequest {
        #[serde(rename = "userActionHttpMethod")]
        pub user_action_http_method: String,
        #[serde(rename = "userActionHttpPath")]
        pub user_action_http_path: String,
        #[serde(rename = "userActionPayload")]
        pub user_action_payload: String,
        #[serde(rename = "userActionServerKind")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_action_server_kind: Option<ActionInitPostRequestUserActionServerKind>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ActionInitPostResponseAllowCredentials {
        pub key: Vec<serde_json::Value>,
        #[serde(rename = "passwordProtectedKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password_protected_key: Option<Vec<serde_json::Value>>,
        pub webauthn: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ActionInitPostResponseAttestation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ActionInitPostResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ActionInitPostResponseUserVerification {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ActionInitPostResponse {
        #[serde(rename = "allowCredentials")]
        pub allow_credentials: ActionInitPostResponseAllowCredentials,
        pub attestation: ActionInitPostResponseAttestation,
        pub challenge: String,
        #[serde(rename = "challengeIdentifier")]
        pub challenge_identifier: String,
        #[serde(rename = "externalAuthenticationUrl")]
        pub external_authentication_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<ActionInitPostResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: Vec<serde_json::Value>,
        #[serde(rename = "userVerification")]
        pub user_verification: ActionInitPostResponseUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ActionLogsIdGetResponseFirstFactorCredentialAssertion {
        #[serde(rename = "authenticatorData")]
        pub authenticator_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        pub signature: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ActionLogsIdGetResponseFirstFactorCredential {
        pub assertion: ActionLogsIdGetResponseFirstFactorCredentialAssertion,
        pub id: String,
        pub kind: String,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ActionLogsIdGetResponse {
        pub action: String,
        #[serde(rename = "actionToken")]
        pub action_token: String,
        #[serde(rename = "datePerformed")]
        pub date_performed: String,
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: ActionLogsIdGetResponseFirstFactorCredential,
        pub id: String,
        #[serde(rename = "userId")]
        pub user_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct AppsGetResponse {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum AppsAppIdGetResponseKind {
        ServerSideApplication,
        ClientSideApplication,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct AppsAppIdGetResponse {
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
        pub kind: AppsAppIdGetResponseKind,
        pub name: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(rename = "permissionAssignments")]
        pub permission_assignments: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsGetResponse {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum CredentialsPostResponseKind {
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsPostResponse {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: CredentialsPostResponseKind,
        pub name: String,
        pub origin: String,
        #[serde(rename = "publicKey")]
        pub public_key: String,
        #[serde(rename = "relyingPartyId")]
        pub relying_party_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsActivatePutRequest {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsActivatePutResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsCodePostRequest {
        /// Code expiration, as an ISO-8601 datetime string or a unix timestamp
        pub expiration: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsCodePostResponse {
        pub code: String,
        pub expiration: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum CredentialsCodeInitPostRequestCredentialKind {
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsCodeInitPostRequest {
        pub code: String,
        #[serde(rename = "credentialKind")]
        pub credential_kind: CredentialsCodeInitPostRequestCredentialKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum CredentialsCodeVerifyPostResponseKind {
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsCodeVerifyPostResponse {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: CredentialsCodeVerifyPostResponseKind,
        pub name: String,
        pub origin: String,
        #[serde(rename = "publicKey")]
        pub public_key: String,
        #[serde(rename = "relyingPartyId")]
        pub relying_party_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsDeactivatePutRequest {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsDeactivatePutResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum CredentialsInitPostRequestKind {
        Fido2,
        Key,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CredentialsInitPostRequest {
        pub kind: CredentialsInitPostRequestKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginCodePostRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginCodePostResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginDelegatedPostRequest {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginDelegatedPostResponse {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginInitPostRequest {
        #[serde(rename = "loginCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub login_code: Option<String>,
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginInitPostResponseAllowCredentials {
        pub key: Vec<serde_json::Value>,
        #[serde(rename = "passwordProtectedKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password_protected_key: Option<Vec<serde_json::Value>>,
        pub webauthn: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum LoginInitPostResponseAttestation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginInitPostResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum LoginInitPostResponseUserVerification {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginInitPostResponse {
        #[serde(rename = "allowCredentials")]
        pub allow_credentials: LoginInitPostResponseAllowCredentials,
        pub attestation: LoginInitPostResponseAttestation,
        pub challenge: String,
        #[serde(rename = "challengeIdentifier")]
        pub challenge_identifier: String,
        #[serde(rename = "externalAuthenticationUrl")]
        pub external_authentication_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<LoginInitPostResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: Vec<serde_json::Value>,
        #[serde(rename = "userVerification")]
        pub user_verification: LoginInitPostResponseUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum LoginSocialPostRequestSocialLoginProviderKind {
        Oidc,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginSocialPostRequest {
        #[serde(rename = "idToken")]
        pub id_token: String,
        #[serde(rename = "orgId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub org_id: Option<String>,
        #[serde(rename = "socialLoginProviderKind")]
        pub social_login_provider_kind: LoginSocialPostRequestSocialLoginProviderKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginSocialPostResponse {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginSsoPostRequest {
        /// Authorization code obtained from the IdP
        pub code: String,
        /// State forwarded by the IdP
        pub state: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginSsoPostResponse {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LoginSsoInitPostResponse {
        /// The URL to redirect the user to authenticate with the IdP
        #[serde(rename = "ssoRedirectUrl")]
        pub sso_redirect_url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LogoutPutRequest {
        #[serde(rename = "allSessions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub all_sessions: Option<bool>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct LogoutPutResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PatsGetResponse {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PatsPostResponseKind {
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PatsPostResponse {
        #[serde(rename = "accessToken")]
        pub access_token: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsPostResponseKind,
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

    /// Access token kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PatsTokenIdGetResponseKind {
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PatsTokenIdGetResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdGetResponseKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PatsTokenIdPutRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// Access token kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PatsTokenIdPutResponseKind {
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PatsTokenIdPutResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdPutResponseKind,
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

    /// Access token kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PatsTokenIdDeleteResponseKind {
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PatsTokenIdDeleteResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdDeleteResponseKind,
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

    /// Access token kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PatsTokenIdActivatePutResponseKind {
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PatsTokenIdActivatePutResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdActivatePutResponseKind,
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

    /// Access token kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PatsTokenIdDeactivatePutResponseKind {
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PatsTokenIdDeactivatePutResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdDeactivatePutResponseKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostRequestNewCredentialsRecoveryCredentialCredentialInfo {
        #[serde(rename = "attestationData")]
        pub attestation_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserPostRequestNewCredentialsRecoveryCredentialCredentialKind {
        RecoveryKey,
    }

    /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostRequestNewCredentialsRecoveryCredential {
        #[serde(rename = "credentialInfo")]
        pub credential_info: RecoverUserPostRequestNewCredentialsRecoveryCredentialCredentialInfo,
        #[serde(rename = "credentialKind")]
        pub credential_kind: RecoverUserPostRequestNewCredentialsRecoveryCredentialCredentialKind,
        #[serde(rename = "credentialName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credential_name: Option<String>,
        #[serde(rename = "encryptedPrivateKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encrypted_private_key: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostRequestNewCredentials {
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: serde_json::Value,
        /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
        #[serde(rename = "recoveryCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recovery_credential: Option<RecoverUserPostRequestNewCredentialsRecoveryCredential>,
        #[serde(rename = "secondFactorCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub second_factor_credential: Option<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostRequestRecoveryCredentialAssertion {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub algorithm: Option<String>,
        #[serde(rename = "clientData")]
        pub client_data: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
        pub signature: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserPostRequestRecoveryKind {
        RecoveryKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostRequestRecovery {
        #[serde(rename = "credentialAssertion")]
        pub credential_assertion: RecoverUserPostRequestRecoveryCredentialAssertion,
        pub kind: RecoverUserPostRequestRecoveryKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostRequest {
        #[serde(rename = "newCredentials")]
        pub new_credentials: RecoverUserPostRequestNewCredentials,
        pub recovery: RecoverUserPostRequestRecovery,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserPostResponseCredentialKind {
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostResponseCredential {
        pub kind: RecoverUserPostResponseCredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostResponseUser {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserPostResponse {
        pub credential: RecoverUserPostResponseCredential,
        pub user: RecoverUserPostResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserCodePostRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserCodePostResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserDelegatedPostRequest {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserDelegatedPostResponseAttestation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserDelegatedPostResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserDelegatedPostResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserDelegatedPostResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserDelegatedPostResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RecoverUserDelegatedPostResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RecoverUserDelegatedPostResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RecoverUserDelegatedPostResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserDelegatedPostResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserDelegatedPostResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserDelegatedPostResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserDelegatedPostResponse {
        #[serde(rename = "allowedRecoveryCredentials")]
        pub allowed_recovery_credentials: Vec<serde_json::Value>,
        pub attestation: RecoverUserDelegatedPostResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RecoverUserDelegatedPostResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RecoverUserDelegatedPostResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RecoverUserDelegatedPostResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RecoverUserDelegatedPostResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserInitPostRequest {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
        #[serde(rename = "verificationCode")]
        pub verification_code: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserInitPostResponseAttestation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserInitPostResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserInitPostResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RecoverUserInitPostResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserInitPostResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RecoverUserInitPostResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RecoverUserInitPostResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RecoverUserInitPostResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserInitPostResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserInitPostResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserInitPostResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RecoverUserInitPostResponse {
        #[serde(rename = "allowedRecoveryCredentials")]
        pub allowed_recovery_credentials: Vec<serde_json::Value>,
        pub attestation: RecoverUserInitPostResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RecoverUserInitPostResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RecoverUserInitPostResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RecoverUserInitPostResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RecoverUserInitPostResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationPostRequestRecoveryCredentialCredentialInfo {
        #[serde(rename = "attestationData")]
        pub attestation_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationPostRequestRecoveryCredentialCredentialKind {
        RecoveryKey,
    }

    /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationPostRequestRecoveryCredential {
        #[serde(rename = "credentialInfo")]
        pub credential_info: RegistrationPostRequestRecoveryCredentialCredentialInfo,
        #[serde(rename = "credentialKind")]
        pub credential_kind: RegistrationPostRequestRecoveryCredentialCredentialKind,
        #[serde(rename = "credentialName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credential_name: Option<String>,
        #[serde(rename = "encryptedPrivateKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encrypted_private_key: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationPostRequest {
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: serde_json::Value,
        /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
        #[serde(rename = "recoveryCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recovery_credential: Option<RegistrationPostRequestRecoveryCredential>,
        #[serde(rename = "secondFactorCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub second_factor_credential: Option<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationPostResponseCredentialKind {
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationPostResponseCredential {
        pub kind: RegistrationPostResponseCredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationPostResponseUser {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationPostResponse {
        pub credential: RegistrationPostResponseCredential,
        pub user: RegistrationPostResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationCodePutRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationCodePutResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationDelegatedPostRequestKind {
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationDelegatedPostRequest {
        pub email: String,
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        pub kind: RegistrationDelegatedPostRequestKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationDelegatedPostResponseAttestation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationDelegatedPostResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationDelegatedPostResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationDelegatedPostResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationDelegatedPostResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationDelegatedPostResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationDelegatedPostResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationDelegatedPostResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationDelegatedPostResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationDelegatedPostResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationDelegatedPostResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationDelegatedPostResponse {
        pub attestation: RegistrationDelegatedPostResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationDelegatedPostResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationDelegatedPostResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationDelegatedPostResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationDelegatedPostResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationEnduserPostRequestRecoveryCredentialCredentialInfo {
        #[serde(rename = "attestationData")]
        pub attestation_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationEnduserPostRequestRecoveryCredentialCredentialKind {
        RecoveryKey,
    }

    /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationEnduserPostRequestRecoveryCredential {
        #[serde(rename = "credentialInfo")]
        pub credential_info: RegistrationEnduserPostRequestRecoveryCredentialCredentialInfo,
        #[serde(rename = "credentialKind")]
        pub credential_kind: RegistrationEnduserPostRequestRecoveryCredentialCredentialKind,
        #[serde(rename = "credentialName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credential_name: Option<String>,
        #[serde(rename = "encryptedPrivateKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encrypted_private_key: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationEnduserPostRequest {
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: serde_json::Value,
        /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
        #[serde(rename = "recoveryCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recovery_credential: Option<RegistrationEnduserPostRequestRecoveryCredential>,
        #[serde(rename = "secondFactorCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub second_factor_credential: Option<serde_json::Value>,
        pub wallets: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationEnduserPostResponseAuthentication {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationEnduserPostResponseCredentialKind {
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationEnduserPostResponseCredential {
        pub kind: RegistrationEnduserPostResponseCredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationEnduserPostResponseUser {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationEnduserPostResponse {
        pub authentication: RegistrationEnduserPostResponseAuthentication,
        pub credential: RegistrationEnduserPostResponseCredential,
        pub user: RegistrationEnduserPostResponseUser,
        pub wallets: Vec<Wallet>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationInitPostRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(rename = "registrationCode")]
        pub registration_code: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationInitPostResponseAttestation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationInitPostResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationInitPostResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationInitPostResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationInitPostResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationInitPostResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationInitPostResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationInitPostResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationInitPostResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationInitPostResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationInitPostResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationInitPostResponse {
        pub attestation: RegistrationInitPostResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationInitPostResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationInitPostResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationInitPostResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationInitPostResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationSocialPostRequestSocialLoginProviderKind {
        Oidc,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationSocialPostRequest {
        #[serde(rename = "idToken")]
        pub id_token: String,
        #[serde(rename = "orgId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub org_id: Option<String>,
        #[serde(rename = "socialLoginProviderKind")]
        pub social_login_provider_kind: RegistrationSocialPostRequestSocialLoginProviderKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationSocialPostResponseAttestation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationSocialPostResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationSocialPostResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum RegistrationSocialPostResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationSocialPostResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationSocialPostResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationSocialPostResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationSocialPostResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationSocialPostResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationSocialPostResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationSocialPostResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct RegistrationSocialPostResponse {
        pub attestation: RegistrationSocialPostResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationSocialPostResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationSocialPostResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationSocialPostResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationSocialPostResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsGetResponse {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ServiceAccountsPostResponseUserInfoKind {
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsPostResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsPostResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsPostResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsPostResponseUserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ServiceAccountsServiceAccountIdGetResponseUserInfoKind {
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdGetResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdGetResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdGetResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdGetResponseUserInfo,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdPutRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ServiceAccountsServiceAccountIdPutResponseUserInfoKind {
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdPutResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdPutResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdPutResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdPutResponseUserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ServiceAccountsServiceAccountIdDeleteResponseUserInfoKind {
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdDeleteResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdDeleteResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdDeleteResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdDeleteResponseUserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ServiceAccountsServiceAccountIdActivatePutResponseUserInfoKind {
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdActivatePutResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdActivatePutResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdActivatePutResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdActivatePutResponseUserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ServiceAccountsServiceAccountIdDeactivatePutResponseUserInfoKind {
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdDeactivatePutResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdDeactivatePutResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ServiceAccountsServiceAccountIdDeactivatePutResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdDeactivatePutResponseUserInfo,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct UsersGetResponse {
        pub items: Vec<User>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// The kind of user being created. 
    ///       In this endpoint it can only be "`CustomerEmployee`" (creating an "`EndUser`" is done through the [Delegated Registration](https://docs.dfns.co/api-reference/auth/registration-flows#delegated-users-registration-flow) endpoint)
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum UsersPostRequestKind {
        CustomerEmployee,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
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
        pub kind: UsersPostRequestKind,
        #[serde(rename = "publicKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub public_key: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct UsersUserIdPutRequest {
        #[serde(rename = "isSSORequired")]
        pub is_ssorequired: bool,
    }

}

pub mod exchanges {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct GetResponse {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PostRequestKind {
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PostRequestReadConfiguration {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub otp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[serde(rename = "privateApiKey")]
        pub private_api_key: String,
        #[serde(rename = "publicApiKey")]
        pub public_api_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PostRequestWriteConfiguration {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub otp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[serde(rename = "privateApiKey")]
        pub private_api_key: String,
        #[serde(rename = "publicApiKey")]
        pub public_api_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PostRequest {
        pub kind: PostRequestKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "readConfiguration")]
        pub read_configuration: PostRequestReadConfiguration,
        #[serde(rename = "writeConfiguration")]
        pub write_configuration: PostRequestWriteConfiguration,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PostResponseKind {
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PostResponse {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: PostResponseKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ExchangeIdGetResponseKind {
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExchangeIdGetResponse {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: ExchangeIdGetResponseKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ExchangeIdDeleteResponseDeleted {
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExchangeIdDeleteResponse {
        pub deleted: ExchangeIdDeleteResponseDeleted,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExchangeIdAccountsGetResponse {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExchangeIdAccountsAccountIdAssetsGetResponse {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ExchangeIdAccountsAccountIdDepositsPostResponseKind {
        Withdrawal,
        Deposit,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExchangeIdAccountsAccountIdDepositsPostResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExchangeIdAccountsAccountIdDepositsPostResponse {
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
        pub kind: ExchangeIdAccountsAccountIdDepositsPostResponseKind,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: ExchangeIdAccountsAccountIdDepositsPostResponseRequester,
        #[serde(rename = "transferId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_id: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ExchangeIdAccountsAccountIdWithdrawalsPostResponseKind {
        Withdrawal,
        Deposit,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExchangeIdAccountsAccountIdWithdrawalsPostResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExchangeIdAccountsAccountIdWithdrawalsPostResponse {
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
        pub kind: ExchangeIdAccountsAccountIdWithdrawalsPostResponseKind,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: ExchangeIdAccountsAccountIdWithdrawalsPostResponseRequester,
        #[serde(rename = "transferId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_id: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

}

pub mod feesponsors {
    use super::*;

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum FeeSponsorIdGetResponseStatus {
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct FeeSponsorIdGetResponse {
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
        pub status: FeeSponsorIdGetResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum FeeSponsorIdDeleteResponseStatus {
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct FeeSponsorIdDeleteResponse {
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
        pub status: FeeSponsorIdDeleteResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum FeeSponsorIdActivatePutResponseStatus {
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct FeeSponsorIdActivatePutResponse {
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
        pub status: FeeSponsorIdActivatePutResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum FeeSponsorIdDeactivatePutResponseStatus {
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct FeeSponsorIdDeactivatePutResponse {
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
        pub status: FeeSponsorIdDeactivatePutResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct FeeSponsorIdFeesGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

}

pub mod keys {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ImportPostRequestCurve {
        #[serde(rename = "ed25519")]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ImportPostRequest {
        pub curve: ImportPostRequestCurve,
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        #[serde(rename = "minSigners")]
        pub min_signers: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub protocol: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ImportPostResponseCurve {
        #[serde(rename = "ed25519")]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ImportPostResponseScheme {
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ImportPostResponseStatus {
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ImportPostResponse {
        pub curve: ImportPostResponseCurve,
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
        pub scheme: ImportPostResponseScheme,
        pub status: ImportPostResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdGetResponseCurve {
        #[serde(rename = "ed25519")]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdGetResponseScheme {
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdGetResponseStatus {
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdGetResponseStoreKind {
        Hsm,
        Mpc,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdGetResponseStore {
        pub id: String,
        #[serde(rename = "keyId")]
        pub key_id: String,
        pub kind: KeyIdGetResponseStoreKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdGetResponse {
        pub curve: KeyIdGetResponseCurve,
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
        pub scheme: KeyIdGetResponseScheme,
        pub status: KeyIdGetResponseStatus,
        pub store: KeyIdGetResponseStore,
        pub wallets: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdPutRequest {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdPutResponseCurve {
        #[serde(rename = "ed25519")]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdPutResponseScheme {
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdPutResponseStatus {
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdPutResponse {
        pub curve: KeyIdPutResponseCurve,
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
        pub scheme: KeyIdPutResponseScheme,
        pub status: KeyIdPutResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdDeleteResponseCurve {
        #[serde(rename = "ed25519")]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdDeleteResponseScheme {
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdDeleteResponseStatus {
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdDeleteResponse {
        pub curve: KeyIdDeleteResponseCurve,
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
        pub scheme: KeyIdDeleteResponseScheme,
        pub status: KeyIdDeleteResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdDelegatePostRequest {
        #[serde(rename = "delegateTo")]
        pub delegate_to: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdDelegatePostResponseStatus {
        Delegated,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdDelegatePostResponse {
        #[serde(rename = "keyId")]
        pub key_id: String,
        pub status: KeyIdDelegatePostResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdDerivePostRequest {
        pub domain: String,
        pub seed: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdDerivePostResponse {
        pub output: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdExportPostRequest {
        #[serde(rename = "encryptionKey")]
        pub encryption_key: String,
        #[serde(rename = "supportedSchemes")]
        pub supported_schemes: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdExportPostResponseCurve {
        #[serde(rename = "ed25519")]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdExportPostResponseProtocol {
        CGGMP24,
        FROST,
        #[serde(rename = "FROST_BITCOIN")]
        FROSTBITCOIN,
        #[serde(rename = "GLOW20_DH")]
        GLOW20DH,
        KU23,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdExportPostResponse {
        pub curve: KeyIdExportPostResponseCurve,
        /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        /// The TSS threshold of the wallet private signing key shares
        #[serde(rename = "minSigners")]
        pub min_signers: f64,
        pub protocol: KeyIdExportPostResponseProtocol,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdSignaturesGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "keyId")]
        pub key_id: String,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdSignaturesPostResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdSignaturesPostResponseSignature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdSignaturesPostResponseStatus {
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdSignaturesPostResponse {
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
        pub requester: KeyIdSignaturesPostResponseRequester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<KeyIdSignaturesPostResponseSignature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: KeyIdSignaturesPostResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdSignaturesSignatureIdGetResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdSignaturesSignatureIdGetResponseSignature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum KeyIdSignaturesSignatureIdGetResponseStatus {
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct KeyIdSignaturesSignatureIdGetResponse {
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
        pub requester: KeyIdSignaturesSignatureIdGetResponseRequester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<KeyIdSignaturesSignatureIdGetResponseSignature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: KeyIdSignaturesSignatureIdGetResponseStatus,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum ReadContractPostResponseKind {
        Evm,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ReadContractPostResponse {
        pub data: String,
        pub kind: ReadContractPostResponseKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct NetworkValidatorsGetResponse {
        /// Current page items.
        pub items: Vec<CantonValidator>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct NetworkValidatorsValidatorIdPutRequestLedgerOauth2 {
        /// the audience your configured on your auth provider. It is suggested to start with `https://canton.network.global`.
        pub audience: String,
        /// The client id from your auth provider for this application.
        #[serde(rename = "clientId")]
        pub client_id: String,
        /// The client secret from your auth provider for this application.
        #[serde(rename = "clientSecret")]
        pub client_secret: String,
        /// your OAuth2 tenant domain. Provided by your auth provider. 
        pub domain: String,
        /// token endpoint from your authorization provider. We will call this endpoint on your tenant domain (i.e.: `<domain>/<token path>`)
        #[serde(rename = "tokenPath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_path: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct NetworkValidatorsValidatorIdPutRequestLedger {
        /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
        pub oauth2: NetworkValidatorsValidatorIdPutRequestLedgerOauth2,
        /// URL to reach the API at this address. The calls will be originating from our IP addresses (see [Dfns Environments](https://docs.dfns.co/api-reference/environments))
        pub url: String,
    }

    /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct NetworkValidatorsValidatorIdPutRequestValidatorOauth2 {
        /// the audience your configured on your auth provider. It is suggested to start with `https://canton.network.global`.
        pub audience: String,
        /// The client id from your auth provider for this application.
        #[serde(rename = "clientId")]
        pub client_id: String,
        /// The client secret from your auth provider for this application.
        #[serde(rename = "clientSecret")]
        pub client_secret: String,
        /// your OAuth2 tenant domain. Provided by your auth provider. 
        pub domain: String,
        /// token endpoint from your authorization provider. We will call this endpoint on your tenant domain (i.e.: `<domain>/<token path>`)
        #[serde(rename = "tokenPath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_path: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct NetworkValidatorsValidatorIdPutRequestValidator {
        /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
        pub oauth2: NetworkValidatorsValidatorIdPutRequestValidatorOauth2,
        /// URL to reach the API at this address. The calls will be originating from our IP addresses (see [Dfns Environments](https://docs.dfns.co/api-reference/environments))
        pub url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct NetworkValidatorsValidatorIdPutRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ledger: Option<NetworkValidatorsValidatorIdPutRequestLedger>,
        /// Nickname for this validator.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validator: Option<NetworkValidatorsValidatorIdPutRequestValidator>,
    }

}

pub mod permissions {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PermissionIdPutRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operations: Option<Vec<serde_json::Value>>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PermissionIdPutResponseStatus {
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PermissionIdPutResponse {
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
        pub status: PermissionIdPutResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PermissionIdArchivePutRequest {
        #[serde(rename = "isArchived")]
        pub is_archived: bool,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PermissionIdArchivePutResponseStatus {
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PermissionIdArchivePutResponse {
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
        pub status: PermissionIdArchivePutResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PermissionIdAssignmentsGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PermissionIdAssignmentsPostRequest {
        #[serde(rename = "identityId")]
        pub identity_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PermissionIdAssignmentsPostResponse {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct StakesGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct StakesStakeIdActionsGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct StakesStakeIdRewardsGetResponse {
        pub balance: String,
        pub symbol: String,
    }

}

pub mod swaps {
    use super::*;

    /// Swap provider.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum QuotesPostRequestProvider {
        UniswapX,
        UniswapClassic,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct QuotesPostRequest {
        pub provider: QuotesPostRequestProvider,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PoliciesGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PolicyApprovalsGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PolicyApprovalsApprovalIdGetResponseStatus {
        Pending,
        Approved,
        Denied,
        Expired,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PolicyApprovalsApprovalIdGetResponse {
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
        pub status: PolicyApprovalsApprovalIdGetResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PolicyApprovalsApprovalIdDecisionsPostRequestValue {
        Approved,
        Denied,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PolicyApprovalsApprovalIdDecisionsPostRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        pub value: PolicyApprovalsApprovalIdDecisionsPostRequestValue,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum PolicyApprovalsApprovalIdDecisionsPostResponseStatus {
        Pending,
        Approved,
        Denied,
        Expired,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PolicyApprovalsApprovalIdDecisionsPostResponse {
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
        pub status: PolicyApprovalsApprovalIdDecisionsPostResponseStatus,
    }

}

pub mod wallets {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdPutRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdAssetsGetResponseNetWorth {
        #[serde(rename = "USD")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub usd: Option<f64>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdAssetsGetResponse {
        pub assets: Vec<serde_json::Value>,
        #[serde(rename = "netWorth")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub net_worth: Option<WalletIdAssetsGetResponseNetWorth>,
        pub network: Network,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdDelegatePostRequest {
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WalletIdDelegatePostResponseStatus {
        Delegated,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdDelegatePostResponse {
        pub status: WalletIdDelegatePostResponseStatus,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdExportPostRequest {
        #[serde(rename = "encryptionKey")]
        pub encryption_key: String,
        #[serde(rename = "supportedSchemes")]
        pub supported_schemes: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WalletIdExportPostResponseCurve {
        #[serde(rename = "ed25519")]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WalletIdExportPostResponseProtocol {
        CGGMP24,
        FROST,
        #[serde(rename = "FROST_BITCOIN")]
        FROSTBITCOIN,
        #[serde(rename = "GLOW20_DH")]
        GLOW20DH,
        KU23,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdExportPostResponse {
        pub curve: WalletIdExportPostResponseCurve,
        /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        /// The TSS threshold of the wallet private signing key shares
        #[serde(rename = "minSigners")]
        pub min_signers: f64,
        pub protocol: WalletIdExportPostResponseProtocol,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdNftsGetResponse {
        pub network: Network,
        pub nfts: Vec<serde_json::Value>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdOffersGetResponse {
        /// Current page items.
        pub items: Vec<Offer>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdSignaturesGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "keyId")]
        pub key_id: String,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WalletIdSignaturesPostResponseNetwork {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdSignaturesPostResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdSignaturesPostResponseSignature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WalletIdSignaturesPostResponseStatus {
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdSignaturesPostResponse {
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
        pub network: WalletIdSignaturesPostResponseNetwork,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: WalletIdSignaturesPostResponseRequester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<WalletIdSignaturesPostResponseSignature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: WalletIdSignaturesPostResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdSignaturesSignatureIdGetResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdSignaturesSignatureIdGetResponseSignature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WalletIdSignaturesSignatureIdGetResponseStatus {
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdSignaturesSignatureIdGetResponse {
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
        pub requester: WalletIdSignaturesSignatureIdGetResponseRequester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<WalletIdSignaturesSignatureIdGetResponseSignature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: WalletIdSignaturesSignatureIdGetResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdTagsPutRequest {
        pub tags: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdTagsDeleteRequest {
        /// List of tags.
        pub tags: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdTransactionsGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdTransactionsPostResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WalletIdTransactionsPostResponseStatus {
        Pending,
        Executing,
        Broadcasted,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdTransactionsPostResponse {
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
        pub requester: WalletIdTransactionsPostResponseRequester,
        pub status: WalletIdTransactionsPostResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdTransactionsTransactionIdGetResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WalletIdTransactionsTransactionIdGetResponseStatus {
        Pending,
        Executing,
        Broadcasted,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdTransactionsTransactionIdGetResponse {
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
        pub requester: WalletIdTransactionsTransactionIdGetResponseRequester,
        pub status: WalletIdTransactionsTransactionIdGetResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WalletIdTransfersGetResponse {
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

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WebhookIdGetResponseStatus {
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WebhookIdGetResponse {
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
        pub status: WebhookIdGetResponseStatus,
        /// Webhook url
        pub url: String,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WebhookIdPutRequestStatus {
        Enabled,
        Disabled,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WebhookIdPutRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// All events this webhook is subscribed to.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub events: Option<Vec<serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<WebhookIdPutRequestStatus>,
        /// Webhook url
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WebhookIdPutResponseStatus {
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WebhookIdPutResponse {
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
        pub status: WebhookIdPutResponseStatus,
        /// Webhook url
        pub url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WebhookIdDeleteResponseDeleted {
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WebhookIdDeleteResponse {
        pub deleted: WebhookIdDeleteResponseDeleted,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WebhookIdEventsGetResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Webhook event
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum WebhookIdEventsWebhookEventIdGetResponseKind {
        #[serde(rename = "policy.triggered")]
        PolicyTriggered,
        #[serde(rename = "policy.approval.pending")]
        PolicyApprovalPending,
        #[serde(rename = "policy.approval.resolved")]
        PolicyApprovalResolved,
        #[serde(rename = "key.created")]
        KeyCreated,
        #[serde(rename = "key.deleted")]
        KeyDeleted,
        #[serde(rename = "key.delegated")]
        KeyDelegated,
        #[serde(rename = "key.exported")]
        KeyExported,
        #[serde(rename = "wallet.blockchainevent.detected")]
        WalletBlockchaineventDetected,
        #[serde(rename = "wallet.created")]
        WalletCreated,
        #[serde(rename = "wallet.delegated")]
        WalletDelegated,
        #[serde(rename = "wallet.exported")]
        WalletExported,
        #[serde(rename = "wallet.signature.failed")]
        WalletSignatureFailed,
        #[serde(rename = "wallet.signature.rejected")]
        WalletSignatureRejected,
        #[serde(rename = "wallet.signature.requested")]
        WalletSignatureRequested,
        #[serde(rename = "wallet.signature.signed")]
        WalletSignatureSigned,
        #[serde(rename = "wallet.transaction.broadcasted")]
        WalletTransactionBroadcasted,
        #[serde(rename = "wallet.transaction.confirmed")]
        WalletTransactionConfirmed,
        #[serde(rename = "wallet.transaction.failed")]
        WalletTransactionFailed,
        #[serde(rename = "wallet.transaction.rejected")]
        WalletTransactionRejected,
        #[serde(rename = "wallet.transaction.requested")]
        WalletTransactionRequested,
        #[serde(rename = "wallet.transfer.broadcasted")]
        WalletTransferBroadcasted,
        #[serde(rename = "wallet.transfer.confirmed")]
        WalletTransferConfirmed,
        #[serde(rename = "wallet.transfer.failed")]
        WalletTransferFailed,
        #[serde(rename = "wallet.transfer.rejected")]
        WalletTransferRejected,
        #[serde(rename = "wallet.transfer.requested")]
        WalletTransferRequested,
        #[serde(rename = "wallet.offer.received")]
        WalletOfferReceived,
        #[serde(rename = "wallet.offer.accepted")]
        WalletOfferAccepted,
        #[serde(rename = "wallet.offer.rejected")]
        WalletOfferRejected,
        #[serde(rename = "wallet.tags.modified")]
        WalletTagsModified,
    }

    /// WebhookEvent
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WebhookIdEventsWebhookEventIdGetResponse {
        pub data: serde_json::Value,
        /// ISO date string when event was raised
        pub date: String,
        /// Error message if any error happened during the webhook request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        /// WebhookEvent ID
        pub id: String,
        pub kind: WebhookIdEventsWebhookEventIdGetResponseKind,
        /// Status code of the webhook request
        pub status: String,
        /// Unix timestamp when the event was forwarded to the webhook url by our servers.
        #[serde(rename = "timestampSent")]
        pub timestamp_sent: i64,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WebhookIdPingPostResponse {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        pub status: String,
    }

}

pub mod yields {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct YieldIdActionsGetResponse {
        /// Current page items.
        pub items: Vec<YieldAction>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// The type of action being performed on the yield investment: Deposit to add funds or Withdraw to remove funds.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum YieldIdActionsPostRequestKind {
        Deposit,
        Withdraw,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum YieldIdActionsPostRequestSourceAssetKind {
        Erc20,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct YieldIdActionsPostRequestSourceAsset {
        pub amount: String,
        pub contract: String,
        pub kind: YieldIdActionsPostRequestSourceAssetKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub enum YieldIdActionsPostRequestTargetAssetKind {
        Erc20,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct YieldIdActionsPostRequestTargetAsset {
        pub amount: String,
        pub contract: String,
        pub kind: YieldIdActionsPostRequestTargetAssetKind,
    }

    /// Request body for creating a yield action. Different protocols may have different requirements.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct YieldIdActionsPostRequest {
        /// An optional external identifier provided by the client to ensure idempotency and prevent duplicate operations.
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        pub kind: YieldIdActionsPostRequestKind,
        /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you're willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%.
        #[serde(rename = "slippageBps")]
        pub slippage_bps: f64,
        #[serde(rename = "sourceAsset")]
        pub source_asset: YieldIdActionsPostRequestSourceAsset,
        #[serde(rename = "targetAsset")]
        pub target_asset: YieldIdActionsPostRequestTargetAsset,
    }

}

