<script type="text/javascript" src="http://cdn.mathjax.org/mathjax/latest/MathJax.js?config=default"></script>

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

