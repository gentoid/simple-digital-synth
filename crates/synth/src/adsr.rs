pub struct TimeMs(pub u16);

impl TimeMs {
    pub fn init() -> Self {
        Self(0)
    }
}

pub struct Adsr {
    pub attack: TimeMs,
    pub decay: TimeMs,
    pub sustain_level: f32,
    pub release: TimeMs,
}

#[derive(PartialEq)]
pub enum Phase {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct Envelope {
    config: Adsr,
    sample_rate: f32,
    current_value: f32,
    phase: Phase,
    timer: u32,
    release_start_value: f32,
    attack_samples: u32,
    decay_samples: u32,
    release_samples: u32,
}

impl Envelope {
    pub const fn new(config: Adsr, sample_rate: f32) -> Self {
        let attack_samples = (config.attack.0 as f32 * sample_rate / 1000.0) as u32;
        let decay_samples = (config.decay.0 as f32 * sample_rate / 1000.0) as u32;
        let release_samples = (config.release.0 as f32 * sample_rate / 1000.0) as u32;

        Self {
            config,
            sample_rate,
            current_value: 0.0,
            phase: Phase::Idle,
            timer: 0,
            release_start_value: 0.0,
            attack_samples,
            decay_samples,
            release_samples,
        }
    }

    pub fn note_on(&mut self) {
        self.phase = Phase::Attack;
        self.timer = 0;
    }

    pub fn note_off(&mut self) {
        self.phase = Phase::Release;
        self.timer = 0;
        self.release_start_value = self.current_value;
    }

    pub fn is_active(&self) -> bool {
        self.phase != Phase::Idle
    }

    pub fn next(&mut self) -> f32 {
        match self.phase {
            Phase::Idle => self.current_value = 0.0,
            Phase::Attack => {
                if self.timer >= self.attack_samples - 1 {
                    self.current_value = 1.0;
                    self.phase = Phase::Decay;
                    self.timer = 0;
                } else {
                    self.current_value = (self.timer as f32) / (self.attack_samples as f32);
                }
                self.timer += 1;
            }
            Phase::Decay => {
                if self.timer >= self.decay_samples - 1 {
                    self.current_value = self.config.sustain_level;
                    self.phase = Phase::Sustain;
                    self.timer = 0;
                } else {
                    let t = self.timer as f32 / self.decay_samples as f32;
                    self.current_value = 1.0 - t * (1.0 - self.config.sustain_level);
                }

                self.timer += 1;
            }
            Phase::Sustain => self.current_value = self.config.sustain_level,
            Phase::Release => {
                if self.timer >= self.release_samples - 1 || self.current_value <= 0.001 {
                    self.phase = Phase::Idle;
                    self.timer = 0;
                } else {
                    let t = self.timer as f32 / self.release_samples as f32;
                    self.current_value = self.release_start_value * (1.0 - t);
                }

                self.timer += 1;
            }
        }

        self.current_value.clamp(0.0, 1.0)
    }
}
