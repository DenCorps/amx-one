use mp_api::ApiError;
use substrate_test_runtime_client::runtime::Block;

mp_api::decl_runtime_apis! {
    pub trait Api {
        fn test();
    }
}

struct MockApi;

mp_api::mock_impl_runtime_apis! {
    impl Api<Block> for MockApi {
        #[advanced]
        fn test(&self, _: &Hash) -> Result<(), ApiError> {
            Ok(().into())
        }
    }
}

fn main() {}
