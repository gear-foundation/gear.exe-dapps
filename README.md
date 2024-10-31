# Arkanoid Simulation Benchmark

This document describes the setup, execution, and benchmark results of the **Arkanoid Simulation** across different platforms: Ethereum, Arbitrum EVM, Arbitrum Nitro, and Gear.exe. The simulation tests high-load computations to measure cost, speed, and scalability on each network.

## Cost and Gas Comparison

| Platform           | Instances        | Total Gas Used       | Blocks Used | Cost (USD)  | Contract/Transaction Link |
|--------------------|------------------|----------------------|-------------|-------------|----------------------------|
| **Ethereum**       | 1                | 789,113,326         | 26          | $27,491     | [Contract](https://holesky.etherscan.io/address/0x352f3a3F3EbcfcB5bb5CF8b9D1F3BfAD0142718f#readContract) / [Transactions](https://holesky.etherscan.io/txs?a=0x352f3a3F3EbcfcB5bb5CF8b9D1F3BfAD0142718f) |
| **Arbitrum EVM**   | 1                | 789,113,326         | 26          | ~$200       | [Contract](https://sepolia.arbiscan.io/address/0xd133536f5ea11d8d1e8eb39b872ded09271eba9f) |
| **Arbitrum Nitro** | 1                | 85,164,788          | 4           | $4          | [Contract](https://sepolia.arbiscan.io/address/0xd133536f5ea11d8d1e8eb39b872ded09271eba9f) |
| **Gear.exe**       | 16 (parallel)    | 1.7T internal Gear gas | 1       | $0.17       | [Transaction](https://holesky.etherscan.io/tx/0x0b7eadb0bf73476fa90d80a2b761fc5fb1d3b19a031bed7e7d98a978824b3d50) |

## Benchmark Overview and Advantages

1. **Ethereum**:
   - **Purpose**: Provides a baseline for comparison. Running high-load simulations like Arkanoid on Ethereum highlights the expense and limitations due to high gas fees.

2. **Arbitrum EVM**:
   - **Setup**: The simulation ran on Arbitrum’s standard EVM with similar gas consumption to Ethereum. However, transaction costs are lower due to reduced gas prices.

3. **Arbitrum Nitro**:
   - **Setup**: Allows larger transaction capacity with up to 1,000 iterations per block. Provides a cost-efficient solution compared to Ethereum but still involves multiple transactions for high-load processes.

4. **Gear.exe**:
   - **Key Advantage**: Gear.exe allowed us to run **16 simultaneous Arkanoid simulations**, all of which fit within a single block. Each simulation involved high-load calculations, and Gear.exe’s architecture enabled these processes to complete without interruptions or additional messages.

## Conclusion

This comparison illustrates that Gear.exe offers unparalleled efficiency by fitting multiple high-load simulations into a single block at minimal cost.