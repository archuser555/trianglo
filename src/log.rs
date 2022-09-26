//Here Im Using My OWN Algorithm To Calculate Log, Just Kidding, I Found It On A Website...
pub fn log2n(n: f32) -> f32 {//This Is Log Base 2 Of n
    if n > 1.0 {1.0 + log2n(n / 2.0)}
    else {0.0}
}

pub fn log(n: f32, r: f32) -> f32 {//And This Is Log Base r Of n
    if n > r - 1.0 {1.0 + log(n / r, r)}
    else {0.0}
}