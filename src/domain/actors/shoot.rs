use crate::domain::topology::index::TextureIndex;

#[derive(Clone, Copy)]
pub struct AnimationStep {
    duration_in_microseconds: u128,
    texture: TextureIndex,
}

#[derive(Clone, Copy)]
pub struct WeaponConfiguration {
    startup: AnimationStep,
    active: AnimationStep,
    recovery: AnimationStep,
    default: TextureIndex,
    damage: u32,
}

#[derive(Debug, PartialEq)]
pub enum ShootState {
    Startup,
    Active,
    AlreadyHit,
    Recovery,
    Finished,
}

#[derive(Clone, Copy)]
pub struct Weapon {
    configuration: WeaponConfiguration,
    already_hit: bool,
    elapsed_in_microseconds: u128,
}

impl WeaponConfiguration {
    pub fn new(default: TextureIndex, startup: AnimationStep, active: AnimationStep, recovery: AnimationStep, damage: u32) -> Self {
        Self {
            default,
            startup,
            active,
            recovery,
            damage,
        }
    }

    pub fn state(&self, elapsed_in_microseconds: u128) -> ShootState {
        if elapsed_in_microseconds < self.startup.duration_in_microseconds {
            ShootState::Startup
        } else if elapsed_in_microseconds < self.startup.duration_in_microseconds + self.active.duration_in_microseconds {
            ShootState::Active
        } else if elapsed_in_microseconds < self.startup.duration_in_microseconds + self.active.duration_in_microseconds + self.recovery.duration_in_microseconds {
            ShootState::Recovery
        } else {
            ShootState::Finished
        }
    }


    pub fn default(&self) -> TextureIndex {
        self.default
    }
    pub fn startup(&self) -> AnimationStep {
        self.startup
    }
    pub fn active(&self) -> AnimationStep {
        self.active
    }
    pub fn recovery(&self) -> AnimationStep {
        self.recovery
    }
    pub fn damage(&self) -> u32 {
        self.damage
    }
}

impl AnimationStep {
    pub fn new(duration_in_seconds: f32, texture: TextureIndex) -> Self {
        Self {
            duration_in_microseconds: (duration_in_seconds * 1000000.0) as u128,
            texture,
        }
    }

    pub fn duration_in_microseconds(&self) -> u128 {
        self.duration_in_microseconds
    }

    pub fn texture(&self) -> TextureIndex {
        self.texture
    }
}

impl Weapon {
    pub fn new(configuration: WeaponConfiguration) -> Self {
        Self {
            configuration,
            already_hit: false,
            elapsed_in_microseconds: 0,
        }
    }

    pub fn action(&mut self) {
        let current_state = self.state();

        if current_state == ShootState::Finished {
            self.already_hit = false;
            self.elapsed_in_microseconds = 0;
        }
    }

    pub fn notify_hit(&mut self) {
        self.already_hit = true;
    }

    pub fn state(&self) -> ShootState {
        let normal_state = self.configuration.state(self.elapsed_in_microseconds);
        if normal_state == ShootState::Active && self.already_hit {
            ShootState::AlreadyHit
        } else {
            normal_state
        }
    }

    pub fn notify_elapsed(&mut self, microseconds: u128) {
        self.elapsed_in_microseconds += microseconds;
    }


    pub fn configuration(&self) -> WeaponConfiguration {
        self.configuration
    }
}

#[cfg(test)]
mod weapon_test {
    use crate::domain::actors::shoot::{AnimationStep, ShootState, Weapon, WeaponConfiguration};
    use crate::domain::topology::index::TextureIndex;

    #[test]
    fn should_go_to_finished_state() {
        let conf = build_configuration(0.1, 1.0, 1.0);
        let mut weapon = Weapon::new(conf);

        weapon.action();
        weapon.notify_elapsed(100);

        weapon.action();

        assert_eq!(100, weapon.elapsed_in_microseconds);
    }

    #[test]
    fn should_go_to_already_hit_state() {
        let conf = build_configuration(0.1, 100.0, 1.0);

        let mut weapon = Weapon::new(conf);
        weapon.action();
        weapon.notify_elapsed(1000000);
        weapon.notify_hit();

        assert_eq!(weapon.state(), ShootState::AlreadyHit);
    }

    #[test]
    fn already_hit_should_be_reset_after_a_new_action_call() {
        let conf = build_configuration(0.1, 100.0, 1.0);

        let mut weapon = Weapon::new(conf);
        weapon.action();
        weapon.notify_hit();
        weapon.notify_elapsed(1000000000);

        weapon.action();
        weapon.notify_elapsed(1000000);

        assert_eq!(weapon.state(), ShootState::Active);
    }

    fn build_configuration(startup: f32, active: f32, recovery: f32) -> WeaponConfiguration {
        let texture: TextureIndex = TextureIndex::new(0);

        let startup = AnimationStep::new(startup, texture);
        let active = AnimationStep::new(active, texture);
        let recovery = AnimationStep::new(recovery, texture);

        WeaponConfiguration::new(texture, startup, active, recovery, 100)
    }
}

#[cfg(test)]
mod weapon_configuration_test {
    use crate::domain::actors::shoot::{AnimationStep, ShootState, WeaponConfiguration};
    use crate::domain::topology::index::TextureIndex;

    #[test]
    fn default_state_is_startup() {
        let conf = build_configuration(1.0, 1.0, 1.0);

        assert_eq!(ShootState::Startup, conf.state(0));
    }

    #[test]
    fn should_go_to_active_state() {
        let conf = build_configuration(0.1, 1.0, 1.0);

        assert_eq!(ShootState::Active, conf.state(to_microseconds(0.2)));
    }

    #[test]
    fn should_go_to_recovery_state() {
        let conf = build_configuration(0.1, 1.0, 1.0);

        assert_eq!(ShootState::Recovery, conf.state(to_microseconds(1.2)));
    }


    fn build_configuration(startup: f32, active: f32, recovery: f32) -> WeaponConfiguration {
        let texture: TextureIndex = TextureIndex::new(0);

        let startup = AnimationStep::new(startup, texture);
        let active = AnimationStep::new(active, texture);
        let recovery = AnimationStep::new(recovery, texture);

        let conf = WeaponConfiguration::new(texture, startup, active, recovery, 150);
        conf
    }

    fn to_microseconds(seconds: f32) -> u128 {
        (seconds * 1000000.0) as u128
    }
}