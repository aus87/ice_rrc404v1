use scrypto::prelude::*;

#[derive(ScryptoSbor, PartialEq, Debug, Clone)]
pub enum Color {
    Green,
    Blue,
    Red,
    Orange,
    Purple,
}

#[derive(NonFungibleData, ScryptoSbor, Debug)]
pub struct Rrc404NFT {
    key_image_url: Url,
    color: Color,
    fungible_address: ResourceAddress,
    last_roll: Instant,
}

impl Color {
    fn url(&self) -> Url {
        match self {
            Color::Green => Url::of("https://bafybeib6euav3xgpncqjtwmpz7ebckh37gccu3bvrnvyx3376dc57v6nyi.ipfs.w3s.link/ice_green.png"),
            Color::Blue => Url::of("https://bafybeihqaeyl2wpph27k5rkfzyhl5ddiy7an77prsznejojf2ahkxutdva.ipfs.w3s.link/ice_blue.png"),
            Color::Red => Url::of("https://bafybeib2ljsnwbntzynbgdvk75fvkab5ok6qmqxlkvtn3o33r3gpzvwohq.ipfs.w3s.link/ice_red.png"),
            Color::Orange => Url::of("https://bafybeie6mwtsxh4lyo72y3lp4y4qlsuk7s7szeaspks7wuklo7izll3bxu.ipfs.w3s.link/ice_orange.png"),
            Color::Purple => Url::of("https://bafybeib5ox4fdopvdcsonov6zwlbsy772fgwj6ufk6uq27rnrqfqtma4ri.ipfs.w3s.link/ice_purple.png"),
        }
    }
}

#[blueprint]
#[types(Vault, NonFungibleLocalId, NonFungibleGlobalId)]
mod rrc404 {

    const ICE_DEV: ResourceManager = resource_manager!(
        "resource_rdx1thcm6cm0v8km0n0auxf3fzg7tnghld99p37sl74zxugvzknde8qmwe"
    );

    struct Rrc404 {
        rrc404_fungible: ResourceManager,
        rrc404_nft: ResourceManager,
        nft_counter: u64,
        max_supply: Decimal,
        fungible_supply: Decimal,
        green_supply: Decimal,
        blue_supply: Decimal,
        red_supply: Decimal,
        orange_supply: Decimal,
        purple_supply: Decimal,
    }

    impl Rrc404 {

        pub fn instantiate_rrc404(
            max_supply: Decimal,
            nft_name: String,
            fungible_name: String,
            symbol: String,
            description: String,
        ) -> (Global<Rrc404>, Bucket) {

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(<Rrc404>::blueprint_id());
            
            let rrc404_fungible = ResourceBuilder::new_fungible(OwnerRole::Fixed(
                rule!(require(global_caller(component_address)))))
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata! {
                    init {
                        "name" => fungible_name.clone(), locked;
                        "symbol" => symbol, locked;
                        "description" => description.clone(), locked;
                        "icon_url" => Url::of("https://bafybeih7t7wrwqw2qhygocp3j5mjzajbiu44rg5yb57bcq5h3bsshpbwae.ipfs.w3s.link/ice_water.png"), updatable;
                    }
                })
                .mint_roles(mint_roles!{
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all); 
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .mint_initial_supply(max_supply);
            
            let rrc404_nft = ResourceBuilder::new_integer_non_fungible::<Rrc404NFT>(OwnerRole::Fixed(
                rule!(require(global_caller(component_address)))))
                .metadata(metadata!(
                    init {
                        "name" => nft_name.to_string(), updatable;
                        "description" => description, locked;
                        "icon_url" => Url::of("https://bafybeiet45kuv5ybffjcmowb5j6unp45aubtopag634ahmmnnhwk2ejnpe.ipfs.w3s.link/ice_badge.png"), updatable;
                    }
                ))
                .mint_roles(mint_roles!{
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all); 
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                    non_fungible_data_updater => rule!(require(global_caller(component_address)));
                    non_fungible_data_updater_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            let component = Self {
                rrc404_fungible: rrc404_fungible.resource_manager(),
                rrc404_nft,
                nft_counter: 1,
                max_supply,
                fungible_supply: Decimal::from(max_supply),
                green_supply: dec!(0),
                blue_supply: dec!(0),
                red_supply: dec!(0),
                orange_supply: dec!(0),
                purple_supply: dec!(0),
            }
            .instantiate()
            // .prepare_to_globalize(OwnerRole::None)
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(ICE_DEV.address()))))
            .with_address(address_reservation)
            .enable_component_royalties(component_royalties! {
                init {
                    freeze => Xrd(1.into()), updatable;
                    melt => Xrd(1.into()), updatable;
                }
            })
            .globalize();

            (component, rrc404_fungible.into())
        }

        /// Freeze takes in a bucket of fungibles, and returns a bucket of nfts and a bucket of change if there is any
        /// because only full tokens may be converted to NFTs.  
        pub fn freeze(&mut self, mut deposit: Bucket) -> (Bucket, Bucket) {
            assert!(deposit.resource_address() == self.rrc404_fungible.address(), "Incorrect resource address");
        
            let floor_amount = deposit.amount().checked_floor().unwrap();
            let deposit_amount = floor_amount.to_string().parse::<u64>().unwrap();
            let mut nft_bucket: Bucket = Bucket::new(self.rrc404_nft.address());
        
            for _ in 0..deposit_amount {
                let nft_id = NonFungibleLocalId::from(self.nft_counter);
        
                let circulating_nfts = self.green_supply + self.blue_supply + self.red_supply + self.orange_supply + self.purple_supply;

                // Target percentages to maintain for rarity
                let percentages: [Decimal; 5] = [dec!(0.42), dec!(0.30), dec!(0.16), dec!(0.08), dec!(0.04)];
                let current_counts = [self.green_supply, self.blue_supply, self.red_supply, self.orange_supply, self.purple_supply];
        
                let deficits: Vec<Decimal> = percentages.iter().enumerate().map(|(i, &percentage)| {
                    let target = Decimal::from(circulating_nfts) * percentage;
                    target - current_counts[i]
                }).collect();
        
                let color_index = deficits.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).map(|(index, _)| index).unwrap_or(0);

                let mut color = match color_index {
                    0 => Color::Green,
                    1 => Color::Blue,
                    2 => Color::Red,
                    3 => Color::Orange,
                    _ => Color::Purple,
                };

                if circulating_nfts == dec!(0) {
                    color = Color::Green;
                }
                
                let data = Rrc404NFT {
                    key_image_url: color.url(),
                    color: color.clone(),
                    fungible_address: self.rrc404_fungible.address(),
                    last_roll: Clock::current_time_rounded_to_minutes(),
                };
        
                match &color {
                    Color::Green => self.green_supply += dec!(1),
                    Color::Blue => self.blue_supply += dec!(1),
                    Color::Red => self.red_supply += dec!(1),
                    Color::Orange => self.orange_supply += dec!(1),
                    Color::Purple => self.purple_supply += dec!(1),
                }
        
                let mut new_nft: Bucket = Bucket::new(self.rrc404_nft.address());
                new_nft.put(self.rrc404_nft.mint_non_fungible(&nft_id.into(), data));
                new_nft.resource_manager().set_metadata("key_image_url", color.url());
        
                nft_bucket.put(new_nft);
                self.nft_counter += 1;
                self.fungible_supply -= dec!(1);
            }
        
            let burn_bucket: Bucket = deposit.take(floor_amount);
            burn_bucket.burn();
        
            // this is the logic that ensures the circulating supply never exceeds the max supply
            assert!(self.fungible_supply + self.green_supply + self.blue_supply + 
                self.red_supply + self.orange_supply + self.purple_supply <= self.max_supply, 
                "Circulating supply can't exceed max supply of {} tokens", self.max_supply
            );
        
            (nft_bucket, deposit)
        }

        // Convert nfts to fungibles
        pub fn melt(&mut self, nft_bucket: Bucket) -> Bucket{
            
            // Assert resource address matches the resource address of the vault
            assert!(nft_bucket.resource_address() == self.rrc404_nft.address(), "Incorrect resource address");

            for nft_id in nft_bucket.as_non_fungible().non_fungible_local_ids() {
                
                let data = self.rrc404_nft.get_non_fungible_data::<Rrc404NFT>(&nft_id);

                // Check that each nft is past a 4 hour cooldown period for rerolling
                let last_roll = data.last_roll;   
                let last_roll_utc = UtcDateTime::try_from(last_roll).unwrap();
                let next_roll_utc= last_roll_utc.add_hours(4).unwrap();
                let next_roll = Instant::try_from(next_roll_utc).unwrap();            

                assert!(Clock::current_time_is_at_or_after(next_roll, TimePrecision::Minute),
                    "There is a 4 hour delay between minting and rerolling"
                );

                let color = data.color.clone();

                if color == Color::Green {
                    self.green_supply -= dec!(1);
                } else if color == Color::Blue {
                    self.blue_supply -= dec!(1);
                } else  if  color == Color::Red {
                    self.red_supply -= dec!(1);
                } else if color == Color::Orange {
                    self.orange_supply -= dec!(1);
                } else if color == Color::Purple {
                    self.purple_supply -= dec!(1);
                }

                self.fungible_supply += dec!(1);
            }

            assert!(self.fungible_supply + self.green_supply + self.blue_supply + self.red_supply + 
                self.orange_supply + self.purple_supply <= self.max_supply, 
                "Circulating supply cant exceed max supply of {} tokens", self.max_supply
            );

            let fungible_bucket = self.rrc404_fungible.mint(nft_bucket.amount());
            nft_bucket.burn();

            fungible_bucket
        }
    }
}