// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "abdk-libraries-solidity/ABDKMath64x64.sol";

contract ArkanoidSim {
    using ABDKMath64x64 for int128;

    uint constant PADDLE_WIDTH = 350;
    uint constant PADDLE_HEIGHT = 15;
    uint constant PADDLE_SPEED = 6;
    uint constant BALL_SIZE = 20;
    uint constant SCREEN_WIDTH = 600;
    uint constant SCREEN_HEIGHT = 800;
    uint constant BRICK_WIDTH = 40;
    uint constant BRICK_HEIGHT = 30;
    uint constant BALL_SPEED = 5;
    uint constant MAX_STEPS = 10000;
    int constant BALL_RADIUS = int(BALL_SIZE / 2);

    struct GameState {
        int ballX;
        int ballY;
        int ballSpeedX;
        int ballSpeedY;
        int paddleX;
        uint score;
        uint hits;
        bool gameOver;
        string gameStatus;
        bool[11][15] bricks;  // Bricks state (arr size 15x11)
    }

    GameState public state;

    event GameUpdated(int ballX, int ballY, int paddleX);
    event GameResult(uint score, uint hits, string status);

    function initializeGame(int _paddleX, int _ballSpeedX, int _ballSpeedY) public {
        state.paddleX = _paddleX;
        state.ballX = _paddleX + int(PADDLE_WIDTH / 2) - int(BALL_SIZE / 2);
        state.ballY = int(SCREEN_HEIGHT - PADDLE_HEIGHT - BALL_SIZE);
        state.ballSpeedX = _ballSpeedX;
        state.ballSpeedY = _ballSpeedY;
        state.score = 0;
        state.hits = 0;
        state.gameOver = false;

        state.bricks = [
            [false, false, true, false, false, false, false, false, true, false, false],
            [false, false, true, false, false, false, false, false, true, false, false],
            [false, false, false, true, false, false, false, true, false, false, false],
            [false, false, false, true, false, false, false, true, false, false, false],
            [false, false, true, true, true, true, true, true, true, false, false],
            [false, false, true, false, true, true, true, false, true, false, false],
            [false, true, true, false, true, true, true, false, true, true, false],
            [false, true, true, true, true, true, true, true, true, true, false],
            [true, true, true, true, true, true, true, true, true, true, true],
            [true, true, true, true, true, true, true, true, true, true, true],
            [true, true, true, true, true, true, true, true, true, true, true],
            [true, false, true, true, true, true, true, true, true, false, true],
            [true, false, true, false, false, false, false, false, true, false, true],
            [true, false, true, false, false, false, false, false, true, false, true],
            [false, false, false, true, true, false, true, true, false, false, false]
        ];
    }

    function distanceToBrick(int ballX, int ballY, int brickX, int brickY, int brickWidth, int brickHeight) internal pure returns (int128) {
        int nearestX = ballX < brickX ? brickX : (ballX > brickX + brickWidth ? brickX + brickWidth : ballX);
        int nearestY = ballY < brickY ? brickY : (ballY > brickY + brickHeight ? brickY + brickHeight : ballY);

        int deltaX = ballX - nearestX;
        int deltaY = ballY - nearestY;

        // Calculate sqrt(deltaX^2 + deltaY^2) using ABDKMath64x64
        return ABDKMath64x64.sqrt(ABDKMath64x64.fromInt(deltaX * deltaX + deltaY * deltaY));
    }

    function startBounce(int _paddleX, int _ballSpeedX, int _ballSpeedY) public {
        state.paddleX = _paddleX;
        state.ballX = _paddleX + int(PADDLE_WIDTH / 2) - int(BALL_SIZE / 2);
        state.ballY = int(SCREEN_HEIGHT - PADDLE_HEIGHT - BALL_SIZE);
        state.ballSpeedX = _ballSpeedX;
        state.ballSpeedY = _ballSpeedY;

        for (uint i = 0; i < MAX_STEPS; i++) {
            updateGame();

            if (state.gameOver) {
                emit GameResult(state.score, state.hits, state.gameStatus);
                break;
            }
        }
    }

    function updateGame() public {

        // Paddle is automatic, bounded by screen limits
        state.paddleX += int(PADDLE_SPEED);
        if (state.paddleX < 0) {
            state.paddleX = 0;
        }
        if (state.paddleX + int(PADDLE_WIDTH) > int(SCREEN_WIDTH)) {
            state.paddleX = int(SCREEN_WIDTH) - int(PADDLE_WIDTH);
        }

        // Ball movement
        state.ballX += state.ballSpeedX;
        state.ballY += state.ballSpeedY;


        // Ball collision with the walls
        if (state.ballX <= 0 || state.ballX + int(BALL_SIZE) >= int(SCREEN_WIDTH)) {
            state.ballSpeedX *= -1; // Reflect horizontally
        }
        if (state.ballY <= 0) {
            state.ballSpeedY *= -1; // Reflect vertically
        }

        // Ball-paddle collision
        if (distanceToRect(state.ballX, state.ballY, state.paddleX, int(SCREEN_HEIGHT - PADDLE_HEIGHT), PADDLE_WIDTH, PADDLE_HEIGHT) <= int128(BALL_RADIUS)) {
            state.ballSpeedY *= -1;
            state.hits += 1;
        }

        //  Ball-brick collision
        for (uint i = 0; i < 15; i++) {
            for (uint j = 0; j < 11; j++) {
                if (state.bricks[i][j]) {
                    int brickX = int(j * BRICK_WIDTH);
                    int brickY = int(i * BRICK_HEIGHT);

                    if (distanceToBrick(state.ballX, state.ballY, brickX, brickY, BRICK_WIDTH, BRICK_HEIGHT) <= int128(BALL_RADIUS)) {
                        state.bricks[i][j] = false;
                        state.ballSpeedY *= -1;
                        state.score += 10;
                        state.hits += 1;
                        break;
                    }
                }
            }
        }

        // Check if the ball falls below the paddle
        if (state.ballY > int(SCREEN_HEIGHT)) {
            state.gameOver = true;
            state.gameStatus = "Game Over";
        }

        // 7. Check if all bricks are destroyed
        bool allBricksDestroyed = true;
        for (uint i = 0; i < 15; i++) {
            for (uint j = 0; j < 11; j++) {
                if (state.bricks[i][j]) {
                    allBricksDestroyed = false;
                    break;
                }
            }
        }
        
        if (allBricksDestroyed) {
            state.gameOver = true;
            state.gameStatus = "You Win!";
        }
    }

    function isGameOver() public view returns (bool) {
        return state.gameOver;
    }
}
