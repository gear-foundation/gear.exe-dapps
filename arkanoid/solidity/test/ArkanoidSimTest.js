const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("ArkanoidSim", function () {
  let ArkanoidSim, arkanoidSim;

  beforeEach(async function () {

    ArkanoidSim = await ethers.getContractFactory("ArkanoidSim");

    try {
      arkanoidSim = await ArkanoidSim.deploy();
    } catch (error) {
      console.error("Contract deployment error:", error);
    }
  });

  it("should deploy the contract correctly", async function () {
    expect(arkanoidSim.target).to.be.properAddress;
  });
});

describe("ArkanoidSim StartBounce", function () {
  let ArkanoidSim, arkanoidSim;

  beforeEach(async function () {
    ArkanoidSim = await ethers.getContractFactory("ArkanoidSim");

    try {
      arkanoidSim = await ArkanoidSim.deploy();
      console.log("Contract deployed at address:", arkanoidSim.target);

      // Initializing the game with paddle position and ball speed
      const initialPaddleX = 270;
      const initialBallSpeedX = 6;
      const initialBallSpeedY = -6;

      await arkanoidSim.initializeGame(initialPaddleX, initialBallSpeedX, initialBallSpeedY);
      console.log("Game initialized");
    } catch (error) {
      console.error("Contract deployment error:", error);
    }
  });

  it("should complete the game", async function () {
    this.timeout(60000);  // Set timeout for longer execution if needed

    // Fetching initial game state
    let state = await arkanoidSim.state();
    console.log("Initial game state:", state);
    expect(state.gameOver).to.equal(false);  // Ensure game is not over at start

    // Starting the game without additional parameters
    console.log("Starting the game...");
    const tx = await arkanoidSim.startBounce({
      gasLimit: 1000000000
    });

    // Wait for the transaction to complete and log the gas used
    const receipt = await tx.wait();
    console.log("Transaction finished. Gas used:", receipt.gasUsed.toString());

    // Fetching the game state after the transaction
    state = await arkanoidSim.state();
    console.log("Game state after the transaction:", state);

    // Verifying that the game is over
    expect(state.gameOver).to.equal(true);

    // Logging final game statistics
    console.log("Bricks destroyed:", state.destroyedBricks.toString());
    console.log("Total Hits:", state.hits.toString());
    console.log("Steps:", state.stepCount.toString());
    console.log("Game Status:", state.gameStatus);

    // Checking that the game status is either "Game Over" or "You Win!"
    expect(state.gameStatus).to.be.oneOf(["Game Over", "You Win!"]);
  });
});