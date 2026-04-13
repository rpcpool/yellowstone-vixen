// Regression test for pod_types_missing: loopscale.json defines Pod* types
// (PodU32, PodU64, PodU32CBPS, etc.) as single-item tupleTypeNodes that are
// registered as type aliases. Structs earlier in the defined_types list
// reference these aliases. Without the two-pass fix in `build_defined_types`
// (registering all aliases before processing structs), those fields would
// retain unresolved `Message("PodU32CBPS")` references, causing E0412
// ("cannot find type PodU32CBPS in this scope").

use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/loopscale.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(loopscale::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(loopscale::PROTOBUF_SCHEMA);
}

#[test]
fn instruction_dispatch_index_is_some() {
    assert!(
        loopscale::INSTRUCTION_DISPATCH_MESSAGE_INDEX.is_some(),
        "expected InstructionDispatch message index for a program with instructions"
    );
}
