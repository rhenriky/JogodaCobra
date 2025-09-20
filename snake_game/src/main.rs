// Snake Game - Implementação em Rust
// Funcionalidades: Velocidade progressiva, Obstáculos, Paredes sólidas, Restart
// Desenvolvido com crossterm para interface de terminal

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Result,
};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: u16,
    y: u16,
}

struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
}

impl Snake {
    fn new(start_pos: Position) -> Self {
        let mut body = VecDeque::new();
        body.push_back(start_pos);
        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn move_snake(&mut self, width: u16, height: u16) -> bool {
        let head = *self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => {
                if head.y <= 2 {
                    return false; // Bateu na parede superior
                }
                Position {
                    x: head.x,
                    y: head.y - 1,
                }
            },
            Direction::Down => {
                if head.y >= height - 1 {
                    return false; // Bateu na parede inferior
                }
                Position {
                    x: head.x,
                    y: head.y + 1,
                }
            },
            Direction::Left => {
                if head.x == 0 {
                    return false; // Bateu na parede esquerda
                }
                Position {
                    x: head.x - 1,
                    y: head.y,
                }
            },
            Direction::Right => {
                if head.x >= width - 1 {
                    return false; // Bateu na parede direita
                }
                Position {
                    x: head.x + 1,
                    y: head.y,
                }
            },
        };

        if self.body.contains(&new_head) {
            return false;
        }

        self.body.push_front(new_head);
        true
    }

    fn grow(&mut self) {
    }

    fn shrink(&mut self) {
        self.body.pop_back();
    }

    fn change_direction(&mut self, new_direction: Direction) {
        match (self.direction, new_direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {
            }
            _ => self.direction = new_direction,
        }
    }
}

struct Food {
    position: Position,
    points: u32,
}

impl Food {
    fn new(width: u16, height: u16, snake: &Snake, obstacles: &Vec<Obstacle>) -> Self {
        let mut rng = rand::thread_rng();
        let mut position;
        
        loop {
            position = Position {
                x: rng.gen_range(0..width),
                y: rng.gen_range(2..height),
            };
            
            if !snake.body.contains(&position) && 
               !obstacles.iter().any(|obs| obs.position == position) {
                break;
            }
        }

        Food {
            position,
            points: rng.gen_range(1..=5),
        }
    }
}

struct Obstacle {
    position: Position,
}

impl Obstacle {
    fn new(position: Position) -> Self {
        Obstacle { position }
    }
}

struct Game {
    snake: Snake,
    food: Food,
    obstacles: Vec<Obstacle>,
    score: u32,
    width: u16,
    height: u16,
    game_over: bool,
    speed_level: u32,
}

impl Game {
    fn new(width: u16, height: u16) -> Self {
        let start_pos = Position {
            x: width / 2,
            y: height / 2,
        };
        let snake = Snake::new(start_pos);
        
        // Gerar obstáculos aleatórios
        let mut obstacles = Vec::new();
        let mut rng = rand::thread_rng();
        let num_obstacles = rng.gen_range(5..=8);
        
        for _ in 0..num_obstacles {
            loop {
                let obstacle_pos = Position {
                    x: rng.gen_range(0..width),
                    y: rng.gen_range(2..height),
                };
                
                // Verificar se não está colidindo com a cobra inicial
                if !snake.body.contains(&obstacle_pos) && 
                   (obstacle_pos.x < start_pos.x - 2 || obstacle_pos.x > start_pos.x + 2 ||
                    obstacle_pos.y < start_pos.y - 2 || obstacle_pos.y > start_pos.y + 2) {
                    obstacles.push(Obstacle::new(obstacle_pos));
                    break;
                }
            }
        }
        
        let food = Food::new(width, height, &snake, &obstacles);

        Game {
            snake,
            food,
            obstacles,
            score: 0,
            width,
            height,
            game_over: false,
            speed_level: 1,
        }
    }

    fn update(&mut self) {
        if !self.snake.move_snake(self.width, self.height) {
            self.game_over = true;
            return;
        }

        let head = *self.snake.body.front().unwrap();
        
        // Verificar colisão com obstáculos
        if self.obstacles.iter().any(|obs| obs.position == head) {
            self.game_over = true;
            return;
        }
        
        if head == self.food.position {
            self.score += self.food.points;
            self.snake.grow();
            self.speed_level += 1;
            self.food = Food::new(self.width, self.height, &self.snake, &self.obstacles);
        } else {
            self.snake.shrink();
        }
    }

    fn get_update_interval(&self) -> Duration {
        let base_speed = 200;
        let speed_decrease = (self.speed_level - 1) * 15;
        let min_speed = 50;
        let interval = if base_speed > speed_decrease + min_speed {
            base_speed - speed_decrease
        } else {
            min_speed
        };
        Duration::from_millis(interval as u64)
    }

    fn restart(&mut self) {
        // Reinicializar cobra
        let start_pos = Position {
            x: self.width / 2,
            y: self.height / 2,
        };
        self.snake = Snake::new(start_pos);
        
        // Gerar novos obstáculos
        self.obstacles.clear();
        let mut rng = rand::thread_rng();
        let num_obstacles = rng.gen_range(5..=8);
        
        for _ in 0..num_obstacles {
            loop {
                let obstacle_pos = Position {
                    x: rng.gen_range(0..self.width),
                    y: rng.gen_range(2..self.height),
                };
                
                if !self.snake.body.contains(&obstacle_pos) && 
                   (obstacle_pos.x < start_pos.x - 2 || obstacle_pos.x > start_pos.x + 2 ||
                    obstacle_pos.y < start_pos.y - 2 || obstacle_pos.y > start_pos.y + 2) {
                    self.obstacles.push(Obstacle::new(obstacle_pos));
                    break;
                }
            }
        }
        
        // Resetar outros valores
        self.food = Food::new(self.width, self.height, &self.snake, &self.obstacles);
        self.score = 0;
        self.speed_level = 1;
        self.game_over = false;
    }

    fn draw(&self) -> Result<()> {
        queue!(stdout(), Clear(ClearType::All))?;

        queue!(stdout(), SetForegroundColor(Color::White))?;
        for x in 0..self.width {
            queue!(stdout(), MoveTo(x, 1), Print("-"))?;
        }

        queue!(
            stdout(),
            MoveTo(0, 0),
            SetForegroundColor(Color::Yellow),
            Print(format!("Jogo da Cobra | Score: {} | Maca: {} pts", self.score, self.food.points))
        )?;

        queue!(stdout(), SetForegroundColor(Color::Green))?;
        for (i, segment) in self.snake.body.iter().enumerate() {
            queue!(stdout(), MoveTo(segment.x, segment.y))?;
            if i == 0 {
                queue!(stdout(), Print("O"))?;
            } else {
                queue!(stdout(), Print("o"))?;
            }
        }

        queue!(
            stdout(),
            MoveTo(self.food.position.x, self.food.position.y),
            SetForegroundColor(Color::Red),
            Print("@")
        )?;

        // Desenhar obstáculos
        queue!(stdout(), SetForegroundColor(Color::DarkRed))?;
        for obstacle in &self.obstacles {
            queue!(
                stdout(),
                MoveTo(obstacle.position.x, obstacle.position.y),
                Print("■")
            )?;
        }

        queue!(
            stdout(),
            MoveTo(0, self.height),
            SetForegroundColor(Color::Cyan),
            Print("WASD para mover | Q para sair | Evite paredes e obstáculos!")
        )?;

        stdout().flush()?;
        Ok(())
    }

    fn handle_input(&mut self) -> Result<bool> {
        if poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(false),
                    KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                        self.snake.change_direction(Direction::Up);
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                        self.snake.change_direction(Direction::Down);
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => {
                        self.snake.change_direction(Direction::Left);
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => {
                        self.snake.change_direction(Direction::Right);
                    }
                    _ => {}
                }
            }
        }
        Ok(true)
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let (width, height) = size()?;
    let mut game = Game::new(width, height - 2);

    let mut last_update = Instant::now();

    'main_loop: loop {
        if !game.handle_input()? {
            break;
        }

        let update_interval = game.get_update_interval();
        if last_update.elapsed() >= update_interval {
            game.update();
            last_update = Instant::now();
        }

        game.draw()?;

        if game.game_over {
            queue!(
                stdout,
                MoveTo(game.width / 2 - 5, game.height / 2),
                SetForegroundColor(Color::Red),
                Print("GAME OVER!")
            )?;
            queue!(
                stdout,
                MoveTo(game.width / 2 - 8, game.height / 2 + 1),
                SetForegroundColor(Color::White),
                Print(format!("Score final: {}", game.score))
            )?;
            queue!(
                stdout,
                MoveTo(game.width / 2 - 10, game.height / 2 + 2),
                SetForegroundColor(Color::Cyan),
                Print("R para reiniciar | Q para sair")
            )?;
            stdout.flush()?;

            loop {
                if let Event::Key(KeyEvent { code, .. }) = read()? {
                    match code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                            break 'main_loop;
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            game.restart();
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    println!("Obrigado por jogar! Score final: {}", game.score);
    Ok(())
}
