#![allow(dead_code)]
use e2e::alloy::sol;

sol!(
    #[sol(rpc)]
    contract Counter {
        function number() external view returns (uint256 number);
        function setNumber(uint256 new_number) external;
        function mulNumber(uint256 new_number) external;
        function addNumber(uint256 new_number) external;
        function increment() external;
    }
);
