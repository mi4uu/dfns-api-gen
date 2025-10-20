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
    pub enum LatestUnacceptedGETResponse200LatestAgreementAgreementType {
    #[default]
        PrivacyPolicy,
        TermsAndConditions,
        UniswapTermsOfService,
        UniswapPrivacyPolicy,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LatestUnacceptedGETResponse200LatestAgreement {
        #[serde(rename = "agreementType")]
        pub agreement_type: LatestUnacceptedGETResponse200LatestAgreementAgreementType,
        #[serde(rename = "agreementUrl")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub agreement_url: Option<String>,
        pub details: String,
        pub id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LatestUnacceptedGETResponse200 {
        #[serde(rename = "latestAgreement")]
        pub latest_agreement: LatestUnacceptedGETResponse200LatestAgreement,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct AgreementIdAcceptPOSTResponse200 {
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
    pub struct ActionPOSTResponse200 {
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
    pub struct ActionInitPOSTResponse200AllowCredentials {
        pub key: Vec<serde_json::Value>,
        #[serde(rename = "passwordProtectedKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password_protected_key: Option<Vec<serde_json::Value>>,
        pub webauthn: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ActionInitPOSTResponse200Attestation {
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
    pub struct ActionInitPOSTResponse200Rp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ActionInitPOSTResponse200UserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionInitPOSTResponse200 {
        #[serde(rename = "allowCredentials")]
        pub allow_credentials: ActionInitPOSTResponse200AllowCredentials,
        pub attestation: ActionInitPOSTResponse200Attestation,
        pub challenge: String,
        #[serde(rename = "challengeIdentifier")]
        pub challenge_identifier: String,
        #[serde(rename = "externalAuthenticationUrl")]
        pub external_authentication_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<ActionInitPOSTResponse200Rp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: Vec<serde_json::Value>,
        #[serde(rename = "userVerification")]
        pub user_verification: ActionInitPOSTResponse200UserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionLogsIdGETResponse200FirstFactorCredentialAssertion {
        #[serde(rename = "authenticatorData")]
        pub authenticator_data: String,
        #[serde(rename = "clientData")]
        pub client_data: String,
        pub signature: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionLogsIdGETResponse200FirstFactorCredential {
        pub assertion: ActionLogsIdGETResponse200FirstFactorCredentialAssertion,
        pub id: String,
        pub kind: String,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ActionLogsIdGETResponse200 {
        pub action: String,
        #[serde(rename = "actionToken")]
        pub action_token: String,
        #[serde(rename = "datePerformed")]
        pub date_performed: String,
        #[serde(rename = "firstFactorCredential")]
        pub first_factor_credential: ActionLogsIdGETResponse200FirstFactorCredential,
        pub id: String,
        #[serde(rename = "userId")]
        pub user_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct AppsGETResponse200 {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum AppsAppIdGETResponse200Kind {
    #[default]
        ServerSideApplication,
        ClientSideApplication,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct AppsAppIdGETResponse200 {
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
        pub kind: AppsAppIdGETResponse200Kind,
        pub name: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        #[serde(rename = "permissionAssignments")]
        pub permission_assignments: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsGETResponse200 {
        pub items: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CredentialsPOSTResponse200Kind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsPOSTResponse200 {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: CredentialsPOSTResponse200Kind,
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
    pub struct CredentialsActivatePUTResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsCodePOSTRequest {
        /// Code expiration, as an ISO-8601 datetime string or a unix timestamp
        pub expiration: serde_json::Value,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsCodePOSTResponse200 {
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
    pub enum CredentialsCodeVerifyPOSTResponse200Kind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CredentialsCodeVerifyPOSTResponse200 {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: CredentialsCodeVerifyPOSTResponse200Kind,
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
    pub struct CredentialsDeactivatePUTResponse200 {
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
    pub struct LoginCodePOSTResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginDelegatedPOSTRequest {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginDelegatedPOSTResponse200 {
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
    pub struct LoginInitPOSTResponse200AllowCredentials {
        pub key: Vec<serde_json::Value>,
        #[serde(rename = "passwordProtectedKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password_protected_key: Option<Vec<serde_json::Value>>,
        pub webauthn: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum LoginInitPOSTResponse200Attestation {
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
    pub struct LoginInitPOSTResponse200Rp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum LoginInitPOSTResponse200UserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct LoginInitPOSTResponse200 {
        #[serde(rename = "allowCredentials")]
        pub allow_credentials: LoginInitPOSTResponse200AllowCredentials,
        pub attestation: LoginInitPOSTResponse200Attestation,
        pub challenge: String,
        #[serde(rename = "challengeIdentifier")]
        pub challenge_identifier: String,
        #[serde(rename = "externalAuthenticationUrl")]
        pub external_authentication_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<LoginInitPOSTResponse200Rp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: Vec<serde_json::Value>,
        #[serde(rename = "userVerification")]
        pub user_verification: LoginInitPOSTResponse200UserVerification,
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
    pub struct LoginSocialPOSTResponse200 {
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
    pub struct LoginSsoPOSTResponse200 {
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
    pub struct LoginSsoInitPOSTResponse200 {
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
    pub struct LogoutPUTResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PatsGETResponse200 {
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
    pub enum PatsPOSTResponse200Kind {
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
    pub struct PatsPOSTResponse200 {
        #[serde(rename = "accessToken")]
        pub access_token: String,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsPOSTResponse200Kind,
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
    pub enum PatsTokenIdGETResponse200Kind {
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
    pub struct PatsTokenIdGETResponse200 {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdGETResponse200Kind,
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
    pub enum PatsTokenIdPUTResponse200Kind {
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
    pub struct PatsTokenIdPUTResponse200 {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdPUTResponse200Kind,
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
    pub enum PatsTokenIdDELETEResponse200Kind {
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
    pub struct PatsTokenIdDELETEResponse200 {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdDELETEResponse200Kind,
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
    pub enum PatsTokenIdActivatePUTResponse200Kind {
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
    pub struct PatsTokenIdActivatePUTResponse200 {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdActivatePUTResponse200Kind,
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
    pub enum PatsTokenIdDeactivatePUTResponse200Kind {
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
    pub struct PatsTokenIdDeactivatePUTResponse200 {
        #[serde(rename = "accessToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        #[serde(rename = "credId")]
        pub cred_id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        pub kind: PatsTokenIdDeactivatePUTResponse200Kind,
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
    pub enum RecoverUserPOSTResponse200CredentialKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTResponse200Credential {
        pub kind: RecoverUserPOSTResponse200CredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTResponse200User {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserPOSTResponse200 {
        pub credential: RecoverUserPOSTResponse200Credential,
        pub user: RecoverUserPOSTResponse200User,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserCodePOSTRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserCodePOSTResponse200 {
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTRequest {
        #[serde(rename = "credentialId")]
        pub credential_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserDelegatedPOSTResponse200Attestation {
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
    pub enum RecoverUserDelegatedPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserDelegatedPOSTResponse200AuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserDelegatedPOSTResponse200AuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponse200AuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RecoverUserDelegatedPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RecoverUserDelegatedPOSTResponse200AuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RecoverUserDelegatedPOSTResponse200AuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponse200Rp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponse200SupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponse200User {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserDelegatedPOSTResponse200 {
        #[serde(rename = "allowedRecoveryCredentials")]
        pub allowed_recovery_credentials: Vec<serde_json::Value>,
        pub attestation: RecoverUserDelegatedPOSTResponse200Attestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RecoverUserDelegatedPOSTResponse200AuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RecoverUserDelegatedPOSTResponse200Rp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RecoverUserDelegatedPOSTResponse200SupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RecoverUserDelegatedPOSTResponse200User,
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
    pub enum RecoverUserInitPOSTResponse200Attestation {
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
    pub enum RecoverUserInitPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserInitPOSTResponse200AuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RecoverUserInitPOSTResponse200AuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponse200AuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RecoverUserInitPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RecoverUserInitPOSTResponse200AuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RecoverUserInitPOSTResponse200AuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponse200Rp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponse200SupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponse200User {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RecoverUserInitPOSTResponse200 {
        #[serde(rename = "allowedRecoveryCredentials")]
        pub allowed_recovery_credentials: Vec<serde_json::Value>,
        pub attestation: RecoverUserInitPOSTResponse200Attestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RecoverUserInitPOSTResponse200AuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RecoverUserInitPOSTResponse200Rp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RecoverUserInitPOSTResponse200SupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RecoverUserInitPOSTResponse200User,
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
    pub enum RegistrationPOSTResponse200CredentialKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTResponse200Credential {
        pub kind: RegistrationPOSTResponse200CredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTResponse200User {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationPOSTResponse200 {
        pub credential: RegistrationPOSTResponse200Credential,
        pub user: RegistrationPOSTResponse200User,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationCodePUTRequest {
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationCodePUTResponse200 {
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
    pub enum RegistrationDelegatedPOSTResponse200Attestation {
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
    pub enum RegistrationDelegatedPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationDelegatedPOSTResponse200AuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationDelegatedPOSTResponse200AuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponse200AuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationDelegatedPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationDelegatedPOSTResponse200AuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationDelegatedPOSTResponse200AuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponse200Rp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponse200SupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponse200User {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationDelegatedPOSTResponse200 {
        pub attestation: RegistrationDelegatedPOSTResponse200Attestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationDelegatedPOSTResponse200AuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationDelegatedPOSTResponse200Rp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationDelegatedPOSTResponse200SupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationDelegatedPOSTResponse200User,
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
    pub struct RegistrationEnduserPOSTResponse200Authentication {
        pub token: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationEnduserPOSTResponse200CredentialKind {
    #[default]
        Fido2,
        Key,
        Password,
        Totp,
        RecoveryKey,
        PasswordProtectedKey,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTResponse200Credential {
        pub kind: RegistrationEnduserPOSTResponse200CredentialKind,
        pub name: String,
        pub uuid: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTResponse200User {
        pub id: String,
        #[serde(rename = "orgId")]
        pub org_id: String,
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationEnduserPOSTResponse200 {
        pub authentication: RegistrationEnduserPOSTResponse200Authentication,
        pub credential: RegistrationEnduserPOSTResponse200Credential,
        pub user: RegistrationEnduserPOSTResponse200User,
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
    pub enum RegistrationInitPOSTResponse200Attestation {
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
    pub enum RegistrationInitPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationInitPOSTResponse200AuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationInitPOSTResponse200AuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponse200AuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationInitPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationInitPOSTResponse200AuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationInitPOSTResponse200AuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponse200Rp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponse200SupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponse200User {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationInitPOSTResponse200 {
        pub attestation: RegistrationInitPOSTResponse200Attestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationInitPOSTResponse200AuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationInitPOSTResponse200Rp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationInitPOSTResponse200SupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationInitPOSTResponse200User,
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
    pub enum RegistrationSocialPOSTResponse200Attestation {
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
    pub enum RegistrationSocialPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment {
        #[serde(rename = "platform")]
    #[default]
        Platform,
        #[serde(rename = "cross-platform")]
        CrossPlatform,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationSocialPOSTResponse200AuthenticatorSelectionResidentKey {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum RegistrationSocialPOSTResponse200AuthenticatorSelectionUserVerification {
        #[serde(rename = "required")]
    #[default]
        Required,
        #[serde(rename = "preferred")]
        Preferred,
        #[serde(rename = "discouraged")]
        Discouraged,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponse200AuthenticatorSelection {
        #[serde(rename = "authenticatorAttachment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authenticator_attachment: Option<RegistrationSocialPOSTResponse200AuthenticatorSelectionAuthenticatorAttachment>,
        #[serde(rename = "requireResidentKey")]
        pub require_resident_key: bool,
        #[serde(rename = "residentKey")]
        pub resident_key: RegistrationSocialPOSTResponse200AuthenticatorSelectionResidentKey,
        #[serde(rename = "userVerification")]
        pub user_verification: RegistrationSocialPOSTResponse200AuthenticatorSelectionUserVerification,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponse200Rp {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponse200SupportedCredentialKinds {
        #[serde(rename = "firstFactor")]
        pub first_factor: Vec<String>,
        #[serde(rename = "secondFactor")]
        pub second_factor: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponse200User {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct RegistrationSocialPOSTResponse200 {
        pub attestation: RegistrationSocialPOSTResponse200Attestation,
        #[serde(rename = "authenticatorSelection")]
        pub authenticator_selection: RegistrationSocialPOSTResponse200AuthenticatorSelection,
        pub challenge: String,
        #[serde(rename = "excludeCredentials")]
        pub exclude_credentials: Vec<serde_json::Value>,
        #[serde(rename = "otpUrl")]
        pub otp_url: String,
        #[serde(rename = "pubKeyCredParams")]
        pub pub_key_cred_params: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rp: Option<RegistrationSocialPOSTResponse200Rp>,
        #[serde(rename = "supportedCredentialKinds")]
        pub supported_credential_kinds: RegistrationSocialPOSTResponse200SupportedCredentialKinds,
        #[serde(rename = "temporaryAuthenticationToken")]
        pub temporary_authentication_token: String,
        pub user: RegistrationSocialPOSTResponse200User,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsGETResponse200 {
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
    pub enum ServiceAccountsPOSTResponse200UserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsPOSTResponse200UserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsPOSTResponse200UserInfoKind,
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
    pub struct ServiceAccountsPOSTResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsPOSTResponse200UserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdGETResponse200UserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdGETResponse200UserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdGETResponse200UserInfoKind,
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
    pub struct ServiceAccountsServiceAccountIdGETResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdGETResponse200UserInfo,
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
    pub enum ServiceAccountsServiceAccountIdPUTResponse200UserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdPUTResponse200UserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdPUTResponse200UserInfoKind,
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
    pub struct ServiceAccountsServiceAccountIdPUTResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdPUTResponse200UserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdDELETEResponse200UserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdDELETEResponse200UserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdDELETEResponse200UserInfoKind,
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
    pub struct ServiceAccountsServiceAccountIdDELETEResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdDELETEResponse200UserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdActivatePUTResponse200UserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdActivatePUTResponse200UserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdActivatePUTResponse200UserInfoKind,
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
    pub struct ServiceAccountsServiceAccountIdActivatePUTResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdActivatePUTResponse200UserInfo,
    }

    /// User kind.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ServiceAccountsServiceAccountIdDeactivatePUTResponse200UserInfoKind {
    #[default]
        CustomerEmployee,
        EndUser,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ServiceAccountsServiceAccountIdDeactivatePUTResponse200UserInfo {
        #[serde(rename = "credentialUuid")]
        pub credential_uuid: String,
        #[serde(rename = "isActive")]
        pub is_active: bool,
        #[serde(rename = "isRegistered")]
        pub is_registered: bool,
        #[serde(rename = "isServiceAccount")]
        pub is_service_account: bool,
        pub kind: ServiceAccountsServiceAccountIdDeactivatePUTResponse200UserInfoKind,
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
    pub struct ServiceAccountsServiceAccountIdDeactivatePUTResponse200 {
        #[serde(rename = "accessTokens")]
        pub access_tokens: Vec<serde_json::Value>,
        #[serde(rename = "userInfo")]
        pub user_info: ServiceAccountsServiceAccountIdDeactivatePUTResponse200UserInfo,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct UsersGETResponse200 {
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
    pub struct ExchangesGETResponse200 {
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
    pub enum ExchangesPOSTResponse200Kind {
    #[default]
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangesPOSTResponse200 {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: ExchangesPOSTResponse200Kind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangeIdGETResponse200Kind {
    #[default]
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdGETResponse200 {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: ExchangeIdGETResponse200Kind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangeIdDELETEResponse200Deleted {
    #[default]
    ExchangeIdDELETEResponse200Deleted
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdDELETEResponse200 {
        pub deleted: ExchangeIdDELETEResponse200Deleted,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsGETResponse200 {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdAssetsGETResponse200 {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangeIdAccountsAccountIdDepositsPOSTResponse200Kind {
    #[default]
        Withdrawal,
        Deposit,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdDepositsPOSTResponse200Requester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdDepositsPOSTResponse200 {
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
        pub kind: ExchangeIdAccountsAccountIdDepositsPOSTResponse200Kind,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: ExchangeIdAccountsAccountIdDepositsPOSTResponse200Requester,
        #[serde(rename = "transferId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_id: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200Kind {
    #[default]
        Withdrawal,
        Deposit,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200Requester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200 {
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
        pub kind: ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200Kind,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: ExchangeIdAccountsAccountIdWithdrawalsPOSTResponse200Requester,
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
    pub struct FeesponsorsGETResponse200 {
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
    pub enum FeesponsorsPOSTResponse200Status {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeesponsorsPOSTResponse200 {
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
        pub status: FeesponsorsPOSTResponse200Status,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeeSponsorIdGETResponse200Status {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdGETResponse200 {
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
        pub status: FeeSponsorIdGETResponse200Status,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeeSponsorIdDELETEResponse200Status {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdDELETEResponse200 {
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
        pub status: FeeSponsorIdDELETEResponse200Status,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeeSponsorIdActivatePUTResponse200Status {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdActivatePUTResponse200 {
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
        pub status: FeeSponsorIdActivatePUTResponse200Status,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    /// Fee sponsor status.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum FeeSponsorIdDeactivatePUTResponse200Status {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdDeactivatePUTResponse200 {
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
        pub status: FeeSponsorIdDeactivatePUTResponse200Status,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct FeeSponsorIdFeesGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

}

pub mod keys {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeysGETResponse200 {
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
    pub enum KeysPOSTResponse200Curve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeysPOSTResponse200Scheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeysPOSTResponse200Status {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeysPOSTResponse200 {
        pub curve: KeysPOSTResponse200Curve,
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
        pub scheme: KeysPOSTResponse200Scheme,
        pub status: KeysPOSTResponse200Status,
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
    pub enum ImportPOSTResponse200Curve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ImportPOSTResponse200Scheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ImportPOSTResponse200Status {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ImportPOSTResponse200 {
        pub curve: ImportPOSTResponse200Curve,
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
        pub scheme: ImportPOSTResponse200Scheme,
        pub status: ImportPOSTResponse200Status,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdGETResponse200Curve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdGETResponse200Scheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdGETResponse200Status {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdGETResponse200StoreKind {
    #[default]
        Hsm,
        Mpc,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdGETResponse200Store {
        pub id: String,
        #[serde(rename = "keyId")]
        pub key_id: String,
        pub kind: KeyIdGETResponse200StoreKind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdGETResponse200 {
        pub curve: KeyIdGETResponse200Curve,
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
        pub scheme: KeyIdGETResponse200Scheme,
        pub status: KeyIdGETResponse200Status,
        pub store: KeyIdGETResponse200Store,
        pub wallets: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdPUTRequest {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdPUTResponse200Curve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdPUTResponse200Scheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdPUTResponse200Status {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdPUTResponse200 {
        pub curve: KeyIdPUTResponse200Curve,
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
        pub scheme: KeyIdPUTResponse200Scheme,
        pub status: KeyIdPUTResponse200Status,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdDELETEResponse200Curve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdDELETEResponse200Scheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdDELETEResponse200Status {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDELETEResponse200 {
        pub curve: KeyIdDELETEResponse200Curve,
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
        pub scheme: KeyIdDELETEResponse200Scheme,
        pub status: KeyIdDELETEResponse200Status,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDelegatePOSTRequest {
        #[serde(rename = "delegateTo")]
        pub delegate_to: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdDelegatePOSTResponse200Status {
    #[default]
        Delegated,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDelegatePOSTResponse200 {
        #[serde(rename = "keyId")]
        pub key_id: String,
        pub status: KeyIdDelegatePOSTResponse200Status,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDerivePOSTRequest {
        pub domain: String,
        pub seed: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdDerivePOSTResponse200 {
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
    pub enum KeyIdExportPOSTResponse200Curve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdExportPOSTResponse200Protocol {
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
    pub struct KeyIdExportPOSTResponse200 {
        pub curve: KeyIdExportPOSTResponse200Curve,
        /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        /// The TSS threshold of the wallet private signing key shares
        #[serde(rename = "minSigners")]
        pub min_signers: f64,
        pub protocol: KeyIdExportPOSTResponse200Protocol,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "keyId")]
        pub key_id: String,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesPOSTResponse200Requester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesPOSTResponse200Signature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdSignaturesPOSTResponse200Status {
    #[default]
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesPOSTResponse200 {
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
        pub requester: KeyIdSignaturesPOSTResponse200Requester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<KeyIdSignaturesPOSTResponse200Signature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: KeyIdSignaturesPOSTResponse200Status,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesSignatureIdGETResponse200Requester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesSignatureIdGETResponse200Signature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum KeyIdSignaturesSignatureIdGETResponse200Status {
    #[default]
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeyIdSignaturesSignatureIdGETResponse200 {
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
        pub requester: KeyIdSignaturesSignatureIdGETResponse200Requester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<KeyIdSignaturesSignatureIdGETResponse200Signature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: KeyIdSignaturesSignatureIdGETResponse200Status,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
    }

}

pub mod keystores {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct KeystoresGETResponse200 {
        pub items: Vec<serde_json::Value>,
    }

}

pub mod networks {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum ReadContractPOSTResponse200Kind {
    #[default]
        Evm,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ReadContractPOSTResponse200 {
        pub data: String,
        pub kind: ReadContractPOSTResponse200Kind,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct NetworkValidatorsGETResponse200 {
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
    pub struct PermissionsGETResponse200 {
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
    pub enum PermissionsPOSTResponse200Status {
    #[default]
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionsPOSTResponse200 {
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
        pub status: PermissionsPOSTResponse200Status,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdPUTRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operations: Option<Vec<serde_json::Value>>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PermissionIdPUTResponse200Status {
    #[default]
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdPUTResponse200 {
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
        pub status: PermissionIdPUTResponse200Status,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdArchivePUTRequest {
        #[serde(rename = "isArchived")]
        pub is_archived: bool,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PermissionIdArchivePUTResponse200Status {
    #[default]
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdArchivePUTResponse200 {
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
        pub status: PermissionIdArchivePUTResponse200Status,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PermissionIdAssignmentsGETResponse200 {
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
    pub struct PermissionIdAssignmentsPOSTResponse200 {
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
    pub struct SignersGETResponse200 {
        pub clusters: Vec<serde_json::Value>,
    }

}

pub mod staking {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct StakesGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct StakesStakeIdActionsGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct StakesStakeIdRewardsGETResponse200 {
        pub balance: String,
        pub symbol: String,
    }

}

pub mod swaps {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct SwapsGETResponse200 {
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
    pub struct PoliciesGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PolicyApprovalsGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum PolicyApprovalsApprovalIdGETResponse200Status {
    #[default]
        Pending,
        Approved,
        Denied,
        Expired,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PolicyApprovalsApprovalIdGETResponse200 {
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
        pub status: PolicyApprovalsApprovalIdGETResponse200Status,
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
    pub enum PolicyApprovalsApprovalIdDecisionsPOSTResponse200Status {
    #[default]
        Pending,
        Approved,
        Denied,
        Expired,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct PolicyApprovalsApprovalIdDecisionsPOSTResponse200 {
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
        pub status: PolicyApprovalsApprovalIdDecisionsPOSTResponse200Status,
    }

}

pub mod wallets {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletsGETResponse200 {
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdPUTRequest {
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdAssetsGETResponse200NetWorth {
        #[serde(rename = "USD")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub usd: Option<f64>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdAssetsGETResponse200 {
        pub assets: Vec<serde_json::Value>,
        #[serde(rename = "netWorth")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub net_worth: Option<WalletIdAssetsGETResponse200NetWorth>,
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
    pub enum WalletIdDelegatePOSTResponse200Status {
    #[default]
        Delegated,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdDelegatePOSTResponse200 {
        pub status: WalletIdDelegatePOSTResponse200Status,
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
    pub enum WalletIdExportPOSTResponse200Curve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdExportPOSTResponse200Protocol {
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
    pub struct WalletIdExportPOSTResponse200 {
        pub curve: WalletIdExportPOSTResponse200Curve,
        /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
        #[serde(rename = "encryptedKeyShares")]
        pub encrypted_key_shares: Vec<serde_json::Value>,
        /// The TSS threshold of the wallet private signing key shares
        #[serde(rename = "minSigners")]
        pub min_signers: f64,
        pub protocol: WalletIdExportPOSTResponse200Protocol,
        #[serde(rename = "publicKey")]
        pub public_key: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdNftsGETResponse200 {
        pub network: Network,
        pub nfts: Vec<serde_json::Value>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdOffersGETResponse200 {
        /// Current page items.
        pub items: Vec<Offer>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "keyId")]
        pub key_id: String,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdSignaturesPOSTResponse200Network {
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
    pub struct WalletIdSignaturesPOSTResponse200Requester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesPOSTResponse200Signature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdSignaturesPOSTResponse200Status {
    #[default]
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesPOSTResponse200 {
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
        pub network: WalletIdSignaturesPOSTResponse200Network,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        #[serde(rename = "requestBody")]
        pub request_body: serde_json::Value,
        pub requester: WalletIdSignaturesPOSTResponse200Requester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<WalletIdSignaturesPOSTResponse200Signature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: WalletIdSignaturesPOSTResponse200Status,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesSignatureIdGETResponse200Requester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesSignatureIdGETResponse200Signature {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoded: Option<String>,
        pub r: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recid: Option<f64>,
        pub s: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdSignaturesSignatureIdGETResponse200Status {
    #[default]
        Pending,
        Executing,
        Signed,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdSignaturesSignatureIdGETResponse200 {
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
        pub requester: WalletIdSignaturesSignatureIdGETResponse200Requester,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<WalletIdSignaturesSignatureIdGETResponse200Signature>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<serde_json::Value>>,
        #[serde(rename = "signedData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_data: Option<String>,
        pub status: WalletIdSignaturesSignatureIdGETResponse200Status,
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
    pub struct WalletIdTransactionsGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsPOSTResponse200Requester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdTransactionsPOSTResponse200Status {
    #[default]
        Pending,
        Executing,
        Broadcasted,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsPOSTResponse200 {
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
        pub requester: WalletIdTransactionsPOSTResponse200Requester,
        pub status: WalletIdTransactionsPOSTResponse200Status,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsTransactionIdGETResponse200Requester {
        #[serde(rename = "tokenId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub token_id: Option<String>,
        #[serde(rename = "userId")]
        pub user_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WalletIdTransactionsTransactionIdGETResponse200Status {
    #[default]
        Pending,
        Executing,
        Broadcasted,
        Confirmed,
        Failed,
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransactionsTransactionIdGETResponse200 {
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
        pub requester: WalletIdTransactionsTransactionIdGETResponse200Requester,
        pub status: WalletIdTransactionsTransactionIdGETResponse200Status,
        #[serde(rename = "txHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_hash: Option<String>,
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WalletIdTransfersGETResponse200 {
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
    pub struct WebhooksGETResponse200 {
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
    pub enum WebhooksPOSTResponse200Status {
    #[default]
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhooksPOSTResponse200 {
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
        pub status: WebhooksPOSTResponse200Status,
        /// Webhook url
        pub url: String,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhookIdGETResponse200Status {
    #[default]
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdGETResponse200 {
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
        pub status: WebhookIdGETResponse200Status,
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
    pub enum WebhookIdPUTResponse200Status {
    #[default]
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdPUTResponse200 {
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
        pub status: WebhookIdPUTResponse200Status,
        /// Webhook url
        pub url: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhookIdDELETEResponse200Deleted {
    #[default]
    WebhookIdDELETEResponse200Deleted
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdDELETEResponse200 {
        pub deleted: WebhookIdDELETEResponse200Deleted,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdEventsGETResponse200 {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Webhook event
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum WebhookIdEventsWebhookEventIdGETResponse200Kind {
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
    pub struct WebhookIdEventsWebhookEventIdGETResponse200 {
        pub data: serde_json::Value,
        /// ISO date string when event was raised
        pub date: String,
        /// Error message if any error happened during the webhook request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        /// WebhookEvent ID
        pub id: String,
        pub kind: WebhookIdEventsWebhookEventIdGETResponse200Kind,
        /// Status code of the webhook request
        pub status: String,
        /// Unix timestamp when the event was forwarded to the webhook url by our servers.
        #[serde(rename = "timestampSent")]
        pub timestamp_sent: i64,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct WebhookIdPingPOSTResponse200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        pub status: String,
    }

}

pub mod yields {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct YieldsGETResponse200 {
        /// Current page items.
        pub items: Vec<Yield>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct YieldIdActionsGETResponse200 {
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

