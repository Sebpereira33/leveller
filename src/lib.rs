use scrypto::prelude::*;

#[blueprint]
mod stable_token {
    struct StableToken {
        stable_token_manager: ResourceManager,
    }
    impl StableToken {
        
        pub fn instantiate() -> Global<StableToken> {
        
            let special_tokens: ResourceManager = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata! (
                    init {
                        "name" => "Leveller", locked;
                        "symbol" => "LV", locked;
                        }
                    ))
                .mint_roles(mint_roles!{
                    minter => rule!(allow_all);
                    minter_updater => rule!(deny_all);    
                    })
                .burn_roles(burn_roles!(
                        burner => rule!(allow_all);
                        burner_updater => rule!(deny_all);
                    ))
                .create_with_no_initial_supply();

            Self {
                stable_token_manager: special_tokens,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }
    }
}
