pub fn cohen_d(v: &[f64]) -> f64 {
    mean(v) / sd(v)
}

pub fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

pub fn variance(v: &[f64]) -> f64 {

    let mu = mean(v);
    let n = v.len() as f64;

    v.iter()
        .map(|x| (x - mu).powf(2.0))
        .sum::<f64>() / (n - 1.0)

}

pub fn sd(v: &[f64]) -> f64 {
    variance(v).sqrt()
}

