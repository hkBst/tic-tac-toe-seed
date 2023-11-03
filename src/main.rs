use seed::{prelude::*, *};
use tic_tac_toe::*;

fn main() {
    App::start("app", init, update, view);
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Game {
    Game::default()
}

enum Msg {
    Move(FieldName),
}

fn update(msg: Msg, game: &mut Game, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Move(field) => {
            game.act(field);
        }
    }
}

fn view(game: &Game) -> Node<Msg> {
    let board = [Hor::Left, Hor::Mid, Hor::Right].into_iter().flat_map(|h| {
        [Vert::Top, Vert::Mid, Vert::Bottom]
            .into_iter()
            .map(move |v| view_field(game, FieldName { v, h }))
    });

    div![
        C!["game"],
        div![C!["board"], board],
        div![C!["state"], game.state().to_string()]
    ]
}

fn view_field(game: &Game, field: FieldName) -> Node<Msg> {
    let class = match game.get(field).0 {
        Some(Side::X) => "cell side-X",
        Some(Side::O) => "cell side-O",
        None => "cell",
    };

    div![
        C![class],
        ev(Ev::Click, move |_| Msg::Move(field)),
        game.get(field).to_string()
    ]
}
