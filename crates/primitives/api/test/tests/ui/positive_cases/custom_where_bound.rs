use codec::{Decode, Encode};
use mp_runtime::traits::{Block as BlockT, GetNodeBlockType};
use scale_info::TypeInfo;
use substrate_test_runtime_client::runtime::Block;

struct Runtime {}
impl GetNodeBlockType for Runtime {
    type NodeBlock = Block;
}

pub trait CustomTrait: Encode + Decode + TypeInfo {}

#[derive(Encode, Decode, TypeInfo)]
pub struct SomeImpl;
impl CustomTrait for SomeImpl {}

#[derive(Encode, Decode, TypeInfo)]
pub struct SomeOtherType<C: CustomTrait>(C);

mp_api::decl_runtime_apis! {
    pub trait Api<A> where A: CustomTrait {
        fn test() -> A;
        fn test2() -> SomeOtherType<A>;
    }
}

mp_api::impl_runtime_apis! {
    impl self::Api<Block, SomeImpl> for Runtime {
        fn test() -> SomeImpl { SomeImpl }
        fn test2() -> SomeOtherType<SomeImpl> { SomeOtherType(SomeImpl) }
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
