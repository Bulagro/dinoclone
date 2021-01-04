use ncurses::*;
pub mod player;
pub mod terrain;

pub use player as p;
pub use terrain as t;

// Color stuff
pub const PAIR_WHITE: i16 = 0;
pub const PAIR_GREEN: i16 = 1;
pub const PAIR_YELLOW: i16 = 2;
pub const PAIR_RED: i16 = 3;
pub const PAIR_BLUE: i16 = 4;

pub const KEY_QUIT: i32 = 'q' as i32;
pub const KEY_PAUSE: i32 = 'p' as i32;
pub const KEY_JUMP: i32 = 'j' as i32;

pub const IY: i32 = 6;
pub const IX: i32 = 1;
pub const PX: i32 = 23;

pub const MAX_SPEED: i64 = 40; // milliseconds update time
pub const SPEED_CHANGE_INTERVAL: u32 = 300;
pub const SPEED_MULT_CONST: f64 = 0.1;
pub const INITIAL_SPEED: i64 = 100;
pub const INITIAL_AIR_TIME: i32 = 7;

pub fn initialize_colors() {
    start_color();

    init_pair(PAIR_WHITE, COLOR_WHITE, COLOR_BLACK);
    init_pair(PAIR_GREEN, COLOR_GREEN, COLOR_BLACK);
    init_pair(PAIR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
    init_pair(PAIR_RED, COLOR_RED, COLOR_BLACK);
    init_pair(PAIR_BLUE, COLOR_BLUE, COLOR_BLACK);
}

pub fn draw(terrain: &t::Terrain, player: &p::Player, game_data: &Game) {
    clear();
    terrain.draw_terrain();
    player.draw_player();

    mvprintw(LINES() - 1, 0, &format!("Score: {}", game_data.score));
    refresh();
}

pub fn exit_config() {
    nocbreak();
    endwin();
}

pub struct Game {
    pub playing: bool,
    pub pause: bool,
    pub score: u32,
    pub speed: i64,
    pub max_air_time: i32,
    speed_mult: f64,
}

impl Game {
    pub fn new() -> Self {
        Game {
            playing: true,
            pause: false,
            score: 0,
            speed: INITIAL_SPEED,
            speed_mult: 1.0,
            max_air_time: INITIAL_AIR_TIME,
        }
    }

    pub fn update_speed(&mut self) {
        if self.score % SPEED_CHANGE_INTERVAL == 0 && self.speed > MAX_SPEED {
            self.speed_mult -= SPEED_MULT_CONST;
            self.speed = (INITIAL_SPEED as f64 * self.speed_mult) as i64; // linear
                                                                // speed = (speed as f64 * speed_mult) as i64; // mon-linear
            self.max_air_time =
                INITIAL_AIR_TIME + (self.max_air_time as f64 * (1.0 - self.speed_mult)) as i32;
        }
    }

    pub fn update_score(&mut self) {
        self.score += 1;
    }
}
