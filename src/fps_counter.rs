use std::time::{UNIX_EPOCH, SystemTime};


#[derive(Debug)] #[allow(dead_code)]
pub struct FpsCounter {
    frames: u32,
    last_time: u64,
}

impl Default for FpsCounter {
    fn default() -> FpsCounter {
        FpsCounter::new()
    }
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            frames: 0,
            last_time: 0
        }
    }
    #[allow(dead_code)]
    pub fn tick(&mut self) -> f64 {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let now = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
        if now - self.last_time as u64 >= 1000 {
            self.frames = 0;
        }
        let temp_last_time = self.last_time;
        self.last_time = now;
        self.frames += 1;
        1000. / (now - temp_last_time) as f64
    }
}