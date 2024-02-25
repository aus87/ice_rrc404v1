use radix_engine::blueprints::account;
use radix_engine::blueprints::package;
use radix_engine::vm::NoExtension;
use scrypto_test::prelude::InMemorySubstateDatabase;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::{builder::ManifestBuilder, manifest::decompiler::ManifestObjectNames};
use radix_engine::transaction::{CommitResult, TransactionReceipt};
use radix_engine::transaction::BalanceChange;
use scrypto_unit::{TestRunner, TestRunnerBuilder};
use std::env;
use transaction::prelude::*;
// use dot_random_test_utils::{deploy_random_component, RandomTestEnv};

#[derive(Clone)]
pub struct Account {
    public_key: Secp256k1PublicKey,
    account_address: ComponentAddress,
} 

pub struct TestEnvironment {
    test_runner: TestRunner<NoExtension, InMemorySubstateDatabase>,
    account1: Account,
    account2: Account,
    account3: Account,
    account4: Account,
    account5: Account,
    account6: Account,
    package_address: PackageAddress,
    rrc404_component: ComponentAddress,
    rrc404_fungible: ResourceAddress,
    rrc404_nft: ResourceAddress,
}

impl TestEnvironment {
    pub fn instantiate_test() -> Self {

        // Step 1 - get all data to set up test
        let mut test_runner = TestRunnerBuilder::new().without_trace().build();

        let (public_key1, _private_key1, account_address1) = test_runner.new_allocated_account();
        let account1 = Account { public_key: public_key1, account_address: account_address1 };
        
        let (public_key2, _private_key2, account_address2) = test_runner.new_allocated_account();
        let account2 = Account { public_key: public_key2, account_address: account_address2 };

        let (public_key3, _private_key3, account_address3) = test_runner.new_allocated_account();
        let account3 = Account { public_key: public_key3, account_address: account_address3 };

        let (public_key4, _private_key4, account_address4) = test_runner.new_allocated_account();
        let account4 = Account { public_key: public_key4, account_address: account_address4 };

        let (public_key5, _private_key5, account_address5) = test_runner.new_allocated_account();
        let account5 = Account { public_key: public_key5, account_address: account_address5 };

        let (public_key6, _private_key6, account_address6) = test_runner.new_allocated_account();
        let account6 = Account { public_key: public_key6, account_address: account_address6 };

        // MLEEKO CODE *********
        // publish mleeko's package
        // let mut random_env = deploy_random_component(&mut test_runner, "f37fe3e");
        // MLEEKO CODE *********
        
        // let dec = AddressBech32Decoder::new(&NetworkDefinition::mainnet());
        // let mut pre_allocated_addresses: Vec<PreAllocatedAddress> = Vec::new();
        // // NodeId("5db295063f684cc508046fd6ed57d17916a82e48303c683cff03bc63913f")
        // let BOBBY_MASTER_KEY_ADDR: GlobalAddress = GlobalAddress::try_from_bech32(&dec, "resource_rdx1tkef2p3ldpxv2zqydltw64730yt2stjgxq7xs08lqw7x8yfl3avncz").unwrap();

        // pre_allocated_addresses.push((
        //     BlueprintId {
        //          package_address: RESOURCE_PACKAGE,
        //          blueprint_name: FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
        //      },
        //     BOBBY_MASTER_KEY_ADDR,
        // ).into());

        // let receipt = test_runner.execute_system_transaction_with_preallocated_addresses(
        //     vec![
        //         InstructionV1::CallFunction {
        //             package_address: RESOURCE_PACKAGE.into(),
        //             blueprint_name: FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
        //             function_name: FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
        //                 .to_string(),
        //             args: to_manifest_value_and_unwrap!(
        //                     &FungibleResourceManagerCreateWithInitialSupplyManifestInput {
        //                         owner_role: OwnerRole::None,
        //                         divisibility: DIVISIBILITY_NONE,
        //                         track_total_supply: false,
        //                         metadata: Default::default(),
        //                         resource_roles: FungibleResourceRoles::default(),
        //                         initial_supply: dec!(10),
        //                         address_reservation: Some(ManifestAddressReservation(0)),
        //                     }
        //                 ),
        //         }.into(),
        //         InstructionV1::CallMethod {
        //             address: DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(account1.account_address.into())),
        //             method_name: "deposit_batch".to_string(),
        //             args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        //         }],
        //     pre_allocated_addresses,
        //     btreeset!(NonFungibleGlobalId::from_public_key(&account1.public_key)),
        // );

        // receipt.expect_commit_success();

        let package_address = test_runner.compile_and_publish(this_package!());
        
        // Step 2 - Build transaction manifest with data from step 1
        let manifest = ManifestBuilder::new()
            .call_function(
                package_address, 
                "Rrc404",
                "instantiate_rrc404",
                manifest_args!(
                    dec!(100),
                    "Ice",
                    "Water",
                    "ICE",
                    "Ice token",
                )
            )
            .deposit_batch(account_address1)
            .build();
        
        // Step 3 - Execute transaction manifest
        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest, 
            vec![NonFungibleGlobalId::from_public_key(&public_key1)]
        );
        
        println!("Transaction Receipt: {:?}", receipt);

        // Step 4 - Expect commit success
        let commit_success = receipt.expect_commit_success();

        println!("Balance Changes: {:?}", commit_success.vault_balance_changes());

        let rrc404_component = commit_success.new_component_addresses()[0];
        let rrc404_fungible = commit_success.new_resource_addresses()[0];
        let rrc404_nft = commit_success.new_resource_addresses()[1];
        
        Self {
            test_runner,
            account1,
            account2,
            account3,
            account4,
            account5,
            account6,
            package_address,
            rrc404_component,
            rrc404_fungible,
            rrc404_nft,

        }
    }

    pub fn execute_manifest_ignoring_fee_account_with_mani(
        &mut self, 
        manifest_names: ManifestObjectNames, 
        manifest: TransactionManifestV1, 
        name: &str,
        network: &NetworkDefinition,
        account: Account,
    ) -> TransactionReceipt {
    
        dump_manifest_to_file_system(
            manifest_names,
            &manifest,
            "./manis",
            Some(name),
            network,
        )
        .err();
    
        self.test_runner.execute_manifest_ignoring_fee(
            manifest, 
            vec![NonFungibleGlobalId::from_public_key(&account.public_key)]
        )
    }

    pub fn execute_manifest_ignoring_fee_account(
        &mut self, 
        manifest_names: ManifestObjectNames, 
        manifest: TransactionManifestV1, 
        account: Account,
    ) -> TransactionReceipt {
    
        self.test_runner.execute_manifest_ignoring_fee(
            manifest, 
            vec![NonFungibleGlobalId::from_public_key(&account.public_key)]
        )
    }

    pub fn execute_manifest(
        &mut self,
        manifest: TransactionManifestV1, 
        account: Account,
    ) -> TransactionReceipt {
    
        self.test_runner.execute_manifest(
            manifest, 
            vec![NonFungibleGlobalId::from_public_key(&account.public_key)]
        )
    }

    // pub fn execute_next(&mut self, random_number: u32) -> CommitResult {
    //     self.random_env.execute_next(&mut self.test_runner, random_number)
    // }
}

#[test]
fn instantiate_test() {
    TestEnvironment::instantiate_test();
}

#[test]
fn convert_fungibles_to_nfts() {
    let mut test_environment = TestEnvironment::instantiate_test();
    let account_address = test_environment.account1.account_address;
    let rrc404_component = test_environment.rrc404_component;
    let package_address = test_environment.package_address;
    let rrc404_fungible = test_environment.rrc404_fungible;

    // Part 2 Build Manifest
    let manifest = ManifestBuilder::new()
        .lock_fee(account_address, 50)
        .withdraw_from_account(
            account_address,
            rrc404_fungible,
            dec!(100)
        )
        .take_from_worktop(rrc404_fungible, dec!(100), "fungible_bucket")
        .with_name_lookup(|builder, lookup| { 
            builder.call_method(
                rrc404_component, 
                "freeze", 
                manifest_args!(
                    lookup.bucket("fungible_bucket"),
                )
            )
        })
        .deposit_batch(account_address);

    // Part 3 Execute Manifest
    let receipt = test_environment.execute_manifest(
        manifest.build(),
        test_environment.account1.clone(),
    );
    println!("***********Transaction Receipt***************: {:?}", receipt);
}

#[test]
fn convert_nfts_to_fungibles() {
    let mut test_environment = TestEnvironment::instantiate_test();
    let account_address = test_environment.account1.account_address;
    let rrc404_component = test_environment.rrc404_component;
    let package_address = test_environment.package_address;
    let rrc404_fungible = test_environment.rrc404_fungible;
    let rrc404_nft = test_environment.rrc404_nft;

    // Part 2 Build Manifest
    let manifest = ManifestBuilder::new()
        .lock_fee(account_address, 50)
        .withdraw_from_account(
            account_address,
            rrc404_fungible,
            dec!(50.9)
        )
        .take_from_worktop(rrc404_fungible, dec!(50.9), "fungible_bucket")
        .with_name_lookup(|builder, lookup| { 
            builder.call_method(
                rrc404_component, 
                "convert_fungibles_to_nfts", 
                manifest_args!(
                    lookup.bucket("fungible_bucket"),
                )
            )
        })
        .deposit_batch(account_address);

    // Part 3 Execute Manifest
    let receipt = test_environment.execute_manifest(
        manifest.build(),
        test_environment.account1.clone(),
    );
    println!("***********Transaction Receipt***************: {:?}", receipt);

    // Part 2 Build Manifest
    let manifest = ManifestBuilder::new()
        .lock_fee(account_address, 50)
        .withdraw_from_account(
            account_address,
            rrc404_nft,
            dec!(40)
        )
        .take_from_worktop(rrc404_nft, dec!(40), "nft_bucket")
        .with_name_lookup(|builder, lookup| { 
            builder.call_method(
                rrc404_component, 
                "convert_nfts_to_fungibles", 
                manifest_args!(
                    lookup.bucket("nft_bucket"),
                )
            )
        })
        .deposit_batch(account_address);

    // Part 3 Execute Manifest
    let receipt = test_environment.execute_manifest(
        manifest.build(),
        test_environment.account1.clone(),
    );
    println!("***********Transaction Receipt***************: {:?}", receipt);

}

#[test]
fn multiple_swaps() {
    let mut test_environment = TestEnvironment::instantiate_test();
    let account_address = test_environment.account1.account_address;
    let rrc404_component = test_environment.rrc404_component;
    let package_address = test_environment.package_address;
    let rrc404_fungible = test_environment.rrc404_fungible;
    let rrc404_nft = test_environment.rrc404_nft;

    // Part 2 Build Manifest
    let manifest = ManifestBuilder::new()
        .lock_fee(account_address, 50)
        .withdraw_from_account(
            account_address,
            rrc404_fungible,
            dec!(50.9)
        )
        .take_from_worktop(rrc404_fungible, dec!(50.9), "fungible_bucket")
        .with_name_lookup(|builder, lookup| { 
            builder.call_method(
                rrc404_component, 
                "convert_fungibles_to_nfts", 
                manifest_args!(
                    lookup.bucket("fungible_bucket"),
                )
            )
        })
        .deposit_batch(account_address);

    // Part 3 Execute Manifest
    let receipt = test_environment.execute_manifest(
        manifest.build(),
        test_environment.account1.clone(),
    );
    println!("***********Transaction Receipt***************: {:?}", receipt);

    // Part 2 Build Manifest
    let manifest = ManifestBuilder::new()
        .lock_fee(account_address, 50)
        .withdraw_from_account(
            account_address,
            rrc404_nft,
            dec!(40)
        )
        .take_from_worktop(rrc404_nft, dec!(40), "nft_bucket")
        .with_name_lookup(|builder, lookup| { 
            builder.call_method(
                rrc404_component, 
                "convert_nfts_to_fungibles", 
                manifest_args!(
                    lookup.bucket("nft_bucket"),
                )
            )
        })
        .deposit_batch(account_address);

    // Part 3 Execute Manifest
    let receipt = test_environment.execute_manifest(
        manifest.build(),
        test_environment.account1.clone(),
    );
    println!("***********Transaction Receipt***************: {:?}", receipt);

}
