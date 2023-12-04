use bevy::{prelude::*, ecs::system::{Command, SystemState}, gltf::Gltf, render::primitives::Aabb, };
use crate::{AppState, };
use super::controller;
use std::collections::HashMap;

pub struct PathPlugin;
impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PathManager>();

        #[cfg(feature = "gizmos")]
        app.add_systems(Update, display_path);
    }
}

pub struct PathAdder {
    pub global_transform: GlobalTransform,
    pub aabb: Aabb,
    pub name: String,
}
impl Command for PathAdder {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            ResMut<PathManager >,
        )> = SystemState::new(world);

        let (mut path_manager, ) = system_state.get_mut(world);
        let matrix = self.global_transform.compute_matrix();
        let point = matrix.transform_point3(self.aabb.center.into());

        if let Some(last_dot_index) = self.name.rfind('.') {
            let substring = &self.name[last_dot_index..];

            if let Some(index) = substring.chars().rev().take_while(|&c| c.is_digit(10)).collect::<String>().chars().rev().collect::<String>().parse::<usize>().ok() {

                path_manager.points.insert(index, point);
            } else {
                println!("Just once please");
                path_manager.points.insert(0, point);
            }

            path_manager.build(); // TODO: find a better place to do this
        }
    }
}

#[derive(Default, Resource)]
pub struct PathManager {
    points: HashMap<usize, Vec3>,
    path: Vec<Vec3>,
}

impl PathManager {
    pub fn clear(&mut self) {
        self.points = HashMap::default();
        self.path = Vec::default();
    }

    pub fn build(&mut self) {
        let mut points = self.points.iter().collect::<Vec::<_>>(); 
        points.sort_by_key(|x| x.0);
        self.path = points.into_iter().map(|x| *x.1).collect();
    }

    pub fn get_closest_index(&self, point: Vec3) -> Option<usize> {
        let mut closest_index = None;
        let mut closest= f32::MAX;
        for (key, val) in self.points.iter() {
            let distance = point.distance(*val);
            if distance < closest {
                closest = distance;
                closest_index = Some(*key);
            }
        }

        closest_index
    }

    pub fn get_next(&self, index: usize) -> Option<usize> {
        let next = index + 1;
        if let Some(point) = self.points.get(&next) {
            Some(next)
        } else if let Some(point) = self.points.get(&0) {
            Some(0)
        } else {
            None
        }
    }

    /// Assumes index is valid and will panic otherwise
    pub fn get(&self, index: usize) -> Vec3 {
        self.points[&index]
    }

}

#[cfg(feature = "gizmos")]
use bevy::gizmos::gizmos::Gizmos;

fn display_path(
    path_manager: Res<PathManager>,
    mut gizmos: Gizmos,
) {
    for (i, p) in path_manager.path.iter().enumerate() {
        gizmos.sphere(p.clone() + Vec3::new(0., 0.5 * (i as f32), 0.), Quat::IDENTITY, 1., Color::BLUE);
    }
}
