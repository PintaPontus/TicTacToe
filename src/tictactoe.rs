use std::fmt;

use leptos::*;

const GRID_DIM: usize = 3;

pub enum Player {
    X,
    O,
}

impl Clone for Player{
    fn clone(&self) -> Self {
        match self {
            Player::X => { Player::X }
            Player::O => { Player::O }
        }
    }
}

impl Copy for Player { }

impl fmt::Display for Player{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => { write!(f, "X") }
            Player::O => { write!(f, "O") }
        }
    }
}

impl Player {
    fn select_player(b: bool) -> Player{
        if b {
            Player::X
        }else{
            Player::O
        }
    }

    fn check_player(option: Option<Player>) -> String {
        match option {
            None => { " ".to_string() }
            Some(p) => { p.to_string() }
        }
    }
}

#[component]
fn Counter(cx: Scope,
           player: Player,
           score: ReadSignal<i32>,
) -> impl IntoView {

    view! { cx,  <div class="ttt-cnt">
            <div>
                {player.to_string()}
            </div>
            <div>
                {move || score.get()}
            </div>
        </div>
    }
}

#[component]
pub fn Board(cx: Scope) -> impl IntoView {
    let (turn, set_turn) = create_signal(cx, true);
    provide_context(cx, turn);
    provide_context(cx, set_turn);

    let (counterX, set_counterX) = create_signal(cx, 0);
    let (counterO, set_counterO) = create_signal(cx, 0);

    let score = move |cnt: WriteSignal<i32>| {
        cnt.update(|n| {*n = (*n)+1})
    };

    let win_match = move || {
        match Player::select_player(turn.get()) {
            Player::X => {score(set_counterX);}
            Player::O => {score(set_counterO);}
        }
    };

    view! { cx, <div class="ttt-board">
        <div class="ttt-info">
            <Counter player=Player::X score=counterX/>
            <div class="ttt-turn">
                {move || Player::select_player(turn.get()).to_string() }
            </div>
            <Counter player=Player::O score=counterO/>
        </div>
        <Grid on_win=win_match/>
        </div>
    }
}

#[component]
fn Grid<F>(
    cx: Scope,
    on_win: F
) -> impl IntoView where F: Fn() + 'static{

    let turn = use_context::<ReadSignal<bool>>(cx)
        .expect("to have found the getter provided");

    let set_turn = use_context::<WriteSignal<bool>>(cx)
        .expect("to have found the setter provided");

    let grid : RwSignal<[[Option<Player>; GRID_DIM]; GRID_DIM]>= create_rw_signal(cx, [[None; GRID_DIM]; GRID_DIM]);

    let fill_cell = move |c: usize| {
        let actual_player = Player::select_player(turn.get());
        grid.update(|g| g[c/GRID_DIM][c%GRID_DIM] = Some(actual_player));
        let mut has_won = true;
        (0..GRID_DIM).for_each(|i| {
            let cell_value = grid.get()[c/GRID_DIM][i].unwrap_or(Player::select_player(!turn.get()));
            match (actual_player, cell_value) {
                (Player::X, Player::X) => {},
                (Player::O, Player::O) => {},
                _ => has_won = false,
            }
        });
        if has_won {
            log!("VINTO");
            // (on_win)();
        } else {
            has_won = true;
        }
        (0..GRID_DIM).for_each(|i| {
            let cell_value = grid.get()[i][c%GRID_DIM].unwrap_or(Player::select_player(!turn.get()));
            match (actual_player, cell_value) {
                (Player::X, Player::X) => {},
                (Player::O, Player::O) => {},
                _ => has_won = false,
            }
        });
        if has_won {
            log!("VINTO");
            // (on_win)();
        } else {
            has_won = true;
        }

        set_turn.update(|t| *t=!(*t));
    };

    view! { cx, <div class="ttt-grid">
        {(0..9).into_iter()
        .map(|i| {
            log!("Iter: {}", i);

            view! {cx,
                <Cell on_fill=move || {fill_cell(i); !turn.get()} />
            }
        })
        .collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn Cell<F>(
    cx: Scope,
    on_fill: F
) -> impl IntoView where F: Fn() -> bool + 'static{
    let (symbol, set_symbol) = create_signal::<Option<Player>>(cx, None);

    view! { cx, <div class="ttt-cell"
        on:click = move |_| {
            if symbol.get().is_none() {
                set_symbol.set(Some(Player::select_player(on_fill())));
            }
        }>
            <div class="ttt-cell-label">
            {move || Player::check_player(symbol.get())}
            </div>
        </div>
    }
}