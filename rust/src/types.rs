use serde::{Deserialize, Serialize};

///A transaction output, which defines new coins to be created from old ones.
pub struct TxOut {
    /// The value of the output, in satoshis.
    pub value: u64,
    /// The address of the output.
    pub address: String,
}

/// A reference to a transaction output.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutPoint {
    /// The referenced transaction's txid.
    pub(crate) txid: String,
    /// The index of the referenced output in its transaction's vout.
    pub(crate) vout: u32,
}
/// Unspent outputs of this wallet
pub struct LocalUtxo {
    /// Reference to a transaction output
    pub outpoint: OutPoint,
    ///Transaction output
    pub txout: TxOut,
    ///Whether this UTXO is spent or not
    pub is_spent: bool,
}

/// Local Wallet's Balance
#[derive(Deserialize)]
pub struct Balance {
    // All coinbase outputs not yet matured
    pub immature: u64,
    /// Unconfirmed UTXOs generated by a wallet tx
    pub trusted_pending: u64,
    /// Unconfirmed UTXOs received from an external wallet
    pub untrusted_pending: u64,
    /// Confirmed and immediately spendable balance
    pub confirmed: u64,
    /// Get sum of trusted_pending and confirmed coins
    pub spendable: u64,
    /// Get the whole balance visible to the wallet
    pub total: u64,
}
/// The address index selection strategy to use to derived an address from the wallet's external
/// descriptor.
pub enum AddressIndex {
    ///Return a new address after incrementing the current descriptor index.
    New,
    ///Return the address for the current descriptor index if it has not been used in a received transaction. Otherwise return a new address as with AddressIndex.New.
    ///Use with caution, if the wallet has not yet detected an address has been used it could return an already used address. This function is primarily meant for situations where the caller is untrusted; for example when deriving donation addresses on-demand for a public web page.
    LastUnused,
}

///A derived address and the index it was found at For convenience this automatically derefs to Address
pub struct AddressInfo {
    ///Child index of this address
    pub index: u32,
    /// Address
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
///A wallet transaction
pub struct TransactionDetails {
    /// Transaction id.
    pub txid: String,
    /// Received value (sats)
    /// Sum of owned outputs of this transaction.
    pub received: u64,
    /// Sent value (sats)
    /// Sum of owned inputs of this transaction.
    pub sent: u64,
    /// Fee value (sats) if confirmed.
    /// The availability of the fee depends on the backend. It's never None with an Electrum
    /// Server backend, but it could be None with a Bitcoin RPC node without txindex that receive
    /// funds while offline.
    pub fee: Option<u64>,
    /// If the transaction is confirmed, contains height and timestamp of the block containing the
    /// transaction, unconfirmed transaction contains `None`.
    pub confirmation_time: Option<BlockTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]

///Block height and timestamp of a block
pub struct BlockTime {
    ///Confirmation block height
    pub height: u32,
    ///Confirmation block timestamp
    pub timestamp: u64,
}

/// A output script and an amount of satoshis.
#[derive(Clone, Serialize, Deserialize)]
pub struct ScriptAmount {
    pub script: String,
    //Transaction amount
    pub amount: u64,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum RbfValue {
    RbfDefault,
    Value(u32),
}

/// The result after calling the TxBuilder finish() function. Contains unsigned PSBT and
/// transaction details.
pub struct TxBuilderResult {
    pub psbt: String,
    ///A wallet transaction
    pub transaction_details: TransactionDetails,
}

/// Configuration for an ElectrumBlockchain
pub struct ElectrumConfig {
    ///URL of the Electrum server (such as ElectrumX, Esplora, BWT) may start with ssl:// or tcp:// and include a port
    ///eg. ssl://electrum.blockstream.info:60002
    pub url: String,
    ///URL of the socks5 proxy server or a Tor service
    pub socks5: Option<String>,
    ///Request retry count
    pub retry: u8,
    ///Request timeout (seconds)
    pub timeout: Option<u8>,
    ///Stop searching addresses for transactions after finding an unused gap of this length
    pub stop_gap: u64,
}
///Configuration for an EsploraBlockchain
pub struct EsploraConfig {
    ///Base URL of the esplora service
    ///eg. https://blockstream.info/api/
    pub base_url: String,
    ///  Optional URL of the proxy to use to make requests to the Esplora server
    /// The string should be formatted as: <protocol>://<user>:<password>@host:<port>.
    /// Note that the format of this value and the supported protocols change slightly between the sync version of esplora (using ureq) and the async version (using reqwest).
    ///  For more details check with the documentation of the two crates. Both of them are compiled with the socks feature enabled.
    /// The proxy is ignored when targeting wasm32.  
    pub proxy: Option<String>,
    ///Number of parallel requests sent to the esplora service (default: 4)
    pub concurrency: Option<u8>,
    ///Stop searching addresses for transactions after finding an unused gap of this length.
    pub stop_gap: u64,
    ///Socket timeout.
    pub timeout: Option<u64>,
}

/// Type that can contain any of the blockchain configurations defined by the library
///This allows storing a single configuration that can be loaded into an Blockchain instance.
///Wallets that plan to offer users the ability to switch blockchain backend at runtime will find this particularly useful.
pub enum BlockchainConfig {
    Electrum { config: ElectrumConfig },
    Esplora { config: EsploraConfig },
}
///Configuration type for a SqliteDatabase database
pub struct SqliteDbConfiguration {
    ///Main directory of the db
    pub path: String,
}
///Configuration type for a sled Tree database
pub struct SledDbConfiguration {
    ///Main directory of the db
    pub path: String,
    ///Name of the database tree, a separated namespace for the data
    pub tree_name: String,
}

/// Type that can contain any of the database configurations defined by the library
/// This allows storing a single configuration that can be loaded into an DatabaseConfig
/// instance. Wallets that plan to offer users the ability to switch blockchain backend at runtime
/// will find this particularly useful.
pub enum DatabaseConfig {
    Memory,
    ///Simple key-value embedded database based on sled
    Sqlite {
        config: SqliteDbConfiguration,
    },
    ///Sqlite embedded database using rusqlite
    Sled {
        config: SledDbConfiguration,
    },
}

#[allow(dead_code)]
///Types of keychains
pub enum KeyChainKind {
    External,
    ///Internal, usually used for change outputs
    Internal,
}

#[derive(Clone)]
///The cryptocurrency to act on
pub enum Network {
    ///Bitcoin’s testnet
    Testnet,
    ///Bitcoin’s regtest
    Regtest,
    ///Classic Bitcoin
    Bitcoin,
    ///Bitcoin’s signet
    Signet,
}

///Type describing entropy length (aka word count) in the mnemonic
pub enum WordCount {
    ///12 words mnemonic (128 bits entropy)
    Words12,
    ///18 words mnemonic (192 bits entropy)
    Words18,
    ///24 words mnemonic (256 bits entropy)
    Words24,
}
