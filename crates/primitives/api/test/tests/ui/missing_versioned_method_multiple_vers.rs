use mp_runtime::traits::{Block as BlockT, GetNodeBlockType};
use substrate_test_runtime_client::runtime::Block;

struct Runtime {}
impl GetNodeBlockType for Runtime {
    type NodeBlock = Block;
}

mp_api::decl_runtime_apis! {
    #[api_version(2)]
    pub trait Api {
        fn test1();
        fn test2();
        #[api_version(3)]
        fn test3();
        #[api_version(4)]
        fn test4();
    }
}

mp_api::impl_runtime_apis! {
    #[api_version(4)]
    impl self::Api<Block> for Runtime {
        fn test1() {}
        fn test2() {}
        fn test4() {}
    }

    impl mp_api::Core<Block> for Runtime {
        fn version() -> mp_version::RuntimeVersion {
            unimplemented!()
        }
        fn execute_block(_: Block) {
            unimplemented!()
        }
        fn initialize_block(_: &<Block as BlockT>::Header) {
            unimplemented!()
        }
    }
}

fn main() {}
