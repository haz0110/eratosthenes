const FAC_RAD_TO_DEG: f64 = 57.29578;
const FAC_DEG_TO_RAD: f64 = 0.01745329;

pub fn rad_to_deg(radian: f64) -> f64 {
    radian * FAC_RAD_TO_DEG
}

pub fn deg_to_rad(degree: f64) -> f64 {
    degree * FAC_DEG_TO_RAD
}

