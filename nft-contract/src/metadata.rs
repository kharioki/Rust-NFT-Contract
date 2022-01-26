use crate::*;
pub type TokenId = String;
//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
} 

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String, // required, essentially a version like "0.1.0"
    pub name: String, // required, the name of the contract e.g. "Mosaics"
    pub symbol: String, // required, the symbol of the contract e.g. "MOSAIC"
    pub icon: Option<String>, // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by 'reference' or 'media' URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<String>, // Base64-encoded SHA-256 hash of the JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Parcel #5055"
    pub description: Option<String>, // free-form description ex. "A parcel of land"
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<String>, // Base64-encoded SHA-256 hash of the media. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set metadata in existence when token was minted
    pub issued_at: Option<u64>, // when token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // when token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // when token starts being redeemable/valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // when token metadata was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info
    pub reference_hash: Option<String>, // Base64-encoded SHA-256 hash of the JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    // owner of the token
    pub owner_id: AccountId,
}

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    // token ID
    pub token_id: TokenId,
    // owner of the token
    pub owner_id: AccountId,
    // token metadata
    pub metadata: TokenMetadata,
}

pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
