
use crate::app::{Msg, Model};
use seed::*;
use seed::prelude::*;

pub fn view(model: &Model) -> Vec<Node<Msg>> {
	nodes![
        main![
            view_header(),
            view_controls(
                model.simulation_timer_handle.is_some(),
                model.simulation.get_bucket_count()
            ),
            //view_results(),
            div![C!["spacer"]],
            //view_footer(model.prefers_dark_mode)
        ],
        div![
            id!["visualization"],
            // canvas![
            //     el_ref(&model.canvas),
            //     attrs![
            //         At::Width => px(CANVAS_SIZE),
            //         At::Height => px(CANVAS_SIZE),
            //     ],
            // ],
        ]
    ]
}

fn view_header() -> Node<Msg> {
	header![h1!["Population Simulation"], h3!["h3 here"]]
}

fn view_controls(is_playing: bool, bucket_count: u8) -> Node<Msg> {
	div![
        id!["controls"],
        h5!["Configure Population"],
        div![
            C!["horizontal-group"],
            if is_playing {
				empty!()
            } else {
                empty!()
            },
            empty!()
        ],
    ]
}

fn view_footer(prefers_dark_mode: bool) -> Node<Msg> {
	div![
        id!["footer-actions"],
        if prefers_dark_mode {
            empty!()
            //icon_button!["fas fa-sun", "Light Mode", |_| Msg::ToggleDarkMode,]
        } else {
            empty!()
            //icon_button!["fas fa-moon", "Dark Mode", |_| Msg::ToggleDarkMode,]
        },
        a![
            C!["button", "icon-only"],
            attrs![At::Title => "Project structure and event binding code taken from Divya Jain:", At::Href => "https://github.com/divykj"],
            div![id!["profile-photo"]],
        ],
    ]
}

/*
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}
*/