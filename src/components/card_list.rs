use crate::card::Card;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardListProps {
    pub cards: Vec<Card>,
    pub on_edit: Callback<Card>,
    pub on_delete: Callback<String>,
    pub on_select: Callback<usize>,
    pub selected_index: Option<usize>,
}

#[function_component(CardList)]
pub fn card_list(props: &CardListProps) -> Html {
    let list_ref = use_node_ref();

    // Scroll to selected card when selected_index changes
    {
        let list_ref = list_ref.clone();
        let selected_index = props.selected_index;
        use_effect_with(selected_index, move |idx| {
            if let Some(index) = idx {
                if let Some(list_el) = list_ref.cast::<Element>() {
                    if let Ok(children) = list_el.query_selector_all(".card-item") {
                        if let Some(child) = children.get(*index as u32) {
                            if let Ok(element) = child.dyn_into::<Element>() {
                                element.scroll_into_view_with_bool(true);
                            }
                        }
                    }
                }
            }
            || ()
        });
    }

    html! {
        <div class="card-list" ref={list_ref}>
            {props.cards.iter().enumerate().map(|(index, card)| {
                let on_edit = {
                    let on_edit = props.on_edit.clone();
                    let card = card.clone();
                    Callback::from(move |_| on_edit.emit(card.clone()))
                };

                let on_delete = {
                    let on_delete = props.on_delete.clone();
                    let id = card.id.clone();
                    Callback::from(move |_| on_delete.emit(id.clone()))
                };

                let on_click = {
                    let on_select = props.on_select.clone();
                    Callback::from(move |_| on_select.emit(index))
                };

                let is_selected = props.selected_index == Some(index);
                let class = if is_selected { "card-item selected" } else { "card-item" };

                html! {
                    <div class={class} onclick={on_click} key={card.id.clone()}>
                        <div class="card-content">
                            <h3 class="card-name">{&card.name}</h3>
                            {if !card.company.is_empty() {
                                html! { <p class="card-company">{&card.company}</p> }
                            } else {
                                html! {}
                            }}
                            {if !card.email.is_empty() {
                                html! { <p class="card-email">{&card.email}</p> }
                            } else {
                                html! {}
                            }}
                            {if !card.phone.is_empty() {
                                html! { <p class="card-phone">{&card.phone}</p> }
                            } else {
                                html! {}
                            }}
                        </div>
                        <div class="card-actions">
                            <button class="btn-edit" onclick={on_edit}>{"Edit"}</button>
                            <button class="btn-delete" onclick={on_delete}>{"Delete"}</button>
                        </div>
                    </div>
                }
            }).collect::<Html>()}

            {if props.cards.is_empty() {
                html! {
                    <div class="empty-state">
                        <p>{"No cards found. Add your first contact!"}</p>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
