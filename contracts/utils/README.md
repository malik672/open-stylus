# Utilities

This was adapted from [openzepplin](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/utils#utilities)

Note: Not everything that's present in openzeppelin is present in open-stylus some contract were simply not needed e.g and most are not yet implemented

This check box will be marked x when it's complete
```
- [] I have done full implementation of utils.
```

Miscellaneous contracts and libraries containing utility functions you can use to improve security, work with new data types, or safely use low-level primitives.

-   {Math}, {SignedMath}: Implementation of various arithmetic functions.
    
-   {SafeCast}: Checked downcasting functions to avoid silent truncation.
    
-   {ECDSA}, {MessageHashUtils}: Libraries for interacting with ECDSA signatures.
    
-   {SignatureChecker}: A library helper to support regular ECDSA from EOAs as well as ERC-1271 signatures for smart contracts.
    
-   {Hashes}: Commonly used hash functions.
    
-   {MerkleProof}: Functions for verifying  [Merkle Tree](https://en.wikipedia.org/wiki/Merkle_tree)  proofs.
    
-   {EIP712}: Contract with functions to allow processing signed typed structure data according to  [EIP-712](https://eips.ethereum.org/EIPS/eip-712).
    
-   {ReentrancyGuard}: A modifier that can prevent reentrancy during certain functions.
    
-   {Pausable}: A common emergency response mechanism that can pause functionality while a remediation is pending.
    
-   {Nonces}: Utility for tracking and verifying address nonces that only increment.
    
-   {ERC165, ERC165Checker}: Utilities for inspecting interfaces supported by contracts.
    
-   {BitMaps}: A simple library to manage boolean value mapped to a numerical index in an efficient way.
    
-   {EnumerableMap}: A type like Solidity’s  [`mapping`](https://solidity.readthedocs.io/en/latest/types.html#mapping-types), but with key-value  _enumeration_: this will let you know how many entries a mapping has, and iterate over them (which is not possible with  `mapping`).
    
-   {EnumerableSet}: Like {EnumerableMap}, but for  [sets](https://en.wikipedia.org/wiki/Set_(abstract_data_type)). Can be used to store privileged accounts, issued IDs, etc.
    
-   {DoubleEndedQueue}: An implementation of a  [double ended queue](https://en.wikipedia.org/wiki/Double-ended_queue)  whose values can be removed added or remove from both sides. Useful for FIFO and LIFO structures.
    
-   {Checkpoints}: A data structure to store values mapped to an strictly increasing key. Can be used for storing and accessing values over time.
    
-   {MerkleTree}: A library with  [Merkle Tree](https://wikipedia.org/wiki/Merkle_Tree)  data structures and helper functions.
    
-   {Create2}: Wrapper around the  [`CREATE2`  EVM opcode](https://blog.openzeppelin.com/getting-the-most-out-of-create2/)  for safe use without having to deal with low-level assembly.
    
-   {Address}: Collection of functions for overloading Solidity’s  [`address`](https://docs.soliditylang.org/en/latest/types.html#address)  type.
    
-   {Arrays}: Collection of functions that operate on  [`arrays`](https://docs.soliditylang.org/en/latest/types.html#arrays).
    
-   {Base64}: On-chain base64 and base64URL encoding according to  [RFC-4648](https://datatracker.ietf.org/doc/html/rfc4648).
    
-   {Multicall}: Abstract contract with an utility to allow batching together multiple calls in a single transaction. Useful for allowing EOAs to perform multiple operations at once.
    
-   {Context}: An utility for abstracting the sender and calldata in the current execution context.
    
-   {Panic}: A library to revert with  [Solidity panic codes](https://docs.soliditylang.org/en/v0.8.20/control-structures.html#panic-via-assert-and-error-via-require).
    

Note

Because Solidity does not support generic types, {EnumerableMap} and {EnumerableSet} are specialized to a limited number of key-value types.

## Math

[](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/utils#math)

{{Math}}

{{SignedMath}}

{{SafeCast}}

## Cryptography

[](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/utils#cryptography)

{{ECDSA}}

{{EIP712}}

{{MessageHashUtils}}

{{SignatureChecker}}

{{Hashes}}

{{MerkleProof}}

## Security

[](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/utils#security)

{{Pausable}}

{{Nonces}}

## Introspection

[](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/utils#introspection)

This set of interfaces and contracts deal with  [type introspection](https://en.wikipedia.org/wiki/Type_introspection)  of contracts, that is, examining which functions can be called on them. This is usually referred to as a contract’s  _interface_.

Ethereum contracts have no native concept of an interface, so applications must usually simply trust they are not making an incorrect call. For trusted setups this is a non-issue, but often unknown and untrusted third-party addresses need to be interacted with. There may even not be any direct calls to them! (e.g. ERC-20 tokens may be sent to a contract that lacks a way to transfer them out of it, locking them forever). In these cases, a contract  _declaring_  its interface can be very helpful in preventing errors.

{{IERC165}}

{{ERC165}}

{{ERC165Checker}}

## Data Structures

[](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/utils#data-structures)

{{BitMaps}}

{{EnumerableMap}}

{{EnumerableSet}}

{{DoubleEndedQueue}}

{{Checkpoints}}

{{MerkleTree}}

## Libraries

[](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/utils#libraries)

{{Create2}}

{{Address}}

{{Arrays}}

{{Base64}}

{{Multicall}}

{{Context}}
