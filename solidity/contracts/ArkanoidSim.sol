// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

contract ArkanoidSim {
    uint constant PADDLE_WIDTH = 350;
    uint constant PADDLE_HEIGHT = 15;
    uint constant BALL_SIZE = 20;
    uint constant SCREEN_WIDTH = 600;
    uint constant SCREEN_HEIGHT = 800;
    uint constant BRICK_WIDTH = 40;
    uint constant BRICK_HEIGHT = 30;
    uint constant BALL_SPEED = 5;
    struct GameState {
        int ballX;
        int ballY;
        int ballSpeedX;
        int ballSpeedY;
        int paddleX;
        uint score;
        uint hits;
        bool gameOver;
        bool[11][15] bricks;  // Bricks state (arr size 15x11)
    }

    GameState public state;

    event GameResult(uint score, uint hits, string status);
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

        // Bricks template
        bool[11] memory row1 = [false, false, true, false, false, false, false, false, true, false, false];
        bool[11] memory row2 = [false, false, true, false, false, false, false, false, true, false, false];
        bool[11] memory row3 = [false, false, false, true, false, false, false, true, false, false, false];
        bool[11] memory row4 = [false, false, false, true, false, false, false, true, false, false, false];
        bool[11] memory row5 = [false, false, true, true, true, true, true, true, true, false, false];
        bool[11] memory row6 = [false, false, true, false, true, true, true, false, true, false, false];
        bool[11] memory row7 = [false, true, true, false, true, true, true, false, true, true, false];
        bool[11] memory row8 = [false, true, true, true, true, true, true, true, true, true, false];
        bool[11] memory row9 = [true, true, true, true, true, true, true, true, true, true, true];
        bool[11] memory row10 = [true, true, true, true, true, true, true, true, true, true, true];
        bool[11] memory row11 = [true, true, true, true, true, true, true, true, true, true, true];
        bool[11] memory row12 = [true, false, true, true, true, true, true, true, true, false, true];
        bool[11] memory row13 = [true, false, true, false, false, false, false, false, true, false, true];
        bool[11] memory row14 = [true, false, true, false, false, false, false, false, true, false, true];
        bool[11] memory row15 = [false, false, false, true, true, false, true, true, false, false, false];

        state.bricks[0] = row1;
        state.bricks[1] = row2;
        state.bricks[2] = row3;
        state.bricks[3] = row4;
        state.bricks[4] = row5;
        state.bricks[5] = row6;
        state.bricks[6] = row7;
        state.bricks[7] = row8;
        state.bricks[8] = row9;
        state.bricks[9] = row10;
        state.bricks[10] = row11;
        state.bricks[11] = row12;
        state.bricks[12] = row13;
        state.bricks[13] = row14;
        state.bricks[14] = row15;
    }

    function startBounce(int _paddleX, int _ballSpeedX, int _ballSpeedY) public {
        state.paddleX = _paddleX;
        state.ballX = _paddleX + int(PADDLE_WIDTH / 2) - int(BALL_SIZE / 2);
        state.ballY = int(SCREEN_HEIGHT - PADDLE_HEIGHT - BALL_SIZE);
        state.ballSpeedX = _ballSpeedX;
        state.ballSpeedY = _ballSpeedY;

        emit GameUpdated(state.ballX, state.ballY, state.paddleX);

        function updateGame() public {
            
        }
    }


}

