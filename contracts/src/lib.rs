#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use stylus_sdk::{
    alloc::string::ToString,
    prelude::*,
    stylus_proc::external,
    traits::{AbiDecode, AbiEncode},
};

// Constants
const MINIMUM_EVENT_COST: u128 = 1_000_000; // 0.001 ETH in wei
const PLATFORM_FEE_PERCENTAGE: u8 = 5; // 5% platform fee
const TOKENS_PER_EVENT: U256 = U256::from(100); // 100 tokens per event completion
const NFT_BASE_URI: &str = "ipfs://QmPoleCampNFT/"; // Base URI for NFTs

// Token interfaces
#[external]
pub trait IERC20 {
    fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, Vec<u8>>;
    fn balance_of(&self, account: Address) -> U256;
    fn total_supply(&self) -> U256;
}

#[external]
pub trait IERC721 {
    fn mint(&mut self, to: Address, token_id: U256) -> Result<bool, Vec<u8>>;
    fn owner_of(&self, token_id: U256) -> Address;
    fn token_uri(&self, token_id: U256) -> String;
}

#[derive(PartialEq, Eq, Debug, AbiEncode, AbiDecode)]
pub enum UserType {
    Student,
    Instructor,
}

#[derive(PartialEq, Eq, Debug, AbiEncode, AbiDecode)]
pub struct User {
    ipfs_url: String,
    user_address: Address,
    category: String,
    user_type: UserType,
}

#[derive(PartialEq, Eq, Debug, AbiEncode, AbiDecode)]
pub struct Event {
    id: U256,
    creator: Address,
    name: String,
    description: String,
    cost: U256,
    capacity: u32,
    start_time: U256,
    end_time: U256,
    is_group_event: bool,
    completed: bool,
}

#[derive(PartialEq, Eq, Debug, AbiEncode, AbiDecode)]
pub struct NFTMetadata {
    event_id: U256,
    user_type: UserType,
    timestamp: U256,
}

#[external]
#[storage]
pub struct Polecamp {
    owner: StorageAddress,
    pending_instructors: StorageMap<Address, User>,
    approved_instructors: StorageMap<Address, User>,
    events: StorageMap<U256, Event>,
    event_instructors: StorageMap<U256, Vec<Address>>,
    event_users: StorageMap<U256, Vec<Address>>,
    user_points: StorageMap<Address, U256>,
    event_counter: StorageU256,
    instructor_ratings: StorageMap<(Address, U256), Vec<u8>>,
    
    // Token related storage
    token_balances: StorageMap<Address, U256>,
    total_token_supply: StorageU256,
    
    // NFT related storage
    nft_counter: StorageU256,
    nft_owners: StorageMap<U256, Address>,
    nft_metadata: StorageMap<U256, NFTMetadata>,
    user_nfts: StorageMap<Address, Vec<U256>>,
    completed_events: StorageMap<(Address, U256), bool>, // (user, eventId) -> completed
}

// Events
#[event]
pub struct InstructorCreated {
    instructor_address: Address,
    ipfs_url: String,
    category: String,
}

#[event]
pub struct InstructorApproved {
    instructor_address: Address,
}

#[event]
pub struct EventCreated {
    event_id: U256,
    creator: Address,
    name: String,
    cost: U256,
}

#[event]
pub struct NFTMinted {
    token_id: U256,
    recipient: Address,
    event_id: U256,
    user_type: UserType,
}

#[event]
pub struct TokensAwarded {
    recipient: Address,
    amount: U256,
    event_id: U256,
}

#[event]
pub struct EventCompleted {
    event_id: U256,
    timestamp: U256,
}

impl Polecamp {
    #[constructor]
    pub fn new() -> Self {
        let mut instance = Self::default();
        instance.owner.set(msg::sender());
        instance.event_counter.set(U256::ZERO);
        instance.nft_counter.set(U256::ZERO);
        instance.total_token_supply.set(U256::ZERO);
        instance
    }

    // ... [Previous functions remain the same until create_event] ...

    #[external]
    pub fn complete_event(&mut self, event_id: U256) -> Result<bool, Vec<u8>> {
        let mut event = self.events.get(&event_id)
            .ok_or("Event not found".as_bytes().to_vec())?;
            
        ensure!(event.creator == msg::sender(), "Only creator can complete event");
        ensure!(!event.completed, "Event already completed");
        
        let users = self.event_users.get(&event_id).unwrap_or_default();
        let instructors = self.event_instructors.get(&event_id).unwrap_or_default();
        
        event.completed = true;
        self.events.insert(event_id, event);
        
        // Mint NFTs and award tokens to participants
        for user in users.iter() {
            self.mint_completion_nft(*user, event_id, UserType::Student)?;
            self.award_tokens(*user, event_id)?;
            self.completed_events.insert((*user, event_id), true);
        }
        
        // Mint NFTs and award tokens to instructors
        for instructor in instructors.iter() {
            self.mint_completion_nft(*instructor, event_id, UserType::Instructor)?;
            self.award_tokens(*instructor, event_id)?;
            self.completed_events.insert((*instructor, event_id), true);
        }
        
        evm::log(EventCompleted {
            event_id,
            timestamp: block::timestamp(),
        });
        
        Ok(true)
    }

    #[external]
    fn mint_completion_nft(
        &mut self,
        recipient: Address,
        event_id: U256,
        user_type: UserType,
    ) -> Result<bool, Vec<u8>> {
        let token_id = self.nft_counter.get() + 1.into();
        self.nft_counter.set(token_id);
        
        let metadata = NFTMetadata {
            event_id,
            user_type,
            timestamp: block::timestamp(),
        };
        
        self.nft_metadata.insert(token_id, metadata);
        self.nft_owners.insert(token_id, recipient);
        
        let mut user_nfts = self.user_nfts.get(&recipient).unwrap_or_default();
        user_nfts.push(token_id);
        self.user_nfts.insert(recipient, user_nfts);
        
        evm::log(NFTMinted {
            token_id,
            recipient,
            event_id,
            user_type,
        });
        
        Ok(true)
    }

    #[external]
    fn award_tokens(&mut self, recipient: Address, event_id: U256) -> Result<bool, Vec<u8>> {
        let current_balance = self.token_balances.get(&recipient).unwrap_or_default();
        let new_balance = current_balance + TOKENS_PER_EVENT;
        
        self.token_balances.insert(recipient, new_balance);
        self.total_token_supply.set(self.total_token_supply.get() + TOKENS_PER_EVENT);
        
        evm::log(TokensAwarded {
            recipient,
            amount: TOKENS_PER_EVENT,
            event_id,
        });
        
        Ok(true)
    }

    // Token view functions
    #[external(view)]
    pub fn token_balance_of(&self, account: Address) -> U256 {
        self.token_balances.get(&account).unwrap_or_default()
    }

    #[external(view)]
    pub fn total_token_supply(&self) -> U256 {
        self.total_token_supply.get()
    }

    // NFT view functions
    #[external(view)]
    pub fn nft_owner_of(&self, token_id: U256) -> Option<Address> {
        self.nft_owners.get(&token_id)
    }

    #[external(view)]
    pub fn nft_metadata_of(&self, token_id: U256) -> Option<NFTMetadata> {
        self.nft_metadata.get(&token_id)
    }

    #[external(view)]
    pub fn get_user_nfts(&self, user: Address) -> Vec<U256> {
        self.user_nfts.get(&user).unwrap_or_default()
    }

    #[external(view)]
    pub fn has_completed_event(&self, user: Address, event_id: U256) -> bool {
        self.completed_events.get(&(user, event_id)).unwrap_or_default()
    }

    #[external(view)]
    pub fn token_uri(&self, token_id: U256) -> Result<String, Vec<u8>> {
        ensure!(self.nft_owners.contains_key(&token_id), "Token does not exist");
        Ok(format!("{}{}", NFT_BASE_URI, token_id))
    }
}