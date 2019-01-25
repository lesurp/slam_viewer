use alga::linear::Similarity;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Matrix3, Point3, Rotation, Translation3, UnitQuaternion, Vector3};

pub struct SlamViewer {
    w: Window,
    rays: Vec<(Point3<f32>, Point3<f32>, Point3<f32>)>,
    k_inv: Matrix3<f32>,
    scale: f32,
}

pub struct Camera {
    n: SceneNode,
    pub r_wc: Rotation<f32, na::U3>,
    pub t_wc: Vector3<f32>,
}

impl Default for SlamViewer {
    fn default() -> Self {
        SlamViewer {
            w: Window::new("SlamViewer"),
            rays: Vec::new(),
            k_inv: Matrix3::identity(),
            scale: 1.0,
        }
    }
}

impl SlamViewer {
    pub fn new(name: &str, k: &Matrix3<f32>, scale: f32) -> SlamViewer {
        SlamViewer {
            w: Window::new(name),
            rays: Vec::new(),
            k_inv: k.try_inverse().unwrap(),
            scale,
        }
    }

    pub fn draw_coordinate_system(&mut self) {
        let h = self.scale * 10.0;
        let r = self.scale * 0.1;

        let mut xyz = self.w.add_group();
        let mut x_axis = xyz.add_cylinder(r, h);
        let mut y_axis = xyz.add_cylinder(r, h);
        let mut z_axis = xyz.add_cylinder(r, h);

        x_axis.set_local_translation(Translation3::new(h / 2.0, 0.0, 0.0));
        y_axis.set_local_translation(Translation3::new(0.0, h / 2.0, 0.0));
        z_axis.set_local_translation(Translation3::new(0.0, 0.0, h / 2.0));

        x_axis.set_color(1.0, 0.0, 0.0);
        y_axis.set_color(0.0, 1.0, 0.0);
        z_axis.set_color(0.0, 0.0, 1.0);

        x_axis.set_local_rotation(UnitQuaternion::from_axis_angle(
            &Vector3::z_axis(),
            -::std::f32::consts::FRAC_PI_2,
        ));
        z_axis.set_local_rotation(UnitQuaternion::from_axis_angle(
            &Vector3::x_axis(),
            ::std::f32::consts::FRAC_PI_2,
        ));
    }

    pub fn camera_from_p_cw(
        &mut self,
        r_cw: Rotation<f32, na::U3>,
        t_cw: Vector3<f32>,
        c: (f32, f32, f32),
    ) -> Camera {
        let r_wc = r_cw.transpose();
        let t_wc = r_wc * (-1.0 * t_cw);
        self.camera_from_p_wc(r_wc, t_wc, c)
    }

    pub fn camera_from_p_wc(
        &mut self,
        r_wc: Rotation<f32, na::U3>,
        t_wc: Vector3<f32>,
        c: (f32, f32, f32),
    ) -> Camera {
        let mut n = self.w.add_group();
        let mut core = n.add_sphere(1.0 * self.scale);
        let mut lens = n.add_sphere(0.5 * self.scale);
        let mut top = n.add_sphere(0.3 * self.scale);
        core.set_color(c.0, c.1, c.2);
        lens.set_color(c.0, c.1, c.2);
        top.set_color(c.0, c.1, c.2);

        core.set_local_translation(Translation3::from(t_wc));
        let forward_a_bit =
            Translation3::from(r_wc.rotate_vector(&Vector3::new(0.0, 0.0, 1.0 * self.scale)));
        lens.set_local_translation(Translation3::from(t_wc));
        lens.prepend_to_local_translation(&forward_a_bit);

        let up_a_bit = Translation3::from(r_wc.rotate_vector(&Vector3::new(0.0, 1.0 * self.scale, 0.0)));
        top.set_local_translation(Translation3::from(t_wc));
        top.prepend_to_local_translation(&up_a_bit);
        Camera { n, r_wc, t_wc }
    }

    pub fn add_point(&mut self, p: Vector3<f32>, c: (f32, f32, f32)) {
        let mut s = self.w.add_sphere(0.4 * self.scale);
        s.set_local_translation(Translation3::from(p));
        s.set_color(c.0, c.1, c.2);
    }

    pub fn add_ray(&mut self, camera: &Camera, xp: (f32, f32), c: (f32, f32, f32)) {
        let xc = self.k_inv * Vector3::new(xp.0, xp.1, 1.0);
        let x_world = camera.r_wc.rotate_vector(&xc);
        let t_x_1 = Translation3::from(100.0 * x_world);

        self.rays.push((
            Point3::from(camera.t_wc),
            t_x_1 * Point3::from(camera.t_wc),
            Point3::new(c.0, c.1, c.2),
        ));
    }

    pub fn spin(mut self) {
        self.w.set_light(Light::StickToCamera);
        self.w.set_point_size(25.0 * self.scale);
        while self.w.render() {
            for (p1, p2, c) in &self.rays {
                self.w.draw_line(&p1, &p2, &c);
            }
        }
    }
}
