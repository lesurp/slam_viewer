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

    let scale = if let Some(scale_str) = std::env::args().nth(1) {
        scale_str.parse::<f32>().unwrap_or_else(|_| {
            error!("Scale argument couldn't be parsed; defaulting to 1.0");
            1.0
        })
    } else {
        1.0
    };

    // quick n dirty colors
    let colors = vec![
        (1.0, 1.0, 0.0),
        (1.0, 0.0, 1.0),
        (0.0, 1.0, 1.0),
        (1.0, 1.0, 1.0),
    ];
    let mut colors_it = colors.iter().cycle();
    let mut next_color = || *colors_it.next().unwrap();

    let k = Matrix3::new(517.013, 0.0, 323.256, 0.0, 517.516, 251.825, 0.0, 0.0, 1.0);
    let mut sv = SlamViewer::new("Super SLAM viewer", &k, scale);
    sv.draw_coordinate_system();

    let slam_data = Parser::parse_file("data").unwrap();
    let mut color_map = std::collections::HashMap::new();
    for camera in slam_data.cameras {
        let color = color_map.entry(camera.camera_id).or_insert(next_color());
        debug!("Adding camera with color {:?}", color);
        let cam = sv.camera_from_p_cw(camera.r_cw, camera.t_cw, *color);
        for pixel in camera.pixels {
            sv.add_ray(&cam, (pixel[0], pixel[1]), (1.0, 1.0, 1.0));
        }
    }

    for point in slam_data.points {
        sv.add_point(point, (0.0, 1.0, 0.0));
    }

    sv.spin();
}
