use crate::objects::node::{self, GraphicsNode};
use kiss3d::window::Window;
use na::Point3;
use rapier::geometry::{ColliderHandle, ColliderSet};
use rapier::math::Isometry;

pub struct Cone {
    color: Point3<f32>,
    base_color: Point3<f32>,
    gfx: GraphicsNode,
    collider: ColliderHandle,
}

impl Cone {
    pub fn new(
        collider: ColliderHandle,
        half_height: f32,
        radius: f32,
        color: Point3<f32>,
        window: &mut Window,
    ) -> Cone {
        #[cfg(feature = "dim2")]
        let node = window.add_rectangle(radius, half_height);
        #[cfg(feature = "dim3")]
        let node = window.add_cone(radius, half_height * 2.0);

        let mut res = Cone {
            color,
            base_color: color,
            gfx: node,
            collider,
        };

        // res.gfx.set_texture_from_file(&Path::new("media/kitten.png"), "kitten");
        res.gfx.set_color(color.x, color.y, color.z);
        res
    }

    pub fn select(&mut self) {
        self.color = Point3::new(1.0, 0.0, 0.0);
    }

    pub fn unselect(&mut self) {
        self.color = self.base_color;
    }

    pub fn set_color(&mut self, color: Point3<f32>) {
        self.gfx.set_color(color.x, color.y, color.z);
        self.color = color;
        self.base_color = color;
    }

    pub fn update(&mut self, colliders: &ColliderSet) {
        node::update_scene_node(
            &mut self.gfx,
            colliders,
            self.collider,
            &self.color,
            &Isometry::identity(),
        );
    }

    pub fn scene_node(&self) -> &GraphicsNode {
        &self.gfx
    }

    pub fn scene_node_mut(&mut self) -> &mut GraphicsNode {
        &mut self.gfx
    }

    pub fn object(&self) -> ColliderHandle {
        self.collider
    }
}
