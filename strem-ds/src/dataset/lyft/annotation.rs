use nalgebra::{
    Quaternion, RowSVector as StaticRowVector, SMatrix as StaticMatrix, SVector as StaticVector,
    Translation, UnitQuaternion,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Annotation {
    pub token: String,
    pub sample_token: String,
    pub instance_token: String,
    pub attribute_tokens: Vec<String>,
    pub visibility_token: String,
    /// x, y, z
    pub translation: [f64; 3],
    /// width, length, height
    pub size: [f64; 3],
    /// w, x, y, z
    pub rotation: [f64; 4],
    pub num_lidar_pts: i32,
    pub num_radar_pts: i32,
    pub next: String,
    pub prev: String,
}

impl Annotation {
    /// Compute the corners of the box in 3D space.
    ///
    /// This function has been translated from the original implementation found
    /// [here](https://github.com/lyft/nuscenes-devkit/blob/master/lyft_dataset_sdk/utils/data_classes.py#L622).
    fn corners(&self) -> StaticMatrix<f64, 3, 8> {
        let [width, length, height] = self.size;

        // Construct the correctly scaled box about the origin (0, 0, 0) in the
        // 3D coordinate plane.
        //
        // The representation of the box is a 3x8 matrix where each row
        // corresponds to the x, y, and z direction, respectively. Therefore,
        // the unit box can be represented as the following 8 points:
        //
        // x: [1, 1, 1, 1, -1, -1, -1, -1]
        // y: [1, -1, -1, 1, 1, -1, -1, 1]
        // z: [1, 1, -1, -1, 1, 1, -1, -1]
        let xs: StaticRowVector<f64, 8> =
            (length / 2.0) * StaticRowVector::from([1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0]);
        let ys: StaticRowVector<f64, 8> =
            (width / 2.0) * StaticRowVector::from([1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0]);
        let zs: StaticRowVector<f64, 8> =
            (height / 2.0) * StaticRowVector::from([1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0]);

        let corners: StaticMatrix<f64, 3, 8> = StaticMatrix::from_rows(&[xs, ys, zs]);

        // Rotate the box based on the annotation rotation. This rotation is with
        // respect to the global coordinate system.
        //
        // The correct operation to perform here is a matrix multiplication. Not
        // a dot product.
        let [w, i, j, k] = self.rotation;
        let rotation: UnitQuaternion<f64> =
            UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k));

        let corners = rotation.to_rotation_matrix().matrix() * corners;

        // Translate the box based on the annotation translation. This
        // translation is with respect to the global coordinate system.
        let [x, y, z] = self.translation;

        let corners = StaticMatrix::from_rows(&[
            corners.row(0).add_scalar(x),
            corners.row(1).add_scalar(y),
            corners.row(2).add_scalar(z),
        ]);

        corners
    }

    /// Translate (shift) [`Annotation`] by [`Translation`] amount.
    ///
    /// This performs a component-wise scalar addition in the x, y, and z
    /// direction, respectively.
    fn translate(&mut self, translation: Translation<f64, 3>) {
        let [x, y, z] = self.translation;

        self.translation[0] = x + translation.x;
        self.translation[1] = y + translation.y;
        self.translation[2] = z + translation.z;
    }

    /// Rotate [`Annotation`] by a [`UnitQuaternion`] amount.
    ///
    /// This rotates results in the [`Annotation`] rotated about the origin of
    /// the coordinate system (0, 0, 0).
    fn rotate(&mut self, rotation: UnitQuaternion<f64>) {
        let matrix = *rotation.to_rotation_matrix().matrix();
        let translation = matrix * StaticVector::<f64, 3>::from(self.translation);

        self.translation = translation
            .iter()
            .cloned()
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();

        let [w, i, j, k] = self.rotation;
        let rotation = rotation * UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k));

        let [x, y, z, w]: [f64; 4] = rotation
            .as_vector()
            .iter()
            .cloned()
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();

        self.rotation = [w, x, y, z];
    }

    /// Perform a complete transformation (translation and rotation).
    ///
    /// This is a convenience function that performs both a translation and
    /// rotation (in that order).
    pub fn transform(
        mut self,
        translation: Translation<f64, 3>,
        rotation: UnitQuaternion<f64>,
    ) -> Self {
        self.translate(translation);
        self.rotate(rotation);

        self
    }

    /// Check if the annotation is within the image.
    ///
    /// This procedure is ported from the NuScenes SDK provided. For more
    /// information, see:
    /// https://github.com/lyft/nuscenes-devkit/blob/49c36da0a85da6bc9e8f2a39d5d967311cd75069/lyft_dataset_sdk/utils/geometry_utils.py#L62
    pub fn inside(&self, view: StaticMatrix<f64, 3, 3>, dimensions: (f64, f64)) -> bool {
        let corners = self.corners();
        let projection = self.projection(view);

        let visible = projection
            .row(0)
            .iter()
            .all(|x| *x > 0.0 && *x < dimensions.0 as f64);

        let visible = visible
            && projection
                .row(1)
                .iter()
                .all(|x| *x > 0.0 && *x < dimensions.1 as f64);

        let visible = visible && corners.row(2).iter().all(|x| *x > 1.0);

        visible
    }

    /// Project the annotation onto a perspective and normalize.
    ///
    /// This projection does not modify the annotation but simply returns the
    /// set of projected points representing the box.
    pub fn projection(&self, view: StaticMatrix<f64, 3, 3>) -> StaticMatrix<f64, 3, 8> {
        let corners = self.corners();

        // project
        let corners = view * corners;

        // normalize
        let corners = corners.component_div(&StaticMatrix::<f64, 3, 8>::from_rows(&[
            StaticRowVector::<f64, 8>::from(corners.row(2)),
            StaticRowVector::<f64, 8>::from(corners.row(2)),
            StaticRowVector::<f64, 8>::from(corners.row(2)),
        ]));

        corners
    }
}
