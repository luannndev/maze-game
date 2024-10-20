use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::collections::HashSet;
use std::io;
use std::thread;
use std::time::Duration;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
    layout::Constraint,
};

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

#[derive(Clone, Copy)]
struct Player {
    x: usize,
    y: usize,
}

struct Maze {
    grid: Vec<Vec<char>>,
    player: Player,
    exit: (usize, usize),
}

impl Maze {
    fn new() -> Self {
        let mut grid = vec![vec![' '; WIDTH]; HEIGHT];
        let mut rng = rand::thread_rng();
        let mut walls = HashSet::new();

        for _ in 0..(WIDTH * HEIGHT / 3) {
            let wall_x = rng.gen_range(0..WIDTH);
            let wall_y = rng.gen_range(0..HEIGHT);
            walls.insert((wall_x, wall_y));
        }

        for (x, y) in walls {
            grid[y][x] = '#';
        }

        let player = Player { x: 1, y: 1 };
        grid[player.y][player.x] = 'P';
        let exit = (WIDTH - 2, HEIGHT - 2);
        grid[exit.1][exit.0] = 'E';

        Maze { grid, player, exit }
    }

    fn draw(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
        terminal.draw(|f| {
            let mut rows = vec![];

            for y in 0..HEIGHT {
                let row: Vec<Cell> = (0..WIDTH)
                    .map(|x| Cell::from(self.grid[y][x].to_string()))
                    .collect();
                rows.push(Row::new(row));
            }

            let widths: Vec<Constraint> = (0..WIDTH).map(|_| Constraint::Percentage(5)).collect();
            let table = Table::new(rows.clone(), widths.clone())
                .block(Block::default().title("Maze Game").borders(Borders::ALL))
                .column_spacing(1);

            f.render_widget(table, f.area());
        }).unwrap();
    }

    fn move_player(&mut self, dx: isize, dy: isize) -> bool {
        let new_x = (self.player.x as isize + dx) as usize;
        let new_y = (self.player.y as isize + dy) as usize;

        if new_x < WIDTH && new_y < HEIGHT && self.grid[new_y][new_x] != '#' {
            self.grid[self.player.y][self.player.x] = ' ';
            self.player.x = new_x;
            self.player.y = new_y;
            self.grid[self.player.y][self.player.x] = 'P';
            true
        } else {
            false
        }
    }

    fn check_exit(&self) -> bool {
        self.player.x == self.exit.0 && self.player.y == self.exit.1
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(ClearType::All))?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    let mut maze = Maze::new();

    loop {
        maze.draw(&mut terminal);
        if maze.check_exit() {
            terminal.draw(|f| {
                let msg = Block::default().title("You Win!").borders(Borders::ALL);
                f.render_widget(msg, f.area());
            })?;
            thread::sleep(Duration::from_secs(2));
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Char('w') => { maze.move_player(0, -1); },
                    KeyCode::Char('s') => { maze.move_player(0, 1); },
                    KeyCode::Char('a') => { maze.move_player(-1, 0); },
                    KeyCode::Char('d') => { maze.move_player(1, 0); },
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
