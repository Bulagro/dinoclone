extern crate chrono;
extern crate ncurses;
extern crate noise;
extern crate rand;

use chrono::*;
use ncurses::*;

mod player;
mod terrain;
use player as p;
use terrain as t;

const KEY_QUIT: i32 = 'q' as i32;
const KEY_PAUSE: i32 = 'p' as i32;
const KEY_JUMP: i32 = 'w' as i32;

const IY: i32 = 6;
const IX: i32 = 1;
const PX: i32 = 23;

const MAX_SPEED: i64 = 40; // milliseconds update time
const SPEED_CHANGE_INTERVAL: u32 = 300;
const SPEED_MULT_CONST: f64 = 0.1;
const INITIAL_SPEED: i64 = 100;
const INITIAL_AIR_TIME: i32 = 7;

fn draw(terrain: &Vec<t::TerrainUnit>, offset_y: i32, player: &p::Player, score: u32) {
    clear();
    t::draw_terrain(terrain, offset_y, IY, IX);
    p::draw_player(player, PX);

    mvprintw(LINES() - 1, 0, &format!("Score: {}", score));
    refresh();
}

fn main() {
    initscr();
    raw();
    cbreak();
    nodelay(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    loop {
        let mut terrain: Vec<t::TerrainUnit> =
            vec![t::TerrainUnit::new_flat(IY, false); COLS() as usize];
        terrain.push(t::TerrainUnit {
            tiles: [
                t::TerrainTile::new('_'),
                t::TerrainTile::new('#'),
                t::TerrainTile::new('#'),
            ],
            unit_type: t::TerrainType::Flat,
            initial_y: IY,
            obstacle: false,
        });
        terrain.append(&mut vec![
            t::TerrainUnit::new_flat(IY, false);
            COLS() as usize / 3
        ]);
        let mut player: p::Player = p::Player {
            y_pos: IY,
            air_dist: 0,
            state: p::PlayerState::Idle,
            remember_jump: false,
        };

        let mut last_time = offset::Local::now();
        let mut screen_dist: u32 = 0;
        let mut last_incline_dist: u32 = 0;
        let mut last_obst_dist: u32 = 0;

        let mut offset_y: i32 = 0;
        let mut roffset_y: i32 = 0;

        let mut pause: bool = false;
        let mut playing: bool = true;
        let mut score: u32 = 0;

        let mut speed: i64 = INITIAL_SPEED;
        let mut speed_mult: f64 = 1.0;
        let mut max_air_time: i32 = INITIAL_AIR_TIME;

        draw(&terrain, offset_y, &player, score);
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
                player.jump(terrain[PX as usize].unit_type);
            } else if key == KEY_PAUSE && player.state != p::PlayerState::Dead {
                pause = !pause;
            }

            let t = offset::Local::now();
            if t >= last_time + Duration::milliseconds(speed) {
                if !pause && player.state != p::PlayerState::Dead {
                    screen_dist = t::scroll_terrain(
                        &mut terrain,
                        screen_dist,
                        COLS() as u32 / 3,
                        &mut last_incline_dist,
                        &mut last_obst_dist,
                    );
                    last_time = t;

                    if player.state == p::PlayerState::Running && roffset_y != 0 {
                        let d = if roffset_y > 0 { 1 } else { -1 };
                        offset_y += d;
                        roffset_y -= d;
                    }

                    player.update_pos(IY, &terrain[PX as usize], offset_y, roffset_y, max_air_time);
                    draw(&terrain, offset_y, &player, score);
                    score += 1;

                    roffset_y += match terrain[PX as usize].unit_type {
                        t::TerrainType::Flat => 0,
                        t::TerrainType::Down => -1,
                        t::TerrainType::Up => 1,
                    };

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
