use std::time::{Duration, Instant};

pub struct Hertz {
    last_time: Instant,
    accumulator: Duration,
    tick_duration: Duration, // Длина одного такта (для 1000 Гц это 1 мс)
    pub total_ticks: u64,    // Сквозной счётчик тактов с момента запуска
}

impl Hertz {
    pub fn new(hz: u32) -> Self {
        Self {
            last_time: Instant::now(),
            accumulator: Duration::ZERO,
            tick_duration: Duration::from_secs(1) / hz, // 1 сек / 1000 = 1 мс
            total_ticks: 0,
        }
    }

    pub fn update(&mut self) -> u64 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_time);
        self.last_time = now;

        self.accumulator += delta;

        let mut new_ticks = 0;

        // Пока в аккумуляторе хватает времени на целый такт — "тикаем"
        while self.accumulator >= self.tick_duration {
            self.accumulator -= self.tick_duration;
            self.total_ticks += 1;
            new_ticks += 1;
        }

        new_ticks
    }
}
