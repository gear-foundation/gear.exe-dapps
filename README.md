# 🎮 Arkanoid Simulation: Game Overview 🎮

This project is a **Python-based Arkanoid Simulation** aimed at visualizing and simulating the mechanics that will be later implemented in smart contracts on **GEAR.EXE** and **Ethereum**.

## 🛠️ Setup

1. **Install dependencies**:  
   Make sure you have `pygame` installed. You can install it by running:

   ```bash
   pip install pygame
   ```

2. **Run the game**:
   Once dependencies are installed, simply run the game with:

   ```bash
   python main.py
   ```

3. **Controls**:  
   The paddle moves **automatically**. You don't need to control it manually, as the game is designed to simulate deterministic movements.

## 🔄 Game Mechanism

- **Paddle Movement**:  
   The paddle moves automatically, and its initial position is randomized at the start of each game. The speed and direction of the paddle can vary based on preset parameters.

- **Ball Launch**:  
   The ball is launched from the paddle with a random initial vector (either 45 degrees left or right), and bounces off surfaces with the angle of incidence equal to the angle of reflection. This is **fully deterministic** and simulates predictable ball physics.

- **Bricks**:  
   The bricks are arranged in a pattern inspired by **Space Invaders**. Each brick has its own color and value.

## 🧮 Scoring System

- **Basic Hit**:  
   Hitting a brick grants the player 10 points.

- **Ricochet Multiplier**:  
   For each consecutive hit without touching the paddle, a multiplier is applied to the score (e.g., 20, 40 points for each subsequent hit).

- **Win Condition**:  
   If all bricks are destroyed within the set amount of bounces, the player wins.

## ⚙️ Simulation Purpose

This simulation is designed to demonstrate the **parallel execution** model of **GEAR.EXE**. By comparing it with native Solidity execution, we aim to showcase the benefits of parallelism and low gas usage in complex scenarios.

