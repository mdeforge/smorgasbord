pub mod Util {
    #[allow(dead_code)]
    pub fn lbs_to_ounces(lbs: f32) -> f32 {
        lbs * 16.0
    }

    #[allow(dead_code)]
    pub fn ounces_to_lbs(oz: f32) -> f32 {
        oz / 16.0
    }
}