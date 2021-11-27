

const DEFAULT_SPEED: f64 = 100.0;
pub const DECK_SIZE: usize = 36;
pub const SCREENWIDTH: i32 = 1000;
pub const SCREENHEIGHT: i32 = 1000;

pub const centre_x: f64 = SCREENWIDTH as f64 / 2.0;
pub const centre_y: f64 = SCREENHEIGHT as f64 / 2.0;

#[derive(Clone, Copy)]
pub struct Anims {
    pub cur_x: [f64; DECK_SIZE],
    pub cur_y: [f64; DECK_SIZE],
    pub speed_x: [f64; DECK_SIZE],
    pub speed_y: [f64; DECK_SIZE],
    pub dest_x: [f64; DECK_SIZE],
    pub dest_y: [f64; DECK_SIZE],
}

#[derive(Clone, Copy)]
pub struct Anim {
    cur_x: f64,
    cur_y: f64,
    speed_x: f64,
    speed_y: f64,
    dest_x: f64,
    dest_y: f64,
}

impl Anim {
    pub fn get_pos_x(&self) -> f64 {
        self.cur_x
    }
    pub fn get_pos_y(&self) -> f64 {
        self.cur_y
    }
}


impl Anims {
    pub fn change_dest(&mut self, dest_x: f64, dest_y: f64, n: usize) {
        self.dest_x[n] = dest_x;
        self.dest_y[n] = dest_y;
        self.speed_x[n] = (dest_x - self.cur_x[n]) /1.5 ;
        self.speed_y[n] = (dest_y - self.cur_y[n])/ 1.5 ;
    }
    pub fn new(default_x: f64, default_y: f64) -> Self {
        Anims {
            cur_x:   [default_x; DECK_SIZE],
            speed_x: [default_x; DECK_SIZE],
            dest_x:  [default_x; DECK_SIZE],
            cur_y:   [default_y; DECK_SIZE],
            speed_y: [default_y; DECK_SIZE],
            dest_y:  [default_y; DECK_SIZE],
        }
    }
    pub fn animate(&mut self, dt: f64) {
        for ((cx, (&sx, &dx)), (cy, (&sy, &dy))) in self.cur_x
            .iter_mut()
            .zip(self.speed_x.iter()
            .zip(self.dest_x.iter()))
            .zip(
                self.cur_y
                    .iter_mut()
                    .zip(self.speed_y.iter()
                    .zip(self.dest_y.iter())))
        {
            let x_copy = *cx;
            let y_copy = *cy;
            let x_ = dx - x_copy;
            let y_ = dy - y_copy;
            if (x_ * x_ + y_ * y_).sqrt() > 1.0 {
                *cx += sx * dt;
                *cy += sy * dt;
            }
           
        }
    }
    pub fn get_anim(&self, n: usize) -> Anim {
        Anim {
            cur_x: self.cur_x[n],
            cur_y: self.cur_y[n],
            speed_x: self.speed_x[n],
            speed_y: self.speed_y[n],
            dest_x: self.dest_x[n],
            dest_y: self.dest_y[n],
        }
    }
    pub fn apply_anim(&mut self, n: usize, anim: Anim) {
        self.cur_x[n]   = anim.cur_x;
        self.cur_y[n]   = anim.cur_y;
        self.speed_x[n] = anim.speed_x;
        self.speed_y[n] = anim.speed_y;
        self.dest_x[n]  = anim.dest_x;
        self.dest_y[n]  = anim.dest_y;
    }
    pub fn go_circle(&mut self) {
        const changer: f64 = std::f64::consts::PI * 2.0 / DECK_SIZE as f64;
        let mut x = 0.0f64;


        for i in 0..DECK_SIZE {
            self.change_dest(centre_x + x.sin() * (centre_x - 50.0) - 50.0 ,centre_y + x.cos() * (centre_y  - 85.0) - 85.0, i);
            x += changer;
        }
    }
    pub fn go_straight(&mut self) {
        for i in 0..DECK_SIZE {
            self.change_dest(centre_x- 50.0 ,centre_y - 85.0, i);
        }
    }
}