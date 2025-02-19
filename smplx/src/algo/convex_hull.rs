/*
    Appellation: convex_hull <module>
    Contrib: @FL03
*/

pub struct ConvexHull<T = f64, const N: usize = 2>
where
    T: RealField + Scalar,
{
    
    vertices: Vec<Point<T, N>>,
}