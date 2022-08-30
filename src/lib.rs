use std::time::Duration;

mod ui;

pub fn wait(seconds: u64) {
    println!("waiting for {seconds} seconds");
    std::thread::sleep(Duration::from_secs(seconds));
}

#[derive(Debug, Clone, Copy)]
pub struct TrafficLight {
    red: bool,
    orange: bool,
    green: bool,
}

impl TrafficLight {
    /// Create a new traffic light with all lights off
    ///
    /// ```
    /// # use traffic_light::*;
    /// let traffic_light = TrafficLight::new();
    /// ```
    pub fn new() -> Self {
        Self {
            red: false,
            orange: false,
            green: false,
        }
    }

    /// Returns `true` if the red light is active, and `false` otherwise
    pub fn is_red_active(&self) -> bool {
        self.red
    }

    /// Returns `true` if the orange light is active, and `false` otherwise
    pub fn is_orange_active(&self) -> bool {
        self.orange
    }

    /// Returns `true` if the green light is active, and `false` otherwise
    pub fn is_green_active(&self) -> bool {
        self.green
    }

    /// Set the red light active or inactive
    ///
    /// ```rust
    /// # use traffic_light::*;
    /// let mut traffic_light = TrafficLight::new();
    /// traffic_light.set_red_active(true);
    /// // red light is now active
    /// traffic_light.set_red_active(false)
    /// // red light is now inactive
    /// ```
    pub fn set_red_active(&mut self, active: bool) {
        self.red = active;
    }

    /// Set the orange light active or inactive
    ///
    /// ```rust
    /// # use traffic_light::*;
    /// let mut traffic_light = TrafficLight::new();
    /// traffic_light.set_orange_active(true);
    /// // orange light is now active
    /// traffic_light.set_orange_active(false)
    /// // orange light is now inactive
    /// ```
    pub fn set_orange_active(&mut self, active: bool) {
        self.orange = active;
    }

    /// Set the green light active or inactive
    ///
    /// ```rust
    /// # use traffic_light::*;
    /// let mut traffic_light = TrafficLight::new();
    /// traffic_light.set_green_active(true);
    /// // green light is now active
    /// traffic_light.set_green_active(false)
    /// // green light is now inactive
    /// ```
    pub fn set_green_active(&mut self, active: bool) {
        self.green = active;
    }

    pub fn print(&self) {
        ui::draw_light(self).unwrap();
    }
}

impl Default for TrafficLight {
    fn default() -> Self {
        Self::new()
    }
}
