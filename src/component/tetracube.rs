use bevy::math::Vec3;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

macro_rules! shape {
    [$(($x:expr, $y:expr, $z:expr)),*] => {
        [$(
            Vec3::new($x, $y, $z),
        )*]
    }
}

#[derive(Debug)]
pub struct Tetracube {
    pub kind: TetracubeShape,
    pub position: [Vec3; 4],
}

#[derive(Debug)]
pub enum TetracubeShape {
    I,
    O,
    L,
    T,
    N,
    TowerRight,
    TowerLeft,
    Tripod,
}

impl Tetracube {
    fn random() -> Self {
        let kind: TetracubeShape = rand::random();
        Self::from(kind)
    }
}

impl Distribution<TetracubeShape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetracubeShape {
        match rng.gen_range(0, 8) {
            0 => TetracubeShape::I,
            1 => TetracubeShape::O,
            2 => TetracubeShape::L,
            3 => TetracubeShape::T,
            4 => TetracubeShape::N,
            5 => TetracubeShape::TowerRight,
            6 => TetracubeShape::TowerLeft,
            7 => TetracubeShape::Tripod,
            _ => TetracubeShape::I,
        }
    }
}

impl From<TetracubeShape> for Tetracube {
    fn from(kind: TetracubeShape) -> Self {
        let (shape, origin) = match kind {
            TetracubeShape::I => (
                shape![
                    (0.0, 0.0, 0.0),
                    (1.0, 0.0, 0.0),
                    (2.0, 0.0, 0.0),
                    (3.0, 0.0, 0.0)
                ],
                Vec3::new(1.5, 0.5, 0.5),
            ),
            TetracubeShape::O => (
                shape![
                    (0.0, 0.0, 0.0),
                    (0.0, 1.0, 0.0),
                    (1.0, 0.0, 0.0),
                    (1.0, 1.0, 0.0)
                ],
                Vec3::new(0.5, 0.5, 0.5),
            ),
            TetracubeShape::L => (
                shape![
                    (0.0, 0.0, 0.0),
                    (1.0, 0.0, 0.0),
                    (2.0, 0.0, 0.0),
                    (2.0, 1.0, 0.0)
                ],
                Vec3::new(1.0, 0.0, 0.0),
            ),
            TetracubeShape::T => (
                shape![
                    (0.0, 0.0, 0.0),
                    (1.0, 0.0, 0.0),
                    (1.0, 1.0, 0.0),
                    (2.0, 0.0, 0.0)
                ],
                Vec3::new(1.0, 0.0, 0.0),
            ),
            TetracubeShape::N => (
                shape![
                    (0.0, 0.0, 0.0),
                    (1.0, 0.0, 0.0),
                    (1.0, 1.0, 0.0),
                    (2.0, 1.0, 0.0)
                ],
                Vec3::new(1.0, 0.0, 0.0),
            ),
            TetracubeShape::TowerRight => (
                shape![
                    (0.0, 0.0, 0.0),
                    (1.0, 0.0, 0.0),
                    (1.0, 1.0, 0.0),
                    (1.0, 1.0, 1.0)
                ],
                Vec3::new(1.0, 0.0, 0.0),
            ),
            TetracubeShape::TowerLeft => (
                shape![
                    (0.0, 0.0, 0.0),
                    (0.0, 1.0, 0.0),
                    (0.0, 1.0, 1.0),
                    (1.0, 0.0, 0.0)
                ],
                Vec3::new(0.0, 0.0, 0.0),
            ),

            TetracubeShape::Tripod => (
                shape![
                    (0.0, 0.0, 0.0),
                    (0.0, 0.0, 1.0),
                    (1.0, 0.0, 0.0),
                    (0.0, 1.0, 0.0)
                ],
                Vec3::new(0.0, 0.0, 0.0),
            ),
        };
        let position = [
            shape[0] - origin,
            shape[1] - origin,
            shape[2] - origin,
            shape[3] - origin,
        ];
        Self { kind, position }
    }
}
