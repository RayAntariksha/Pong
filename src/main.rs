use core::panic;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let _ = run();
    disable_raw_mode()?;
    Ok(())
}
struct GroundDimension {
    width: i32,
    height: i32,
}
fn draw_ground(lines_vec: Vec<String>, score: i32) {
    print!("┌──────────────────────────────────────────────────┐\n\r");
    for line in lines_vec {
        print!("│{}│\n\r", line);
    }
    print!("└──────────────────────────────────────────────────┘\n\r");
    print!("    Score: {score}\n\r");
    print!("    'q' <Quit>     ↑ <Up>      ↓ <Down>\n\r");
    print!("    Made By : RayAntariksha     Year: 2026         \n\r");
}

#[derive(Clone, Debug)]
struct PixelAddr {
    x: i32,
    y: i32,
}

struct BallVelocity {
    speed: i32,
    direction: i32,
}

fn move_ball_position(
    ball_position: &mut PixelAddr,
    ballvelocity: &BallVelocity,
    lines_vec: &mut Vec<String>,
) {
    change_char(ball_position.clone(), ' ', lines_vec);
    match ballvelocity.direction {
        5 => ball_position.y += ballvelocity.speed,
        2 => {
            ball_position.y -= ballvelocity.speed;
            ball_position.x += ballvelocity.speed;
        }
        3 => ball_position.x += ballvelocity.speed,
        4 => {
            ball_position.y += ballvelocity.speed;
            ball_position.x += ballvelocity.speed;
        }
        7 => ball_position.x -= ballvelocity.speed,
        8 => {
            ball_position.y -= ballvelocity.speed;
            ball_position.x -= ballvelocity.speed;
        }
        1 => ball_position.y -= ballvelocity.speed,
        6 => {
            ball_position.y += ballvelocity.speed;
            ball_position.x -= ballvelocity.speed;
        }
        _ => panic!("Velocity direction invalid"),
    }
    change_char(ball_position.clone(), '◯', lines_vec);
}

fn change_char(addr: PixelAddr, c: char, lines_vec: &mut Vec<String>) {
    //Check if address is out of index
    if addr.y > lines_vec.len() as i32 {
        panic!("pixel address out of index");
    }

    let line = lines_vec[addr.y as usize].clone();
    let mut chars: Vec<char> = line.chars().collect();
    chars[addr.x as usize] = c;
    lines_vec[addr.y as usize] = chars.iter().collect();
}
fn counted_chars(c: char, num: i32) -> String {
    let mut result = String::new();
    for _ in 0..num {
        result.push(c);
    }
    return result;
}
fn render_platform(platform_position: i32, lines_vec: &mut Vec<String>) {
    if platform_position < 0 || platform_position > lines_vec.len() as i32 - 3 {
        panic!("platform postion out of bound");
    }
    for i in 0..(lines_vec.len() as i32) {
        if i >= platform_position && i < platform_position + 3 {
            change_char(PixelAddr { x: 0, y: i }, '█', lines_vec);
        } else {
            change_char(PixelAddr { x: 0, y: i }, ' ', lines_vec);
        }
    }
}
fn render_bot_platform(
    bot_position: &mut i32,
    ball_position: PixelAddr,
    lines_vec: &mut Vec<String>,
) {
    *bot_position = ball_position.y - 1;
    if ball_position.y == 0 {
        *bot_position = 0;
    } else if *bot_position + 4 > lines_vec.len() as i32 {
        *bot_position = lines_vec.len() as i32 - 3;
    }
    for i in 0..(lines_vec.len() as i32) {
        if i >= *bot_position && i < *bot_position + 3 {
            change_char(PixelAddr { x: 49, y: i }, '█', lines_vec);
        } else {
            change_char(PixelAddr { x: 49, y: i }, ' ', lines_vec);
        }
    }
}

fn run() -> io::Result<()> {
    //Initialize the ground
    let dimensions = GroundDimension {
        width: 50,
        height: 10,
    };
    let mut score: i32 = 0;
    let mut lines_vec: Vec<String> = Vec::new();
    for _ in 0..dimensions.height {
        lines_vec.push(counted_chars(' ', dimensions.width));
    }

    let mut ball_position = PixelAddr { x: 5, y: 5 };
    let mut ballvelocity = BallVelocity {
        speed: 1,
        direction: 4,
    };
    let mut platform_position = 2;
    let mut bot_postion = 0;

    //Game mainloop
    loop {
        //Detect keypress
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => {
                            if platform_position > 0 {
                                platform_position -= 1
                            }
                        }
                        KeyCode::Down => {
                            if platform_position + 3 < dimensions.height {
                                platform_position += 1
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        //Bounce
        if ball_position.x == 0 {
            print!("You lost\n\r");
            break;
        }
        if ball_position.x == 49{
            print!("You Won!\n\r");
            break;
        }
        // Horizontal bounce
        if ball_position.y == 0 && ballvelocity.direction >= 5 {
            ballvelocity.direction -= 2;
        } else if ball_position.y == 0 && ballvelocity.direction <= 5 {
            ballvelocity.direction += 2;
        }
        if ball_position.y == 9 && ballvelocity.direction > 5 {
            ballvelocity.direction += 2;
        } else if ball_position.y == 9 && ballvelocity.direction <= 5 {
            ballvelocity.direction -= 2;
        }
        //Robot bounce
        if ball_position.x == 47
            && ballvelocity.direction < 3
            && ball_position.y >= bot_postion
            && ball_position.y <= bot_postion + 3
        {
            ballvelocity.direction += 6;
        } else if ball_position.x == 47
            && ballvelocity.direction > 3
            && ball_position.y >= bot_postion - 1
            && ball_position.y <= bot_postion + 3
        {
            ballvelocity.direction += 2;
        }
        //Player bounce
        if ball_position.x == 1
            && ballvelocity.direction < 7
            && ball_position.y >= platform_position
            && ball_position.y <= platform_position + 3
        {
            ballvelocity.direction -= 2;
            score += 1;
        } else if ball_position.x == 1
            && ballvelocity.direction > 7
            && ball_position.y >= platform_position
            && ball_position.y <= platform_position + 2
        {
            ballvelocity.direction -= 6;
            score += 1;
        }
        print!("\x1b[15F\x1b[J");
        render_platform(platform_position, &mut lines_vec);
        if ballvelocity.direction < 6 {
            render_bot_platform(&mut bot_postion, ball_position.clone(), &mut lines_vec);
        }
        draw_ground(lines_vec.clone(), score);
        move_ball_position(&mut ball_position, &ballvelocity, &mut lines_vec);

        std::thread::sleep(Duration::from_millis(70));
    }
    return Ok(());
}
