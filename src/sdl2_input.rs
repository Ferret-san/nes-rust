use input::Input;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use button;

// @TODO: Be Configurable
fn keycode_to_button(key: Keycode) -> Option<button::Button> {
	match key {
		// joypad1
		Keycode::Space => Some(button::Button::Start),
		Keycode::S => Some(button::Button::Select),
		Keycode::A => Some(button::Button::Joypad1_A),
		Keycode::B => Some(button::Button::Joypad1_B),
		Keycode::Up => Some(button::Button::Joypad1_Up),
		Keycode::Down => Some(button::Button::Joypad1_Down),
		Keycode::Left => Some(button::Button::Joypad1_Left),
		Keycode::Right => Some(button::Button::Joypad1_Right),
		// joypad2
		Keycode::X => Some(button::Button::Joypad2_A),
		Keycode::Z => Some(button::Button::Joypad2_B),
		Keycode::Num8 => Some(button::Button::Joypad2_Up),
		Keycode::Num2 => Some(button::Button::Joypad2_Down),
		Keycode::Num4 => Some(button::Button::Joypad2_Left),
		Keycode::Num6 => Some(button::Button::Joypad2_Right),
		_ => None
	}
}

pub struct Sdl2Input {
	event_pump: EventPump
}

impl Sdl2Input {
	pub fn new(event_pump: EventPump) -> Self {
		Sdl2Input {
			event_pump: event_pump
		}
	}
}

impl Input for Sdl2Input {
	fn get_input(&mut self) -> Option<(button::Button, button::Event)> {
		match self.event_pump.poll_event() {
			Some(ev) => {
				match ev {
					sdl2::event::Event::KeyDown {
						keycode: Some(key), ..
					} => {
						match keycode_to_button(key) {
							Some(button) => Some((button, button::Event::Press)),
							None => self.get_input()
						}
					},
					sdl2::event::Event::KeyUp {
						keycode: Some(key), ..
					} => {
						match keycode_to_button(key) {
							Some(button) => Some((button, button::Event::Release)),
							None => self.get_input()
						}
					},
					_ => self.get_input()
				}
			},
			None => None
		}
	}

	fn press(&mut self, button: button::Button) {
	}

	fn release(&mut self, button: button::Button) {
	}
}
