use crate::card::Card;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

// Three.js bindings - only loaded in non-test builds
#[cfg(not(test))]
#[wasm_bindgen(module = "/js/rolodex3d.js")]
extern "C" {
    #[wasm_bindgen(js_name = initRolodex)]
    fn init_rolodex_impl(container_id: &str);

    #[wasm_bindgen(js_name = updateCards)]
    fn update_cards_impl(cards_json: &str);

    #[wasm_bindgen(js_name = rotateToCard)]
    fn rotate_to_card_impl(index: i32);

    #[wasm_bindgen(js_name = rotateNext)]
    fn rotate_next_impl();

    #[wasm_bindgen(js_name = rotatePrev)]
    fn rotate_prev_impl();

    #[wasm_bindgen(js_name = setCardClickCallback)]
    fn set_card_click_callback_impl(callback: &Closure<dyn Fn(i32, String)>);

    #[wasm_bindgen(js_name = disposeRolodex)]
    fn dispose_rolodex_impl();
}

// Wrapper functions that call the real implementation or no-op in tests
#[cfg(not(test))]
fn init_rolodex(container_id: &str) {
    init_rolodex_impl(container_id);
}

#[cfg(test)]
fn init_rolodex(_container_id: &str) {
    // No-op in tests
}

#[cfg(not(test))]
fn update_cards(cards_json: &str) {
    update_cards_impl(cards_json);
}

#[cfg(test)]
fn update_cards(_cards_json: &str) {
    // No-op in tests
}

#[cfg(not(test))]
fn rotate_to_card(index: i32) {
    rotate_to_card_impl(index);
}

#[cfg(test)]
fn rotate_to_card(_index: i32) {
    // No-op in tests
}

#[cfg(not(test))]
fn rotate_next() {
    rotate_next_impl();
}

#[cfg(test)]
fn rotate_next() {
    // No-op in tests
}

#[cfg(not(test))]
fn rotate_prev() {
    rotate_prev_impl();
}

#[cfg(test)]
fn rotate_prev() {
    // No-op in tests
}

#[cfg(not(test))]
fn set_card_click_callback(callback: &Closure<dyn Fn(i32, String)>) {
    set_card_click_callback_impl(callback);
}

#[cfg(test)]
fn set_card_click_callback(_callback: &Closure<dyn Fn(i32, String)>) {
    // No-op in tests
}

#[cfg(not(test))]
fn dispose_rolodex() {
    dispose_rolodex_impl();
}

#[cfg(test)]
fn dispose_rolodex() {
    // No-op in tests
}

#[derive(Properties, PartialEq)]
pub struct Rolodex3DProps {
    pub cards: Vec<Card>,
    pub selected_index: Option<usize>,
    pub on_card_click: Callback<String>,
}

#[function_component(Rolodex3D)]
pub fn rolodex_3d(props: &Rolodex3DProps) -> Html {
    let initialized = use_state(|| false);
    let callback_set = use_state(|| false);

    // Initialize Three.js scene once on mount
    {
        let initialized = initialized.clone();
        use_effect_with((), move |_| {
            init_rolodex("rolodex-container");
            initialized.set(true);

            || {
                dispose_rolodex();
            }
        });
    }

    // Set up click callback (only once after initialization)
    {
        let on_card_click = props.on_card_click.clone();
        let callback_set = callback_set.clone();
        let is_initialized = *initialized;

        use_effect_with(is_initialized, move |initialized| {
            if *initialized && !*callback_set {
                let closure = Closure::new(move |_index: i32, card_id: String| {
                    on_card_click.emit(card_id);
                });
                set_card_click_callback(&closure);
                closure.forget();
                callback_set.set(true);
            }
            || ()
        });
    }

    // Update cards when they change
    {
        let cards = props.cards.clone();
        let is_initialized = *initialized;
        use_effect_with(
            (cards.clone(), is_initialized),
            move |(cards, initialized)| {
                if *initialized {
                    if let Ok(json) = serde_json::to_string(cards) {
                        update_cards(&json);
                    }
                }
                || ()
            },
        );
    }

    // Rotate to selected card
    {
        let selected_index = props.selected_index;
        let is_initialized = *initialized;
        use_effect_with(
            (selected_index, is_initialized),
            move |(idx, initialized)| {
                if *initialized {
                    if let Some(index) = idx {
                        rotate_to_card(*index as i32);
                    }
                }
                || ()
            },
        );
    }

    let on_prev = Callback::from(|_| {
        rotate_prev();
    });

    let on_next = Callback::from(|_| {
        rotate_next();
    });

    html! {
        <div class="rolodex-3d-wrapper">
            <div id="rolodex-container" class="rolodex-container"></div>
            <div class="rolodex-controls">
                <button class="btn-nav" onclick={on_prev}>{"Previous"}</button>
                <button class="btn-nav" onclick={on_next}>{"Next"}</button>
            </div>
        </div>
    }
}
