pub fn strict_stabilization(criterion_stop: usize, criterion_deviance: f64, v: &[f64]) -> bool {

    let mut cnt = 0;
    let n = v.len();

    let start_value = n - criterion_stop - 1;

    for i in (start_value + 1)..n {

        if (v[start_value] - v[i]).abs()  < criterion_deviance {
            cnt += 1;
        }

    }

    cnt == criterion_stop

}

pub fn absolute_stabilization(criterion_stop: usize, criterion_deviance: f64, v: &[f64]) -> bool {

    let mut cnt = 0;
    let n = v.len();

    for i in (n - criterion_stop - 1)..(n-1) {

        if (v[i] - v[i+1]).abs() < criterion_deviance {
            cnt += 1;
        } else {
            break;
        } 
    } 

    cnt == criterion_stop 

}

pub fn seq(min: f64, max: f64, step: f64) -> Vec<f64> {

    let mut v = vec![0.0; ((max - min) / step + 1.0).round() as usize];

    for i in 0..v.len() {
        v[i] = min + step * i as f64;
    }

    v

}
