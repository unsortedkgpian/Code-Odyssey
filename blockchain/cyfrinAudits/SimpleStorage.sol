// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;  // stating our version of solidity 

contract SimpleStorage {
    // Basic Types: boolean, uint, int, address, bytes 
    bool hasFavoriteNumber = true;
    uint256 favoriteNumber = 88;
    string favoriteNumberInText = "Eighty-eight"; 
    address myAddress = 0xB1fceD8A811c5B955CEdFaE2E1D132674c910547; 
    bytes32 favoriteBytes32 =  "cat";

    bool guess = false;
    bool ok = false;
}
