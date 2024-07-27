use std::default::Default;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Clipboard, Shell};
use iced::widget::{text, Column, Row};
use iced::{event, touch, Background, Theme};
use iced::{mouse, Alignment, Border, Color, Element, Length, Rectangle, Size};

pub struct SheetDots<'a, Message, Theme>
where
	Theme: StyleSheet,
{
	value: u16,
	min: u16,
	max: u16,
	on_click: Box<dyn Fn(u16) -> Message + 'a>,
	size: u16,
	spacing: u16,
	style: <Theme as StyleSheet>::Style,
	shape: Shape,
	row_count: Option<u16>,
	axis: Axis,
	width: Length,
}

fn iter<'a>(
	layout: Layout<'a>,
	axis: &Axis,
) -> itertools::Either<impl Iterator<Item = Layout<'a>>, impl Iterator<Item = Layout<'a>>> {
	let iter = layout.children().flat_map(Layout::children);
	if let Axis::Horizontal = axis {
		itertools::Either::Left(iter)
	} else {
		itertools::Either::Right(iter.collect::<Vec<Layout>>().into_iter().rev())
	}
}

impl<'a, Message, Theme> SheetDots<'a, Message, Theme>
where
	Message: Clone,
	Theme: StyleSheet,
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
		tree: &mut widget::Tree,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node {
		let mut col = Column::new().spacing(self.spacing).width(self.width);

		let per_row_count = self.row_count.unwrap_or(self.max);

		let new_row = || match self.axis {
			Axis::Vertical => ColOrRow::Col(
				Column::new()
					.spacing(self.spacing)
					.align_items(Alignment::Center),
			),
			Axis::Horizontal => ColOrRow::Row(
				Row::new()
					.spacing(self.spacing)
					.align_items(Alignment::Center),
			),
		};

		let mut col_or_row: ColOrRow<Message, Theme, Renderer> = new_row();

		for i in 0..self.max {
			col_or_row = col_or_row.push(Row::new().width(self.size).height(self.size));

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
		let mut mouse_i = None;
		for (i, layout) in iter(layout, &self.axis).enumerate() {
			let bounds = layout.bounds();

			if cursor.position_over(bounds).is_some() {
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
					border: Border {
						radius: match self.shape {
							Shape::Dots => (size / 2.0).into(),
							Shape::Boxes => (0.0).into(),
						},
						width: custom_style.border_width,
						color: custom_style.border_color,
					},
					..Default::default()
				},
				custom_style.background,
			);

			if self.value as usize > i {
				renderer.fill_quad(
					renderer::Quad {
						bounds,
						border: Border {
							radius: match self.shape {
								Shape::Dots => dot_size.into(),
								Shape::Boxes => (0.0).into(),
							},
							width: 0.0,
							color: Color::TRANSPARENT,
						},
						..Default::default()
					},
					custom_style.dot_color,
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
				for (i, layout) in iter(layout, &self.axis).enumerate() {
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
			.flat_map(Layout::children)
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

pub enum ColOrRow<'a, Message, Theme, Renderer> {
	Col(Column<'a, Message, Theme, Renderer>),
	Row(Row<'a, Message, Theme, Renderer>),
}

impl<'a, Message, Theme, Renderer> ColOrRow<'a, Message, Theme, Renderer>
where
	Renderer: renderer::Renderer,
{
	pub fn push(self, child: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
		match self {
			ColOrRow::Col(col) => ColOrRow::Col(col.push(child)),
			ColOrRow::Row(row) => ColOrRow::Row(row.push(child)),
		}
	}
}

impl<'a, Message, Theme, Renderer> From<ColOrRow<'a, Message, Theme, Renderer>>
	for Element<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Renderer: 'a + renderer::Renderer,
	Theme: 'static,
{
	fn from(val: ColOrRow<'a, Message, Theme, Renderer>) -> Self {
		match val {
			ColOrRow::Col(col) => col.into(),
			ColOrRow::Row(row) => row.into(),
		}
	}
}
