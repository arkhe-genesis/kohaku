// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

contract RBB_Cathedral_Token is ERC20, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    uint256 public constant BASE_FEE = 0.001 ether;

    constructor() ERC20("RBB Cathedral Token", "RBBCT") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        _mint(to, amount);
    }

    function calculateFee(uint256 theosisLevel) public pure returns (uint256) {
        // Theosis-based fee discount
        // Higher theosis, lower fee. e.g. level 100 -> 0 fee?
        // We'll implement a simple discount mechanism.
        // Assuming max theosisLevel is 100 for a 100% discount.
        if (theosisLevel >= 100) {
            return 0;
        }

        // discount is theosisLevel % of BASE_FEE
        uint256 discount = (BASE_FEE * theosisLevel) / 100;
        return BASE_FEE - discount;
    }
}
