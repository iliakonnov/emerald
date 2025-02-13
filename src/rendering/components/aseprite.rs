use crate::*;
use crate::{Color, EmeraldError, Rectangle, Vector2, WHITE};

use nanoserde::DeJson;

use types::*;

#[derive(Clone, Debug)]
pub struct Aseprite {
    pub(crate) data: AsepriteData,
    pub(crate) current_tag: AsepriteTag,
    pub(crate) sprite: Sprite,
    pub(crate) elapsed_time: f32,
    pub(crate) is_looping: bool,
    frame_counter: usize,

    pub rotation: f32,
    pub scale: Vector2<f32>,
    pub offset: Vector2<f32>,
    pub visible: bool,
    pub color: Color,
    pub centered: bool,
    pub z_index: f32,
}
impl Aseprite {
    /// Update the inner sprite to reflect the state of the Aseprite.
    /// This should be done before each time the Aseprite is drawn.
    pub(crate) fn update(&mut self) {
        let sheet_size = &self.data.meta.size;
        let frame = self.get_frame();
        let target = &frame.frame;
        let real_y = (sheet_size.h - target.y - target.h) as f32;

        self.sprite.target =
            Rectangle::new(target.x as f32, real_y, target.w as f32, target.h as f32);
    }

    pub(crate) fn new(sprite: Sprite, animation_data: Vec<u8>) -> Result<Aseprite, EmeraldError> {
        let json = String::from_utf8(animation_data)?;
        let data: AsepriteData = DeJson::deserialize_json(&json)?;

        let aseprite = Aseprite {
            data,
            elapsed_time: 0.0,
            frame_counter: 0,
            current_tag: AsepriteTag::default(),
            sprite,
            is_looping: false,
            rotation: 0.0,
            scale: Vector2::new(1.0, 1.0),
            offset: Vector2::new(0.0, 0.0),
            color: WHITE,
            centered: true,
            z_index: 0.0,
            visible: true,
        };

        Ok(aseprite)
    }

    pub fn get_animation_name(&self) -> String {
        self.current_tag.name.clone()
    }

    /// Returns the length of the animation given in seconds.
    /// Returns 0.0 if the animation doesn't exist.
    pub fn get_anim_length<T: Into<String>>(&self, name: T) -> f32 {
        let name: String = name.into();

        for tag in &self.data.meta.frame_tags {
            if tag.name == name {
                let mut total_time = 0;
                let mut i = tag.from;

                while i <= tag.to {
                    if let Some(frame) = self.data.frames.get(i as usize) {
                        total_time += frame.duration;
                    }

                    i += 1;
                }

                return total_time as f32 / 1000.0;
            }
        }

        0.0
    }

    pub fn play<T: Into<String>>(&mut self, new_animation: T) {
        // TODO(bombfuse): Should this reset the the animation or continue?
        self.is_looping = false;
        let new_animation: String = new_animation.into();
        if self.current_tag.name == new_animation {
            return;
        }

        for tag in &self.data.meta.frame_tags {
            if tag.name == new_animation {
                self.current_tag = tag.clone();
                break;
            }
        }

        // TODO(bombfuse): Requested animation couldn't be found
        if self.current_tag.name != new_animation {}

        self.elapsed_time = 0.0;
        self.frame_counter = 0;
    }

    pub fn play_and_loop<T: Into<String>>(&mut self, new_animation: T) {
        self.is_looping = true;
        let new_animation: String = new_animation.into();
        if self.current_tag.name == new_animation {
            return;
        }

        self.elapsed_time = 0.0;
        self.frame_counter = 0;

        for tag in &self.data.meta.frame_tags {
            if tag.name == new_animation {
                self.current_tag = tag.clone();
                return;
            }
        }
    }

    /// !!! WARNING !!!
    /// I have exposed this function to the user in case they choose to toy around with animation speed.
    /// Manually adding delta time may produce undesirable results. Or desirable results, up to you.
    pub fn add_delta(&mut self, delta: f32) {
        self.elapsed_time += delta;
        let frame = self.get_frame();
        let duration = frame.duration as f32 / 1000.0;

        while self.elapsed_time >= duration {
            self.elapsed_time -= duration;
            self.frame_counter += 1;

            if self.frame_counter as u32 > (self.current_tag.to - self.current_tag.from) {
                if self.is_looping {
                    self.frame_counter = 0;
                } else {
                    self.frame_counter = (self.current_tag.to - self.current_tag.from) as usize;
                }
            }
        }
    }

    fn get_frame(&self) -> &AsepriteFrame {
        &self.data.frames[self.current_tag.from as usize + self.frame_counter]
    }
}

pub fn aseprite_update_system(world: &mut EmeraldWorld, delta: f32) {
    for (_, aseprite) in world.query::<&mut Aseprite>().iter() {
        aseprite.add_delta(delta);
    }
}

pub mod types {
    use nanoserde::DeJson;

    #[derive(Clone, Debug, DeJson)]
    pub struct AseRect {
        pub(crate) x: u32,
        pub(crate) y: u32,
        pub(crate) w: u32,
        pub(crate) h: u32,
    }

    #[derive(Clone, Debug, DeJson)]
    pub struct AseSize {
        pub(crate) w: u32,
        pub(crate) h: u32,
    }

    #[derive(Clone, Debug, DeJson)]
    pub struct AsepriteData {
        pub(crate) frames: Vec<AsepriteFrame>,
        pub(crate) meta: AsepriteMeta,
    }

    #[derive(Clone, Debug, DeJson)]
    pub struct AsepriteTag {
        pub(crate) name: String,
        pub(crate) from: u32,
        pub(crate) to: u32,
        pub(crate) direction: String,
    }
    impl Default for AsepriteTag {
        fn default() -> AsepriteTag {
            AsepriteTag {
                name: String::from(""),
                from: 0,
                to: 0,
                direction: String::from("forward"),
            }
        }
    }

    #[derive(Clone, Debug, DeJson)]
    pub struct AsepriteFrame {
        pub(crate) frame: AseRect,
        rotated: bool,
        trimmed: bool,
        pub(crate) duration: u32,
        #[nserde(rename = "spriteSourceSize")]
        sprite_source_size: AseRect,
        #[nserde(rename = "sourceSize")]
        source_size: AseSize,
    }

    #[derive(Clone, Debug, DeJson)]
    pub struct AsepriteMeta {
        format: String,
        pub(crate) size: AseSize,
        scale: String,
        #[nserde(rename = "frameTags")]
        pub(crate) frame_tags: Vec<AsepriteTag>,
    }
}
