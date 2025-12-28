use crate::card::Card;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardFormProps {
    pub card: Option<Card>,
    pub on_save: Callback<Card>,
    pub on_cancel: Callback<()>,
    #[prop_or_default]
    pub on_delete: Option<Callback<String>>,
}

#[function_component(CardForm)]
pub fn card_form(props: &CardFormProps) -> Html {
    let card = props.card.clone().unwrap_or_else(Card::empty);

    let name = use_state(|| card.name.clone());
    let email = use_state(|| card.email.clone());
    let phone = use_state(|| card.phone.clone());
    let company = use_state(|| card.company.clone());
    let notes = use_state(|| card.notes.clone());

    let card_id = card.id.clone();

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_phone_change = {
        let phone = phone.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            phone.set(input.value());
        })
    };

    let on_company_change = {
        let company = company.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            company.set(input.value());
        })
    };

    let on_notes_change = {
        let notes = notes.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            notes.set(input.value());
        })
    };

    let is_editing = props.card.is_some();

    let on_submit = {
        let on_save = props.on_save.clone();
        let name = name.clone();
        let email = email.clone();
        let phone = phone.clone();
        let company = company.clone();
        let notes = notes.clone();
        let card_id = card_id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let card = if is_editing {
                Card {
                    id: card_id.clone(),
                    name: (*name).clone(),
                    email: (*email).clone(),
                    phone: (*phone).clone(),
                    company: (*company).clone(),
                    notes: (*notes).clone(),
                }
            } else {
                Card::new(
                    (*name).clone(),
                    (*email).clone(),
                    (*phone).clone(),
                    (*company).clone(),
                    (*notes).clone(),
                )
            };
            on_save.emit(card);
        })
    };

    let on_cancel_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            on_cancel.emit(());
        })
    };

    let on_delete_click = {
        let on_delete = props.on_delete.clone();
        let card_id = card_id.clone();
        Callback::from(move |_| {
            if let Some(ref callback) = on_delete {
                callback.emit(card_id.clone());
            }
        })
    };

    let title = if is_editing {
        "Edit Card"
    } else {
        "Add New Card"
    };

    let show_delete = is_editing && props.on_delete.is_some();

    html! {
        <div class="card-form-overlay">
            <form class="card-form" onsubmit={on_submit}>
                <h2>{title}</h2>

                <div class="form-group">
                    <label for="name">{"Name"}</label>
                    <input
                        type="text"
                        id="name"
                        value={(*name).clone()}
                        oninput={on_name_change}
                        required=true
                    />
                </div>

                <div class="form-group">
                    <label for="email">{"Email"}</label>
                    <input
                        type="email"
                        id="email"
                        value={(*email).clone()}
                        oninput={on_email_change}
                    />
                </div>

                <div class="form-group">
                    <label for="phone">{"Phone"}</label>
                    <input
                        type="tel"
                        id="phone"
                        value={(*phone).clone()}
                        oninput={on_phone_change}
                    />
                </div>

                <div class="form-group">
                    <label for="company">{"Company"}</label>
                    <input
                        type="text"
                        id="company"
                        value={(*company).clone()}
                        oninput={on_company_change}
                    />
                </div>

                <div class="form-group">
                    <label for="notes">{"Notes"}</label>
                    <textarea
                        id="notes"
                        value={(*notes).clone()}
                        oninput={on_notes_change}
                        rows="3"
                    />
                </div>

                <div class="form-actions">
                    <button type="submit" class="btn-primary">{"Save"}</button>
                    <button type="button" class="btn-secondary" onclick={on_cancel_click}>{"Cancel"}</button>
                    {if show_delete {
                        html! {
                            <button type="button" class="btn-delete" onclick={on_delete_click}>{"Delete"}</button>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </form>
        </div>
    }
}
