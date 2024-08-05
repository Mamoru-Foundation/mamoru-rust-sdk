use crate::component::guest::types::*;
use std::collections::BTreeMap;
#[allow(dead_code)]
/// Structure representing a transaction.
#[derive(Clone)]
pub struct Transaction {
    /// Kind of the transaction.
    kind: TransactionKind,
    /// List of signers' identifiers.
    signers: Vec<String>, //SuiAddress
    /// Gas related data.
    data: GasData,
    /// Identifier of the gas owner.
    gas_owner: String, //SuiAddress
    /// Gas object information.
    gas: Vec<Object>, //Vec<Object>
    /// Price of gas per unit.    
    gas_price: u64,
    /// Total gas budget.
    gas_budget: u64,
    /// Expiration details.
    expiration: TransactionExpiration,
    /// Flag indicating if the transaction contains a shared object.
    contains_shared_object: bool,
    /// Object shared between inputs.
    shared_input_objects: Object,
    ///  list of move calls
    move_calls: Vec<(ObjectID, IdentStr, IdentStr)>, // Only for programmable tranasctions (package, module, function) in
    /// Objects that will receive output from the transaction.
    receiving_objects: Object, //Object,
    /// Flags for different transaction types.
    is_system_tx: bool,
    is_genesis_tx: bool,
    is_end_of_epoch_tx: bool,
    is_sponsored_tx: bool,
    /// Internal security measure.
    inner_sec: u64,
    /// Internal unique transaction identifier.
    inner_id: u64,
    inner_time: i64,
    digest: String,
    success: bool,
    gas_used: u64,
    gas_computation_cost: u64,
    gas_storage_cost: u64,
}

pub fn dummy_trans() -> Transaction {
    // Placeholder values for different fields
    let kind = TransactionKind::Other();
    let signers = vec!["Alice".to_string(), "Bob".to_string()];
    let data = GasData {
        payment: Vec::new(),
        owner: "".to_string(),
        price: 100,
    };
    let gas_owner = "Alice".to_string();
    let gas = Vec::new();
    let gas_price = 100;
    let gas_budget = 1000;
    let expiration = TransactionExpiration::None;
    let contains_shared_object = false;
    let shared_input_objects = new_other_object("0".to_string());
    let move_calls = Vec::new();
    let _input_objects = new_other_object("0".to_string());
    let receiving_objects = new_other_object("0".to_string());
    let is_system_tx = false;
    let is_genesis_tx = false;
    let is_end_of_epoch_tx = false;
    let is_sponsored_tx = false;
    let inner_sec = 0;
    let inner_id = 0;
    let inner_time = 0;

    // Create and return the transaction instance
    Transaction {
        kind,
        signers,
        data,
        gas_owner,
        gas,
        gas_price,
        gas_budget,
        expiration,
        contains_shared_object,
        shared_input_objects,
        move_calls,
        receiving_objects,
        is_system_tx,
        is_genesis_tx,
        is_end_of_epoch_tx,
        is_sponsored_tx,
        inner_sec,
        inner_id,
        inner_time,
        digest: "".to_string(),
        success: false,
        gas_used: 0,
        gas_computation_cost: 0,
        gas_storage_cost: 0,
    }
}

impl From<SuiTransaction> for Transaction {
    fn from(sui_transaction: SuiTransaction) -> Self {
        // Perform the conversion from SuiTransaction to Transaction
        // Extract fields from sui_transaction and create a new Transaction
        let kind = TransactionKind::ProgrammableTransaction(ProgrammableTransaction {
            inputs: Vec::new(),
            commands: Vec::new(),
            calltraces: Vec::new(),
            events: Vec::new(),
        });
        let signers = vec![sui_transaction.sender.clone()]; // Assuming sender is one of the signers
                                                            //let data = GasData::new(/* Fill with appropriate data */);
        let gas_owner = sui_transaction.sender.clone(); // Assuming sender is also the gas owner
        let gas = Vec::new(); // Initialize gas with empty Vec
        let gas_price = 0 /* Fill with appropriate value */;
        let gas_budget = sui_transaction.gas_budget;
        let expiration = TransactionExpiration::None;
        let contains_shared_object = false; // Assuming no shared object initially
        let shared_input_objects = Object::WrappedObject(ObjectId {
            id: "id".to_string(),
        }); // Assuming no shared input objects initially
        let _input_objects = Object::WrappedObject(ObjectId {
            id: "id".to_string(), // Assuming no input objects initially
        }); // Assuming no shared input objects initially
        let receiving_objects = Object::WrappedObject(ObjectId {
            id: "id".to_string(), // Assuming no receiving objects initially
        }); // Assuming no shared input objects initially
        let is_system_tx = false /* Fill with appropriate value */;
        let is_genesis_tx = false/* Fill with appropriate value */;
        let is_end_of_epoch_tx =  false/* Fill with appropriate value */;
        let is_sponsored_tx = false /* Fill with appropriate value */;
        let inner_sec = 0 /* Fill with appropriate value */;
        let inner_id = 0/* Fill with appropriate value */;
        let inner_time = sui_transaction.time;
        let data = GasData {
            payment: Vec::new(),
            owner: sui_transaction.sender.clone(),
            price: 0,
        };
        let move_calls = Vec::new();

        Transaction {
            kind,
            signers,
            data,
            gas_owner,
            gas,
            gas_price,
            gas_budget,
            expiration,
            contains_shared_object,
            shared_input_objects,
            move_calls,
            receiving_objects,
            is_system_tx,
            is_genesis_tx,
            is_end_of_epoch_tx,
            is_sponsored_tx,
            inner_sec,
            inner_id,
            inner_time,
            digest: "".to_string(),
            success: false,
            gas_used: 0,
            gas_computation_cost: 0,
            gas_storage_cost: 0,
        }
    }
}

impl Transaction {
    /// Returns if it is a programmable transaction
    pub fn is_programmable(&self) -> bool {
        true
    }

    /// List of registered calltraces for this transaction
    pub fn calltraces(&self) -> Vec<Calltrace> {
        Vec::new()
    }

    /// List of triggered events for this transaction
    pub fn events(&self) -> Vec<Event> {
        Vec::new()
    }

    /// Inputs for this transaction
    pub fn inputs(&self) -> Vec<CallArg> {
        Vec::new()
    }

    /// Generates a list of commands based on the specified command type.
    ///
    /// Parameters:
    /// - `typ_`: The type of command to generate (e.g., Publish, Upgrade).
    ///
    /// Returns:
    /// A vector of `Command` instances, which are initially empty but can be populated based on the command type.
    pub fn commands(&self, _typ: CommandType) -> Vec<Command> {
        let transaction = self.clone();
        if let TransactionKind::ProgrammableTransaction(trans) = transaction.kind {
            let res: Vec<Command> = trans
                .commands
                .iter()
                .filter(|&c| matches!(c, Command::Publish(_)))
                .cloned()
                .collect();
            return res;
        }

        Vec::new()
    }
}

#[allow(dead_code)]
/// Enum representing the possible types of input.
#[derive(Clone)]
enum Input {
    Pure,
    Object,
}

/// Represents the gas-related data for a transaction.
#[derive(Clone)]
pub struct GasData {
    /// List of objects related to gas payment.
    pub payment: Vec<Object>,
    /// Identifier for the owner.
    pub owner: SuiAddress,
    /// Price in terms of gas.
    pub price: u64,
}

/// A unique identifier representing a transaction epoch.
pub type EpochId = u64;

/// A type alias for a string that represents a Sui address.
pub type SuiAddress = String;

/// A type alias for a string that represents a reference to an object.
pub type ObjectRef = String;

/// A type alias for a string that uniquely identifies an object within the system.
pub type ObjectID = String;

/// A type alias for a string that identifies a struct's type within the system.
pub type StructTag = String;

/// Represents a more efficient way of storing and manipulating string identifiers.
#[derive(Clone)]
pub struct Identifier(Box<str>);

/// Represents an immutable string slice designed for efficient string operations within the system.
#[derive(Clone)]
pub struct IdentStr(String);

/*
 *pub struct ObjectID(
    #[schemars(with = "Hex")]
    #[serde_as(as = "Readable<HexAccountAddress, _>")]
    AccountAddress,
);

pub type VersionDigest = (SequenceNumber, ObjectDigest);

pub type ObjectRef = (ObjectID, SequenceNumber, ObjectDigest);
*/

/// Represents a specific round of randomness generation in a cryptographic protocol.
#[derive(Clone)]
pub struct RandomnessRound(pub u64);

/// Encapsulates the protocol version as a simple integer.
#[derive(Clone)]
pub struct ProtocolVersion(u64);

/// Represents the timestamp for a checkpoint, typically used in logging or snapshot features.
pub type CheckpointTimestamp = u64;

/// Enum defining types of transactions.
#[derive(Clone)]
pub enum TransactionKind {
    /// A transaction capable of executing programmable logic.
    ProgrammableTransaction(ProgrammableTransaction),
    /// Changes the epoch of the system; deprecated in favor of `EndOfEpochTransaction`.
    ChangeEpoch(ChangeEpoch),
    /// Marks the initial transaction of the system, setting up the initial state.
    Genesis(GenesisTransaction),
    /// Begins the process of committing data to consensus, handling preliminary steps.
    ConsensusCommitPrologue(ConsensusCommitPrologue),
    /// Updates the state relevant to the authenticator component of the system.
    AuthenticatorStateUpdate(AuthenticatorStateUpdate),

    /// EndOfEpochTransaction replaces ChangeEpoch with a list of transactions that are allowed to
    /// run at the end of the epoch.
    EndOfEpochTransaction(Vec<EndOfEpochTransactionKind>),

    RandomnessStateUpdate(RandomnessStateUpdate),
    // V2 ConsensusCommitPrologue also includes the digest of the current consensus output.
    ConsensusCommitPrologueV2(ConsensusCommitPrologueV2),

    Other(),
}

#[derive(Clone)]
pub struct RandomnessStateUpdate {
    /// Epoch of the randomness state update transaction
    pub epoch: u64,
    /// Randomness round of the update
    pub randomness_round: RandomnessRound,
    /// Updated random bytes
    pub random_bytes: Vec<u8>,
    /// The initial version of the randomness object that it was shared at.
    pub randomness_obj_initial_shared_version: SequenceNumber,
    // to version this struct, do not add new fields. Instead, add a RandomnessStateUpdateV2 to
    // TransactionKind.
}

/// Represents the different kinds of transactions that can occur at the end of an epoch.
#[derive(Clone)]
pub enum EndOfEpochTransactionKind {
    /// Changes the current epoch to a new epoch. This variant wraps the `ChangeEpoch` struct.
    ChangeEpoch(ChangeEpoch),

    /// Creates a new authenticator state. This does not require any additional data.
    AuthenticatorStateCreate,

    /// Expires an existing authenticator state. This variant wraps the `AuthenticatorStateExpire` struct.
    AuthenticatorStateExpire(AuthenticatorStateExpire),

    /// Creates a new randomness state. Typically used to initiate a new source of randomness for the next epoch.
    RandomnessStateCreate,

    /// Creates a new deny list state. Used to update or establish a list of denied entities or actions.
    DenyListStateCreate,
}

#[derive(Clone)]
pub struct AuthenticatorStateExpire {
    /// expire JWKs that have a lower epoch than this
    pub min_epoch: u64,
    /// The initial version of the authenticator object that it was shared at.
    pub authenticator_obj_initial_shared_version: SequenceNumber,
}

#[derive(Clone)]
pub struct ConsensusCommitPrologue {
    /// Epoch of the commit prologue transaction
    pub epoch: u64,
    /// Consensus round of the commit
    pub round: u64,
    /// Unix timestamp from consensus
    pub commit_timestamp_ms: CheckpointTimestamp,
}

/// A cryptographic digest with a fixed size of 32 bytes, suitable for hash functions.
#[derive(Clone)]
pub struct Digest([u8; 32]);

/// Wraps `Digest` for use in consensus commit operations, ensuring agreement across nodes.
#[derive(Clone)]
pub struct ConsensusCommitDigest(Digest);

#[derive(Clone)]
pub struct ConsensusCommitPrologueV2 {
    /// Epoch of the commit prologue transaction
    pub epoch: u64,
    /// Consensus round of the commit
    pub round: u64,
    /// Unix timestamp from consensus
    pub commit_timestamp_ms: CheckpointTimestamp,
    /// Digest of consensus output
    pub consensus_commit_digest: ConsensusCommitDigest,
}

#[derive(Clone)]
pub struct AuthenticatorStateUpdate {
    /// Epoch of the authenticator state update transaction
    pub epoch: u64,
    /// Consensus round of the authenticator state update
    pub round: u64,
    /// newly active jwks
    pub new_active_jwks: Vec<ActiveJwk>,
    /// The initial version of the authenticator object that it was shared at.
    pub authenticator_obj_initial_shared_version: SequenceNumber,
    // to version this struct, do not add new fields. Instead, add a AuthenticatorStateUpdateV2 to
    // TransactionKind.
}

/// Represents an active JSON Web Key (JWK) with its last validation epoch.
#[derive(Clone)]
pub struct ActiveJwk {
    // Unique identifier for the JWK.
    // pub jwk_id: JwkId,

    // The JSON Web Key (JWK) itself.
    // pub jwk: JWK,
    pub iss: String,
    pub kid: String,
    pub jwk_ty: String,
    pub jwk_n: String,
    pub jwk_alg: String,
    /// The most recent epoch in which the JWK was validated.
    pub epoch: u64,
}

/// Represents a sequence number as a simple wrapper around a 64-bit unsigned integer.
#[derive(Clone)]
pub struct SequenceNumber(u64);

#[derive(Clone)]
pub struct ChangeEpoch {
    /// The next (to become) epoch ID.
    pub epoch: EpochId,
    /// The protocol version in effect in the new epoch.
    pub protocol_version: ProtocolVersion,
    /// The total amount of gas charged for storage during the epoch.
    pub storage_charge: u64,
    /// The total amount of gas charged for computation during the epoch.
    pub computation_charge: u64,
    /// The amount of storage rebate refunded to the txn senders.
    pub storage_rebate: u64,
    /// The non-refundable storage fee.
    pub non_refundable_storage_fee: u64,
    /// Unix timestamp when epoch started
    pub epoch_start_timestamp_ms: u64,
    /// System packages (specifically framework and move stdlib) that are written before the new
    /// epoch starts. This tracks framework upgrades on chain. When executing the ChangeEpoch txn,
    /// the validator must write out the modules below.  Modules are provided with the version they
    /// will be upgraded to, their modules in serialized form (which include their package ID), and
    /// a list of their transitive dependencies.
    pub system_packages: Vec<(SequenceNumber, Vec<Vec<u8>>, Vec<String>)>,
}

#[derive(Clone)]
pub enum Owner {
    /// Object is exclusively owned by a single address, and is mutable.
    AddressOwner(SuiAddress),
    /// Object is exclusively owned by a single object, and is mutable.
    /// The object ID is converted to SuiAddress as SuiAddress is universal.
    ObjectOwner(SuiAddress),
    /// Object is shared, can be used by any address, and is mutable.
    Shared {
        /// The version at which the object became shared
        initial_shared_version: SequenceNumber,
    },
    /// Object is immutable, and hence ownership doesn't matter.
    Immutable,
}

/// Represents the initial transaction containing all objects necessary to initialize the system state.
#[derive(Clone)]
pub struct GenesisTransaction {
    pub objects: Vec<GenesisObject>,
}

/// Defines various types of objects that can be part of a genesis transaction.
#[derive(Clone)]
pub enum GenesisObject {
    RawObject { data: Data, owner: Owner },
}

#[derive(Clone)]
pub enum Data {
    /// An object whose governing logic lives in a published Move module
    Move(MoveObject),
    /// Map from each module name to raw serialized Move module bytes
    Package(MovePackage),
}

/// Defines the kinds of input objects that can be processed, specifically for handling Move language objects.
#[derive(Clone)]
pub enum InputObjectKind {
    // A Move package, must be immutable.
    MovePackage(ObjectID),
    // A Move object, either immutable, or owned mutable.
    ImmOrOwnedMoveObject(ObjectRef),
    // A Move object that's shared and mutable.
    SharedMoveObject {
        id: ObjectID,
        initial_shared_version: SequenceNumber,
        mutable: bool,
    },
}

#[derive(Clone)]
pub struct TypeOrigin {
    pub module_name: String,
    pub struct_name: String,
    pub package: ObjectID,
}

#[derive(Clone)]
pub struct UpgradeInfo {
    /// ID of the upgraded packages
    pub upgraded_id: ObjectID,
    /// Version of the upgraded package
    pub upgraded_version: SequenceNumber,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct MovePackage {
    id: ObjectID,
    /// Most move packages are uniquely identified by their ID (i.e. there is only one version per
    /// ID), but the version is still stored because one package may be an upgrade of another (at a
    /// different ID), in which case its version will be one greater than the version of the
    /// upgraded package.
    ///
    /// Framework packages are an exception to this rule -- all versions of the framework packages
    /// exist at the same ID, at increasing versions.
    ///
    /// In all cases, packages are referred to by move calls using just their ID, and they are
    /// always loaded at their latest version.
    version: SequenceNumber,
    module_map: BTreeMap<String, Vec<u8>>,

    /// Maps struct/module to a package version where it was first defined, stored as a vector for
    /// simple serialization and deserialization.
    type_origin_table: Vec<TypeOrigin>,

    // For each dependency, maps original package ID to the info about the (upgraded) dependency
    // version that this package is using
    linkage_table: BTreeMap<ObjectID, UpgradeInfo>,
}

/// Represents a Move language object with detailed state and behavior characteristics.
#[allow(dead_code)]
#[derive(Clone)]
pub struct MoveObject {
    /// The type of this object. Immutable
    type_: MoveObjectType,
    /// DEPRECATED this field is no longer used to determine whether a tx can transfer this
    /// object. Instead, it is always calculated from the objects type when loaded in execution
    has_public_transfer: bool,
    /// Number that increases each time a tx takes this object as a mutable input
    /// This is a lamport timestamp, not a sequentially increasing version
    version: SequenceNumber,
    /// BCS bytes of a Move struct value
    contents: Vec<u8>,
}
/// Wraps the detailed type definition for a Move object, providing a single-layer abstraction.
#[derive(Clone)]
pub struct MoveObjectType(MoveObjectType_);

/// Distinguishes different types of Move objects, especially in the context of handling various coin types.
#[derive(Clone)]
pub enum MoveObjectType_ {
    /// A type that is not `0x2::coin::Coin<T>`
    Other(StructTag),
    /// A SUI coin (i.e., `0x2::coin::Coin<0x2::sui::SUI>`)
    GasCoin,
    /// A record of a staked SUI coin (i.e., `0x3::staking_pool::StakedSui`)
    StakedSui,
    /// A non-SUI coin type (i.e., `0x2::coin::Coin<T> where T != 0x2::sui::SUI`)
    Coin(TypeTag),
    // NOTE: if adding a new type here, and there are existing on-chain objects of that
    // type with Other(_), that is ok, but you must hand-roll PartialEq/Eq/Ord/maybe Hash
    // to make sure the new type and Other(_) are interpreted consistently.
}

/// Represents the basic types and compound types used in Move-like systems.
#[derive(Clone)]
pub enum TypeTag {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Signer,
    Vector(Box<TypeTag>),
    Struct(Box<StructTag>),
    U16,
    U32,
    U256,
}

#[allow(dead_code)]
/// Structure representing a data object.
#[derive(Clone)]
pub struct DataObject {
    /// Unique identifier for the data object.
    id: String,
    /// Binary data associated with the object.
    data: Vec<u8>, //TODO avoid serialization step
}

#[allow(dead_code)]
/// Structure representing the ID of an object.
#[derive(Clone)]
pub struct ObjectId {
    /// String identifier of the object.
    id: String,
}

/// Enum representing various states of an object.
#[derive(Clone)]
pub enum Object {
    CreatedObject(DataObject),
    MutatedObject(DataObject),
    DeletedObject(ObjectId),
    WrappedObject(ObjectId),
    UnwrappedObject(ObjectId),
    UnwrappedThenDeletedObject(ObjectId),
    OtherObject(ObjectId),
}

fn new_other_object(id: String) -> Object {
    Object::OtherObject(ObjectId { id })
}

/// Enum describing expiration conditions for a transaction.
#[derive(Clone)]
pub enum TransactionExpiration {
    /// The transaction has no expiration
    None,
    /// Validators wont sign a transaction unless the expiration Epoch
    /// is greater than or equal to the current epoch
    Epoch(EpochId),
}

/// Defines various arguments that can be used in commands and transactions.
#[derive(Clone)]
pub enum Argument {
    GasCoin,                // Represents a gas coin used in transactions.
    Input(u16),             // Represents an input argument from a specified source.
    Result(u16),            // Represents a result argument from a specified source.
    NestedResult(u16, u16), // Represents a nested result from specified sources.
}

/// A transaction will have a (unique) digest.
#[derive(Clone)]
pub struct TransactionDigest(Digest);

#[derive(Clone)]
pub struct EventEnvelope {
    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    pub timestamp: u64,
    /// Transaction digest of associated transaction
    pub tx_digest: TransactionDigest,
    /// Consecutive per-tx counter assigned to this event.
    pub event_num: u64,
    /// Specific event type
    pub event: Event,
    /// Move event's json value
    pub parsed_json: String, //TODO Json serialized
}

/// Structure for an event within a transaction.
#[derive(Clone)]
pub struct Event {
    /// ID of the event's package.
    pub package_id: ObjectID,
    pub transaction_module: Identifier,
    /// Sender's identifier.
    pub sender: SuiAddress,
    /// Type of the event.
    pub type_: StructTag,
    //#[serde_as(as = "Bytes")]
    /// Contents of the event, often temporary values.
    pub contents: Vec<u8>, //TODO pending to add as a ValueData type
}
impl Event {
    /// Returns the parent transaction
    pub fn get_parent_transaction(&self) -> Transaction {
        dummy_trans()
    }
}

/// Enum representing argument types for a call within a transaction.
//#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[derive(Clone)]
pub enum ObjectArg {
    // A Move object, either immutable, or owned mutable.
    ImmOrOwnedObject(Object), //TODO Object
    // A Move object that's shared.
    // SharedObject::mutable controls whether caller asks for a mutable reference to shared object.
    SharedObject {
        id: ObjectID, //ObjectID
        initial_shared_version: u64,
        mutable: bool,
    },
    // A Move object that can be received in this transaction.
    Receiving(ObjectRef), //Object
}

// Input in wit component
/// Enum encapsulating the call arguments.
#[derive(Clone)]
pub enum CallArg {
    // contains no structs or objects
    Pure(Vec<u8>), //Can it be mapped as a formatted serialized data?
    // an object
    Object(ObjectArg),
}

/// Struct representing a trace of a call.
#[derive(Clone)]
pub struct Calltrace {
    pub seq: u64,
    pub depth: u32,
    pub call_type: u8,
    pub gas_used: u64,
    pub transaction_module: Option<String>, //module id
    pub function: String,
}

impl Calltrace {
    /// Returns the parent transaction
    pub fn get_parent_transaction(&self) -> Transaction {
        dummy_trans()
    }
}

#[allow(dead_code)]
/// Struct for a transaction with programmable operations.
#[derive(Clone)]
pub struct ProgrammableTransaction {
    inputs: Vec<CallArg>,       // Various inputs for the transaction. // Input
    commands: Vec<Command>,     // Commands to be executed within the transaction.
    calltraces: Vec<Calltrace>, // Trace details of the calls made during the transaction.  //MoveCallTrace
    events: Vec<Event>,         // Events generated during the transaction.
}

/// Struct for other not mapped command.
#[allow(dead_code)]
#[derive(Clone)]
pub struct OtherCommand {
    inner_seq: u64,
}

#[allow(dead_code)]
/// Command to publish an object or objects with specific dependencies and security measures.
#[derive(Clone)]
pub struct PublishCommand {
    seq: u64,          // Sequence number for tracking. //Vec<u8>
    deps: Vec<Object>, // Dependencies for the publish command.
    inner_sec: u64,    // Internal security measure, typically a timestamp or counter.
}

#[allow(dead_code)]
/// Command to upgrade an existing object with additional arguments for controlling behavior.
#[derive(Clone)]
pub struct UpgradeCommand {
    seq: u64,           // Sequence number for tracking.    //Vec<u8>
    deps: Vec<Object>,  // Dependencies related to the upgrade.
    object: Object,     // Object being upgraded.
    argument: Argument, // Additional arguments for the upgrade process.    //calltraces?
    inner_seq: u64,     // Additional internal sequence number.
}

impl UpgradeCommand {
    //TODO pending for improving
    pub fn new(seq: u64, deps: Vec<Object>, inner_seq: u64) -> Self {
        UpgradeCommand {
            seq,
            deps,
            object: new_other_object("0".to_string()),
            argument: Argument::Input(0),
            inner_seq,
        }
    }
}

#[allow(dead_code)]
/// Represents different types of commands that can be executed in a programmable transaction.
#[derive(Clone)]
pub enum Command {
    Publish(PublishCommand),
    Upgrade(UpgradeCommand),
    MoveCall(Box<ProgrammableMoveCall>),
    TransferObjects(Vec<Argument>, Argument),
    SplitCoins(Argument, Vec<Argument>),
    MergeCoins(Argument, Vec<Argument>),
    MakeMoveVec(Option<TypeTag>, Vec<Argument>),
    Other(OtherCommand), // Additional commands can be defined here.
}

#[derive(Clone)]
pub struct ProgrammableMoveCall {
    /// The package containing the module and function.
    pub package: ObjectID,
    /// The specific module in the package containing the function.
    pub module: Identifier,
    /// The function to be called.as de parte de la crítica y
    pub function: Identifier,
    /// The type arguments to the function.
    pub type_arguments: Vec<TypeTag>,
    /// The arguments to the function.
    pub arguments: Vec<Argument>,
}

/// Enumerates the types of commands that can be issued.
#[derive(Clone)]
pub enum CommandType {
    Publish,
    Upgrade,
}

#[allow(dead_code)]
/// Represents the context of a Sui operation, potentially containing a transaction.
#[derive(Clone)]
pub struct SuiCtx {
    /// Optional transaction associated with this context.
    transaction: Option<Transaction>,
}

impl SuiCtx {
    /// Attempts to retrieve a transaction if one is available in the context.
    ///
    /// Returns:
    /// An `Option<Transaction>` which will be `None` as the current implementation does not store or handle transactions.
    pub fn transaction(&self) -> Option<Transaction> {
        self.transaction.clone()
    }
    /// Creates a new `SuiCtx` instance with no active transaction.
    ///
    /// Returns:
    /// A new `SuiCtx` instance with an empty transaction option.
    pub fn new_empty_sui_ctx() -> Self {
        SuiCtx { transaction: None }
    }
}

/// Load Su Context to load information (transactions, events, calltraces..)
pub fn load() -> SuiCtx {
    //crate::component::guest::sui_ctx;
    let wit_transaction: Option<SuiTransaction> =
        crate::component::guest::sui_ctx::get_transaction();

    if wit_transaction.is_none() {
        return SuiCtx::new_empty_sui_ctx();
    }

    let inner_transaction: Transaction = wit_transaction.unwrap().into();

    /*
        /* generic commands */
        let wit_generic_commands =
            crate::component::guest::sui_ctx::get_programmable_transaction_commands();
        /* publish commands */
        let wit_publish_commands =
            crate::component::guest::sui_ctx::get_programmable_transaction_publish_commands();
        //let wit_publish_command_modules =
        //    crate::component::guest::sui_ctx::get_programmable_transaction_publish_command_modules();
        let wit_publish_command_deps =
            crate::component::guest::sui_ctx::get_programmable_transaction_publish_command_dependencies(
            );

        /* upgrade commands */
        let wit_upgrade_commands =
            crate::component::guest::sui_ctx::get_programmable_transaction_upgrade_commands();
        //let wit_upgrade_command_modules =
        //    crate::component::guest::sui_ctx::get_programmable_transaction_upgrade_command_modules();
        let wit_upgrade_command_deps =
            crate::component::guest::sui_ctx::get_programmable_transaction_upgrade_command_dependencies(
            );

        let new_command_seq = 1;
        let mut commands: Vec<Command> = Vec::new();
        for command in wit_generic_commands.iter() {
            let seq = command.seq;

            /* search as a publish command */
            let publish_commands: Vec<&SuiProgrammableTransactionPublishCommand> = wit_publish_commands
                .iter()
                .filter(|c| c.command_seq == seq)
                .collect();

            assert!(
                publish_commands.len() > 1,
                "more of one commands with same seq={}, len={}",
                seq,
                publish_commands.len()
            );

            if !publish_commands.is_empty() {
                let wit_publish_command = publish_commands
                    .first()
                    .expect("It can not be command empty list");

                let publish_seq = wit_publish_command.seq;
                let publish_deps: Vec<&SuiProgrammableTransactionPublishCommandDependency> =
                    wit_publish_command_deps
                        .iter()
                        .filter(|c| c.publish_seq == publish_seq)
                        .collect();

                let deps = publish_deps
                    .iter()
                    .map(|d| new_other_object(d.object_id.clone()))
                    .collect();

                let publish_command: PublishCommand = PublishCommand {
                    seq: new_command_seq,
                    deps,
                    inner_sec: publish_seq,
                };
                commands.push(Command::Publish(publish_command));
                continue;
            }

            /* search as upgrade command */
            let upgrade_commands: Vec<&SuiProgrammableTransactionUpgradeCommand> = wit_upgrade_commands
                .iter()
                .filter(|c| c.command_seq == seq)
                .collect();

            assert!(
                upgrade_commands.len() > 1,
                "more of one commands with same seq={}, len={}",
                seq,
                upgrade_commands.len()
            );

            if !upgrade_commands.is_empty() {
                let wit_upgrade_command = upgrade_commands
                    .first()
                    .expect("It can not be command empty list");

                let upgrade_seq = wit_upgrade_command.seq;
                let upgrade_deps: Vec<&SuiProgrammableTransactionUpgradeCommandDependency> =
                    wit_upgrade_command_deps
                        .iter()
                        .filter(|c| c.upgrade_seq == upgrade_seq)
                        .collect();

                let deps = upgrade_deps
                    .iter()
                    .map(|d| new_other_object(d.object_id.clone()))
                    .collect();

                let upgrade_command: UpgradeCommand =
                    UpgradeCommand::new(new_command_seq, deps, upgrade_seq);
                commands.push(Command::Upgrade(upgrade_command));
                continue;
            }
            commands.push(Command::Other(OtherCommand { inner_seq: seq }));
        }
        if let TransactionKind::ProgrammableTransaction(ref mut kind) = inner_transaction.kind {
            kind.commands.append(&mut commands);
        };
    */
    //inner_transaction
    SuiCtx {
        transaction: Some(inner_transaction),
    }
}
