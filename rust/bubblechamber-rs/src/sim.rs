use ndarray::Array1;

pub fn cross_product(a: &Array1<f32>, b: &Array1<f32>) -> Array1<f32> {
    let res = vec![
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ];
    Array1::from_vec(res)
}
