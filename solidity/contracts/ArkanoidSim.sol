// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "abdk-libraries-solidity/ABDKMath64x64.sol";

contract ArkanoidSim {
    using ABDKMath64x64 for int128;

    uint constant PADDLE_WIDTH = 350;
    uint constant PADDLE_HEIGHT = 15;
    uint constant PADDLE_SPEED = 6;
    uint constant BALL_SIZE = 20;
    uint constant SCREEN_WIDTH = 800;
    uint constant SCREEN_HEIGHT = 800;
    uint constant BRICK_WIDTH = 40;
    uint constant BRICK_HEIGHT = 30;
    uint constant BALL_SPEED = 6;
    uint constant MAX_STEPS = 10000;
    uint constant VERTICAL_OFFSET = 50;
    uint constant BRICK_MARGIN = 2;
    uint constant TOTAL_BRICKS_WIDTH = BRICK_WIDTH * 11 + BRICK_MARGIN * 10;
    int constant HORIZONTAL_OFFSET = int((SCREEN_WIDTH - TOTAL_BRICKS_WIDTH) / 2); 
    int constant BALL_RADIUS = int(BALL_SIZE / 2);

    struct GameState {
        int ballX;
        int ballY;
        int ballSpeedX;
        int ballSpeedY;
        int paddleX;
        int paddleY;
        uint score;
        uint hits;
        uint stepCount;
        bool gameOver;
        string gameStatus;
        bool[11][16] bricks;  // Bricks state (arr size 15x11)
    }

    GameState public state;

    event GameUpdated(int ballX, int ballY, int paddleX);
    event GameResult(uint score, uint hits, string status);
    event BallCollision(string object);
    event BallPosition(int ballX, int ballY);
    event BrickState(uint row, uint col, bool state);

    function initializeGame(int _paddleX, int _ballSpeedX, int _ballSpeedY) public {
        state.paddleX = _paddleX;
        state.paddleY = int(SCREEN_HEIGHT - PADDLE_HEIGHT - 30); 
        state.ballX = _paddleX + int(PADDLE_WIDTH / 2) - int(BALL_SIZE / 2);
        state.ballY = int(SCREEN_HEIGHT - PADDLE_HEIGHT - BALL_SIZE) - 20;
        state.ballSpeedX = _ballSpeedX;
        state.ballSpeedY = _ballSpeedY;
        state.score = 0;
        state.hits = 0;
        state.stepCount = 0;
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
            [false, false, false, true, true, false, true, true, false, false, false],
            [false, false, false, true, true, false, true, true, false, false, false]
        ];
    }

    function checkCircleRectangleCollision(int ballX, int ballY, int radius, int rectX1, int rectY1, int rectX2, int rectY2) internal pure returns (bool collisionX, bool collisionY) {

        int nearestX = rectX1 > ballX ? rectX1 : (ballX > rectX2 ? rectX2 : ballX);
        int nearestY = rectY1 > ballY ? rectY1 : (ballY > rectY2 ? rectY2 : ballY);

        int deltaX = ballX - nearestX;
        int deltaY = ballY - nearestY;

        int distanceSquared = deltaX * deltaX + deltaY * deltaY;

        int128 distanceSquaredFixed = ABDKMath64x64.fromInt(distanceSquared);
        int128 radiusFixed = ABDKMath64x64.fromInt(radius);
        int128 radiusSquaredFixed = radiusFixed.mul(radiusFixed);

        if (distanceSquaredFixed <= radiusSquaredFixed) {
            collisionX = (nearestX == rectX1 || nearestX == rectX2);  // Collision on the sides
            collisionY = (nearestY == rectY1 || nearestY == rectY2);  // Collision on the top or bottom
            return (collisionX, collisionY);
        } else {
            return (false, false);
        }
    }

    function startBounce() public {
        
        for (uint i = 0; i < MAX_STEPS; i++) {
            updateGame();

            if (state.gameOver) {
                emit GameResult(state.score, state.hits, state.gameStatus);
                break;
            }
        }
    }

    function updateGame() public {

        // Increment steps
        state.stepCount++;

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
        if (state.ballX - int(BALL_RADIUS) <= 0 || state.ballX + int(BALL_RADIUS) >= int(SCREEN_WIDTH)) {
            state.ballSpeedX *= -1;
        }
        if (state.ballY - int(BALL_RADIUS) <= 0) {
            state.ballSpeedY *= -1;
        }

        // Ball-paddle collision
        if (state.ballY + int(BALL_RADIUS) >= state.paddleY 
            && state.ballX >= state.paddleX 
            && state.ballX <= state.paddleX + int(PADDLE_WIDTH)) {
                
            state.ballSpeedY *= -1;
            state.hits += 1;
        }
        
        // Ball-brick collision
        bool brickHit = false;
        for (uint i = 0; i < 16; i++) {
            for (uint j = 0; j < 11; j++) {
                if (state.bricks[i][j]) {
                    int brickX1 = int(j * (BRICK_WIDTH + BRICK_MARGIN)) + HORIZONTAL_OFFSET;
                    int brickY1 = int(i * (BRICK_HEIGHT + BRICK_MARGIN)) + int(VERTICAL_OFFSET);
                    int brickX2 = brickX1 + int(BRICK_WIDTH);
                    int brickY2 = brickY1 + int(BRICK_HEIGHT);

                    (bool collisionX, bool collisionY) = checkCircleRectangleCollision(
                        state.ballX, state.ballY, int(BALL_RADIUS), 
                        brickX1, brickY1, brickX2, brickY2
                    );

                    if (collisionX || collisionY) {
                        state.bricks[i][j] = false;
                        state.score += 10;
                        state.hits += 1;

                        if (collisionX) {
                            state.ballSpeedX *= -1;
                        }
                        if (collisionY) {
                            state.ballSpeedY *= -1;
                        }

                        brickHit = true;
                    }
                }
            }
        }

        // Check if all bricks are destroyed
        if (brickHit) {
            bool allBricksDestroyed = true;
            for (uint i = 0; i < 16; i++) {
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

        // Check if the ball falls below the paddle
        if (state.ballY > int(SCREEN_HEIGHT)) {
            state.gameOver = true;
            state.gameStatus = "Game Over";
        }

        if (state.gameOver) {
            emit GameResult(state.score, state.hits, state.gameStatus);
        }
    }
}