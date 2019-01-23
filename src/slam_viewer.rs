use na::{Translation3, Matrix3, Point3, Rotation, Vector3, UnitQuaternion};
use alga::linear::Similarity;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

const SCALE : f32 = 0.1;

pub struct SlamViewer {
	w: Window,
	rays: Vec<(Point3<f32>, Point3<f32>, Point3<f32>)>,
	k_inv: Matrix3<f32>,
}

pub struct Camera {
    n: SceneNode,
    pub r_wc: Rotation<f32, na::U3>,
    pub t_wc: Vector3<f32>,
}

impl SlamViewer {
    pub fn new(k: &Matrix3<f32>) -> SlamViewer {
		SlamViewer {
			w: Window::new("SlamViewer yo"),
			rays: Vec::new(),
			k_inv: k.try_inverse().unwrap(),
		}
	}

    pub fn draw_coordinate_system(&mut self) {
        let h = SCALE * 10.0;
        let r = SCALE*0.1;

        let mut xyz = self.w.add_group();
        let mut x_axis = xyz.add_cylinder(r, h);
        let mut y_axis = xyz.add_cylinder(r, h);
        let mut z_axis = xyz.add_cylinder(r, h);

        x_axis.set_local_translation(Translation3::new(h/2.0, 0.0, 0.0));
        y_axis.set_local_translation(Translation3::new(0.0, h/2.0, 0.0));
        z_axis.set_local_translation(Translation3::new(0.0, 0.0, h/2.0));

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

    pub fn camera_from_p_wc(&mut self, r_wc: Rotation<f32, na::U3>, t_wc: Vector3<f32>, c: (f32, f32, f32)) -> Camera {
        let r_cw = r_wc.transpose();
		let t_cw = Translation3::from(r_cw * (-1.0 * t_wc));

        let mut n = self.w.add_group();
		let mut core = n.add_sphere(1.0 * SCALE);
		let mut lens = n.add_sphere(0.5 * SCALE);
		let mut top = n.add_cube(0.3 * SCALE,0.3 * SCALE,0.3 * SCALE);
		core.set_color(c.0, c.1, c.2);
		lens.set_color(c.0, c.1, c.2);
		top.set_color(c.0, c.1, c.2);

		core.set_local_translation(t_cw);
		let forward_a_bit = Translation3::from(r_cw.rotate_vector(&Vector3::new(0.0, 0.0, 1.0 * SCALE)));
		lens.set_local_translation(t_cw);
		lens.prepend_to_local_translation(&forward_a_bit);

		let up_a_bit = Translation3::from(r_cw.rotate_vector(&Vector3::new(0.0, 1.0 * SCALE, 0.0)));
		top.set_local_translation(t_cw);
		top.prepend_to_local_translation(&up_a_bit);
        Camera {
            n,
            r_wc,
            t_wc,
        }
	}

    pub fn draw_point(&mut self, p: Vector3<f32>, c: (f32, f32, f32)) {
		let mut s = self.w.add_sphere(0.4 * SCALE);
		s.set_local_translation(Translation3::from(p));
		s.set_color(c.0, c.1, c.2);
	}

    pub fn add_ray(&mut self, camera: &Camera, xp: (f32, f32), c: (f32, f32, f32)) {
		let r_cw = camera.r_wc.transpose();
		let xc = self.k_inv * Vector3::new(xp.0, xp.1, 1.0);
		let x_world = r_cw.rotate_vector(&xc);
		let t = r_cw * (-1.0 * camera.t_wc);
		let t_x_1 = Translation3::from(100.0 * x_world);

		self.rays.push((Point3::from(t), t_x_1 * Point3::from(t) ,Point3::new(c.0, c.1, c.2)));
	}

    pub fn spin(mut self) {
		self.w.set_light(Light::StickToCamera);
		self.w.set_point_size(25.0 * SCALE);
		while self.w.render() {
			for (p1, p2, c) in &self.rays {
				self.w.draw_line(&p1, &p2, &c);
			}
		}
	}
}
