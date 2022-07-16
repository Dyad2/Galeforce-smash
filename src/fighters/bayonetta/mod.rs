mod game;
mod weapon;
mod effect;
mod opff;
mod bullet_arts;
mod sound;

pub fn install() {
    game::install();
    weapon::install();
    effect::install();
    opff::install();
    bullet_arts::install();
    sound::install();
}