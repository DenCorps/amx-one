use mp_runtime::traits::GetNodeBlockType;
use substrate_test_runtime_client::runtime::Block;

/// The declaration of the `Runtime` type and the implementation of the `GetNodeBlockType`
/// trait are done by the `construct_runtime!` macro in a real runtime.
struct Runtime {}
impl GetNodeBlockType for Runtime {
    type NodeBlock = Block;
}

mp_api::decl_runtime_apis! {
    pub trait Api {
        fn test(data: u64);
    }
}

mp_api::impl_runtime_apis! {
    impl self::Api for Runtime {
        fn test(data: u64) {
            unimplemented!()
        }
    }
}

fn main() {}
