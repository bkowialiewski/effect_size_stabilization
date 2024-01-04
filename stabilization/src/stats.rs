use statrs::distribution::{StudentsT, Continuous};

pub fn p_value(v: &[f64]) -> f64 {

    let t = t_value(v);
    let df = (v.len() - 1) as f64;

    StudentsT::new(0.0, 1.0, df).unwrap().pdf(t)

}

pub fn t_value(v: &[f64]) -> f64 {
    mean(v) / se(v)
}

fn se(v: &[f64]) -> f64 {
    sd(v) / (v.len() as f64).sqrt()
}

pub fn cohen_d(v: &[f64]) -> f64 {
    mean(v) / sd(v)
}

pub fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

pub fn sd(v: &[f64]) -> f64 {
    let mu = mean(v);
    (
        v.iter()
        .map(|x| (x - mu).powf(2.0))
        .sum::<f64>() / (v.len() as f64 - 1.0)
    )
        .sqrt()
}

