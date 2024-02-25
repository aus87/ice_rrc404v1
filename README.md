# Rrc404 - Possible ERC404 implementation on the Radix ledger

The approach to building something like ERC404 on Radix with Scrypto is VERY different than Solidity.  Unlike Ethereum and ERC tokens, Radix uses native resources that do not need to be defined in a smart contract every time they are used.  Instead, these resources with their basic and predictable behavior can be used to model a system that can accomplish similar goals to ERC404 contract.

If the goal of ERC404 was to provide a mechanism to blur the lines between fungibles and non fungibles so that an NFT collection could benefit from fungible liquidity such as on Uniswap, I think there are easier and better ways to accomplish this.  Ice is an experimental token that pairs 1 fungible and 1 nonfungible resource together, capping their supply together.  The maximum supply of all Ice Nfts + Ice fungibles is 1000.  Instead of every token and nft being paired in your account with ERC404, tokens exist as EITHER fungible OR NFT at any given time.  

You can lose an NFT you like if you use ERC404 tokens and are not careful about which fungible you transfer.  With RRC404, you can only convert between fungible and nonfungible tokens by interacting with the Ice component.  Instead of "rerolling" an NFT when you transfer the token, Ice NFTs can be transferred or traded in any way without any danger of being lost.  The owner can however use the Ice component to convert back and forth between fungible (Water) and non fungible (Ice) tokens.  In order to deter users from constantly rerolling NFTs, there is a 4 hour delay between when an NFT can be "melted" back to a fungible as well as a small royalty taken at each conversion.  This blueprint uses internal logic mints nfts with nonfungible data based upon circulating supply of each rarity of NFT in a similar way to ERC404.

There are three RTMs that may be used to interact with the Ice component.  There is one that converts fungibles to NFTs, and two that convert NFTs to fungibles.  This is because you could choose to withdraw an amount of NFTs from your wallet or you could withdraw specific IDs.  In order to save some NFTs but withdraw and convert others, using withdraw by IDs would be needed to withdraw specific NFTs.

Ice Methods

Freeze: Convert Fungibles -> NFTs
Melt: Convert NFTs -> Fungibles

// Instructions
// Freeze: Convert Fungibles -> NFTs

CALL_METHOD
    Address("Put_your_address_here")
    "withdraw"
    Address("resource_rdx1t4h4396mukhpzdrr5sfvegjsxl8q7a34q2vkt4quxcxahna8fucuz4")
    Decimal("Put_number_of_tokens_here")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_rdx1t4h4396mukhpzdrr5sfvegjsxl8q7a34q2vkt4quxcxahna8fucuz4")
    Bucket("Fungible_Bucket")
;
CALL_METHOD
    Address("component_rdx1czscv9f2mv034hewjplej5ef4f2ecug2fxxelfpgxrsrhw4mglq2yp")
    "freeze"
    Bucket("Fungible_Bucket")
;
CALL_METHOD
    Address("Put_your_address_here")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;

// Melt: Convert NFTs -> Fungibles
// withdraw by amount

CALL_METHOD
    Address("Put_your_address_here")
    "withdraw"
    Address("resource_tdx_2_1ngqx5y5j0wk5ytzmdupj8raxej8vrsc4qrcq0kl2p0qgsu46tyxneg")
    Decimal("Put_number_of_tokens_here")
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1ngqx5y5j0wk5ytzmdupj8raxej8vrsc4qrcq0kl2p0qgsu46tyxneg")
    Bucket("NFT_Bucket")
;
CALL_METHOD
    Address("component_tdx_2_1cpkcja9cfrgd6dtt0u5jc4893zdd94w060gj6f0jd4l0nl294es6kf")
    "melt"
    Bucket("NFT_Bucket")
;
CALL_METHOD
    Address("Put_your_address_here")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;

// melt nfts -> fungibles
// withdraw by ids

CALL_METHOD
    Address("Put_your_address_here")
    "withdraw"
    Address("resource_tdx_2_1ngqx5y5j0wk5ytzmdupj8raxej8vrsc4qrcq0kl2p0qgsu46tyxneg")
    Decimal("Put_number_of_tokens_here")
;
TAKE_FROM_WORKTOP_BY_IDS 
    Set<NonFungibleId>(NonFungibleId("id")) 
    ResourceAddress("resource_tdx_2_1ngqx5y5j0wk5ytzmdupj8raxej8vrsc4qrcq0kl2p0qgsu46tyxneg") 
    Bucket("NFT_Bucket");
;
CALL_METHOD
    Address("component_tdx_2_1cpkcja9cfrgd6dtt0u5jc4893zdd94w060gj6f0jd4l0nl294es6kf")
    "melt"
    Bucket("NFT_Bucket")
;
CALL_METHOD
    Address("Put_your_address_here")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
