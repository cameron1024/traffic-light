# traffic-light
Teaching library for controlling a virtual traffic light

```rust
use traffic_light::*;

fn main() {
  let mut traffic_light = TrafficLight::new();
  traffic_light.set_red_active(true);
  traffic_light.print();
}
```
