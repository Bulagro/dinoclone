extern crate chrono;
extern crate ncurses;
extern crate noise;
extern crate rand;

use chrono::*;
use ncurses::*;

use dinoclone::*;

fn main() {
    initscr();
    raw();
    cbreak();
    nodelay(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    dinoclone::initialize_colors();

    loop {
        let mut terrain: t::Terrain = t::Terrain::new();
        let mut player: p::Player = p::Player::new();

        let mut last_time = offset::Local::now();

        let mut speed: i64 = INITIAL_SPEED;
        let mut speed_mult: f64 = 1.0;
        let mut max_air_time: i32 = INITIAL_AIR_TIME;

        let mut pause: bool = false;
        let mut playing: bool = true;

        let mut score: u32 = 0;

        draw(&terrain, &player, score);
        mvprintw(LINES() / 2, COLS() / 2 - 12, "PRESS ANY KEY TO PLAY");

        while player.state == p::PlayerState::Idle {
            let key = getch();

            if key == KEY_QUIT {
                nocbreak();
                endwin();
                return;
            } else if key != -1 {
                player.state = p::PlayerState::Running;
            }
        }

        while playing {
            let key = getch();

            if key == KEY_QUIT {
                playing = false;
            } else if key == KEY_JUMP && !pause {
                player.jump(&terrain);
            } else if key == KEY_PAUSE && player.state != p::PlayerState::Dead {
                pause = !pause;
            }

            let t = offset::Local::now();
            if t >= last_time + Duration::milliseconds(speed) {
                if !pause && player.state != p::PlayerState::Dead {
                    last_time = t;

                    terrain.scroll_terrain();
                    terrain.offset(&player);

                    player.update_pos(&terrain, max_air_time);
                    draw(&terrain, &player, score);
                    score += 1;

                    terrain.roffset();

                    if score % SPEED_CHANGE_INTERVAL == 0 && speed > MAX_SPEED {
                        speed_mult -= SPEED_MULT_CONST;
                        speed = (INITIAL_SPEED as f64 * speed_mult) as i64; // linear
                                                                            // speed = (speed as f64 * speed_mult) as i64; // mon-linear
                        max_air_time =
                            INITIAL_AIR_TIME + (max_air_time as f64 * (1.0 - speed_mult)) as i32;
                    }
                } else if pause {
                    mvprintw(0, (COLS() / 2) - 3, "PAUSE");
                } else {
                    mvprintw(0, (COLS() / 2) - 3, "DEAD");
                    break;
                }
            }
        }

        mvprintw(
            2 * LINES() / 3,
            COLS() / 2 - 23,
            "PRESS 'JUMP' TO START AGAIN, 'QUIT' TO QUIT",
        );

        loop {
            let key = getch();
            if key == KEY_QUIT {
                nocbreak();
                endwin();
                return;
            } else if key == KEY_JUMP {
                break; // reset
            }
        }
    }
}
