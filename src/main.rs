extern crate kiss3d;
extern crate alga;
extern crate nalgebra as na;

mod slam_viewer;
use na::{Matrix3, Rotation, Vector3};

use slam_viewer::SlamViewer;

fn main() {
    /*
       let k = Matrix3::new(
       517.013,       0.0, 323.256,
       0.0, 517.516, 251.825,
       0.0,       0.0,       1.0);

       let rot_1 = Rotation::from_matrix_unchecked(Matrix3::new(
       0.7071068,  0.0000000, 0.7071068,
       0.0000000,  1.0000000,  0.0000000,
       -0.7071068,  0.0000000,  0.7071068 
       ));
       let t_1 = Vector3::new(0.0,
       0.0,
       0.0);
       let p_1 = (320.0, 240.0);

       let rot_2 = Rotation::from_matrix_unchecked(Matrix3::new(
       1.0, 0.0,   0.0,  
       0.0,   1.0,  0.0,
       0.0, 0.0,   1.0, 
       ));
       let t_2 = Vector3::new(0.5,
       0.0,
       0.0);
       let p_2 = (320.0, 240.0);
       */
    /*
       let k = Matrix3::new(
       517.013,       0.0, 323.256,
       0.0, 517.516, 251.825,
       0.0,       0.0,       1.0);

       let rot_1 = Rotation::from_matrix_unchecked(Matrix3::new(
       0.986976, 0.0707662,   0.144463,  
       -0.0783759,   0.995783,  0.0476759,
       -0.14048, -0.0583774,   0.988361, 
       ));
       let t_1 = Vector3::new(-0.892905,
       0.0369618,
       0.368998);
       let p_1 = (218.0, 198.0);

       let rot_2 = Rotation::from_matrix_unchecked(Matrix3::new(
       0.976512, 0.182308,   0.114837    ,
       -0.193572,   0.976378,  0.0960001,
       -0.0946229,  -0.115975,   0.988735
       ));
       let t_2 = Vector3::new( -1.24897,
       0.0915136,
       0.437416);
       let p_2 = (105.0, 242.0);
       let point_3d = Vector3::new(0.93506, 0.753344, 2.08746);
*/
    let k = Matrix3::new(
        517.013,       0.0, 323.256,
        0.0, 517.516, 251.825,
        0.0,       0.0,       1.0);
    let rot_1 = Rotation::from_matrix_unchecked(Matrix3::new(
            0.979568,  0.0133216,  0.200674,
0.0421675,  0.962021, -0.269699,
-0.196645,   0.27265,  0.941803,

            ));
    let t_1 = Vector3::new(-0.407394,
0.0549453,
-0.242197,
);
    let p_1 = (257.0, 139.0);

    let rot_2 = Rotation::from_matrix_unchecked(Matrix3::new(
            0.982863,  -0.0410134,  -0.179715, 
 0.0121097,   0.987194,  -0.159064,
  0.183937,   0.154161,   0.970774,
            ));
    let t_2 = Vector3::new(0.195724,
0.0145315,
 -0.26705,
);
    let p_2 = (127.0, 195.0);
    let point_3d = Vector3::new(-0.710494,
                                0.21586,
                                3.44198,
                                );
//       */

    let mut sv = SlamViewer::new(&k);
    let camera_1 = sv.camera_from_p_wc(rot_1, t_1, (0.0, 0.0, 1.0));
    let camera_2 = sv.camera_from_p_wc(rot_2, t_2, (0.0, 1.0, 0.0));

    sv.draw_point(point_3d, (1.0, 0.0, 0.0));
    sv.draw_coordinate_system();

    sv.add_ray(&camera_1, p_1, (1.0, 1.0, 1.0));
    sv.add_ray(&camera_2, p_2, (1.0, 1.0, 1.0));

    sv.spin();
}
