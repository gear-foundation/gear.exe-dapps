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

      // Initializing the game
      await arkanoidSim.initializeGame(300, 0, 0);
      console.log("Game initialized");
    } catch (error) {
      console.error("Contract deployment error:", error);
    }
  });

  it("should complete the game", async function () {
    this.timeout(60000); // Setting a 60-second timeout for the test

    const initialPaddleX = 218;
    const initialBallSpeedX = 6;
    const initialBallSpeedY = -6;

    // Fetching initial game state
    let state = await arkanoidSim.state();
    console.log("Initial game state:", state);
    expect(state.gameOver).to.equal(false);

    // Starting the game with the provided parameters
    console.log("Starting the game with parameters:", { initialPaddleX, initialBallSpeedX, initialBallSpeedY });
    const tx = await arkanoidSim.startBounce(initialPaddleX, initialBallSpeedX, initialBallSpeedY, {
      gasLimit: 1000000000
    });

    // Waiting for the transaction to complete and logging the gas used
    const receipt = await tx.wait();
    console.log("Transaction finished. Gas used:", receipt.gasUsed.toString());

    // Fetching the game state after the transaction
    state = await arkanoidSim.state();
    console.log("Game state after the transaction:", state);

    // Verifying that the game is over
    expect(state.gameOver).to.equal(true);

    // Logging final game statistics
    console.log("Final Score:", state.score.toString());
    console.log("Total Hits:", state.hits.toString());
    console.log("Steps:", state.stepCount.toString());
    console.log("Game Status:", state.gameStatus);

    // Checking that the game status is either "Game Over" or "You Win!"
    expect(state.gameStatus).to.be.oneOf(["Game Over", "You Win!"]);
  });
});