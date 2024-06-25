use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::{render_asset::RenderAssetUsages, render_resource::PrimitiveTopology};

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

use spade::{DelaunayTriangulation, Point2, Triangulation};

#[derive(Component)]
pub struct MapCell {}

#[derive(Bundle)]
struct MapCellBundle {
    map_cell: MapCell,
    pbr_bundle: PbrBundle,
}

pub fn generate_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();
    let xy_distribution = Uniform::from(-10.0..10.0);
    let mut rng = rand::thread_rng();
    for _ in 0..25 {
        let _ = triangulation.insert(Point2::new(
            xy_distribution.sample(&mut rng),
            xy_distribution.sample(&mut rng),
        ));
    }

    for face in triangulation.voronoi_faces() {
        // skip all outer cells. they don't have a finite position.
        if face
            .as_delaunay_vertex()
            .out_edges()
            .any(|edge| edge.is_outer_edge())
        {
            continue;
        }
        let points: VecDeque<_> = face
            .adjacent_edges()
            .map(|e| {
                (
                    e.from().position().unwrap().x,
                    e.from().position().unwrap().y,
                )
            })
            .enumerate()
            .map(|(i, p)| (i.try_into().unwrap(), p))
            .collect();
        let cell_mesh = generate_cell_mesh(points);

        commands.spawn(MapCellBundle {
            map_cell: MapCell {},
            pbr_bundle: PbrBundle {
                mesh: meshes.add(cell_mesh),
                material: materials.add(Color::RgbaLinear {
                    red: rng.gen(),
                    green: rng.gen(),
                    blue: rng.gen(),
                    alpha: 1.0,
                }),
                ..default()
            },
        });
    }
}

fn generate_cell_mesh(mut points: VecDeque<(u32, (f32, f32))>) -> Mesh {
    let point_list: Vec<Vec3> = points
        .iter()
        .map(|(_i, p)| Vec3 {
            x: p.0,
            y: 0.0,
            z: p.1,
        })
        .collect();
    let normal_list: Vec<Vec3> = vec![[0.0, 1.0, 0.0].into(); point_list.len()];
    let mut triangle_list = Vec::with_capacity(point_list.len() - 2);

    while points.len() >= 3 {
        let a = points.pop_front().unwrap();
        let b = points.pop_front().unwrap();
        let c = points.pop_front().unwrap();
        triangle_list.push(a.0);
        triangle_list.push(c.0);
        triangle_list.push(b.0);
        points.push_back(a);
        points.push_front(c);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, point_list)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normal_list)
    .with_inserted_indices(Indices::U32(triangle_list))
}
