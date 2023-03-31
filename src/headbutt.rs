const MAX_HEADBUTT_MOVEMENT: f32 = 64.0;
const HEADBUTT_SECONDS: f32 = 0.2;
const HEADBUTT_BACK_SECONDS: f32 = 0.8;
const HEADBUTT_REST_SECONDS: f32 = 2.0;

#[derive(Eq, PartialEq)]
pub enum HeadbuttStage {
    None,
    Hitting,
    Retreating,
    Resting,
}

pub struct Headbutt {
    seconds_since_start: f32,
    pub stage: HeadbuttStage,
}

impl Headbutt {
    pub fn new() -> Self {
        Self {
            seconds_since_start: 0.0,
            stage: HeadbuttStage::None,
        }
    }

    pub fn start(&mut self) {
        self.stage = HeadbuttStage::Hitting;
    }

    pub fn update(&mut self, seconds_delta: f32) {
        if self.stage != HeadbuttStage::None {
            self.seconds_since_start += seconds_delta;
        }
        if self.seconds_since_start > HEADBUTT_SECONDS {
            self.stage = HeadbuttStage::Retreating;
        }
        if self.seconds_since_start > (HEADBUTT_SECONDS + HEADBUTT_BACK_SECONDS) {
            self.stage = HeadbuttStage::Resting;
        }
        if self.seconds_since_start
            > (HEADBUTT_SECONDS + HEADBUTT_BACK_SECONDS + HEADBUTT_REST_SECONDS)
        {
            self.stage = HeadbuttStage::None;
            self.seconds_since_start = 0.0;
        }
    }

    pub fn pos(&self) -> f32 {
        if self.seconds_since_start < HEADBUTT_SECONDS {
            MAX_HEADBUTT_MOVEMENT * self.seconds_since_start / HEADBUTT_SECONDS
        } else {
            let seconds_back = self.seconds_since_start - HEADBUTT_SECONDS;
            if seconds_back < HEADBUTT_BACK_SECONDS {
                MAX_HEADBUTT_MOVEMENT * (1.0 - (seconds_back / HEADBUTT_BACK_SECONDS))
            } else {
                0.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_pos() {
        let mut headbutt = Headbutt::new();
        headbutt.start();
        assert_eq!(headbutt.in_progress, true);
        let mut max_pos: f32 = 0.0;
        let delta = 0.01;
        for _ in 0..=(((HEADBUTT_SECONDS + HEADBUTT_BACK_SECONDS) / delta) as i32) {
            headbutt.update(delta);
            max_pos = max_pos.max(headbutt.pos());
        }
        assert_eq!(headbutt.in_progress, false);
        assert_eq!(max_pos, MAX_HEADBUTT_MOVEMENT)
    }

    #[test]
    fn test_min_pos() {
        let mut headbutt = Headbutt::new();
        headbutt.start();
        assert_eq!(headbutt.in_progress, true);
        let mut min_pos: f32 = MAX_HEADBUTT_MOVEMENT;
        let delta = 0.01;
        for _ in 0..=(((HEADBUTT_SECONDS + HEADBUTT_BACK_SECONDS) / delta) as i32) {
            min_pos = min_pos.min(headbutt.pos());
            headbutt.update(0.01);
        }
        assert_eq!(headbutt.in_progress, false);
        assert_eq!(min_pos, 0.0)
    }

    #[test]
    fn test_start() {
        let mut headbutt = Headbutt::new();
        assert_eq!(headbutt.in_progress, false);
        let mut max_pos: f32 = 0.0;
        let delta = 0.01;
        for _ in 0..=(((HEADBUTT_SECONDS + HEADBUTT_BACK_SECONDS) / delta) as i32) {
            headbutt.update(delta);
            max_pos = max_pos.max(headbutt.pos());
        }
        assert_eq!(headbutt.in_progress, false);
        assert_eq!(max_pos, 0.0)
    }
}
