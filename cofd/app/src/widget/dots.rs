use std::cmp::min;

use iced::{
	event, mouse, widget::Column, Alignment, Background, Color, Element, Event, Length, Point,
	Rectangle, Theme,
};
use iced_native::{
	layout, renderer, text, touch,
	widget::{self, Column as Col, Row, Tree},
	Clipboard, Element as El, Layout, Shell, Widget,
};

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

pub enum ColOrRow<'a, Message, Renderer> {
	Col(Col<'a, Message, Renderer>),
	Row(Row<'a, Message, Renderer>),
}

impl<'a, Message, Renderer> ColOrRow<'a, Message, Renderer> {
	pub fn push(self, child: impl Into<Element<'a, Message, Renderer>>) -> Self {
		match self {
			ColOrRow::Col(col) => ColOrRow::Col(col.push(child)),
			ColOrRow::Row(row) => ColOrRow::Row(row.push(child)),
		}
	}
}

impl<'a, Message, Renderer> From<ColOrRow<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
	Message: 'a,
	Renderer: 'a + text::Renderer,
{
	fn from(val: ColOrRow<'a, Message, Renderer>) -> Self {
		match val {
			ColOrRow::Col(col) => col.into(),
			ColOrRow::Row(row) => row.into(),
		}
	}
}

pub struct SheetDots<'a, Message, Renderer>
where
	Renderer: text::Renderer,
	Renderer::Theme: StyleSheet,
{
	value: u16,
	min: u16,
	max: u16,
	on_click: Box<dyn Fn(u16) -> Message + 'a>,
	size: u16,
	spacing: u16,
	style: <Renderer::Theme as StyleSheet>::Style,
	shape: Shape,
	row_count: Option<u16>,
	axis: Axis,
	width: Length,
}

fn iter<'a>(
	layout: Layout<'a>,
	axis: &Axis,
) -> itertools::Either<
	impl Iterator<Item = iced_native::Layout<'a>>,
	impl Iterator<Item = iced_native::Layout<'a>>,
> {
	let iter = layout.children().flat_map(Layout::children);
	if let Axis::Horizontal = axis {
		itertools::Either::Left(iter)
	} else {
		itertools::Either::Right(iter.collect::<Vec<iced_native::Layout>>().into_iter().rev())
	}
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
			spacing: Self::DEFAULT_SPACING, //15
			style: Default::default(),
			shape,
			row_count,
			axis: Default::default(),
			width: Length::Shrink,
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

	pub fn spacing(mut self, spacing: u16) -> Self {
		self.spacing = spacing;
		self
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
		self.width
	}

	fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
		let mut col = Column::<(), Renderer>::new()
			.spacing(self.spacing)
			.width(self.width);

		let per_row_count = self.row_count.unwrap_or(self.max);

		let new_row = || match self.axis {
			Axis::Vertical => ColOrRow::Col(
				Col::<(), Renderer>::new()
					.spacing(self.spacing)
					.align_items(Alignment::Center),
			),
			Axis::Horizontal => ColOrRow::Row(
				Row::<(), Renderer>::new()
					.spacing(self.spacing)
					.align_items(Alignment::Center),
			),
		};

		let mut col_or_row: ColOrRow<(), Renderer> = new_row();

		for i in 0..self.max {
			col_or_row = col_or_row.push(
				Row::new()
					.width(Length::Units(self.size))
					.height(Length::Units(self.size)),
			);

			if (i + 1) % per_row_count == 0 {
				col = col.push(col_or_row);
				col_or_row = new_row();
			}
		}

		if match &col_or_row {
			ColOrRow::Col(e) => e.children().len(),
			ColOrRow::Row(e) => e.children().len(),
		} > 0
		{
			col = col.push(col_or_row);
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
				for (i, layout) in iter(layout, &self.axis).enumerate() {
					if layout.bounds().contains(cursor_position) {
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
		let mut mouse_i = None;
		for (i, layout) in iter(layout, &self.axis).enumerate() {
			let bounds = layout.bounds();

			if bounds.contains(cursor_position) {
				mouse_i = Some(i);
			}
		}

		for (i, layout) in iter(layout, &self.axis).enumerate() {
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
