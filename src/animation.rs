use bevy::prelude::*;

#[derive(Component)]
pub struct TranslationAnimation {
    pub start: Vec3,
    pub end: Vec3,
    pub duration: f32,
    pub start_time: f32,
}

pub fn translations(time: Res<Time>, mut query: Query<(&mut Transform, &TranslationAnimation)>) {
    for (mut transform, animation) in query.iter_mut() {
        let mut t = (time.elapsed_seconds() - animation.start_time) / animation.duration;
        t = t.min(1.0);
        transform.translation = animation.start.lerp(animation.end, t);
    }
}
