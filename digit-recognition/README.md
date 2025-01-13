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