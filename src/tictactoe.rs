use std::fmt;
use rand::Rng;

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
    let win: RwSignal<Option<Player>> = create_rw_signal(cx, None);
    provide_context(cx, turn);
    provide_context(cx, set_turn);

    let (counterX, set_counterX) = create_signal(cx, 0);
    let (counterO, set_counterO) = create_signal(cx, 0);

    let win_match = move |p| {
        match p {
            Player::X => {set_counterX.update(|n| {*n = (*n)+1})}
            Player::O => {set_counterO.update(|n| {*n = (*n)+1})}
        }
    };

    create_effect(cx, move |_| {
        if win.get().is_some() {
            win_match(win.get().unwrap());
        }
        log!("X:{} - O:{}", counterX.get(), counterO.get());
    });

    view! { cx, <div class="ttt-board">
        <div class="ttt-info">
            <Counter player=Player::X score=counterX/>
            <div class="ttt-turn">
                {move ||
                    Player::select_player(turn.get()).to_string()
                }
            </div>
            <Counter player=Player::O score=counterO/>
        </div>
        <Grid win=win/>
        <a href="https://github.com/PintaPontus/TicTacToe">
            <div class="ttt-label">
                    <div>
                        <img src="assets/github_logo.svg" alt="GitHub"/>
                    </div>
                    <div>
                        GitHub
                    </div>
            </div>
        </a>
        <span class="material-symbols-outlined ttt-refresh"
            on:click=move |_| { window().location().reload().unwrap(); }
        >
            refresh
        </span>
        </div>
    }
}

#[component]
fn Grid(
    cx: Scope,
    win: RwSignal<Option<Player>>
) -> impl IntoView{

    let turn = use_context::<ReadSignal<bool>>(cx)
        .expect("to have found the getter provided");

    let set_turn = use_context::<WriteSignal<bool>>(cx)
        .expect("to have found the setter provided");

    let grid : RwSignal<[[Option<Player>; GRID_DIM]; GRID_DIM]>= create_rw_signal(cx, [[None; GRID_DIM]; GRID_DIM]);

    let fill_cell = move |c: usize| {
        let actual_player = Player::select_player(turn.get());
        grid.update(|g| g[c/GRID_DIM][c%GRID_DIM] = Some(actual_player));
        let take_value = move |x: usize, y: usize| {
            return grid.get()[x][y].unwrap_or(Player::select_player(!turn.get()));
        };
        let match_players = |pa: Player, pb: Player| {
            match (pa, pb) {
                (Player::X, Player::X) => true,
                (Player::O, Player::O) => true,
                _ => false,
            }
        };

        let on_win = || {
            grid.set([[None; GRID_DIM]; GRID_DIM]);
            win.set(Some(actual_player));
            let coin_flip = rand::thread_rng().gen_range(0..2) >= 1;
            set_turn.set(coin_flip);
        };

        let on_draw = || {
            grid.set([[None; GRID_DIM]; GRID_DIM]);
            let coin_flip = rand::thread_rng().gen_range(0..2) >= 1;
            set_turn.set(coin_flip);
        };

        if (0..GRID_DIM).filter(|i| {
            let cell_value = take_value(c/GRID_DIM,*i);
            return match_players(actual_player, cell_value);
        }).count() == GRID_DIM {
            on_win();
            return;
        }
        if (0..GRID_DIM).filter(|i| {
            let cell_value = take_value(*i,c%GRID_DIM);
            return match_players(actual_player, cell_value);
        }).count() == GRID_DIM {
            on_win();
            return;
        }
        if (0..GRID_DIM).filter(|i| {
            let cell_value = take_value(*i,*i);
            return match_players(actual_player, cell_value);
        }).count() == GRID_DIM {
            on_win();
            return;
        }
        if (0..GRID_DIM).filter(|i| {
            let cell_value = take_value(*i,GRID_DIM-(*i)-1);
            return match_players(actual_player, cell_value);
        }).count() == GRID_DIM {
            on_win();
            return;
        }
        if(0..(GRID_DIM*GRID_DIM)).filter(|i|{
            grid.get()[i/GRID_DIM][i%GRID_DIM].is_some()
        }).count() == (GRID_DIM*GRID_DIM){
            on_draw();
            return;
        }

        set_turn.update(|t| *t=!(*t));
    };

    view! { cx, <div class="ttt-grid">
        {(0..9).into_iter()
        .map(|i| {
            view! {cx,
                <Cell coord=i grid=grid on_fill=move || {fill_cell(i)} />
            }
        })
        .collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn Cell<F>(
    cx: Scope,
    coord: usize,
    grid: RwSignal<[[Option<Player>; GRID_DIM]; GRID_DIM]>,
    on_fill: F
) -> impl IntoView where F: Fn() + 'static{

    view! { cx, <div class="ttt-cell"
        on:click = move |_| {
            if grid.get()[coord/GRID_DIM][coord%GRID_DIM].is_none() {
                on_fill();
            }
        }>
            <div class="ttt-label">
            {move || Player::check_player(grid.get()[coord/GRID_DIM][coord%GRID_DIM])}
            </div>
        </div>
    }
}