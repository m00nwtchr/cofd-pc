use std::cmp::min;

use iced::{
	event, mouse, widget::Column, Alignment, Background, Color, Element, Event, Length, Point,
	Rectangle, Theme,
};
use iced_native::{
	layout, renderer, text, touch,
	widget::{self, Row, Tree},
	Clipboard, Layout, Shell, Widget,
};

pub enum Shape {
	Dots,
	Boxes,
}

pub struct SheetDots<'a, Message, Renderer>
where
	Renderer: text::Renderer,
	Renderer::Theme: StyleSheet,
{
	value: u8,
	min: u8,
	max: u8,
	on_click: Box<dyn Fn(u8) -> Message + 'a>,
	size: u16,
	spacing: u16,
	style: <Renderer::Theme as StyleSheet>::Style,
	shape: Shape,
	row_count: Option<u8>,
}

impl<'a, Message, Renderer> SheetDots<'a, Message, Renderer>
where
	Message: Clone,
	Renderer: text::Renderer,
	Renderer::Theme: StyleSheet,
{
	/// The default size of a [`Radio`] button.
	pub const DEFAULT_SIZE: u16 = 19;

	/// The default spacing of a [`Radio`] button.
	pub const DEFAULT_SPACING: u16 = 2;

	pub fn new<F>(value: u8, min: u8, max: u8, shape: Shape, row_count: Option<u8>, f: F) -> Self
	where
		F: Fn(u8) -> Message + 'a,
	{
		Self {
			value,
			min,
			max,
			on_click: Box::new(f),
			size: Self::DEFAULT_SIZE,
			spacing: Self::DEFAULT_SPACING, //15
			style: Default::default(),
			shape,
			row_count,
		}
	}
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for SheetDots<'a, Message, Renderer>
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
		let mut col = Column::<(), Renderer>::new().spacing(self.spacing);

		let per_row_count = self.row_count.unwrap_or(self.max);
		let row_count: i32 = (f32::from(self.max) / f32::from(per_row_count)).ceil() as i32;

		let mut count = 0;
		for _ in 0..row_count {
			let mut row = Row::<(), Renderer>::new()
				// .width(Length::Shrink)
				.spacing(self.spacing)
				.align_items(Alignment::Center);

			let ii = min(self.max - count, per_row_count);

			for _ in 0..ii {
				count += 1;
				row = row.push(
					Row::new()
						.width(Length::Units(self.size))
						.height(Length::Units(self.size)),
				);
			}
			col = col.push(row);
		}

		col.layout(renderer, limits)
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
				for (i, layout) in layout.children().flat_map(Layout::children).enumerate() {
					if layout.bounds().contains(cursor_position) {
						let i = if self.value as usize == i + 1 {
							i
						} else {
							i + 1
						};

						if i + 1 > self.min as usize {
							shell.publish((self.on_click)(i as u8));
						}

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
		_viewport: &Rectangle,
		_renderer: &Renderer,
	) -> mouse::Interaction {
		if layout.bounds().contains(cursor_position) {
			mouse::Interaction::Pointer
		} else {
			mouse::Interaction::default()
		}
	}

	fn draw(
		&self,
		_state: &Tree,
		renderer: &mut Renderer,
		theme: &Renderer::Theme,
		_style: &renderer::Style,
		layout: Layout<'_>,
		cursor_position: Point,
		_viewport: &Rectangle,
	) {
		// for i in self.min..self.max {
		let mut mouse_i = None;
		for (i, layout) in layout.children().flat_map(Layout::children).enumerate() {
			let bounds = layout.bounds();

			if bounds.contains(cursor_position) {
				mouse_i = Some(i);
			}
		}

		for (i, layout) in layout.children().flat_map(Layout::children).enumerate() {
			let bounds = layout.bounds();

			let custom_style = if mouse_i.is_some_and(|mouse_i| i <= mouse_i) {
				theme.hovered(self.style)
			} else {
				theme.active(self.style)
			};

			let size = bounds.width;
			let dot_size = size / 2.0;

			renderer.fill_quad(
				renderer::Quad {
					bounds,
					border_radius: match self.shape {
						Shape::Dots => size / 2.0,
						Shape::Boxes => 0.0,
					},
					border_width: custom_style.border_width,
					border_color: custom_style.border_color,
				},
				custom_style.background,
			);

			if self.value as usize > i {
				renderer.fill_quad(
					renderer::Quad {
						bounds,
						border_radius: match self.shape {
							Shape::Dots => dot_size,
							Shape::Boxes => 0.0,
						},
						border_width: 0.0,
						border_color: Color::TRANSPARENT,
					},
					custom_style.dot_color,
				);
			}
		}
	}
}

impl<'a, Message, Renderer> From<SheetDots<'a, Message, Renderer>>
	for Element<'a, Message, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'a + text::Renderer,
	Renderer::Theme: StyleSheet + widget::text::StyleSheet,
{
	fn from(radio: SheetDots<'a, Message, Renderer>) -> Self {
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
