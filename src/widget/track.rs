use std::default::Default;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Clipboard, Shell};
use iced::widget::{text, Column, Row};
use iced::{event, touch, Background, Theme};
use iced::{mouse, Alignment, Border, Color, Element, Length, Rectangle, Size};

use cofd::character::{Damage, Wound};
use iced::border::Radius;

pub struct HealthTrack<'a, Message, Theme>
where
	Theme: StyleSheet,
{
	damage: Damage,
	max: usize,
	per_row_count: Option<usize>,
	on_click: Box<dyn Fn(Wound) -> Message + 'a>,
	size: u16,
	spacing: u16,
	style: <Theme as StyleSheet>::Style,
}

impl<'a, Message, Theme> HealthTrack<'a, Message, Theme>
where
	Message: Clone,
	Theme: StyleSheet,
{
	/// The default size of a [`Radio`] button.
	pub const DEFAULT_SIZE: u16 = 19;

	/// The default spacing of a [`Radio`] button.
	pub const DEFAULT_SPACING: u16 = 2;

	pub fn new<F>(damage: Damage, max: usize, f: F) -> Self
	where
		F: Fn(Wound) -> Message + 'a,
	{
		Self {
			damage,
			max,
			per_row_count: Some(13),
			on_click: Box::new(f),
			size: Self::DEFAULT_SIZE,
			spacing: Self::DEFAULT_SPACING, //15
			style: Default::default(),
		}
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
	for HealthTrack<'a, Message, Theme>
where
	Message: Clone,
	Renderer: renderer::Renderer,
	Theme: StyleSheet + text::StyleSheet,
{
	fn size(&self) -> Size<Length> {
		Size {
			width: Length::Shrink,
			height: Length::Shrink,
		}
	}

	fn layout(
		&self,
		tree: &mut widget::Tree,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node {
		let mut col: Column<Message, Theme, Renderer> =
			Column::new().spacing(self.spacing).width(Length::Shrink);

		let per_row_count = self.per_row_count.unwrap_or(self.max);

		let mut row = Row::new()
			.spacing(self.spacing)
			.align_items(Alignment::Center);

		for i in 0..self.max {
			row = row.push(Row::new().width(self.size).height(self.size));

			if (i + 1) % per_row_count == 0 {
				col = col.push(row);
				row = Row::new()
					.spacing(self.spacing)
					.align_items(Alignment::Center);
			}
		}

		if !row.children().is_empty() {
			col = col.push(row);
		}

		col.layout(tree, renderer, limits)
	}

	fn draw(
		&self,
		_state: &widget::Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		_style: &renderer::Style,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		_viewport: &Rectangle,
	) {
		for (i, layout) in layout.children().flat_map(Layout::children).enumerate() {
			let bounds = layout.bounds();
			let custom_style = theme.active(self.style);

			let wound = self.damage.get_i(i);
			renderer.fill_quad(
				renderer::Quad {
					bounds,
					border: Border {
						radius: 0.into(),
						width: custom_style.border_width,
						color: custom_style.border_color,
					},
					..Default::default()
				},
				// custom_style.background,
				match wound {
					Wound::None => Color::from_rgb(0.0, 1.0, 0.0),
					Wound::Bashing => Color::from_rgb(1.0, 1.0, 0.0),
					Wound::Lethal => Color::from_rgb(1.0, 0.8, 0.0),
					Wound::Aggravated => Color::from_rgb(1.0, 0.0, 0.0),
				},
			);
		}
	}

	fn on_event(
		&mut self,
		_state: &mut widget::Tree,
		event: event::Event,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		_renderer: &Renderer,
		_clipboard: &mut dyn Clipboard,
		shell: &mut Shell<'_, Message>,
		_viewport: &Rectangle,
	) -> event::Status {
		match event {
			event::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
			| event::Event::Touch(touch::Event::FingerPressed { .. }) => {
				for (i, layout) in layout.children().flat_map(Layout::children).enumerate() {
					if let Some(_) = cursor.position_over(layout.bounds()) {
						let wound = self.damage.get_i(i);
						shell.publish((self.on_click)(wound));

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
		_state: &widget::Tree,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		_viewport: &Rectangle,
		_renderer: &Renderer,
	) -> mouse::Interaction {
		if layout
			.children()
			.flat_map(Layout::children)
			.any(|layout| cursor.position_over(layout.bounds()).is_some())
		{
			mouse::Interaction::Pointer
		} else {
			mouse::Interaction::default()
		}
	}
}

impl<'a, Message, Theme, Renderer> From<HealthTrack<'a, Message, Theme>>
	for Element<'a, Message, Theme, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'a + renderer::Renderer,
	Theme: StyleSheet + text::StyleSheet + 'static,
{
	fn from(radio: HealthTrack<'a, Message, Theme>) -> Self {
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
