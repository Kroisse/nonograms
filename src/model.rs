use vgtk::{gtk, lib::gio::ApplicationFlags, UpdateAction, VNode};

use crate::{components::Field, game::GameState};

#[derive(Debug)]
pub struct Model {
    game: Option<GameState>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            game: Some(GameState::new((10, 10))),
        }
    }
}

impl vgtk::Component for Model {
    type Message = ();
    type Properties = ();

    fn update(&mut self, _: Self::Message) -> UpdateAction<Self> {
        UpdateAction::None
    }

    fn view(&self) -> VNode<Model> {
        use vgtk::{ext::*, lib::gtk::*};
        gtk! {
            <Application::new_unwrap(Some("se.krois.nonograms"), ApplicationFlags::empty())>
                <Window title="Nonograms">
                    <@Field />
                </Window>
            </Application>
        }
    }
}
