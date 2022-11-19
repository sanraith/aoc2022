// Useful: https://easings.net/
pub enum EaseType {
    Linear,
    EaseInOutQuad,
    EaseInOutCubic,
}
pub fn ease_progress(x: f32, transition_type: &EaseType) -> f32 {
    match transition_type {
        EaseType::Linear => x,
        EaseType::EaseInOutQuad => {
            if x < 0.5 {
                2.0 * x * x
            } else {
                1.0 - (-2.0 * x + 2.0).powi(2) / 2.0
            }
        }
        EaseType::EaseInOutCubic => {
            if x < 0.5 {
                4.0 * x * x * x
            } else {
                1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
            }
        }
    }
}
