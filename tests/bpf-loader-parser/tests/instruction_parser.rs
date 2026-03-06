use yellowstone_vixen_bpf_loader_parser::{instruction, InstructionParser};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;

#[tokio::test]
async fn parse_initialize_buffer_ix() {
    let parser = InstructionParser;

    let ixs = tx_fixture!(
        "2k5ruz3otFDFtszxAPm6MmB6YSEsXLDgEMPzA9ZK3dZquXmpPCPkx5HR9r9idZKywSZTwGWZ2gAvGgVXYyfWWeFC",
        &parser
    );

    let ix0 = &ixs[0];

    let Some(instruction::Instruction::InitializeBuffer(ib)) =
        ix0.as_ref().and_then(|i| i.instruction.as_ref())
    else {
        panic!("Invalid Instruction");
    };

    let accounts = ib.accounts.as_ref().expect("missing accounts");
    assert!(!accounts.buffer.value.is_empty());
    assert!(!accounts.authority.value.is_empty());
}

#[tokio::test]
async fn parse_upgrade_ix() {
    let parser = InstructionParser;

    let ixs = tx_fixture!(
        "31AUfFXG6BJQjaqwBsCjjZV5ojEL4zbrJ9gKQfKHDMosPvJKQBy6dKTiZgkkjoKbG1StD11csqgWn1KU5EwQsUgX",
        &parser
    );

    let ix0 = &ixs[0];

    let Some(instruction::Instruction::Upgrade(up)) =
        ix0.as_ref().and_then(|i| i.instruction.as_ref())
    else {
        panic!("Invalid Instruction");
    };

    let accounts = up.accounts.as_ref().expect("missing accounts");
    assert!(!accounts.program_data.value.is_empty());
    assert!(!accounts.program.value.is_empty());
    assert!(!accounts.buffer.value.is_empty());
    assert!(!accounts.spill.value.is_empty());
    assert!(!accounts.authority.value.is_empty());
}
