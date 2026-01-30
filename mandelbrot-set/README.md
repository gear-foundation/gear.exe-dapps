## Parallel Mandelbrot Set Calculation Using Smart Contracts

The Mandelbrot set represents a classic example of computational complexity. Calculating this set often involves handling millions of data points and requires significant computational power. This example demonstrates how these computations can be performed using smart contracts on **gear.exe**.

### Mandelbrot Manager and Checker Smart Contracts

These smart contracts collaboratively calculate the Mandelbrot set by generating and evaluating points using distributed computation. The system comprises two contracts: **Manager** and **Checker**.

### Manager Contract
The Manager contract is responsible for orchestrating the computation. Its primary functions include:

1. **Point Generation**:
- Divides the complex plane into a grid of points based on user-defined parameters (e.g., resolution, bounds).
- Generates points and stores them along with their metadata.

2. **Task Distribution**:
- Distributes the generated points to multiple Checker contracts for computation.

3. **Result Aggregation**:
- Collects results from the Checker contracts to determine whether points belong to the Mandelbrot set.
- Updates the state for each processed point.

4. **Key Features**:
- **Parallelism**: Multiple Checker contracts work in parallel to compute the Mandelbrot set, demonstrating the power of distributed computation.
- **Continuous Execution with Reverse Gas Model**: Using the reverse gas model, the Manager contract can continuously compute the entire set of points after sending a single `generate_and_store_points` message with `check_points_after_generation = true`. The contract spends its own balance to fund this operation, ensuring uninterrupted execution.

### Checker Contract
The Checker contract evaluates whether points belong to the Mandelbrot set. Its primary functions include:

1. **Point Evaluation**:
- Accepts batches of points from the Manager contract.
- Iteratively computes the Mandelbrot escape condition for each point up to a maximum number of iterations.
2. **Result Reporting**:
- Returns the computation results (e.g., iteration counts) to the Manager contract.
3. **Computation Details**:
- Evaluates each point based on its coordinates in the complex plane and determines whether the point "escapes" or remains bounded.

### Workflow
1. The Manager generates a grid of complex points within user-defined bounds and parameters.
2. Points are distributed to Checker contracts for parallel evaluation.
3. Each Checker processes its batch of points and reports results back to the Manager.
4. The Manager collects and stores the results, marking points as either inside or outside the Mandelbrot set.


## Running on Vara-Eth

This example can be executed on **Vara-Eth** using a TypeScript runner that deploys multiple Checker programs and a single Manager program via Router + Mirror, tops up executable balance, and runs the full distributed computation flow.

### Prerequisites

- **Node.js** (LTS recommended, v20+)
- **pnpm** (v10+)
- A configured `.env` with RPC endpoints and deployed IDs

### 1) Environment configuration

Create your `.env` from the example:

```bash
cp .env.example .env
```
Open .env and set your Ethereum private key:

```
PRIVATE_KEY=0xYOUR_PRIVATE_KEY_HERE
```
Update these variables:
- `ETHEREUM_RPC` — Ethereum WebSocket RPC endpoint
- `VARA_ETH_RPC` — Vara-Eth validator WebSocket endpoint
- `ROUTER_ADDRESS` — Router contract address on Ethereum
- `PRIVATE_KEY` — your Ethereum private key (used to sign Router/Mirror txs)
Program deployment inputs:
- `CHECKER_CODE_ID` — deployed code ID for the Checker contract on Vara-Eth
- `MAN_CODE_ID` — deployed code ID for the Manager contract on Vara-Eth
- `PROGRAM_COUNT` — number of Checker programs to create (default: 16)

### 2) Install dependencies
```
pnpm install
```

### 3) Run modes

The runner supports three modes:
- **create-checkers** — creates `PROGRAM_COUNT` Checker programs and saves their IDs to checker-programs.json
- **run-manager** — reads `checker-programs.json`, deploys a `Manager` program, registers the checkers, and starts computation
- **full** — runs both steps sequentially (create checkers → run manager)

## Recommended workflow (reuse deployed checkers)

The recommended workflow is to deploy Checker programs once, save their program IDs, and then run Manager-only computations repeatedly.

Deploying checkers requires creating many programs and topping up executable balances, which is time-consuming and costly.

### 1) One-time setup: deploy checkers
```
pnpm run mandelbrot:deploy-checkers
```

This command will:
- deploy `PROGRAM_COUNT` Checker programs from `CHECKER_CODE_ID`;
- top up executable balance for each `Checker`;
- initialize each `Checker` program;
- persist all Checker programIds into `checker-programs.json`

Example output:
```
[
  '0x728a6d34f8293310c9145f5c23b6181a2b5588a6',
  '0xe743690f4de51cc1227ef8f7302a723a83376c3a',
  '0xfa9777d7edc611c1c867b7fb7e77ff7e066f896a',
  '0xe036e7bd5fca2e8ee2536824abff0d00a2c57b7f'
]
```
### 2) Regular runs: calculate using existing checkers
```
pnpm run mandelbrot:calculate
```

This command will:
- load checker program IDs from `checker-programs.json`;
- deploy a new Manager program from `MAN_CODE_ID`;
- top up Manager executable balance and initialize it
- generate and store the computation grid (e.g. 100×100 = 10,000 points);
- register the checkers in the manager;
- distribute the work and aggregate results.

Example output:
```
Amount of points: 10000
```
`Amount of points` corresponds to the size of the generated grid (e.g. 100×100).