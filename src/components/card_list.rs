use crate::card::Card;
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
    html! {
        <div class="card-list">
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
