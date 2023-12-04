use bevy::prelude::*;

pub struct ZoomPlugin;

impl Plugin for ZoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_zoom_time, zoom).chain());
    }
}

#[derive(Component)]
pub struct Zoom {
    pub zoom_level: f32,
    pub duration: f32,
}

impl Default for Zoom {
    fn default() -> Self {
        Self {
            zoom_level: 1.0,
            duration: 1.0,
        }
    }
}

#[derive(Component)]
pub struct ZoomTime(pub f32);

pub fn set_zoom_time(
    mut commands: Commands,
    time: Res<Time>,
    mut entities: Query<(Entity, Option<&mut ZoomTime>), Changed<Zoom>>,
) {
    let elapsed = time.elapsed_seconds();

    for (ent, zoom_time) in entities.iter_mut() {
        info!("Starting zoom");

        match zoom_time {
            Some(mut zoom_time) => zoom_time.0 = elapsed,
            None => {
                commands.entity(ent).insert(ZoomTime(elapsed));
            }
        }
    }
}

pub fn zoom(
    mut commands: Commands,
    time: Res<Time>,
    mut entities: Query<(Entity, &mut Transform, &Zoom, &ZoomTime)>,
) {
    let elapsed = time.elapsed_seconds();

    for (ent, mut transform, zoom, zoom_time) in entities.iter_mut() {
        let delta = elapsed - zoom_time.0;
        let progress = delta / zoom.duration;
        let progress = progress.min(1.0);

        let scale = 1.0 + (1.0 / zoom.zoom_level - 1.0) * progress;
        transform.scale = Vec3::splat(scale);

        if progress >= 1.0 {
            info!("Zoom complete");
            commands.entity(ent).remove::<ZoomTime>();
        }
    }
}
