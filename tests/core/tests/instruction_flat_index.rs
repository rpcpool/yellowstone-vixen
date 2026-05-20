use yellowstone_grpc_proto::prost::Message as _;
use yellowstone_vixen_core::{instruction::InstructionUpdate, TransactionUpdate};
use yellowstone_vixen_mock::{FixtureData, SerializableInstructionUpdate};

const DP4_TX_FIXTURE: &[u8] = include_bytes!("../fixtures/transactions/dp4_flat_index_tx.bin");
const DP4_JSON_FIXTURE: &[u8] = include_bytes!(concat!(
    "../fixtures/transactions/",
    "dP4TfbypYZoTkDLFAojCXQHmxyV6vQ3sgbHJtQhMHWpTAEpxGSTTAG63woEqxVvgq9QQLSFhRXpRwii2RkF6xnD",
    "_tx.json",
));
const DP4_SIGNATURE: &str =
    "dP4TfbypYZoTkDLFAojCXQHmxyV6vQ3sgbHJtQhMHWpTAEpxGSTTAG63woEqxVvgq9QQLSFhRXpRwii2RkF6xnD";

#[derive(Debug, PartialEq, Eq)]
struct FlatInstructionRecord {
    flat_index: String,
    program: String,
    accounts: Vec<String>,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
struct TreeInstructionRecord {
    program: String,
    tree_path: String,
    flat_index: String,
}

#[test]
fn real_backfilled_tx_keeps_flat_indices_from_solana_inner_instruction_order() {
    let transaction =
        TransactionUpdate::decode(DP4_TX_FIXTURE).expect("backfilled transaction should decode");
    let transaction_info = transaction
        .transaction
        .as_ref()
        .expect("fixture should contain transaction info");

    assert_eq!(transaction.slot, 420_181_274);
    assert_eq!(
        yellowstone_vixen_core::bs58::encode(&transaction_info.signature).into_string(),
        DP4_SIGNATURE
    );

    let instructions =
        InstructionUpdate::build_from_txn(&transaction).expect("fixture should build");
    let json_fixture = match yellowstone_vixen_mock::read_instructions_fixture(DP4_JSON_FIXTURE)
        .expect("Solana RPC JSON fixture should decode")
    {
        FixtureData::Instructions(fixture) => fixture,
        FixtureData::Account(_) => panic!("expected transaction fixture"),
    };

    assert_eq!(
        json_fixture.log_messages, instructions[0].shared.log_messages,
        "JSON fixture should carry the same transaction logs as the protobuf fixture",
    );
    assert_eq!(
        observed_flat_records(&instructions),
        json_flat_records(&json_fixture.instructions, instructions.len()),
        "backfilled protobuf fixture should match the Solana RPC JSON fixture",
    );

    let observed = instructions[2]
        .visit_all_with_flat_indices()
        .map(|(instruction, flat_index)| TreeInstructionRecord {
            program: instruction.program.to_string(),
            tree_path: format!("{:?}", instruction.path),
            flat_index: format!("{flat_index:?}"),
        })
        .collect::<Vec<_>>();

    let expected = [
        ("RLAYHr9TRFcKB2ubYQhspcnXiaGpaVzNQvHytt47RZu", "3", "3"),
        ("11111111111111111111111111111111", "3.1", "3.1"),
        ("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL", "3.2", "3.2"),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3.2.1",
            "3.3",
        ),
        ("11111111111111111111111111111111", "3.2.2", "3.4"),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3.2.3",
            "3.5",
        ),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3.2.4",
            "3.6",
        ),
        ("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4", "3.3", "3.7"),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3.3.1",
            "3.8",
        ),
        (
            "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
            "3.3.2",
            "3.9",
        ),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3.3.3",
            "3.10",
        ),
        (
            "ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA",
            "3.3.4",
            "3.11",
        ),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3.3.4.1",
            "3.12",
        ),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3.3.4.2",
            "3.13",
        ),
        (
            "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
            "3.3.5",
            "3.14",
        ),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3.3.6",
            "3.15",
        ),
        ("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", "3.4", "3.16"),
        ("RLAYHr9TRFcKB2ubYQhspcnXiaGpaVzNQvHytt47RZu", "3.5", "3.17"),
    ];

    assert_eq!(
        observed,
        expected.map(|(program, tree_path, flat_index)| TreeInstructionRecord {
            program: program.to_owned(),
            tree_path: tree_path.to_owned(),
            flat_index: flat_index.to_owned(),
        })
    );
}

fn observed_flat_records(instructions: &[InstructionUpdate]) -> Vec<FlatInstructionRecord> {
    instructions
        .iter()
        .flat_map(|instruction| instruction.visit_all_with_flat_indices())
        .map(|(instruction, flat_index)| FlatInstructionRecord {
            flat_index: format!("{flat_index:?}"),
            program: instruction.program.to_string(),
            accounts: instruction
                .accounts
                .iter()
                .map(ToString::to_string)
                .collect(),
            data: instruction.data.clone(),
        })
        .collect()
}

fn json_flat_records(
    instructions: &[SerializableInstructionUpdate],
    outer_instruction_count: usize,
) -> Vec<FlatInstructionRecord> {
    instructions
        .iter()
        .enumerate()
        .map(|(position, instruction)| FlatInstructionRecord {
            flat_index: json_flat_index(instruction.ix_index, position < outer_instruction_count),
            program: instruction.program.to_string(),
            accounts: instruction
                .accounts
                .iter()
                .map(ToString::to_string)
                .collect(),
            data: instruction.data.clone(),
        })
        .collect()
}

fn json_flat_index([outer, inner]: [usize; 2], is_outer_instruction: bool) -> String {
    if is_outer_instruction {
        return (outer + 1).to_string();
    }

    format!("{}.{}", outer + 1, inner + 1)
}
