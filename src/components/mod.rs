use ndarray::Array2;
use vgtk::{ext::*, gtk, lib::gtk::*, Callback, UpdateAction, VNode};

use crate::{ext::*, game::Cell};

#[derive(Default)]
pub struct Field {
    props: FieldProps,
    board: Array2<Cell>,
}

#[derive(Clone)]
pub struct FieldProps {
    pub size: (usize, usize),
}

impl Default for FieldProps {
    fn default() -> Self {
        Self { size: (10, 10) }
    }
}

#[derive(Clone, Debug)]
pub enum FieldMessage {
    Toggle((usize, usize), Cell),
}

impl vgtk::Component for Field {
    type Message = FieldMessage;
    type Properties = FieldProps;

    fn create(props: Self::Properties) -> Self {
        Field {
            board: Array2::default(props.size),
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> UpdateAction<Self> {
        self.props = props;
        UpdateAction::Render
    }

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            FieldMessage::Toggle(ix, intent) => {
                use Cell::*;
                let cell = &mut self.board[ix];
                *cell = match (*cell, intent) {
                    (Empty, next) => next,
                    (_, _) => Empty,
                };
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        let (height, width) = self.props.size;
        gtk! {
            <Grid
                margin_start=10
                margin_top=10
                margin_end=10
                margin_bottom=10
            >
                {
                    (0..height).flat_map(|r| (0..width).map(move |c| gtk! {
                        <Box
                            Grid::left={c as i32 + 1}
                            Grid::top={r as i32 + 1}
                        >
                            <@FieldCell
                                value={self.board[[r, c]]}
                                on pressed=|btn| { FieldMessage::Toggle((r, c), match btn {
                                    MouseButton::Primary => Cell::Filled,
                                    MouseButton::Secondary => Cell::Space,
                                }) }
                            />
                        </Box>
                    }))
                }
            </Grid>
        }
    }
}

#[derive(Clone, Default)]
struct FieldCell {
    value: Cell,
    on_pressed: Callback<MouseButton>,
}

#[derive(Clone, Debug)]
enum MouseButton {
    Primary,
    Secondary,
}

impl FieldCell {
    fn label(&self) -> &str {
        match self.value {
            Cell::Empty => "",
            Cell::Space => "x",
            Cell::Filled => "@",
        }
    }
}

impl vgtk::Component for FieldCell {
    type Message = Option<MouseButton>;
    type Properties = Self;

    fn create(props: Self::Properties) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        if let Some(btn) = msg {
            self.on_pressed.send(btn);
        }
        UpdateAction::Render
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Button
                @WidgetExtHelpers::size_request=(40, 40)
                label={self.label()}
                on button_pressed=|_, e| {
                    match e.get_button() {
                        1 => Some(MouseButton::Primary),
                        3 => Some(MouseButton::Secondary),
                        _ => None,
                    }
                }
            />
        }
    }
}
