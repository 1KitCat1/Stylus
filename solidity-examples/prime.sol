// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Prime {
    function isPrime(uint256 number) public pure returns(bool) {
        bool _isPrime = true;
        for (uint256 i = 2; i*i <= number; i++) {
            if (number % i == 0) {
                _isPrime = false;
                // no *break* in order to get stable gas-consumption
            }
        }
        return _isPrime;
    }
}