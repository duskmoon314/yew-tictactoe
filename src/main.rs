use std::fmt::Display;
use yew::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
enum SquareValue {
    X,
    O,
    None,
}

impl Display for SquareValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
            Self::None => write!(f, ""),
        }
    }
}

#[derive(Properties, PartialEq)]
struct SquareProps {
    value: SquareValue,
    on_click: Callback<()>,
}

#[function_component(Square)]
fn square(SquareProps { value, on_click }: &SquareProps) -> Html {
    let on_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| on_click.emit(()))
    };
    html! {
        <button class="square" onclick={on_click}>{ value }</button>
    }
}

#[derive(Properties, PartialEq)]
struct BoardProps {
    squares: [SquareValue; 9],
    on_click: Callback<usize>,
}

#[function_component(Board)]
fn board(BoardProps { squares, on_click }: &BoardProps) -> Html {
    let render_square = |i: usize| {
        let on_click = {
            let on_click = on_click.clone();
            Callback::from(move |_| on_click.emit(i))
        };
        html! {
            <Square value={squares[i]} on_click={on_click}/>
        }
    };

    html! {
        <div>
            <div class="board-row">
                { render_square(0) }
                { render_square(1) }
                { render_square(2) }
            </div>
            <div class="board-row">
                { render_square(3) }
                { render_square(4) }
                { render_square(5) }
            </div>
            <div class="board-row">
                { render_square(6) }
                { render_square(7) }
                { render_square(8) }
            </div>
        </div>
    }
}

#[function_component(TicTacToe)]
fn tictactoe() -> Html {
    let history = use_state_eq(|| vec![[SquareValue::None; 9]]);
    let step = use_state_eq(|| 0);
    let winner = use_state_eq(|| SquareValue::None);

    fn calculate_winner(squares: [SquareValue; 9]) -> SquareValue {
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in lines {
            let [a, b, c] = line;
            if squares[a] != SquareValue::None
                && squares[a] == squares[b]
                && squares[a] == squares[c]
            {
                return squares[a];
            }
        }

        SquareValue::None
    }

    use_effect_with_deps(
        move |(history, step, winner)| {
            let current = history.clone()[*step.clone()];
            winner.clone().set(calculate_winner(current));

            || ()
        },
        (history.clone(), step.clone(), winner.clone()),
    );

    let handle_click = {
        let history = history.clone();
        let step = step.clone();
        let winner = winner.clone();
        Callback::from(move |i: usize| {
            let mut new_history = (*history).clone();
            new_history.truncate(*step + 1);
            let mut current = new_history[*step];
            if *winner != SquareValue::None || current[i] != SquareValue::None {
                return;
            }

            current[i] = if *step % 2 == 0 {
                SquareValue::X
            } else {
                SquareValue::O
            };

            new_history.append(&mut Vec::from([current]));
            history.set(new_history);
            step.set(*step + 1);
        })
    };

    let jump_to = {
        let step = step.clone();
        Callback::from(move |s: usize| step.set(s))
    };

    html! {
        <main class="game">
            <div class="game-board">
                <Board squares={history[*step]} on_click={handle_click}/>
            </div>
            <div class="game-info">
                <div>
                    {match *winner {
                        SquareValue::None => format!("Next player {}", if *step % 2 == 0 {"X"} else {"O"}),
                        w => format!("Winner {}", w)
                    }}
                </div>
                // <button onclick={undo}> { "UNDO" } </button>
                <ol>
                    {
                        (0..history.len())
                        .map(|i| {
                            html! {
                                <li key={i}>
                                    <button onclick={{
                                        let jump_to = jump_to.clone();
                                        Callback::from(move |_| jump_to.emit(i))
                                    }}>
                                        {match i {
                                            0 => "Go to game start".to_string(),
                                            i => format!("Go to move #{}", i)
                                        }}
                                    </button>
                                </li>
                            }
                        })
                        .collect::<Html>()
                    }
                </ol>
            </div>
        </main>
    }
}

fn main() {
    yew::start_app::<TicTacToe>();
}
