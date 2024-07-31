use std::array;
use std::default::Default;

use iced::advanced::layout::{self, Layout, Limits, Node};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Clipboard, Shell};
use iced::widget::{text, Column, Row};
use iced::{event, touch, Background, Padding, Point, Theme};
use iced::{mouse, Alignment, Border, Color, Element, Length, Rectangle, Size};

pub struct SheetDots<'a, Message, Theme = iced::Theme>
where
	Theme: StyleSheet,
{
	value: u16,
	min: u16,
	max: u16,
	on_click: Box<dyn Fn(u16) -> Message + 'a>,
	size: f32,
	spacing: f32,
	style: <Theme as StyleSheet>::Style,
	shape: Shape,
	row_count: Option<u16>,
	axis: Axis,
	width: Length,
	// child: Column<'a, Message, Theme>,
}

impl<'a, Message, Theme> SheetDots<'a, Message, Theme>
where
	Message: Clone,
	Theme: StyleSheet,
{
	/// The default size of a [`Radio`] button.
	pub const DEFAULT_SIZE: f32 = 19.0;

	/// The default spacing of a [`Radio`] button.
	pub const DEFAULT_SPACING: f32 = 2.0;

	pub fn new<F>(
		value: u16,
		min: u16,
		max: u16,
		shape: Shape,
		row_count: Option<u16>,
		f: F,
	) -> Self
	where
		F: Fn(u16) -> Message + 'a,
	{
		Self {
			value,
			min,
			max,
			on_click: Box::new(f),
			size: Self::DEFAULT_SIZE,
			spacing: Self::DEFAULT_SPACING,
			style: Default::default(),
			shape,
			row_count,
			axis: Axis::Horizontal,
			width: Length::Shrink,
			//
			// child:
		}
	}

	pub fn axis(mut self, axis: Axis) -> Self {
		self.axis = axis;
		self
	}

	pub fn width(mut self, width: Length) -> Self {
		self.width = width;
		self
	}

	pub fn spacing(mut self, spacing: f32) -> Self {
		self.spacing = spacing;
		self
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
	for SheetDots<'a, Message, Theme>
where
	Message: Clone,
	Renderer: renderer::Renderer,
	Theme: StyleSheet + text::StyleSheet + 'static,
{
	fn size(&self) -> Size<Length> {
		Size {
			width: self.width,
			height: Length::Shrink,
		}
	}

	fn layout(
		&self,
		_tree: &mut widget::Tree,
		_renderer: &Renderer,
		limits: &Limits,
	) -> layout::Node {
		let limits = limits.width(self.width);

		let is_vertical = matches!(self.axis, Axis::Vertical);
		let per_row_count = self.row_count.unwrap_or(self.max);
		let size = Size::new(self.size, self.size);

		let mut nodes = Vec::new();

		for i in 0..self.max {
			let row = i / per_row_count;
			let col = i % per_row_count;

			let (xi, yi) = if is_vertical {
				(f32::from(row), f32::from(col))
			} else {
				(f32::from(col), f32::from(row))
			};

			let x = (self.size + self.spacing) * xi;
			let y = (self.size + self.spacing) * yi;

			nodes.push(Node::new(size).move_to(Point::new(x, y)));
		}

		let num_rows = if is_vertical {
			per_row_count
		} else {
			(self.max + per_row_count - 1) / per_row_count
		};
		let num_cols = if is_vertical {
			(self.max + per_row_count - 1) / per_row_count
		} else {
			per_row_count
		};

		let total_width = self.size * f32::from(num_cols) + self.spacing * f32::from(num_cols - 1);
		let total_height = self.size * f32::from(num_rows) + self.spacing * f32::from(num_rows - 1);

		if is_vertical {
			nodes.reverse();
		}

		Node::with_children(
			limits.resolve(
				self.width,
				Length::Shrink,
				Size::new(total_width, total_height),
			),
			nodes,
		)
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
		let layout_bounds = layout.bounds();
		let mut mouse_i = None;
		for (i, layout) in layout.children().enumerate() {
			let bounds = layout.bounds();

			if cursor.position_over(bounds).is_some() {
				mouse_i = Some(i);
			}
		}

		let active = theme.active(self.style);
		let hovered = theme.hovered(self.style);

		for (i, dot_layout) in layout.children().enumerate() {
			let bounds = dot_layout.bounds();

			let style = if mouse_i.is_some_and(|mouse_i| i <= mouse_i) {
				hovered
			} else {
				active
			};

			let size = bounds.width;
			let dot_size = size / 2.0;
			if bounds.intersects(&layout_bounds) {
				renderer.fill_quad(
					renderer::Quad {
						bounds,
						border: Border {
							radius: match self.shape {
								Shape::Dots => dot_size.into(),
								Shape::Boxes => 0.into(),
							},
							width: style.border_width,
							color: style.border_color,
						},
						..Default::default()
					},
					if self.value as usize > i {
						style.dot_color.into()
					} else {
						style.background
					},
				);
			}
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
				for (i, layout) in layout.children().enumerate() {
					if cursor.position_over(layout.bounds()).is_some() {
						let i = if self.value as usize == i + 1 {
							i
						} else {
							i + 1
						};

						if i + 1 > self.min as usize {
							shell.publish((self.on_click)(i as u16));
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
		_state: &widget::Tree,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		_viewport: &Rectangle,
		_renderer: &Renderer,
	) -> mouse::Interaction {
		if layout
			.children()
			.any(|layout| cursor.position_over(layout.bounds()).is_some())
		{
			mouse::Interaction::Pointer
		} else {
			mouse::Interaction::default()
		}
	}
}

impl<'a, Message, Theme, Renderer> From<SheetDots<'a, Message, Theme>>
	for Element<'a, Message, Theme, Renderer>
where
	Message: 'a + Clone,
	Renderer: renderer::Renderer,
	Theme: StyleSheet + text::StyleSheet + 'static,
{
	fn from(radio: SheetDots<'a, Message, Theme>) -> Self {
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

pub enum Shape {
	Dots,
	Boxes,
}

#[derive(Default)]
pub enum Axis {
	Vertical,
	#[default]
	Horizontal,
}
