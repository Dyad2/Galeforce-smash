use super::*;

mod bullet_arts;
mod effect;
mod normals;
mod sound;
mod special;
mod weapon;

pub fn install(agent: &mut smashline::Agent) {
    bullet_arts::install(agent);
    effect::install(agent);
    normals::install(agent);
    sound::install(agent);
    special::install(agent);
    weapon::install();
}