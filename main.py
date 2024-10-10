import pygame
import random

pygame.init()

# Constants
SCREEN_WIDTH = 600
SCREEN_HEIGHT = 800
BRICK_WIDTH = 40
BRICK_HEIGHT = 30
PADDLE_WIDTH = 350
PADDLE_HEIGHT = 15
PADDLE_Y = SCREEN_HEIGHT - 30
BALL_SIZE = 20 
FPS = 60
BALL_SPEED = 5
PADDLE_SPEED = 5

# Colors
WHITE = (255, 255, 255)
YELLOW = (255, 255, 0)
RED = (255, 0, 0)
GRAY = (200, 200, 200)
BLACK = (0, 0, 0)
GREEN = (0, 255, 0)

# Set up the screen
screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
pygame.display.set_caption("Arkanoid Simulation")
clock = pygame.time.Clock()

background_image = pygame.image.load("img/background.jpg")
background_image = pygame.transform.scale(background_image, (SCREEN_WIDTH, SCREEN_HEIGHT))

font = pygame.font.SysFont(None, 36)

# Paddle setup
paddle_x_start = random.randint(0, SCREEN_WIDTH - PADDLE_WIDTH)
paddle = pygame.Rect(paddle_x_start, PADDLE_Y, PADDLE_WIDTH, PADDLE_HEIGHT)

# Ball setup
ball = pygame.Rect(paddle.centerx - BALL_SIZE // 2, paddle.top - BALL_SIZE, BALL_SIZE, BALL_SIZE)
ball_speed_x = random.choice([-BALL_SPEED, BALL_SPEED])
ball_speed_y = -BALL_SPEED

print(f"Initial paddle position: {paddle_x_start}, Initial ball vector: ({ball_speed_x}, {ball_speed_y})")

# Space Invader brick layout (0 - empty, 1 - yellow, 2 - red, 3 - gray)
invader_pattern = [
    [0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0], 
    [0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0], 
    [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0], 
    [0, 0, 3, 3, 3, 3, 3, 3, 3, 0, 0], 
    [0, 0, 3, 2, 3, 3, 3, 2, 3, 0, 0], 
    [0, 3, 3, 2, 3, 3, 3, 2, 3, 3, 0], 
    [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0], 
    [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 
    [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 
    [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 
    [3, 0, 3, 3, 3, 3, 3, 3, 3, 0, 3], 
    [3, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3],
    [3, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3],
    [0, 0, 0, 3, 3, 0, 3, 3, 0, 0, 0],
    [0, 0, 0, 3, 3, 0, 3, 3, 0, 0, 0],
]

colors = {
    1: YELLOW,
    2: RED,
    3: GRAY
}

INVADER_COLUMNS = len(invader_pattern[0]) 
total_invader_width = INVADER_COLUMNS * (BRICK_WIDTH + 2)
horizontal_offset = (SCREEN_WIDTH - total_invader_width) // 2

# Generate bricks based on the Space Invader layout
def generate_bricks():
    bricks = []
    for row_index, row in enumerate(invader_pattern):
        for col_index, col in enumerate(row):
            if col != 0:
                brick = pygame.Rect(
                    horizontal_offset + col_index * (BRICK_WIDTH + 2),
                    row_index * (BRICK_HEIGHT + 2) + 50,
                    BRICK_WIDTH, BRICK_HEIGHT
                )
                bricks.append((brick, colors[col]))
    return bricks

# Initialize bricks
bricks = generate_bricks()

# Scoring and hits
score = 0
hits = 0
ricochet_multiplier = 1  # Reset multiplier when ball hits platform

# Button for restarting the game
def draw_button(text, x, y, width, height, color):
    pygame.draw.rect(screen, color, (x, y, width, height))
    text_surface = font.render(text, True, WHITE)
    screen.blit(text_surface, (x + 10, y + 10))
    return pygame.Rect(x, y, width, height)  # Return rect for button hit detection

# Display final game over screen
def game_over_screen():
    screen.fill(BLACK)
    game_over_text = font.render(f"Game Over! Final Score: {score}, Hits: {hits}", True, WHITE)
    screen.blit(game_over_text, (SCREEN_WIDTH // 2 - 200, SCREEN_HEIGHT // 2 - 50))
    button_rect = draw_button("Repeat Game", SCREEN_WIDTH // 2 - 100, SCREEN_HEIGHT // 2 + 50, 200, 50, GREEN)
    return button_rect

def restart_game():
    global ball, ball_speed_x, ball_speed_y, paddle, bricks, score, hits, game_over
    paddle_x_start = random.randint(0, SCREEN_WIDTH - PADDLE_WIDTH)
    paddle = pygame.Rect(paddle_x_start, PADDLE_Y, PADDLE_WIDTH, PADDLE_HEIGHT)
    ball = pygame.Rect(paddle.centerx - BALL_SIZE // 2, paddle.top - BALL_SIZE, BALL_SIZE, BALL_SIZE)
    ball_speed_x = random.choice([-BALL_SPEED, BALL_SPEED])
    ball_speed_y = -BALL_SPEED
    bricks = generate_bricks()
    score = 0
    hits = 0
    game_over = False
    print(f"Restarted game with paddle at {paddle_x_start}")

# Automatic paddle movement
paddle_direction = 1  # 1 means right

# Main game loop
running = True
game_over = False
button_rect = None

while running:
    screen.blit(background_image, (0, 0))
    
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        if event.type == pygame.MOUSEBUTTONDOWN and game_over:  # Check for mouse click
            print(f"Mouse clicked at: {event.pos}")  # Debugging print for mouse click
            if button_rect is not None and button_rect.collidepoint(event.pos):  # Click on button
                print("Button clicked!")  # Debugging print when button is clicked
                restart_game()  # Restart the game when button is clicked

    if not game_over:
        # Draw paddle
        pygame.draw.rect(screen, WHITE, paddle)
        
        # Draw ball
        pygame.draw.ellipse(screen, WHITE, ball)
        
        # Move the ball
        ball.x += ball_speed_x
        ball.y += ball_speed_y

        # Ball collision with walls
        if ball.left <= 0 or ball.right >= SCREEN_WIDTH:
            ball_speed_x *= -1  # Reflect horizontally
        if ball.top <= 0:
            ball_speed_y *= -1  # Reflect vertically

        # Ball collision with paddle
        if ball.colliderect(paddle):
            ball_speed_y = -BALL_SPEED  # Reflect vertically upwards
            hits += 1
            ricochet_multiplier = 1  # Reset multiplier for paddle hits

        # Ball collision with bricks
        for brick in bricks[:]:
            if ball.colliderect(brick[0]):
                ball_speed_y *= -1  # Reflect vertically
                if ricochet_multiplier > 1:  # Check if it's a ricochet
                    score += 10 * ricochet_multiplier  # Apply multiplier to score
                else:
                    score += 10  # Normal hit, no multiplier
                hits += 1
                bricks.remove(brick)
                ricochet_multiplier += 1  # Increase multiplier after brick hit

        # Ball falls below screen (Game Over)
        if ball.bottom >= SCREEN_HEIGHT:
            game_over = True

        # Draw the bricks (Space Invader)
        for brick, color in bricks:
            pygame.draw.rect(screen, color, brick)

        # Move paddle automatically
        paddle.x += PADDLE_SPEED * paddle_direction
        if paddle.left <= 0 or paddle.right >= SCREEN_WIDTH:
            paddle_direction *= -1  # Change direction when paddle hits screen edges

        # Display score and hits
        score_text = font.render(f"Score: {score}", True, WHITE)
        hits_text = font.render(f"Hits: {hits}", True, WHITE)
        screen.blit(score_text, (20, 20))
        screen.blit(hits_text, (20, 50))

        if not bricks:
            game_over = True
            print("You Win!")

    else:
        button_rect = game_over_screen()

    pygame.display.flip()
    clock.tick(FPS)

pygame.quit()