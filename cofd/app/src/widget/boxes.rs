use iced::{
	alignment, event, mouse, Alignment, Background, Color, Element, Event, Length, Point,
	Rectangle, Theme,
};
use iced_native::{
	layout, renderer, text, touch,
	widget::{self, Column, Row, Text, Tree},
	Clipboard, Layout, Shell, Widget,
};

pub struct SheetBoxes<Message, Renderer: text::Renderer>
where
	Renderer: text::Renderer,
	Renderer::Theme: StyleSheet,
{
	value: u8,
	// min: u8,
	max: u8,
	on_click: Vec<Message>,
	size: u16,
	spacing: u16,
	style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer: text::Renderer> SheetBoxes<Message, Renderer>
where
	Message: Clone,
	Renderer: text::Renderer,
	Renderer::Theme: StyleSheet,
{
	/// The default size of a [`Radio`] button.
	pub const DEFAULT_SIZE: u16 = 19;

	/// The default spacing of a [`Radio`] button.
	pub const DEFAULT_SPACING: u16 = 2;

	pub fn new<F>(value: u8, min: u8, max: u8, f: F) -> Self
	where
		F: FnMut(u8) -> Message,
	{
		Self {
			value,
			// min,
			max,
			on_click: (0..max + 1).map(f).collect(),
			size: Self::DEFAULT_SIZE,
			spacing: Self::DEFAULT_SPACING, //15
			style: Default::default(),
		}
	}
}

impl<Message, Renderer: text::Renderer> Widget<Message, Renderer> for SheetBoxes<Message, Renderer>
where
	Message: Clone,
	Renderer: text::Renderer,
	Renderer::Theme: StyleSheet + widget::text::StyleSheet,
{
	fn width(&self) -> iced::Length {
		iced::Length::Fill
	}

	fn height(&self) -> iced::Length {
		iced::Length::Units(15)
	}

	fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
		let mut row = Row::<(), Renderer>::new()
			// .width(Length::Shrink)
			.spacing(self.spacing)
			.align_items(Alignment::Center);

		for _ in 0..self.max {
			row = row.push(
				Row::new()
					.width(Length::Units(self.size))
					.height(Length::Units(self.size)),
			);
		}
		row.layout(renderer, limits)
	}

	fn on_event(
		&mut self,
		_state: &mut Tree,
		event: Event,
		layout: Layout<'_>,
		cursor_position: Point,
		_renderer: &Renderer,
		_clipboard: &mut dyn Clipboard,
		shell: &mut Shell<'_, Message>,
	) -> event::Status {
		match event {
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
			| Event::Touch(touch::Event::FingerPressed { .. }) => {
				for (i, layout) in layout.children().enumerate() {
					if layout.bounds().contains(cursor_position) {
						let i = if (self.value as isize - 1) == i as isize {
							i
						} else {
							i + 1
						};

						// if i + 1 > self.min as usize {
						if let Some(message) = self.on_click.get(i) {
							shell.publish(message.clone());
						}
						// }

						return event::Status::Captured;
					}
				}
			}
			_ => {}
		}

		event::Status::Ignored
	}

	fn mouse_interaction(
		&self,
		_state: &Tree,
		layout: Layout<'_>,
		cursor_position: Point,
		viewport: &Rectangle,
		renderer: &Renderer,
	) -> mouse::Interaction {
		if layout.bounds().contains(cursor_position) {
			mouse::Interaction::Pointer
		} else {
			mouse::Interaction::default()
		}
	}

	fn draw(
		&self,
		state: &Tree,
		renderer: &mut Renderer,
		theme: &Renderer::Theme,
		style: &renderer::Style,
		layout: Layout<'_>,
		cursor_position: Point,
		viewport: &Rectangle,
	) {
		// for i in self.min..self.max {
		let mut mouse_i = None;
		for (i, layout) in layout.children().enumerate() {
			let bounds = layout.bounds();

			if bounds.contains(cursor_position) {
				mouse_i = Some(i);
			}
		}

		for (i, layout) in layout.children().enumerate() {
			let bounds = layout.bounds();

			let custom_style = if mouse_i.is_some_and(|mouse_i| i <= *mouse_i) {
				theme.hovered(self.style)
			} else {
				theme.active(self.style)
			};

			let size = bounds.width;
			let dot_size = size / 2.0;

			renderer.fill_quad(
				renderer::Quad {
					bounds,
					border_radius: size / 2.0,
					border_width: custom_style.border_width,
					border_color: custom_style.border_color,
				},
				custom_style.background,
			);

			if (self.value as isize - 1) >= i as isize {
				renderer.fill_quad(
					renderer::Quad {
						bounds,
						border_radius: dot_size,
						border_width: 0.0,
						border_color: Color::TRANSPARENT,
					},
					custom_style.dot_color,
				);
			}
		}
	}
}

impl<'a, Message, Renderer> From<SheetBoxes<Message, Renderer>> for Element<'a, Message, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'a + text::Renderer,
	Renderer::Theme: StyleSheet + widget::text::StyleSheet,
{
	fn from(radio: SheetBoxes<Message, Renderer>) -> Element<'a, Message, Renderer> {
		Element::new(radio)
	}
}

/// The appearance of a radio button.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
	pub background: Background,
	pub dot_color: Color,
	pub border_width: f32,
	pub border_color: Color,
	pub text_color: Option<Color>,
}

/// A set of rules that dictate the style of a radio button.
pub trait StyleSheet {
	type Style: Default + Copy;

	fn active(&self, style: Self::Style) -> Appearance;

	fn hovered(&self, style: Self::Style) -> Appearance;
}

impl StyleSheet for Theme {
	type Style = ();

	fn active(&self, _style: Self::Style) -> Appearance {
		let palette = self.extended_palette();

		Appearance {
			background: Color::TRANSPARENT.into(),
			dot_color: palette.primary.strong.color,
			border_width: 1.0,
			border_color: palette.primary.strong.color,
			text_color: None,
		}
	}

	fn hovered(&self, style: Self::Style) -> Appearance {
		let active = self.active(style);
		let palette = self.extended_palette();

		Appearance {
			dot_color: palette.primary.strong.color,
			background: palette.primary.weak.color.into(),
			..active
		}
	}
}
