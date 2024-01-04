pub fn seq(min: f64, max: f64, step: f64) -> Vec<f64> {

    let mut v = vec![0.0; ((max - min) / step + 1.0).round() as usize];

    for i in 0..v.len() {
        v[i] = min + step * i as f64;
    }

    v

}
