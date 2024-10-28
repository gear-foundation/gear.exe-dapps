// e2e module
use e2e::{
    alloy::{
        primitives::{utils::parse_ether, U256},
        providers::Provider,
    },
    eyre::Result,
    send, tokio, Account, ReceiptExt,
};

// Counter ABI
use abi::Counter;
mod abi;

// Test 1: Accounts are correctly funded
#[e2e::test]
async fn accounts_are_funded(alice: Account) -> Result<()> {
    let balance = alice.wallet.get_balance(alice.address()).await?;
    let expected = parse_ether("100")?;
    assert_eq!(expected, balance);
    Ok(())
}

// Test 2: Contract deploys correctly
#[e2e::test]
async fn deploys(alice: Account) -> Result<()> {
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = Counter::new(contract_addr, &alice.wallet);
    let Counter::numberReturn { number } = contract.number().call().await?;

    assert_eq!(U256::ZERO, number);
    Ok(())
}

// Test 3: Performs all operations
#[e2e::test]
async fn performs_all_operations(alice: Account) -> Result<()> {
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = Counter::new(contract_addr, &alice.wallet);

    // Number should be 0 on initialization
    let Counter::numberReturn { number } = contract.number().call().await?;
    assert_eq!(U256::ZERO, number);

    // Incrementing
    let _ = send!(contract.increment());
    let Counter::numberReturn { number } = contract.number().call().await?;
    assert_eq!(U256::from(1), number);

    // Adding
    let number_to_add = U256::from(5);
    let _ = send!(contract.addNumber(number_to_add));
    let Counter::numberReturn { number } = contract.number().call().await?;
    assert_eq!(U256::from(6), number);

    // Multiplying
    let number_to_multiply = U256::from(2);
    let _ = send!(contract.mulNumber(number_to_multiply));
    let Counter::numberReturn { number } = contract.number().call().await?;
    assert_eq!(U256::from(12), number);

    // Setting new number
    let number_to_set = U256::from(2);
    let _ = send!(contract.setNumber(number_to_set));
    let Counter::numberReturn { number } = contract.number().call().await?;
    assert_eq!(number_to_set, number);

    Ok(())
}
