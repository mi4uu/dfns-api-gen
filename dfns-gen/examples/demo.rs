// Demo showing how to use the generated types
use dfns_gen::generated::*;

fn main() {
    // Example: Create a BlockchainKind enum
    let blockchain = BlockchainKind::Evm;
    println!("Blockchain: {:?}", blockchain);

    // Serialize to JSON
    let json = serde_json::to_string(&blockchain).unwrap();
    println!("Serialized: {}", json);

    // Deserialize from JSON
    let deserialized: BlockchainKind = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {:?}", deserialized);

    // Example: Create a CantonValidator struct
    let validator = CantonValidator {
        date_created: "2025-10-20T00:00:00Z".to_string(),
        id: "cv-12345".to_string(),
        kind: "Validator".to_string(),
        name: Some("My Validator".to_string()),
        network: "CantonTestnet".to_string(),
        org_id: "or-12345-67890-abcdefghijklmnop".to_string(),
        party_hint: "party::validator::1".to_string(),
    };

    println!("\nValidator: {:#?}", validator);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&validator).unwrap();
    println!("\nSerialized Validator:\n{}", json);
}
