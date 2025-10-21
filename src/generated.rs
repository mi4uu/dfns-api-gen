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

    pub mod agreement_id {
        use super::*;

        pub mod accept {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                #[serde(rename = "agreementId")]
                pub agreement_id: String,
                #[serde(rename = "dateAccepted")]
                pub date_accepted: String,
                #[serde(rename = "userId")]
                pub user_id: String,
            }

        }

    }

    pub mod latest-unaccepted {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum ListResponseLatestAgreementAgreementType {
        #[default]
            PrivacyPolicy,
            TermsAndConditions,
            UniswapTermsOfService,
            UniswapPrivacyPolicy,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponseLatestAgreement {
            #[serde(rename = "agreementType")]
            pub agreement_type: ListResponseLatestAgreementAgreementType,
            #[serde(rename = "agreementUrl")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub agreement_url: Option<String>,
            pub details: String,
            pub id: String,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            #[serde(rename = "latestAgreement")]
            pub latest_agreement: ListResponseLatestAgreement,
        }

    }

}

pub mod auth {
    use super::*;

    pub mod action {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequest {
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
        pub struct CreateResponse {
            #[serde(rename = "userAction")]
            pub user_action: String,
        }

        pub mod init {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateRequestUserActionServerKind {
            #[default]
                Api,
                Staff,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                #[serde(rename = "userActionHttpMethod")]
                pub user_action_http_method: String,
                #[serde(rename = "userActionHttpPath")]
                pub user_action_http_path: String,
                #[serde(rename = "userActionPayload")]
                pub user_action_payload: String,
                #[serde(rename = "userActionServerKind")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub user_action_server_kind: Option<CreateRequestUserActionServerKind>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseAllowCredentials {
                pub key: Vec<serde_json::Value>,
                #[serde(rename = "passwordProtectedKey")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub password_protected_key: Option<Vec<serde_json::Value>>,
                pub webauthn: Vec<serde_json::Value>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseAttestation {
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
            pub struct CreateResponseRp {
                pub id: String,
                pub name: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseUserVerification {
                #[serde(rename = "required")]
            #[default]
                Required,
                #[serde(rename = "preferred")]
                Preferred,
                #[serde(rename = "discouraged")]
                Discouraged,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                #[serde(rename = "allowCredentials")]
                pub allow_credentials: CreateResponseAllowCredentials,
                pub attestation: CreateResponseAttestation,
                pub challenge: String,
                #[serde(rename = "challengeIdentifier")]
                pub challenge_identifier: String,
                #[serde(rename = "externalAuthenticationUrl")]
                pub external_authentication_url: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub rp: Option<CreateResponseRp>,
                #[serde(rename = "supportedCredentialKinds")]
                pub supported_credential_kinds: Vec<serde_json::Value>,
                #[serde(rename = "userVerification")]
                pub user_verification: CreateResponseUserVerification,
            }

        }

        pub mod logs {
            use super::*;

            pub mod id {
                use super::*;

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponseFirstFactorCredentialAssertion {
                    #[serde(rename = "authenticatorData")]
                    pub authenticator_data: String,
                    #[serde(rename = "clientData")]
                    pub client_data: String,
                    pub signature: String,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponseFirstFactorCredential {
                    pub assertion: GetResponseFirstFactorCredentialAssertion,
                    pub id: String,
                    pub kind: String,
                    #[serde(rename = "publicKey")]
                    pub public_key: String,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponse {
                    pub action: String,
                    #[serde(rename = "actionToken")]
                    pub action_token: String,
                    #[serde(rename = "datePerformed")]
                    pub date_performed: String,
                    #[serde(rename = "firstFactorCredential")]
                    pub first_factor_credential: GetResponseFirstFactorCredential,
                    pub id: String,
                    #[serde(rename = "userId")]
                    pub user_id: String,
                    pub username: String,
                }

            }

        }

    }

    pub mod apps {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            pub items: Vec<serde_json::Value>,
        }

        pub mod app_id {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum GetResponseKind {
            #[default]
                ServerSideApplication,
                ClientSideApplication,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
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
                pub kind: GetResponseKind,
                pub name: String,
                #[serde(rename = "orgId")]
                pub org_id: String,
                #[serde(rename = "permissionAssignments")]
                pub permission_assignments: Vec<serde_json::Value>,
            }

        }

    }

    pub mod credentials {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            pub items: Vec<serde_json::Value>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateResponseKind {
        #[default]
            Fido2,
            Key,
            Password,
            Totp,
            RecoveryKey,
            PasswordProtectedKey,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateResponse {
            #[serde(rename = "credentialId")]
            pub credential_id: String,
            #[serde(rename = "credentialUuid")]
            pub credential_uuid: String,
            #[serde(rename = "dateCreated")]
            pub date_created: String,
            #[serde(rename = "isActive")]
            pub is_active: bool,
            pub kind: CreateResponseKind,
            pub name: String,
            pub origin: String,
            #[serde(rename = "publicKey")]
            pub public_key: String,
            #[serde(rename = "relyingPartyId")]
            pub relying_party_id: String,
        }

        pub mod activate {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateRequest {
                #[serde(rename = "credentialUuid")]
                pub credential_uuid: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateResponse {
                pub message: String,
            }

        }

        pub mod code {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                /// Code expiration, as an ISO-8601 datetime string or a unix timestamp
                pub expiration: serde_json::Value,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                pub code: String,
                pub expiration: String,
            }

            pub mod init {
                use super::*;

            }

            pub mod verify {
                use super::*;

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
                pub enum CreateResponseKind {
                #[default]
                    Fido2,
                    Key,
                    Password,
                    Totp,
                    RecoveryKey,
                    PasswordProtectedKey,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct CreateResponse {
                    #[serde(rename = "credentialId")]
                    pub credential_id: String,
                    #[serde(rename = "credentialUuid")]
                    pub credential_uuid: String,
                    #[serde(rename = "dateCreated")]
                    pub date_created: String,
                    #[serde(rename = "isActive")]
                    pub is_active: bool,
                    pub kind: CreateResponseKind,
                    pub name: String,
                    pub origin: String,
                    #[serde(rename = "publicKey")]
                    pub public_key: String,
                    #[serde(rename = "relyingPartyId")]
                    pub relying_party_id: String,
                }

            }

        }

        pub mod deactivate {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateRequest {
                #[serde(rename = "credentialUuid")]
                pub credential_uuid: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateResponse {
                pub message: String,
            }

        }

        pub mod init {
            use super::*;

        }

    }

    pub mod login {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequest {
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

        pub mod code {
            use super::*;

        }

        pub mod delegated {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                pub username: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                pub token: String,
            }

        }

        pub mod init {
            use super::*;

        }

        pub mod social {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateRequestSocialLoginProviderKind {
            #[default]
                Oidc,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                #[serde(rename = "idToken")]
                pub id_token: String,
                #[serde(rename = "orgId")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub org_id: Option<String>,
                #[serde(rename = "socialLoginProviderKind")]
                pub social_login_provider_kind: CreateRequestSocialLoginProviderKind,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                pub token: String,
            }

        }

        pub mod sso {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                /// Authorization code obtained from the IdP
                pub code: String,
                /// State forwarded by the IdP
                pub state: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                pub token: String,
            }

            pub mod init {
                use super::*;

            }

        }

    }

    pub mod logout {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateRequest {
            #[serde(rename = "allSessions")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub all_sessions: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateResponse {
            pub message: String,
        }

    }

    pub mod pats {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            pub items: Vec<serde_json::Value>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequest {
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
        pub enum CreateResponseKind {
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
        pub struct CreateResponse {
            #[serde(rename = "accessToken")]
            pub access_token: String,
            #[serde(rename = "credId")]
            pub cred_id: String,
            #[serde(rename = "dateCreated")]
            pub date_created: String,
            #[serde(rename = "isActive")]
            pub is_active: bool,
            pub kind: CreateResponseKind,
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

        pub mod token_id {
            use super::*;

            /// Access token kind.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum GetResponseKind {
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
            pub struct GetResponse {
                #[serde(rename = "accessToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub access_token: Option<String>,
                #[serde(rename = "credId")]
                pub cred_id: String,
                #[serde(rename = "dateCreated")]
                pub date_created: String,
                #[serde(rename = "isActive")]
                pub is_active: bool,
                pub kind: GetResponseKind,
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
            pub struct UpdateRequest {
                #[serde(rename = "externalId")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub external_id: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub name: Option<String>,
            }

            /// Access token kind.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum UpdateResponseKind {
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
            pub struct UpdateResponse {
                #[serde(rename = "accessToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub access_token: Option<String>,
                #[serde(rename = "credId")]
                pub cred_id: String,
                #[serde(rename = "dateCreated")]
                pub date_created: String,
                #[serde(rename = "isActive")]
                pub is_active: bool,
                pub kind: UpdateResponseKind,
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
            pub enum DeleteResponseKind {
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
            pub struct DeleteResponse {
                #[serde(rename = "accessToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub access_token: Option<String>,
                #[serde(rename = "credId")]
                pub cred_id: String,
                #[serde(rename = "dateCreated")]
                pub date_created: String,
                #[serde(rename = "isActive")]
                pub is_active: bool,
                pub kind: DeleteResponseKind,
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

            pub mod activate {
                use super::*;

            }

            pub mod deactivate {
                use super::*;

            }

        }

    }

    pub mod recover {
        use super::*;

        pub mod user {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestNewCredentialsRecoveryCredentialCredentialInfo {
                #[serde(rename = "attestationData")]
                pub attestation_data: String,
                #[serde(rename = "clientData")]
                pub client_data: String,
                #[serde(rename = "credId")]
                pub cred_id: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateRequestNewCredentialsRecoveryCredentialCredentialKind {
            #[default]
                RecoveryKey,
            }

            /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestNewCredentialsRecoveryCredential {
                #[serde(rename = "credentialInfo")]
                pub credential_info: CreateRequestNewCredentialsRecoveryCredentialCredentialInfo,
                #[serde(rename = "credentialKind")]
                pub credential_kind: CreateRequestNewCredentialsRecoveryCredentialCredentialKind,
                #[serde(rename = "credentialName")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub credential_name: Option<String>,
                #[serde(rename = "encryptedPrivateKey")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub encrypted_private_key: Option<String>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestNewCredentials {
                #[serde(rename = "firstFactorCredential")]
                pub first_factor_credential: serde_json::Value,
                /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
                #[serde(rename = "recoveryCredential")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recovery_credential: Option<CreateRequestNewCredentialsRecoveryCredential>,
                #[serde(rename = "secondFactorCredential")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub second_factor_credential: Option<serde_json::Value>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestRecoveryCredentialAssertion {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub algorithm: Option<String>,
                #[serde(rename = "clientData")]
                pub client_data: String,
                #[serde(rename = "credId")]
                pub cred_id: String,
                pub signature: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateRequestRecoveryKind {
            #[default]
                RecoveryKey,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestRecovery {
                #[serde(rename = "credentialAssertion")]
                pub credential_assertion: CreateRequestRecoveryCredentialAssertion,
                pub kind: CreateRequestRecoveryKind,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                #[serde(rename = "newCredentials")]
                pub new_credentials: CreateRequestNewCredentials,
                pub recovery: CreateRequestRecovery,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseCredentialKind {
            #[default]
                Fido2,
                Key,
                Password,
                Totp,
                RecoveryKey,
                PasswordProtectedKey,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseCredential {
                pub kind: CreateResponseCredentialKind,
                pub name: String,
                pub uuid: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseUser {
                pub id: String,
                #[serde(rename = "orgId")]
                pub org_id: String,
                pub username: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                pub credential: CreateResponseCredential,
                pub user: CreateResponseUser,
            }

            pub mod code {
                use super::*;

            }

            pub mod delegated {
                use super::*;

            }

            pub mod init {
                use super::*;

            }

        }

    }

    pub mod registration {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequestRecoveryCredentialCredentialInfo {
            #[serde(rename = "attestationData")]
            pub attestation_data: String,
            #[serde(rename = "clientData")]
            pub client_data: String,
            #[serde(rename = "credId")]
            pub cred_id: String,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateRequestRecoveryCredentialCredentialKind {
        #[default]
            RecoveryKey,
        }

        /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequestRecoveryCredential {
            #[serde(rename = "credentialInfo")]
            pub credential_info: CreateRequestRecoveryCredentialCredentialInfo,
            #[serde(rename = "credentialKind")]
            pub credential_kind: CreateRequestRecoveryCredentialCredentialKind,
            #[serde(rename = "credentialName")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub credential_name: Option<String>,
            #[serde(rename = "encryptedPrivateKey")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub encrypted_private_key: Option<String>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequest {
            #[serde(rename = "firstFactorCredential")]
            pub first_factor_credential: serde_json::Value,
            /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
            #[serde(rename = "recoveryCredential")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub recovery_credential: Option<CreateRequestRecoveryCredential>,
            #[serde(rename = "secondFactorCredential")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub second_factor_credential: Option<serde_json::Value>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateResponseCredentialKind {
        #[default]
            Fido2,
            Key,
            Password,
            Totp,
            RecoveryKey,
            PasswordProtectedKey,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateResponseCredential {
            pub kind: CreateResponseCredentialKind,
            pub name: String,
            pub uuid: String,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateResponseUser {
            pub id: String,
            #[serde(rename = "orgId")]
            pub org_id: String,
            pub username: String,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateResponse {
            pub credential: CreateResponseCredential,
            pub user: CreateResponseUser,
        }

        pub mod code {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateRequest {
                #[serde(rename = "orgId")]
                pub org_id: String,
                pub username: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateResponse {
                pub message: String,
            }

        }

        pub mod delegated {
            use super::*;

        }

        pub mod enduser {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestRecoveryCredentialCredentialInfo {
                #[serde(rename = "attestationData")]
                pub attestation_data: String,
                #[serde(rename = "clientData")]
                pub client_data: String,
                #[serde(rename = "credId")]
                pub cred_id: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateRequestRecoveryCredentialCredentialKind {
            #[default]
                RecoveryKey,
            }

            /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestRecoveryCredential {
                #[serde(rename = "credentialInfo")]
                pub credential_info: CreateRequestRecoveryCredentialCredentialInfo,
                #[serde(rename = "credentialKind")]
                pub credential_kind: CreateRequestRecoveryCredentialCredentialKind,
                #[serde(rename = "credentialName")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub credential_name: Option<String>,
                #[serde(rename = "encryptedPrivateKey")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub encrypted_private_key: Option<String>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                #[serde(rename = "firstFactorCredential")]
                pub first_factor_credential: serde_json::Value,
                /// Register a recovery key. See [Account Recovery](https://docs.dfns.co/api-reference/auth/account-recovery) for more details.
                #[serde(rename = "recoveryCredential")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recovery_credential: Option<CreateRequestRecoveryCredential>,
                #[serde(rename = "secondFactorCredential")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub second_factor_credential: Option<serde_json::Value>,
                pub wallets: Vec<serde_json::Value>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseAuthentication {
                pub token: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseCredentialKind {
            #[default]
                Fido2,
                Key,
                Password,
                Totp,
                RecoveryKey,
                PasswordProtectedKey,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseCredential {
                pub kind: CreateResponseCredentialKind,
                pub name: String,
                pub uuid: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseUser {
                pub id: String,
                #[serde(rename = "orgId")]
                pub org_id: String,
                pub username: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                pub authentication: CreateResponseAuthentication,
                pub credential: CreateResponseCredential,
                pub user: CreateResponseUser,
                pub wallets: Vec<Wallet>,
            }

        }

        pub mod init {
            use super::*;

        }

        pub mod social {
            use super::*;

        }

    }

    pub mod service-accounts {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            pub items: Vec<serde_json::Value>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequest {
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
        pub enum CreateResponseUserInfoKind {
        #[default]
            CustomerEmployee,
            EndUser,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateResponseUserInfo {
            #[serde(rename = "credentialUuid")]
            pub credential_uuid: String,
            #[serde(rename = "isActive")]
            pub is_active: bool,
            #[serde(rename = "isRegistered")]
            pub is_registered: bool,
            #[serde(rename = "isServiceAccount")]
            pub is_service_account: bool,
            pub kind: CreateResponseUserInfoKind,
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
        pub struct CreateResponse {
            #[serde(rename = "accessTokens")]
            pub access_tokens: Vec<serde_json::Value>,
            #[serde(rename = "userInfo")]
            pub user_info: CreateResponseUserInfo,
        }

        pub mod service_account_id {
            use super::*;

            /// User kind.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum GetResponseUserInfoKind {
            #[default]
                CustomerEmployee,
                EndUser,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponseUserInfo {
                #[serde(rename = "credentialUuid")]
                pub credential_uuid: String,
                #[serde(rename = "isActive")]
                pub is_active: bool,
                #[serde(rename = "isRegistered")]
                pub is_registered: bool,
                #[serde(rename = "isServiceAccount")]
                pub is_service_account: bool,
                pub kind: GetResponseUserInfoKind,
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
            pub struct GetResponse {
                #[serde(rename = "accessTokens")]
                pub access_tokens: Vec<serde_json::Value>,
                #[serde(rename = "userInfo")]
                pub user_info: GetResponseUserInfo,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateRequest {
                #[serde(rename = "externalId")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub external_id: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub name: Option<String>,
            }

            /// User kind.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum UpdateResponseUserInfoKind {
            #[default]
                CustomerEmployee,
                EndUser,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateResponseUserInfo {
                #[serde(rename = "credentialUuid")]
                pub credential_uuid: String,
                #[serde(rename = "isActive")]
                pub is_active: bool,
                #[serde(rename = "isRegistered")]
                pub is_registered: bool,
                #[serde(rename = "isServiceAccount")]
                pub is_service_account: bool,
                pub kind: UpdateResponseUserInfoKind,
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
            pub struct UpdateResponse {
                #[serde(rename = "accessTokens")]
                pub access_tokens: Vec<serde_json::Value>,
                #[serde(rename = "userInfo")]
                pub user_info: UpdateResponseUserInfo,
            }

            /// User kind.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum DeleteResponseUserInfoKind {
            #[default]
                CustomerEmployee,
                EndUser,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct DeleteResponseUserInfo {
                #[serde(rename = "credentialUuid")]
                pub credential_uuid: String,
                #[serde(rename = "isActive")]
                pub is_active: bool,
                #[serde(rename = "isRegistered")]
                pub is_registered: bool,
                #[serde(rename = "isServiceAccount")]
                pub is_service_account: bool,
                pub kind: DeleteResponseUserInfoKind,
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
            pub struct DeleteResponse {
                #[serde(rename = "accessTokens")]
                pub access_tokens: Vec<serde_json::Value>,
                #[serde(rename = "userInfo")]
                pub user_info: DeleteResponseUserInfo,
            }

            pub mod activate {
                use super::*;

            }

            pub mod deactivate {
                use super::*;

            }

        }

    }

    pub mod users {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            pub items: Vec<User>,
            #[serde(rename = "nextPageToken")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub next_page_token: Option<String>,
        }

        /// The kind of user being created. 
        ///       In this endpoint it can only be "`CustomerEmployee`" (creating an "`EndUser`" is done through the [Delegated Registration](https://docs.dfns.co/api-reference/auth/registration-flows#delegated-users-registration-flow) endpoint)
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateRequestKind {
        #[default]
            CustomerEmployee,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequest {
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
            pub kind: CreateRequestKind,
            #[serde(rename = "publicKey")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub public_key: Option<String>,
        }

        pub mod user_id {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateRequest {
                #[serde(rename = "isSSORequired")]
                pub is_ssorequired: bool,
            }

        }

    }

}

pub mod exchanges {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        /// Current page items.
        pub items: Vec<serde_json::Value>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateRequestKind {
    #[default]
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateRequestReadConfiguration {
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
    pub struct CreateRequestWriteConfiguration {
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
    pub struct CreateRequest {
        pub kind: CreateRequestKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "readConfiguration")]
        pub read_configuration: CreateRequestReadConfiguration,
        #[serde(rename = "writeConfiguration")]
        pub write_configuration: CreateRequestWriteConfiguration,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateResponseKind {
    #[default]
        Binance,
        Kraken,
        CoinbaseApp,
        CoinbasePrime,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateResponse {
        #[serde(rename = "dateCreated")]
        pub date_created: String,
        pub id: String,
        pub kind: CreateResponseKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    pub mod exchange_id {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum GetResponseKind {
        #[default]
            Binance,
            Kraken,
            CoinbaseApp,
            CoinbasePrime,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct GetResponse {
            #[serde(rename = "dateCreated")]
            pub date_created: String,
            pub id: String,
            pub kind: GetResponseKind,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum DeleteResponseDeleted {
        #[default]
        DeleteResponseDeleted
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct DeleteResponse {
            pub deleted: DeleteResponseDeleted,
        }

        pub mod accounts {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                /// Current page items.
                pub items: Vec<serde_json::Value>,
                /// token to use as `paginationToken` to request the next page.
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
            }

            pub mod account_id {
                use super::*;

                pub mod assets {
                    use super::*;

                    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                    pub struct GetResponse {
                        /// Current page items.
                        pub items: Vec<serde_json::Value>,
                        /// token to use as `paginationToken` to request the next page.
                        #[serde(rename = "nextPageToken")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub next_page_token: Option<String>,
                    }

                }

                pub mod deposits {
                    use super::*;

                    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
                    pub enum CreateResponseKind {
                    #[default]
                        Withdrawal,
                        Deposit,
                    }

                    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                    pub struct CreateResponseRequester {
                        #[serde(rename = "tokenId")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub token_id: Option<String>,
                        #[serde(rename = "userId")]
                        pub user_id: String,
                    }

                    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                    pub struct CreateResponse {
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
                        pub kind: CreateResponseKind,
                        #[serde(rename = "requestBody")]
                        pub request_body: serde_json::Value,
                        pub requester: CreateResponseRequester,
                        #[serde(rename = "transferId")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub transfer_id: Option<String>,
                        #[serde(rename = "walletId")]
                        pub wallet_id: String,
                    }

                }

                pub mod withdrawals {
                    use super::*;

                    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
                    pub enum CreateResponseKind {
                    #[default]
                        Withdrawal,
                        Deposit,
                    }

                    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                    pub struct CreateResponseRequester {
                        #[serde(rename = "tokenId")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub token_id: Option<String>,
                        #[serde(rename = "userId")]
                        pub user_id: String,
                    }

                    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                    pub struct CreateResponse {
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
                        pub kind: CreateResponseKind,
                        #[serde(rename = "requestBody")]
                        pub request_body: serde_json::Value,
                        pub requester: CreateResponseRequester,
                        #[serde(rename = "transferId")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub transfer_id: Option<String>,
                        #[serde(rename = "walletId")]
                        pub wallet_id: String,
                    }

                }

            }

        }

    }

}

pub mod fee-sponsors {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateRequest {
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
    pub enum CreateResponseStatus {
    #[default]
        Active,
        Deactivated,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateResponse {
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
        pub status: CreateResponseStatus,
        /// Id of the wallet that is used to sponsor the fee for other wallets
        #[serde(rename = "walletId")]
        pub wallet_id: String,
    }

    pub mod fee_sponsor_id {
        use super::*;

        /// Fee sponsor status.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum GetResponseStatus {
        #[default]
            Active,
            Deactivated,
            Archived,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct GetResponse {
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
            pub status: GetResponseStatus,
            /// Id of the wallet that is used to sponsor the fee for other wallets
            #[serde(rename = "walletId")]
            pub wallet_id: String,
        }

        /// Fee sponsor status.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum DeleteResponseStatus {
        #[default]
            Active,
            Deactivated,
            Archived,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct DeleteResponse {
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
            pub status: DeleteResponseStatus,
            /// Id of the wallet that is used to sponsor the fee for other wallets
            #[serde(rename = "walletId")]
            pub wallet_id: String,
        }

        pub mod activate {
            use super::*;

        }

        pub mod deactivate {
            use super::*;

        }

        pub mod fees {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                pub items: Vec<serde_json::Value>,
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
            }

        }

    }

}

pub mod key-stores {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        pub items: Vec<serde_json::Value>,
    }

}

pub mod keys {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateRequestCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateRequestScheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateRequest {
        pub curve: CreateRequestCurve,
        #[serde(rename = "delayDelegation")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delay_delegation: Option<bool>,
        #[serde(rename = "delegateTo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delegate_to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub scheme: CreateRequestScheme,
        #[serde(rename = "storeId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub store_id: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateResponseCurve {
        #[serde(rename = "ed25519")]
    #[default]
        Ed25519,
        #[serde(rename = "secp256k1")]
        Secp256k1,
        #[serde(rename = "stark")]
        Stark,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateResponseScheme {
    #[default]
        DH,
        ECDSA,
        EdDSA,
        Schnorr,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateResponseStatus {
    #[default]
        Active,
        Archived,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateResponse {
        pub curve: CreateResponseCurve,
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
        pub scheme: CreateResponseScheme,
        pub status: CreateResponseStatus,
    }

    pub mod import {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateRequestCurve {
            #[serde(rename = "ed25519")]
        #[default]
            Ed25519,
            #[serde(rename = "secp256k1")]
            Secp256k1,
            #[serde(rename = "stark")]
            Stark,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequest {
            pub curve: CreateRequestCurve,
            #[serde(rename = "encryptedKeyShares")]
            pub encrypted_key_shares: Vec<serde_json::Value>,
            #[serde(rename = "minSigners")]
            pub min_signers: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
            pub protocol: serde_json::Value,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateResponseCurve {
            #[serde(rename = "ed25519")]
        #[default]
            Ed25519,
            #[serde(rename = "secp256k1")]
            Secp256k1,
            #[serde(rename = "stark")]
            Stark,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateResponseScheme {
        #[default]
            DH,
            ECDSA,
            EdDSA,
            Schnorr,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateResponseStatus {
        #[default]
            Active,
            Archived,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateResponse {
            pub curve: CreateResponseCurve,
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
            pub scheme: CreateResponseScheme,
            pub status: CreateResponseStatus,
        }

    }

    pub mod key_id {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum GetResponseCurve {
            #[serde(rename = "ed25519")]
        #[default]
            Ed25519,
            #[serde(rename = "secp256k1")]
            Secp256k1,
            #[serde(rename = "stark")]
            Stark,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum GetResponseScheme {
        #[default]
            DH,
            ECDSA,
            EdDSA,
            Schnorr,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum GetResponseStatus {
        #[default]
            Active,
            Archived,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum GetResponseStoreKind {
        #[default]
            Hsm,
            Mpc,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct GetResponseStore {
            pub id: String,
            #[serde(rename = "keyId")]
            pub key_id: String,
            pub kind: GetResponseStoreKind,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct GetResponse {
            pub curve: GetResponseCurve,
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
            pub scheme: GetResponseScheme,
            pub status: GetResponseStatus,
            pub store: GetResponseStore,
            pub wallets: Vec<serde_json::Value>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateRequest {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum UpdateResponseCurve {
            #[serde(rename = "ed25519")]
        #[default]
            Ed25519,
            #[serde(rename = "secp256k1")]
            Secp256k1,
            #[serde(rename = "stark")]
            Stark,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum UpdateResponseScheme {
        #[default]
            DH,
            ECDSA,
            EdDSA,
            Schnorr,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum UpdateResponseStatus {
        #[default]
            Active,
            Archived,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateResponse {
            pub curve: UpdateResponseCurve,
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
            pub scheme: UpdateResponseScheme,
            pub status: UpdateResponseStatus,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum DeleteResponseCurve {
            #[serde(rename = "ed25519")]
        #[default]
            Ed25519,
            #[serde(rename = "secp256k1")]
            Secp256k1,
            #[serde(rename = "stark")]
            Stark,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum DeleteResponseScheme {
        #[default]
            DH,
            ECDSA,
            EdDSA,
            Schnorr,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum DeleteResponseStatus {
        #[default]
            Active,
            Archived,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct DeleteResponse {
            pub curve: DeleteResponseCurve,
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
            pub scheme: DeleteResponseScheme,
            pub status: DeleteResponseStatus,
        }

        pub mod delegate {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                #[serde(rename = "delegateTo")]
                pub delegate_to: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseStatus {
            #[default]
                Delegated,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                #[serde(rename = "keyId")]
                pub key_id: String,
                pub status: CreateResponseStatus,
            }

        }

        pub mod derive {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                pub domain: String,
                pub seed: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                pub output: String,
            }

        }

        pub mod export {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                #[serde(rename = "encryptionKey")]
                pub encryption_key: String,
                #[serde(rename = "supportedSchemes")]
                pub supported_schemes: Vec<serde_json::Value>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseCurve {
                #[serde(rename = "ed25519")]
            #[default]
                Ed25519,
                #[serde(rename = "secp256k1")]
                Secp256k1,
                #[serde(rename = "stark")]
                Stark,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseProtocol {
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
            pub struct CreateResponse {
                pub curve: CreateResponseCurve,
                /// Keyshares of the exported wallet. They are encrypted with the provided encryption key. The exported private key is re-constructed from these keyshares.
                #[serde(rename = "encryptedKeyShares")]
                pub encrypted_key_shares: Vec<serde_json::Value>,
                /// The TSS threshold of the wallet private signing key shares
                #[serde(rename = "minSigners")]
                pub min_signers: f64,
                pub protocol: CreateResponseProtocol,
                #[serde(rename = "publicKey")]
                pub public_key: String,
            }

        }

        pub mod signatures {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                pub items: Vec<serde_json::Value>,
                #[serde(rename = "keyId")]
                pub key_id: String,
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseRequester {
                #[serde(rename = "tokenId")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub token_id: Option<String>,
                #[serde(rename = "userId")]
                pub user_id: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseSignature {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub encoded: Option<String>,
                pub r: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recid: Option<f64>,
                pub s: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseStatus {
            #[default]
                Pending,
                Executing,
                Signed,
                Confirmed,
                Failed,
                Rejected,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
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
                pub requester: CreateResponseRequester,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub signature: Option<CreateResponseSignature>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub signatures: Option<Vec<serde_json::Value>>,
                #[serde(rename = "signedData")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub signed_data: Option<String>,
                pub status: CreateResponseStatus,
                #[serde(rename = "txHash")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub tx_hash: Option<String>,
            }

            pub mod signature_id {
                use super::*;

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponseRequester {
                    #[serde(rename = "tokenId")]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub token_id: Option<String>,
                    #[serde(rename = "userId")]
                    pub user_id: String,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponseSignature {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub encoded: Option<String>,
                    pub r: String,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub recid: Option<f64>,
                    pub s: String,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
                pub enum GetResponseStatus {
                #[default]
                    Pending,
                    Executing,
                    Signed,
                    Confirmed,
                    Failed,
                    Rejected,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponse {
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
                    pub requester: GetResponseRequester,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub signature: Option<GetResponseSignature>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub signatures: Option<Vec<serde_json::Value>>,
                    #[serde(rename = "signedData")]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub signed_data: Option<String>,
                    pub status: GetResponseStatus,
                    #[serde(rename = "txHash")]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub tx_hash: Option<String>,
                }

            }

        }

    }

}

pub mod networks {
    use super::*;

    pub mod network {
        use super::*;

        pub mod validators {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                /// Current page items.
                pub items: Vec<CantonValidator>,
                /// token to use as `paginationToken` to request the next page.
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
            }

            pub mod validator_id {
                use super::*;

                /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct UpdateRequestLedgerOauth2 {
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
                pub struct UpdateRequestLedger {
                    /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
                    pub oauth2: UpdateRequestLedgerOauth2,
                    /// URL to reach the API at this address. The calls will be originating from our IP addresses (see [Dfns Environments](https://docs.dfns.co/api-reference/environments))
                    pub url: String,
                }

                /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct UpdateRequestValidatorOauth2 {
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
                pub struct UpdateRequestValidator {
                    /// How Dfns will authenticate into your validator/ledger. You should have setup authentication already (see details [here](https://docs.dev.sync.global/validator_operator/validator_helm.html#helm-validator-auth)), you can reuse the same Application details. See examples in this endpoint payload examples above.
                    pub oauth2: UpdateRequestValidatorOauth2,
                    /// URL to reach the API at this address. The calls will be originating from our IP addresses (see [Dfns Environments](https://docs.dfns.co/api-reference/environments))
                    pub url: String,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct UpdateRequest {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub ledger: Option<UpdateRequestLedger>,
                    /// Nickname for this validator.
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub validator: Option<UpdateRequestValidator>,
                }

            }

        }

    }

    pub mod read-contract {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateResponseKind {
        #[default]
            Evm,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateResponse {
            pub data: String,
            pub kind: CreateResponseKind,
        }

    }

}

pub mod permissions {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateRequest {
        pub name: String,
        pub operations: Vec<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateResponseStatus {
    #[default]
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateResponse {
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
        pub status: CreateResponseStatus,
    }

    pub mod permission_id {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub operations: Option<Vec<serde_json::Value>>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum UpdateResponseStatus {
        #[default]
            Active,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateResponse {
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
            pub status: UpdateResponseStatus,
        }

        pub mod archive {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateRequest {
                #[serde(rename = "isArchived")]
                pub is_archived: bool,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum UpdateResponseStatus {
            #[default]
                Active,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateResponse {
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
                pub status: UpdateResponseStatus,
            }

        }

        pub mod assignments {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                pub items: Vec<serde_json::Value>,
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                #[serde(rename = "identityId")]
                pub identity_id: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
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

    }

}

pub mod signers {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        pub clusters: Vec<serde_json::Value>,
    }

}

pub mod staking {
    use super::*;

    pub mod stakes {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            pub items: Vec<serde_json::Value>,
            #[serde(rename = "nextPageToken")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub next_page_token: Option<String>,
        }

        pub mod stake_id {
            use super::*;

            pub mod actions {
                use super::*;

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponse {
                    pub items: Vec<serde_json::Value>,
                    #[serde(rename = "nextPageToken")]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub next_page_token: Option<String>,
                }

            }

            pub mod rewards {
                use super::*;

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponse {
                    pub balance: String,
                    pub symbol: String,
                }

            }

        }

    }

}

pub mod swaps {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        /// Current page items.
        pub items: Vec<Swap>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Provided for this swap. Used for attesting that the swap is being created with the same parameters as the quote.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateRequestProvider {
    #[default]
        UniswapX,
        UniswapClassic,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateRequest {
        pub provider: CreateRequestProvider,
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

    pub mod quotes {
        use super::*;

        /// Swap provider.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum CreateRequestProvider {
        #[default]
            UniswapX,
            UniswapClassic,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct CreateRequest {
            pub provider: CreateRequestProvider,
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

}

pub mod v2 {
    use super::*;

    pub mod policies {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            pub items: Vec<serde_json::Value>,
            #[serde(rename = "nextPageToken")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub next_page_token: Option<String>,
        }

    }

    pub mod policy-approvals {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct ListResponse {
            pub items: Vec<serde_json::Value>,
            #[serde(rename = "nextPageToken")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub next_page_token: Option<String>,
        }

        pub mod approval_id {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum GetResponseStatus {
            #[default]
                Pending,
                Approved,
                Denied,
                Expired,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
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
                pub status: GetResponseStatus,
            }

            pub mod decisions {
                use super::*;

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
                pub enum CreateRequestValue {
                #[default]
                    Approved,
                    Denied,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct CreateRequest {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub reason: Option<String>,
                    pub value: CreateRequestValue,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
                pub enum CreateResponseStatus {
                #[default]
                    Pending,
                    Approved,
                    Denied,
                    Expired,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct CreateResponse {
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
                    pub status: CreateResponseStatus,
                }

            }

        }

    }

}

pub mod wallets {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        pub items: Vec<Wallet>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Network used for the wallet.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateRequestNetwork {
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
    pub enum CreateRequestSigningKeyCurve {
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
    pub enum CreateRequestSigningKeyScheme {
    #[default]
        ECDSA,
        EdDSA,
        Schnorr,
    }

    /// Options for the wallet's underlying key
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateRequestSigningKey {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub curve: Option<CreateRequestSigningKeyCurve>,
        /// Use this parameter to create a wallet from an existing key. This enables one key to be used across multiple networks and have the same address if networks share the same address format, ex. `Ethereum` and `Polygon`. If specified, requires the `Keys:Reuse` permission. If the key is delegated to an end user, then the new wallet will be automatically delegated to the same end user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scheme: Option<CreateRequestSigningKeyScheme>,
        /// Use this to specify the key store the key material is saved to.
        #[serde(rename = "storeId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub store_id: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateRequest {
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
        pub network: CreateRequestNetwork,
        /// Options for the wallet's underlying key
        #[serde(rename = "signingKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signing_key: Option<CreateRequestSigningKey>,
        /// List of tags to be created for this wallet. If specified, requires the `Wallets:Tags:Add` permission, like the [Tag Wallet](https://docs.dfns.co/api-reference/wallets/tag-wallet) endpoint.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<String>>,
        /// Id of the validator on which the wallet is created for Canton networks
        #[serde(rename = "validatorId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validator_id: Option<String>,
    }

    pub mod import {
        use super::*;

    }

    pub mod wallet_id {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateRequest {
            #[serde(rename = "externalId")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub external_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
        }

        pub mod assets {
            use super::*;

        }

        pub mod delegate {
            use super::*;

        }

        pub mod export {
            use super::*;

        }

        pub mod nfts {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                pub network: Network,
                pub nfts: Vec<serde_json::Value>,
                #[serde(rename = "walletId")]
                pub wallet_id: String,
            }

        }

        pub mod offers {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                /// Current page items.
                pub items: Vec<Offer>,
                /// token to use as `paginationToken` to request the next page.
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
            }

        }

        pub mod signatures {
            use super::*;

            pub mod signature_id {
                use super::*;

            }

        }

        pub mod tags {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct UpdateRequest {
                pub tags: Vec<String>,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct DeleteRequest {
                /// List of tags.
                pub tags: Vec<String>,
            }

        }

        pub mod transactions {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                pub items: Vec<serde_json::Value>,
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
                #[serde(rename = "walletId")]
                pub wallet_id: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponseRequester {
                #[serde(rename = "tokenId")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub token_id: Option<String>,
                #[serde(rename = "userId")]
                pub user_id: String,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateResponseStatus {
            #[default]
                Pending,
                Executing,
                Broadcasted,
                Confirmed,
                Failed,
                Rejected,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
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
                pub requester: CreateResponseRequester,
                pub status: CreateResponseStatus,
                #[serde(rename = "txHash")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub tx_hash: Option<String>,
                #[serde(rename = "walletId")]
                pub wallet_id: String,
            }

            pub mod transaction_id {
                use super::*;

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponseRequester {
                    #[serde(rename = "tokenId")]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub token_id: Option<String>,
                    #[serde(rename = "userId")]
                    pub user_id: String,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
                pub enum GetResponseStatus {
                #[default]
                    Pending,
                    Executing,
                    Broadcasted,
                    Confirmed,
                    Failed,
                    Rejected,
                }

                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
                pub struct GetResponse {
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
                    pub requester: GetResponseRequester,
                    pub status: GetResponseStatus,
                    #[serde(rename = "txHash")]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub tx_hash: Option<String>,
                    #[serde(rename = "walletId")]
                    pub wallet_id: String,
                }

            }

        }

        pub mod transfers {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                pub items: Vec<TransferRequest>,
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
                #[serde(rename = "walletId")]
                pub wallet_id: String,
            }

        }

    }

}

pub mod webhooks {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        pub items: Vec<serde_json::Value>,
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateRequestStatus {
    #[default]
        Enabled,
        Disabled,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// All events this webhook is subscribed to.
        pub events: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<CreateRequestStatus>,
        /// Webhook url
        pub url: String,
    }

    /// Webhook status
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
    pub enum CreateResponseStatus {
    #[default]
        Enabled,
        Disabled,
    }

    /// Webhook
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct CreateResponse {
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
        pub status: CreateResponseStatus,
        /// Webhook url
        pub url: String,
    }

    pub mod webhook_id {
        use super::*;

        /// Webhook status
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum GetResponseStatus {
        #[default]
            Enabled,
            Disabled,
        }

        /// Webhook
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct GetResponse {
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
            pub status: GetResponseStatus,
            /// Webhook url
            pub url: String,
        }

        /// Webhook status
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum UpdateRequestStatus {
        #[default]
            Enabled,
            Disabled,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
            /// All events this webhook is subscribed to.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub events: Option<Vec<serde_json::Value>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<UpdateRequestStatus>,
            /// Webhook url
            #[serde(skip_serializing_if = "Option::is_none")]
            pub url: Option<String>,
        }

        /// Webhook status
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum UpdateResponseStatus {
        #[default]
            Enabled,
            Disabled,
        }

        /// Webhook
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct UpdateResponse {
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
            pub status: UpdateResponseStatus,
            /// Webhook url
            pub url: String,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
        pub enum DeleteResponseDeleted {
        #[default]
        DeleteResponseDeleted
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
        pub struct DeleteResponse {
            pub deleted: DeleteResponseDeleted,
        }

        pub mod events {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct GetResponse {
                pub items: Vec<serde_json::Value>,
                #[serde(rename = "nextPageToken")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub next_page_token: Option<String>,
            }

            pub mod webhook_event_id {
                use super::*;

                /// Webhook event
                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
                pub enum GetResponseKind {
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
                pub struct GetResponse {
                    pub data: serde_json::Value,
                    /// ISO date string when event was raised
                    pub date: String,
                    /// Error message if any error happened during the webhook request.
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub error: Option<String>,
                    /// WebhookEvent ID
                    pub id: String,
                    pub kind: GetResponseKind,
                    /// Status code of the webhook request
                    pub status: String,
                    /// Unix timestamp when the event was forwarded to the webhook url by our servers.
                    #[serde(rename = "timestampSent")]
                    pub timestamp_sent: i64,
                }

            }

        }

        pub mod ping {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateResponse {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub error: Option<String>,
                pub status: String,
            }

        }

    }

}

pub mod yields {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
    pub struct ListResponse {
        /// Current page items.
        pub items: Vec<Yield>,
        /// token to use as `paginationToken` to request the next page.
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_page_token: Option<String>,
    }

    pub mod yield_id {
        use super::*;

        pub mod actions {
            use super::*;

            /// The type of action being performed on the yield investment: Deposit to add funds or Withdraw to remove funds.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateRequestKind {
            #[default]
                Deposit,
                Withdraw,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateRequestSourceAssetKind {
            #[default]
                Erc20,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestSourceAsset {
                pub amount: String,
                pub contract: String,
                pub kind: CreateRequestSourceAssetKind,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, smart_default::SmartDefault)]
            pub enum CreateRequestTargetAssetKind {
            #[default]
                Erc20,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequestTargetAsset {
                pub amount: String,
                pub contract: String,
                pub kind: CreateRequestTargetAssetKind,
            }

            /// Request body for creating a yield action. Different protocols may have different requirements.
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema,smart_default::SmartDefault)]
            pub struct CreateRequest {
                /// An optional external identifier provided by the client to ensure idempotency and prevent duplicate operations.
                #[serde(rename = "externalId")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub external_id: Option<String>,
                pub kind: CreateRequestKind,
                /// The slippage tolerance for this trade in [basis point](https://en.wikipedia.org/wiki/Basis_point) (BPS). Slippage tolerance defines the maximum price difference you're willing to accept during a trade from the estimated quote, ensuring you still receive at least a minimum number of tokens if the price shifts. One basis point equals one-hundredth of a percentage point, or 0.01%.
                #[serde(rename = "slippageBps")]
                pub slippage_bps: f64,
                #[serde(rename = "sourceAsset")]
                pub source_asset: CreateRequestSourceAsset,
                #[serde(rename = "targetAsset")]
                pub target_asset: CreateRequestTargetAsset,
            }

        }

    }

}

