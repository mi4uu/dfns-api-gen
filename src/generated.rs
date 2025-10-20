// Auto-generated from OpenAPI schema
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum BlockchainKind {
#[default]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum CantonValidatorKind {
#[default]
    Shared,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum CantonValidatorNetwork {
#[default]
    Canton,
    CantonDevnet,
    CantonTestnet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum Network {
#[default]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum OfferKind {
#[default]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
pub struct OfferMetadataAssetQuotes {
    #[serde(rename = "EUR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<f64>,
    #[serde(rename = "USD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
pub struct OfferMetadata {
    pub asset: OfferMetadataAsset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum OfferStatus {
#[default]
    Pending,
    Accepted,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum SwapProvider {
#[default]
    UniswapX,
    UniswapClassic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum SwapStatus {
#[default]
    PendingPolicyApproval,
    InProgress,
    Completed,
    Failed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum SwapQuoteProvider {
#[default]
    UniswapX,
    UniswapClassic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
pub struct SwapQuoteRequester {
    /// Service Account token or Personal Access token used when requesting the quote.
    #[serde(rename = "tokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// User (could be a service account) who requested the quote.
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
pub struct TransferRequestMetadataAssetQuotes {
    #[serde(rename = "EUR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<f64>,
    #[serde(rename = "USD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
pub struct TransferRequestMetadata {
    pub asset: TransferRequestMetadataAsset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
pub struct TransferRequestRequester {
    #[serde(rename = "tokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum TransferRequestStatus {
#[default]
    Pending,
    Executing,
    Broadcasted,
    Confirmed,
    Failed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum UserKind {
#[default]
    CustomerEmployee,
    EndUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum WalletNetwork {
#[default]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum WalletSigningKeyCurve {
    #[serde(rename = "ed25519")]
#[default]
    Ed25519,
    #[serde(rename = "secp256k1")]
    Secp256k1,
    #[serde(rename = "stark")]
    Stark,
}

/// Key scheme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum WalletSigningKeyScheme {
#[default]
    ECDSA,
    EdDSA,
    Schnorr,
}

/// Details about the key underlying the wallet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum WalletStatus {
#[default]
    Active,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum YieldProtocol {
    #[serde(rename = "0fns")]
#[default]
    _0fns,
}

/// A yield investment representing funds deposited to earn interest from a DeFi protocol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum YieldActionKind {
#[default]
    Deposit,
    Withdraw,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
pub enum YieldActionStatus {
#[default]
    PendingPolicyApproval,
    InProgress,
    Completed,
    Failed,
    Rejected,
}

/// A specific action performed on a yield investment, such as depositing or withdrawing funds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum LatestUnacceptedGETResponseLatestAgreementAgreementType {
    #[default]
        PrivacyPolicy,
        TermsAndConditions,
        UniswapTermsOfService,
        UniswapPrivacyPolicy,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LatestUnacceptedGETResponseLatestAgreement {
        #[serde(rename = "agreementType")]
        pub agreement_type: LatestUnacceptedGETResponseLatestAgreementAgreementType,
        #[serde(rename = "agreementUrl")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub agreement_url: Option<String>,
        pub details: String,
        pub id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LatestUnacceptedGETResponse {
        #[serde(rename = "latestAgreement")]
        pub latest_agreement: LatestUnacceptedGETResponseLatestAgreement,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct AgreementIdAcceptPOSTResponse {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionPOSTRequest {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionPOSTResponse {
        #[serde(rename = "userAction")]
        pub user_action: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ActionInitPOSTRequestUserActionServerKind {
    #[default]
        Api,
        Staff,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionInitPOSTRequest {
        #[serde(rename = "userActionHttpMethod")]
        pub user_action_http_method: String,
        #[serde(rename = "userActionHttpPath")]
        pub user_action_http_path: String,
        #[serde(rename = "userActionPayload")]
        pub user_action_payload: String,
        #[serde(rename = "userActionServerKind")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_action_server_kind: Option<ActionInitPOSTRequestUserActionServerKind>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionInitPOSTResponseAllowCredentials {
        pub key: Vec<serde_json::Value>,
        #[serde(rename = "passwordProtectedKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password_protected_key: Option<Vec<serde_json::Value>>,
        pub webauthn: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ActionInitPOSTResponseAttestation {
        #[serde(rename = "none")]
    #[default]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionInitPOSTResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ActionInitPOSTResponseUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionInitPOSTResponse {
        #[serde(rename = "allowCredentials")]
        pub allow_credentials: ActionInitPOSTResponseAllowCredentials,
        pub attestation: ActionInitPOSTResponseAttestation,
        pub challenge: String,
        #[serde(rename = "challengeIdentifier")]
        pub challenge_identifier: String,
        #[serde(rename = "externalAuthenticationUrl")]
        pub external_authentication_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<ActionInitPOSTResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: Vec<serde_json::Value>,
        #[serde(rename = "userVerification")]
        pub user_verification: ActionInitPOSTResponseUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionLogsIdGETResponseFirstFactorCredentialAssertion {
        #[serde(rename = "authenticatorData")]
        pub authenticator_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        pub signature: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionLogsIdGETResponseFirstFactorCredential {
        pub assertion: ActionLogsIdGETResponseFirstFactorCredentialAssertion,
        pub id: String,
        pub kind: String,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionLogsIdGETResponse {
        pub action: String,
        #[serde(rename = "actionToken")]
        pub action_token: String,
        #[serde(rename = "datePerformed")]
        pub date_performed: String,
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: ActionLogsIdGETResponseFirstFactorCredential,
        pub id: String,
        #[serde(rename = "userId")]
        pub user_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct AppsGETResponse {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum AppsAppIdGETResponseKind {
    #[default]
        ServerSideApplication,
        ClientSideApplication,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct AppsAppIdGETResponse {
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
        pub kind: AppsAppIdGETResponseKind,
        pub name: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(rename = "permissionAssignments")]
        pub permission_assignments: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsGETResponse {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CredentialsPOSTResponseKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsPOSTResponse {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: CredentialsPOSTResponseKind,
        pub name: String,
        pub origin: String,
        #[serde(rename = "publicKey")]
        pub public_key: String,
        #[serde(rename = "relyingPartyId")]
        pub relying_party_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsActivatePUTRequest {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsActivatePUTResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsCodePOSTRequest {
        /// Code expiration, as an ISO-8601 datetime string or a unix timestamp
        pub expiration: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsCodePOSTResponse {
        pub code: String,
        pub expiration: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CredentialsCodeInitPOSTRequestCredentialKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsCodeInitPOSTRequest {
        pub code: String,
        #[serde(rename = "credentialKind")]
        pub credential_kind: CredentialsCodeInitPOSTRequestCredentialKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CredentialsCodeVerifyPOSTResponseKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsCodeVerifyPOSTResponse {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: CredentialsCodeVerifyPOSTResponseKind,
        pub name: String,
        pub origin: String,
        #[serde(rename = "publicKey")]
        pub public_key: String,
        #[serde(rename = "relyingPartyId")]
        pub relying_party_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsDeactivatePUTRequest {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsDeactivatePUTResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CredentialsInitPOSTRequestKind {
    #[default]
        Fido2,
        Key,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsInitPOSTRequest {
        pub kind: CredentialsInitPOSTRequestKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginPOSTRequest {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginCodePOSTRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginCodePOSTResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginDelegatedPOSTRequest {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginDelegatedPOSTResponse {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginInitPOSTRequest {
        #[serde(rename = "loginCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub login_code: Option<String>,
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginInitPOSTResponseAllowCredentials {
        pub key: Vec<serde_json::Value>,
        #[serde(rename = "passwordProtectedKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password_protected_key: Option<Vec<serde_json::Value>>,
        pub webauthn: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum LoginInitPOSTResponseAttestation {
        #[serde(rename = "none")]
    #[default]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginInitPOSTResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum LoginInitPOSTResponseUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginInitPOSTResponse {
        #[serde(rename = "allowCredentials")]
        pub allow_credentials: LoginInitPOSTResponseAllowCredentials,
        pub attestation: LoginInitPOSTResponseAttestation,
        pub challenge: String,
        #[serde(rename = "challengeIdentifier")]
        pub challenge_identifier: String,
        #[serde(rename = "externalAuthenticationUrl")]
        pub external_authentication_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<LoginInitPOSTResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: Vec<serde_json::Value>,
        #[serde(rename = "userVerification")]
        pub user_verification: LoginInitPOSTResponseUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum LoginSocialPOSTRequestSocialLoginProviderKind {
    #[default]
        Oidc,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginSocialPOSTRequest {
        #[serde(rename = "idToken")]
        pub id_token: String,
        #[serde(rename = "orgId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub org_id: Option<String>,
        #[serde(rename = "socialLoginProviderKind")]
        pub social_login_provider_kind: LoginSocialPOSTRequestSocialLoginProviderKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginSocialPOSTResponse {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginSsoPOSTRequest {
        /// Authorization code obtained from the IdP
        pub code: String,
        /// State forwarded by the IdP
        pub state: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginSsoPOSTResponse {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginSsoInitPOSTRequest {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginSsoInitPOSTResponse {
        /// The URL to redirect the user to authenticate with the IdP
        #[serde(rename = "ssoRedirectUrl")]
        pub sso_redirect_url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LogoutPUTRequest {
        #[serde(rename = "allSessions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub all_sessions: Option<bool>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LogoutPUTResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsGETResponse {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsPOSTRequest {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PatsPOSTResponseKind {
    #[default]
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsPOSTResponse {
        #[serde(rename = "accessToken")]
        pub access_token: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsPOSTResponseKind,
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
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PatsTokenIdGETResponseKind {
    #[default]
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsTokenIdGETResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdGETResponseKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsTokenIdPUTRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// Access token kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PatsTokenIdPUTResponseKind {
    #[default]
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsTokenIdPUTResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdPUTResponseKind,
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
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PatsTokenIdDELETEResponseKind {
    #[default]
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsTokenIdDELETEResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdDELETEResponseKind,
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
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PatsTokenIdActivatePUTResponseKind {
    #[default]
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsTokenIdActivatePUTResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdActivatePUTResponseKind,
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
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PatsTokenIdDeactivatePUTResponseKind {
    #[default]
        Pat,
        ServiceAccount,
        Token,
        Code,
        Recovery,
        Temp,
        Application,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsTokenIdDeactivatePUTResponse {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdDeactivatePUTResponseKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTRequestNewCredentialsRecoveryCredentialCredentialInfo {
        #[serde(rename = "attestationData")]
        pub attestation_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserPOSTRequestNewCredentialsRecoveryCredentialCredentialKind {
    #[default]
        RecoveryKey,
    }

    /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTRequestNewCredentialsRecoveryCredential {
        #[serde(rename = "credentialInfo")]
        pub credential_info: RecoverUserPOSTRequestNewCredentialsRecoveryCredentialCredentialInfo,
        #[serde(rename = "credentialKind")]
        pub credential_kind: RecoverUserPOSTRequestNewCredentialsRecoveryCredentialCredentialKind,
        #[serde(rename = "credentialName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credential_name: Option<String>,
        #[serde(rename = "encryptedPrivateKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encrypted_private_key: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTRequestNewCredentials {
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: serde_json::Value,
        /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
        #[serde(rename = "recoveryCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recovery_credential: Option<RecoverUserPOSTRequestNewCredentialsRecoveryCredential>,
        #[serde(rename = "secondFactorCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub second_factor_credential: Option<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTRequestRecoveryCredentialAssertion {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub algorithm: Option<String>,
        #[serde(rename = "clientData")]
        pub client_data: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
        pub signature: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserPOSTRequestRecoveryKind {
    #[default]
        RecoveryKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTRequestRecovery {
        #[serde(rename = "credentialAssertion")]
        pub credential_assertion: RecoverUserPOSTRequestRecoveryCredentialAssertion,
        pub kind: RecoverUserPOSTRequestRecoveryKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTRequest {
        #[serde(rename = "newCredentials")]
        pub new_credentials: RecoverUserPOSTRequestNewCredentials,
        pub recovery: RecoverUserPOSTRequestRecovery,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserPOSTResponseCredentialKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTResponseCredential {
        pub kind: RecoverUserPOSTResponseCredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTResponseUser {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTResponse {
        pub credential: RecoverUserPOSTResponseCredential,
        pub user: RecoverUserPOSTResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserCodePOSTRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserCodePOSTResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTRequest {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserDelegatedPOSTResponseAttestation {
        #[serde(rename = "none")]
    #[default]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserDelegatedPOSTResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserDelegatedPOSTResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserDelegatedPOSTResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RecoverUserDelegatedPOSTResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RecoverUserDelegatedPOSTResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RecoverUserDelegatedPOSTResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponse {
        #[serde(rename = "allowedRecoveryCredentials")]
        pub allowed_recovery_credentials: Vec<serde_json::Value>,
        pub attestation: RecoverUserDelegatedPOSTResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RecoverUserDelegatedPOSTResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RecoverUserDelegatedPOSTResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RecoverUserDelegatedPOSTResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RecoverUserDelegatedPOSTResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTRequest {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
        #[serde(rename = "verificationCode")]
        pub verification_code: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserInitPOSTResponseAttestation {
        #[serde(rename = "none")]
    #[default]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserInitPOSTResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserInitPOSTResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserInitPOSTResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RecoverUserInitPOSTResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RecoverUserInitPOSTResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RecoverUserInitPOSTResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponse {
        #[serde(rename = "allowedRecoveryCredentials")]
        pub allowed_recovery_credentials: Vec<serde_json::Value>,
        pub attestation: RecoverUserInitPOSTResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RecoverUserInitPOSTResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RecoverUserInitPOSTResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RecoverUserInitPOSTResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RecoverUserInitPOSTResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTRequestRecoveryCredentialCredentialInfo {
        #[serde(rename = "attestationData")]
        pub attestation_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationPOSTRequestRecoveryCredentialCredentialKind {
    #[default]
        RecoveryKey,
    }

    /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTRequestRecoveryCredential {
        #[serde(rename = "credentialInfo")]
        pub credential_info: RegistrationPOSTRequestRecoveryCredentialCredentialInfo,
        #[serde(rename = "credentialKind")]
        pub credential_kind: RegistrationPOSTRequestRecoveryCredentialCredentialKind,
        #[serde(rename = "credentialName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credential_name: Option<String>,
        #[serde(rename = "encryptedPrivateKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encrypted_private_key: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTRequest {
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: serde_json::Value,
        /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
        #[serde(rename = "recoveryCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recovery_credential: Option<RegistrationPOSTRequestRecoveryCredential>,
        #[serde(rename = "secondFactorCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub second_factor_credential: Option<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationPOSTResponseCredentialKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTResponseCredential {
        pub kind: RegistrationPOSTResponseCredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTResponseUser {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTResponse {
        pub credential: RegistrationPOSTResponseCredential,
        pub user: RegistrationPOSTResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationCodePUTRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationCodePUTResponse {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationDelegatedPOSTRequestKind {
    #[default]
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTRequest {
        pub email: String,
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        pub kind: RegistrationDelegatedPOSTRequestKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationDelegatedPOSTResponseAttestation {
        #[serde(rename = "none")]
    #[default]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationDelegatedPOSTResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationDelegatedPOSTResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationDelegatedPOSTResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationDelegatedPOSTResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationDelegatedPOSTResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationDelegatedPOSTResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponse {
        pub attestation: RegistrationDelegatedPOSTResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationDelegatedPOSTResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationDelegatedPOSTResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationDelegatedPOSTResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationDelegatedPOSTResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTRequestRecoveryCredentialCredentialInfo {
        #[serde(rename = "attestationData")]
        pub attestation_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationEnduserPOSTRequestRecoveryCredentialCredentialKind {
    #[default]
        RecoveryKey,
    }

    /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTRequestRecoveryCredential {
        #[serde(rename = "credentialInfo")]
        pub credential_info: RegistrationEnduserPOSTRequestRecoveryCredentialCredentialInfo,
        #[serde(rename = "credentialKind")]
        pub credential_kind: RegistrationEnduserPOSTRequestRecoveryCredentialCredentialKind,
        #[serde(rename = "credentialName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credential_name: Option<String>,
        #[serde(rename = "encryptedPrivateKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encrypted_private_key: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTRequest {
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: serde_json::Value,
        /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
        #[serde(rename = "recoveryCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recovery_credential: Option<RegistrationEnduserPOSTRequestRecoveryCredential>,
        #[serde(rename = "secondFactorCredential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub second_factor_credential: Option<serde_json::Value>,
        pub wallets: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTResponseAuthentication {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationEnduserPOSTResponseCredentialKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTResponseCredential {
        pub kind: RegistrationEnduserPOSTResponseCredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTResponseUser {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTResponse {
        pub authentication: RegistrationEnduserPOSTResponseAuthentication,
        pub credential: RegistrationEnduserPOSTResponseCredential,
        pub user: RegistrationEnduserPOSTResponseUser,
        pub wallets: Vec<Wallet>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(rename = "registrationCode")]
        pub registration_code: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationInitPOSTResponseAttestation {
        #[serde(rename = "none")]
    #[default]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationInitPOSTResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationInitPOSTResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationInitPOSTResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationInitPOSTResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationInitPOSTResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationInitPOSTResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponse {
        pub attestation: RegistrationInitPOSTResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationInitPOSTResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationInitPOSTResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationInitPOSTResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationInitPOSTResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationSocialPOSTRequestSocialLoginProviderKind {
    #[default]
        Oidc,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTRequest {
        #[serde(rename = "idToken")]
        pub id_token: String,
        #[serde(rename = "orgId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub org_id: Option<String>,
        #[serde(rename = "socialLoginProviderKind")]
        pub social_login_provider_kind: RegistrationSocialPOSTRequestSocialLoginProviderKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationSocialPOSTResponseAttestation {
        #[serde(rename = "none")]
    #[default]
        None,
        #[serde(rename = "indirect")]
        Indirect,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "enterprise")]
        Enterprise,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationSocialPOSTResponseAuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationSocialPOSTResponseAuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationSocialPOSTResponseAuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponseAuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationSocialPOSTResponseAuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationSocialPOSTResponseAuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationSocialPOSTResponseAuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponseRp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponseSupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponseUser {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponse {
        pub attestation: RegistrationSocialPOSTResponseAttestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationSocialPOSTResponseAuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationSocialPOSTResponseRp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationSocialPOSTResponseSupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationSocialPOSTResponseUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsGETResponse {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsPOSTRequest {
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
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsPOSTResponseUserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsPOSTResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsPOSTResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsPOSTResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsPOSTResponseUserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdGETResponseUserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdGETResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdGETResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdGETResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdGETResponseUserInfo,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdPUTRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdPUTResponseUserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdPUTResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdPUTResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdPUTResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdPUTResponseUserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdDELETEResponseUserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdDELETEResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdDELETEResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdDELETEResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdDELETEResponseUserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdActivatePUTResponseUserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdActivatePUTResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdActivatePUTResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdActivatePUTResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdActivatePUTResponseUserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdDeactivatePUTResponseUserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdDeactivatePUTResponseUserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdDeactivatePUTResponseUserInfoKind,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdDeactivatePUTResponse {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdDeactivatePUTResponseUserInfo,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct UsersGETResponse {
        pub items: Vec<User>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// The kind of user being created. 
    ///       In this endpoint it can only be "`CustomerEmployee`" (creating an "`EndUser`" is done through the [Delegated Registration](https://docs.dfns.co/api-reference/auth/registration-flows#delegated-users-registration-flow) endpoint)
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum UsersPOSTRequestKind {
    #[default]
        CustomerEmployee,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct UsersPOSTRequest {
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
        pub kind: UsersPOSTRequestKind,
        #[serde(rename = "publicKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub public_key: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct UsersUserIdPUTRequest {
        #[serde(rename = "isSSORequired")]
        pub is_ssorequired: bool,
    }

}

pub mod exchanges {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangesGETResponse {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangesPOSTRequestKind {
    #[default]
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangesPOSTRequestReadConfiguration {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub otp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[serde(rename = "privateApiKey")]
        pub private_api_key: String,
        #[serde(rename = "publicApiKey")]
        pub public_api_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangesPOSTRequestWriteConfiguration {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub otp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[serde(rename = "privateApiKey")]
        pub private_api_key: String,
        #[serde(rename = "publicApiKey")]
        pub public_api_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangesPOSTRequest {
        pub kind: ExchangesPOSTRequestKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "readConfiguration")]
        pub read_configuration: ExchangesPOSTRequestReadConfiguration,
        #[serde(rename = "writeConfiguration")]
        pub write_configuration: ExchangesPOSTRequestWriteConfiguration,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangesPOSTResponseKind {
    #[default]
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangesPOSTResponse {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: ExchangesPOSTResponseKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangeIdGETResponseKind {
    #[default]
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdGETResponse {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: ExchangeIdGETResponseKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangeIdDELETEResponseDeleted {
    #[default]
    ExchangeIdDELETEResponseDeleted
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdDELETEResponse {
        pub deleted: ExchangeIdDELETEResponseDeleted,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsGETResponse {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdAssetsGETResponse {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangeIdAccountsAccountIdDepositsPOSTResponseKind {
    #[default]
        Withdrawal,
        Deposit,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdDepositsPOSTResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdDepositsPOSTResponse {
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
        pub kind: ExchangeIdAccountsAccountIdDepositsPOSTResponseKind,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: ExchangeIdAccountsAccountIdDepositsPOSTResponseRequester,
        #[serde(rename = "transferId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_id: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangeIdAccountsAccountIdWithdrawalsPOSTResponseKind {
    #[default]
        Withdrawal,
        Deposit,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdWithdrawalsPOSTResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse {
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
        pub kind: ExchangeIdAccountsAccountIdWithdrawalsPOSTResponseKind,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: ExchangeIdAccountsAccountIdWithdrawalsPOSTResponseRequester,
        #[serde(rename = "transferId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_id: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

}

pub mod feesponsors {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeesponsorsGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeesponsorsPOSTRequest {
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

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeesponsorsPOSTResponseStatus {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeesponsorsPOSTResponse {
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
        pub status: FeesponsorsPOSTResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeeSponsorIdGETResponseStatus {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdGETResponse {
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
        pub status: FeeSponsorIdGETResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeeSponsorIdDELETEResponseStatus {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdDELETEResponse {
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
        pub status: FeeSponsorIdDELETEResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeeSponsorIdActivatePUTResponseStatus {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdActivatePUTResponse {
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
        pub status: FeeSponsorIdActivatePUTResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeeSponsorIdDeactivatePUTResponseStatus {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdDeactivatePUTResponse {
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
        pub status: FeeSponsorIdDeactivatePUTResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdFeesGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

}

pub mod keys {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeysGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeysPOSTRequestCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeysPOSTRequestScheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeysPOSTRequest {
        pub curve: KeysPOSTRequestCurve,
        #[serde(rename = "delayDelegation")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delay_delegation: Option<bool>,
        #[serde(rename = "delegateTo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delegate_to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub scheme: KeysPOSTRequestScheme,
        #[serde(rename = "storeId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub store_id: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeysPOSTResponseCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeysPOSTResponseScheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeysPOSTResponseStatus {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeysPOSTResponse {
        pub curve: KeysPOSTResponseCurve,
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
        pub scheme: KeysPOSTResponseScheme,
        pub status: KeysPOSTResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ImportPOSTRequestCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ImportPOSTRequest {
        pub curve: ImportPOSTRequestCurve,
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        #[serde(rename = "minSigners")]
        pub min_signers: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub protocol: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ImportPOSTResponseCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ImportPOSTResponseScheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ImportPOSTResponseStatus {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ImportPOSTResponse {
        pub curve: ImportPOSTResponseCurve,
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
        pub scheme: ImportPOSTResponseScheme,
        pub status: ImportPOSTResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdGETResponseCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdGETResponseScheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdGETResponseStatus {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdGETResponseStoreKind {
    #[default]
        Hsm,
        Mpc,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdGETResponseStore {
        pub id: String,
        #[serde(rename = "keyId")]
        pub key_id: String,
        pub kind: KeyIdGETResponseStoreKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdGETResponse {
        pub curve: KeyIdGETResponseCurve,
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
        pub scheme: KeyIdGETResponseScheme,
        pub status: KeyIdGETResponseStatus,
        pub store: KeyIdGETResponseStore,
        pub wallets: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdPUTRequest {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdPUTResponseCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdPUTResponseScheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdPUTResponseStatus {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdPUTResponse {
        pub curve: KeyIdPUTResponseCurve,
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
        pub scheme: KeyIdPUTResponseScheme,
        pub status: KeyIdPUTResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdDELETEResponseCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdDELETEResponseScheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdDELETEResponseStatus {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDELETEResponse {
        pub curve: KeyIdDELETEResponseCurve,
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
        pub scheme: KeyIdDELETEResponseScheme,
        pub status: KeyIdDELETEResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDelegatePOSTRequest {
        #[serde(rename = "delegateTo")]
        pub delegate_to: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdDelegatePOSTResponseStatus {
    #[default]
        Delegated,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDelegatePOSTResponse {
        #[serde(rename = "keyId")]
        pub key_id: String,
        pub status: KeyIdDelegatePOSTResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDerivePOSTRequest {
        pub domain: String,
        pub seed: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDerivePOSTResponse {
        pub output: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdExportPOSTRequest {
        #[serde(rename = "encryptionKey")]
        pub encryption_key: String,
        #[serde(rename = "supportedSchemes")]
        pub supported_schemes: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdExportPOSTResponseCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdExportPOSTResponseProtocol {
    #[default]
        CGGMP24,
        FROST,
        #[serde(rename = "FROST_BITCOIN")]
        FROSTBITCOIN,
        #[serde(rename = "GLOW20_DH")]
        GLOW20DH,
        KU23,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdExportPOSTResponse {
        pub curve: KeyIdExportPOSTResponseCurve,
        /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        /// The TSS threshold of the wallet private signing key shares
        #[serde(rename = "minSigners")]
        pub min_signers: f64,
        pub protocol: KeyIdExportPOSTResponseProtocol,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "keyId")]
        pub key_id: String,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesPOSTResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesPOSTResponseSignature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdSignaturesPOSTResponseStatus {
    #[default]
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesPOSTResponse {
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
        pub requester: KeyIdSignaturesPOSTResponseRequester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<KeyIdSignaturesPOSTResponseSignature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: KeyIdSignaturesPOSTResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesSignatureIdGETResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesSignatureIdGETResponseSignature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdSignaturesSignatureIdGETResponseStatus {
    #[default]
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesSignatureIdGETResponse {
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
        pub requester: KeyIdSignaturesSignatureIdGETResponseRequester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<KeyIdSignaturesSignatureIdGETResponseSignature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: KeyIdSignaturesSignatureIdGETResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
    }

}

pub mod keystores {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeystoresGETResponse {
        pub items: Vec<serde_json::Value>,
    }

}

pub mod networks {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ReadContractPOSTResponseKind {
    #[default]
        Evm,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ReadContractPOSTResponse {
        pub data: String,
        pub kind: ReadContractPOSTResponseKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct NetworkValidatorsGETResponse {
        /// Current page items.
        pub items: Vec<CantonValidator>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct NetworkValidatorsValidatorIdPUTRequestLedgerOauth2 {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct NetworkValidatorsValidatorIdPUTRequestLedger {
        /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
        pub oauth2: NetworkValidatorsValidatorIdPUTRequestLedgerOauth2,
        /// URL to reach the API at this address. The calls will be originating from our IP addresses (see [Dfns Environments](https://docs.dfns.co/api-reference/environments))
        pub url: String,
    }

    /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct NetworkValidatorsValidatorIdPUTRequestValidatorOauth2 {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct NetworkValidatorsValidatorIdPUTRequestValidator {
        /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
        pub oauth2: NetworkValidatorsValidatorIdPUTRequestValidatorOauth2,
        /// URL to reach the API at this address. The calls will be originating from our IP addresses (see [Dfns Environments](https://docs.dfns.co/api-reference/environments))
        pub url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct NetworkValidatorsValidatorIdPUTRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ledger: Option<NetworkValidatorsValidatorIdPUTRequestLedger>,
        /// Nickname for this validator.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validator: Option<NetworkValidatorsValidatorIdPUTRequestValidator>,
    }

}

pub mod permissions {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionsGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionsPOSTRequest {
        pub name: String,
        pub operations: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PermissionsPOSTResponseStatus {
    #[default]
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionsPOSTResponse {
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
        pub status: PermissionsPOSTResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdPUTRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operations: Option<Vec<serde_json::Value>>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PermissionIdPUTResponseStatus {
    #[default]
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdPUTResponse {
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
        pub status: PermissionIdPUTResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdArchivePUTRequest {
        #[serde(rename = "isArchived")]
        pub is_archived: bool,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PermissionIdArchivePUTResponseStatus {
    #[default]
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdArchivePUTResponse {
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
        pub status: PermissionIdArchivePUTResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdAssignmentsGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdAssignmentsPOSTRequest {
        #[serde(rename = "identityId")]
        pub identity_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdAssignmentsPOSTResponse {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct SignersGETResponse {
        pub clusters: Vec<serde_json::Value>,
    }

}

pub mod staking {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct StakesGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct StakesStakeIdActionsGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct StakesStakeIdRewardsGETResponse {
        pub balance: String,
        pub symbol: String,
    }

}

pub mod swaps {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct SwapsGETResponse {
        /// Current page items.
        pub items: Vec<Swap>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Provided for this swap. Used for attesting that the swap is being created with the same parameters as the quote.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum SwapsPOSTRequestProvider {
    #[default]
        UniswapX,
        UniswapClassic,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct SwapsPOSTRequest {
        pub provider: SwapsPOSTRequestProvider,
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

    /// Swap provider.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum QuotesPOSTRequestProvider {
    #[default]
        UniswapX,
        UniswapClassic,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct QuotesPOSTRequest {
        pub provider: QuotesPOSTRequestProvider,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PoliciesGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PolicyApprovalsGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PolicyApprovalsApprovalIdGETResponseStatus {
    #[default]
        Pending,
        Approved,
        Denied,
        Expired,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PolicyApprovalsApprovalIdGETResponse {
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
        pub status: PolicyApprovalsApprovalIdGETResponseStatus,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PolicyApprovalsApprovalIdDecisionsPOSTRequestValue {
    #[default]
        Approved,
        Denied,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PolicyApprovalsApprovalIdDecisionsPOSTRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        pub value: PolicyApprovalsApprovalIdDecisionsPOSTRequestValue,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PolicyApprovalsApprovalIdDecisionsPOSTResponseStatus {
    #[default]
        Pending,
        Approved,
        Denied,
        Expired,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PolicyApprovalsApprovalIdDecisionsPOSTResponse {
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
        pub status: PolicyApprovalsApprovalIdDecisionsPOSTResponseStatus,
    }

}

pub mod wallets {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletsGETResponse {
        pub items: Vec<Wallet>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Network used for the wallet.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletsPOSTRequestNetwork {
    #[default]
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

    /// Use this to specify the new key curve for networks that support multiple key formats.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletsPOSTRequestSigningKeyCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    /// Use this to specify the new key scheme for networks that support multiple key formats. ex. use `Schnorr` to create a `Bitcoin Taproot` wallet.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletsPOSTRequestSigningKeyScheme {
    #[default]
        ECDSA,
        EdDSA,
        Schnorr,
    }

    /// Options for the wallet's underlying key
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletsPOSTRequestSigningKey {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub curve: Option<WalletsPOSTRequestSigningKeyCurve>,
        /// Use this parameter to create a wallet from an existing key. This enables one key to be used across multiple networks and have the same address if networks share the same address format, ex. `Ethereum` and `Polygon`. If specified, requires the `Keys:Reuse` permission. If the key is delegated to an end user, then the new wallet will be automatically delegated to the same end user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scheme: Option<WalletsPOSTRequestSigningKeyScheme>,
        /// Use this to specify the key store the key material is saved to.
        #[serde(rename = "storeId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub store_id: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletsPOSTRequest {
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
        pub network: WalletsPOSTRequestNetwork,
        /// Options for the wallet's underlying key
        #[serde(rename = "signingKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signing_key: Option<WalletsPOSTRequestSigningKey>,
        /// List of tags to be created for this wallet. If specified, requires the `Wallets:Tags:Add` permission, like the [Tag Wallet](https://docs.dfns.co/api-reference/wallets/tag-wallet) endpoint.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<String>>,
        /// Id of the validator on which the wallet is created for Canton networks
        #[serde(rename = "validatorId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validator_id: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ImportPOSTRequestCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ImportPOSTRequestNetwork {
    #[default]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ImportPOSTRequest {
        pub curve: ImportPOSTRequestCurve,
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(rename = "minSigners")]
        pub min_signers: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub network: ImportPOSTRequestNetwork,
        pub protocol: serde_json::Value,
        /// Id of the validator on which the wallet is created
        #[serde(rename = "validatorId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validator_id: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdPUTRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdAssetsGETResponseNetWorth {
        #[serde(rename = "USD")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub usd: Option<f64>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdAssetsGETResponse {
        pub assets: Vec<serde_json::Value>,
        #[serde(rename = "netWorth")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub net_worth: Option<WalletIdAssetsGETResponseNetWorth>,
        pub network: Network,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdDelegatePOSTRequest {
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdDelegatePOSTResponseStatus {
    #[default]
        Delegated,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdDelegatePOSTResponse {
        pub status: WalletIdDelegatePOSTResponseStatus,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdExportPOSTRequest {
        #[serde(rename = "encryptionKey")]
        pub encryption_key: String,
        #[serde(rename = "supportedSchemes")]
        pub supported_schemes: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdExportPOSTResponseCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdExportPOSTResponseProtocol {
    #[default]
        CGGMP24,
        FROST,
        #[serde(rename = "FROST_BITCOIN")]
        FROSTBITCOIN,
        #[serde(rename = "GLOW20_DH")]
        GLOW20DH,
        KU23,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdExportPOSTResponse {
        pub curve: WalletIdExportPOSTResponseCurve,
        /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        /// The TSS threshold of the wallet private signing key shares
        #[serde(rename = "minSigners")]
        pub min_signers: f64,
        pub protocol: WalletIdExportPOSTResponseProtocol,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdNftsGETResponse {
        pub network: Network,
        pub nfts: Vec<serde_json::Value>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdOffersGETResponse {
        /// Current page items.
        pub items: Vec<Offer>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "keyId")]
        pub key_id: String,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdSignaturesPOSTResponseNetwork {
    #[default]
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesPOSTResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesPOSTResponseSignature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdSignaturesPOSTResponseStatus {
    #[default]
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesPOSTResponse {
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
        pub network: WalletIdSignaturesPOSTResponseNetwork,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: WalletIdSignaturesPOSTResponseRequester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<WalletIdSignaturesPOSTResponseSignature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: WalletIdSignaturesPOSTResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesSignatureIdGETResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesSignatureIdGETResponseSignature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdSignaturesSignatureIdGETResponseStatus {
    #[default]
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesSignatureIdGETResponse {
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
        pub requester: WalletIdSignaturesSignatureIdGETResponseRequester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<WalletIdSignaturesSignatureIdGETResponseSignature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: WalletIdSignaturesSignatureIdGETResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTagsPUTRequest {
        pub tags: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTagsDELETERequest {
        /// List of tags.
        pub tags: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsPOSTResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdTransactionsPOSTResponseStatus {
    #[default]
        Pending,
        Executing,
        Broadcasted,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsPOSTResponse {
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
        pub requester: WalletIdTransactionsPOSTResponseRequester,
        pub status: WalletIdTransactionsPOSTResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsTransactionIdGETResponseRequester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdTransactionsTransactionIdGETResponseStatus {
    #[default]
        Pending,
        Executing,
        Broadcasted,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsTransactionIdGETResponse {
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
        pub requester: WalletIdTransactionsTransactionIdGETResponseRequester,
        pub status: WalletIdTransactionsTransactionIdGETResponseStatus,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransfersGETResponse {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhooksGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhooksPOSTRequestStatus {
    #[default]
        Enabled,
        Disabled,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhooksPOSTRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// All events this webhook is subscribed to.
        pub events: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<WebhooksPOSTRequestStatus>,
        /// Webhook url
        pub url: String,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhooksPOSTResponseStatus {
    #[default]
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhooksPOSTResponse {
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
        pub status: WebhooksPOSTResponseStatus,
        /// Webhook url
        pub url: String,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhookIdGETResponseStatus {
    #[default]
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdGETResponse {
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
        pub status: WebhookIdGETResponseStatus,
        /// Webhook url
        pub url: String,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhookIdPUTRequestStatus {
    #[default]
        Enabled,
        Disabled,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdPUTRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// All events this webhook is subscribed to.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub events: Option<Vec<serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<WebhookIdPUTRequestStatus>,
        /// Webhook url
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhookIdPUTResponseStatus {
    #[default]
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdPUTResponse {
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
        pub status: WebhookIdPUTResponseStatus,
        /// Webhook url
        pub url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhookIdDELETEResponseDeleted {
    #[default]
    WebhookIdDELETEResponseDeleted
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdDELETEResponse {
        pub deleted: WebhookIdDELETEResponseDeleted,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdEventsGETResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Webhook event
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhookIdEventsWebhookEventIdGETResponseKind {
        #[serde(rename = "policy.triggered")]
    #[default]
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
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdEventsWebhookEventIdGETResponse {
        pub data: serde_json::Value,
        /// ISO date string when event was raised
        pub date: String,
        /// Error message if any error happened during the webhook request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        /// WebhookEvent ID
        pub id: String,
        pub kind: WebhookIdEventsWebhookEventIdGETResponseKind,
        /// Status code of the webhook request
        pub status: String,
        /// Unix timestamp when the event was forwarded to the webhook url by our servers.
        #[serde(rename = "timestampSent")]
        pub timestamp_sent: i64,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdPingPOSTResponse {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        pub status: String,
    }

}

pub mod yields {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct YieldsGETResponse {
        /// Current page items.
        pub items: Vec<Yield>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct YieldIdActionsGETResponse {
        /// Current page items.
        pub items: Vec<YieldAction>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// The type of action being performed on the yield investment: Deposit to add funds or Withdraw to remove funds.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum YieldIdActionsPOSTRequestKind {
    #[default]
        Deposit,
        Withdraw,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum YieldIdActionsPOSTRequestSourceAssetKind {
    #[default]
        Erc20,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct YieldIdActionsPOSTRequestSourceAsset {
        pub amount: String,
        pub contract: String,
        pub kind: YieldIdActionsPOSTRequestSourceAssetKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum YieldIdActionsPOSTRequestTargetAssetKind {
    #[default]
        Erc20,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct YieldIdActionsPOSTRequestTargetAsset {
        pub amount: String,
        pub contract: String,
        pub kind: YieldIdActionsPOSTRequestTargetAssetKind,
    }

    /// Request body for creating a yield action. Different protocols may have different requirements.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct YieldIdActionsPOSTRequest {
        /// An optional external identifier provided by the client to ensure idempotency and prevent duplicate operations.
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        pub kind: YieldIdActionsPOSTRequestKind,
        /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you're willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%.
        #[serde(rename = "slippageBps")]
        pub slippage_bps: f64,
        #[serde(rename = "sourceAsset")]
        pub source_asset: YieldIdActionsPOSTRequestSourceAsset,
        #[serde(rename = "targetAsset")]
        pub target_asset: YieldIdActionsPOSTRequestTargetAsset,
    }

}

