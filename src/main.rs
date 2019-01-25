extern crate kiss3d;
#[macro_use]
extern crate log;
extern crate alga;
extern crate env_logger;
extern crate nalgebra as na;
#[macro_use]
extern crate nom;

mod parser;
mod slam_viewer;
use na::Matrix3;

use parser::Parser;
use slam_viewer::SlamViewer;

fn main() {
    env_logger::init();

    let k = Matrix3::new(517.013, 0.0, 323.256, 0.0, 517.516, 251.825, 0.0, 0.0, 1.0);
    let mut sv = SlamViewer::new("Super SLAM viewer", &k, 0.1);
    sv.draw_coordinate_system();

    let slam_data = Parser::parse_file("data").unwrap();
    for camera in slam_data.cameras {
        let cam = sv.camera_from_p_cw(camera.r_cw, camera.t_cw, (1.0, 1.0, 0.0));
        for pixel in camera.pixels {
            sv.add_ray(&cam, (pixel[0], pixel[1]), (1.0, 1.0, 1.0));
        }
    }

    for point in slam_data.points {
        sv.add_point(point, (0.0, 1.0, 0.0));
    }

    sv.spin();
}
