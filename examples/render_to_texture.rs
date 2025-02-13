use emerald::*;

const RES_WIDTH: usize = 320;
const RES_HEIGHT: usize = 160;

pub fn main() {
    let mut settings = GameSettings::default();
    let mut render_settings = RenderSettings::default();
    render_settings.resolution = (320 * 2, 160 * 2);
    settings.render_settings = render_settings;

    emerald::start(Box::new(MyGame { pos: Position::new(320.0, 160.0), scale: 1.0, render_texture: None }), settings)
}

pub struct MyGame {
    pos: Position,
    scale: f32,
    render_texture: Option<TextureKey>
}
impl Game for MyGame {
    fn initialize(&mut self, mut emd: Emerald) {
        self.render_texture = Some(emd.loader().render_texture(RES_WIDTH as usize, RES_HEIGHT as usize).unwrap());
    }

    fn update(&mut self, mut emd: Emerald) {
        let mut input = emd.input();
        let delta = emd.delta();
        let speed = 150.0;

        if input.is_key_pressed(KeyCode::Left) {
            self.pos.x -= speed * delta;
        }

        if input.is_key_pressed(KeyCode::Right) {
            self.pos.x += speed * delta;
        }

        if input.is_key_pressed(KeyCode::Up) {
            self.pos.y += speed * delta;
        }

        if input.is_key_pressed(KeyCode::Down) {
            self.pos.y -= speed * delta;
        }

        if input.is_key_just_pressed(KeyCode::A) {
            self.scale *= 0.5;
        }

        if input.is_key_just_pressed(KeyCode::S) {
            self.scale *= 2.0;
        }

        println!("pos {:?}", self.pos);
    }

    fn draw(&mut self, mut emd: Emerald) {
        let now = std::time::Instant::now();
        emd.graphics().begin_texture(self.render_texture.as_ref().unwrap().clone());

        let rabbit = emd.loader().sprite("./examples/assets/bunny.png").unwrap();
        emd.graphics().draw_color_rect(&ColorRect::new(WHITE, 500 * 500, 500 * 500),
        &Position::new((RES_WIDTH / 2) as f32, (RES_HEIGHT / 2) as f32));
        emd.graphics().draw_sprite(&rabbit, &Position::new((RES_WIDTH / 2) as f32, (RES_HEIGHT / 2) as f32));
        let texture_key = emd.graphics().render_texture().unwrap();
        
        let e = std::time::Instant::now();

        println!("texture render: {:?}", e - now);

        // println!("{:?}", screen_sprite);
        let now = std::time::Instant::now();

        let e = std::time::Instant::now();
        let mut screen_sprite = Sprite::from_texture(texture_key);
        screen_sprite.centered = false;
        screen_sprite.scale.x = self.scale;
        screen_sprite.scale.y = self.scale;

        emd.graphics().begin();
        emd.graphics().draw_sprite(&screen_sprite, &self.pos);
        emd.graphics().render();

        println!("screen draw: {:?}", e - now);
    }
}
