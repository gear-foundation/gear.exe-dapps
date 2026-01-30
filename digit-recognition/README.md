## Convolutional Neural Network in Smart Contracts: Digit Recognition

This project implements a **Convolutional Neural Network (CNN)** for digit recognition as a smart contract using the **Sails Framework**. The CNN processes image inputs (28x28 grayscale images) to classify handwritten digits (0-9) by performing convolutional, pooling, and fully connected layer computations on-chain. 

## How It Works
### Digit Recognition Process
1. **Input**:
- Accepts a flattened grayscale image with dimensions 28x28.
- Pixel values are integers in the range [0, 255].
2. **Normalization**:
- Converts pixel values into a 3D tensor.
- Normalizes values to a range of [0, 1].
- Converts intermediate results to a column format using `im2col` optimization.
3. **Computation**:
- Performs computations through convolutional layers, `ReLU` activation, max pooling, and fully connected layers, optimized with the `im2col` algorithm.
4. **Output**:
- Produces probabilities for each digit (0-9) using softmax.

## Smart Contract Implementation
The smart contract splits the digit recognition process into modular phases for efficient execution and resource management:

1. **Step 1: Input Preparation**:
- The input image is normalized and prepared as a 3D tensor.
- Converts intermediate results to a column format using `im2col` optimization.

2. **Step 2: First Convolutional Layer**:
- Applies the first convolution using pre-trained weights and biases.
- Uses `ReLU` activation and max pooling with a stride of 2.

3. **Step 3: Second Convolutional Layer**:
- Applies the second convolution, `ReLU` activation, and max pooling with a stride of 2.

4. **Step 4: Fully Connected Layers**:
- Flattens the output.
- Passes the flattened data through two fully connected layers.

5. **Step 5: Softmax Computation**:
- Computes probabilities for each digit.
- Outputs the result in a fixed-point format for precision.

### Demonstration
To see how the model works in action, follow these steps:
1. **Run the Test Command**:
Execute the following command in your terminal:
```bash
cargo t -r
```
2. **Draw a Digit**:
A window will appear where you can draw a digit using your mouse.
3. **Finish Drawing**:
Once you are satisfied with your drawing, click the Finish Drawing button in the application window.
4. **Digit Prediction**:
After clicking finish, the smart contract will process the drawn image and predict the digit.
You will see the prediction result in the terminal, including the predicted digit and its confidence level.
**Example Output**:
```
Digit 3 predicted with 99.20% probability
```

## Running on Vara-Eth

This repository also includes a TypeScript runner that interacts with the deployed contract via **Vara-Eth** (Router + Mirror) and provides a simple local UI to draw digits and send them on-chain for inference.

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

Verify the following variables (defaults come from .env.example):
- `ETHEREUM_RPC` — public Hoodi Ethereum RPC (WebSocket)
- `VARA_ETH_RPC` — public Vara-Eth validator WebSocket endpoint
- `ROUTER_ADDRESS` — Router contract address on Ethereum
- `CODE_ID` — deployed code ID on Vara-Eth (used for program creation)
- `PROGRAM_ID` — deployed program ID on Vara-Eth (used for prediction calls)
If you redeploy code or create a new program, update `CODE_ID` / `PROGRAM_ID` accordingly.
### 2) Install dependencies

```
pnpm install
```
### 3) Predict a digit (on-chain)

Run the prediction flow:

```
pnpm run digit-predict
```


A local UI window will open. Draw a digit from 0 to 9, then click `Send to contract`.

After processing, the script prints per-class probabilities and the final prediction in the terminal, for example:
```
0:   0.16% |                              | raw=1551
1:   0.04% |                              | raw=369
2:   0.36% |                              | raw=3598
3:   1.74% |#                             | raw=17432
4:   0.00% |                              | raw=37
5:   0.13% |                              | raw=1297
6:   0.00% |                              | raw=5
7:   0.02% |                              | raw=153
8:  97.55% |##############################| raw=975521 <==
9:   0.00% |                              | raw=36
Prediction: 8 (97.55%)
```
### 4) Deploy a new program (optional)

If you want to deploy a new program instance via Router, run:

```
pnpm run deploy:program
```

The script will:
- create a new program from CODE_ID,
- wait until it appears on Vara-Eth,
- top up executable balance,
- initialize the contract,
- upload model weights via injected transactions.

After deployment, copy the resulting `PROGRAM_ID` from logs into your `.env`.