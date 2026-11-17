use std::time::{Duration, Instant};


// (I’ve noticed that writing code is actually easier when you include comments)

// This structure is necessary to establish the concept of "ticks" in Ferrum
// It’s essentially a requirement for implementing timers and the notion of interrupts 

// I Though suspect Crossterm might already support these—meaning 
// I’m just reinventing the wheel, or rather, the lawnmower—but who cares?)

// This structure handles the concept of "ticks" (clock cycles). 
// In essence, its logic boils down to tracking the current tick count and anticipating potential bottlenecks. 
// We cannot simply process the consequences of a full buffer of ticks all at once. 
// Instead, we must implement a loop that iterates through the accumulated ticks—because an operating system can occasionally lag 
// Causing a large number of ticks to arrive simultaneously
// And we are required to register every single one of them.
pub struct Hertz {
    // This shi- it's needable for multicycle 
    // In the event of a system lag, you simply need to process the loop many times over
    // It’s all quite straightforward.
    accumulator: Duration, 
    // It's just time when we start cycle
    // The loop logic is as follows:
    // Start the loop and increment a global counter by 1.
    // Every 1000 cycles, register one "tick" and subtract those 1000 cycles.
    // 1000 is the maximum threshold; this constitutes the first phase of the loop.

    // In the second phase, whenever a "tick" is registered, a function is executed—it’s that simple!

    // The key issue is that, during a lag spike, the counter might suddenly accumulate far more than 1000 cycles.
    // Therefore, we cannot simply impose a hard limit!
    // Instead, we keep the remainder and check if the cycle count exceeds 1000. 
    // If it does, we register a "tick," subtract 1000 from the total, and retain the remainder.
    last_update: Instant, 
    // It's be a clock_frequence, formula 
    // | freq (1000 ms) > time_start_cycle - present_time |
    // | freq (1000 ms) < time_start_cycle - present_time | start cycle and present_tick++
    // What kind of shit am I writing? I should think twice.
    tick_duration: Duration,       
    pub present_tick: u64,
}

impl Hertz {
    pub fn new(hz: u32) -> Self {
        Self {
            // Math be like
            // Accumulator it's a started instant 
            // When accu have 1000+ millis I do minus freq and present_tick++

            accumulator: Duration::ZERO,
            last_update: Instant::now(),
            // freq is: 1000 / time, not base hz
            // Something like: freq be is one 
            tick_duration: Duration::from_secs(1) / hz, 
            present_tick: 0,
        }
    }

    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update);

        self.last_update = now;
        self.accumulator += delta;

        let mut new_ticks: u64 = 0;

        while self.accumulator >= self.tick_duration {
            self.present_tick += 1;
            // I need create a local integer because I need to take NEW ticks
            // I comment it's because my stupid brain not think about it early.
            new_ticks += 1; 
            self.accumulator -= self.tick_duration; 
        }
        if new_ticks > 0 {
            return true
        }
        false
    }  
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_real_time_single_tick() {
        // 10 Гц = 1 тик каждые 100 мс
        let mut timer = Hertz::new(10);

        // Спим 40 мс — тик еще точно не должен пройти
        thread::sleep(Duration::from_millis(40));
        assert_eq!(timer.update(), false);

        // Спим еще 75 мс (в сумме ~115 мс, с запасом перекрываем 100 мс)
        thread::sleep(Duration::from_millis(75));
        assert_eq!(timer.update(), true);
        assert_eq!(timer.present_tick, 1);
    }

    #[test]
    fn test_real_time_lag_spike() {
        // 10 Гц = 1 тик каждые 100 мс
        let mut timer = Hertz::new(10);

        // Имитируем лаг системы на ~350 мс
        thread::sleep(Duration::from_millis(350));

        // Должно накопиться 3 тика (300 мс), и ~50 мс остаться в аккумуляторе
        let ticks = timer.update();
        assert_eq!(ticks, true);
        assert_eq!(timer.present_tick, 3);

        // Проверяем, что остаток не сгорел: спим еще ~65 мс (50 + 65 > 100 мс)
        thread::sleep(Duration::from_millis(65));
        assert_eq!(timer.update(), true);
        assert_eq!(timer.present_tick, 4);
    }

    #[test]
    #[should_panic]
    fn test_zero_hz_panics() {
        let _timer = Hertz::new(0);
    }
}