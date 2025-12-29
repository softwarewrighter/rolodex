use crate::card::Card;
use crate::components::card_form::CardForm;
use crate::components::card_list::CardList;
use crate::components::rolodex_3d::Rolodex3D;
use crate::components::search_bar::SearchBar;
use crate::storage::CardStorage;
use crate::test_data::generate_fake_cards;
use yew::prelude::*;

pub enum AppMsg {
    LoadCards,
    AddCard,
    EditCard(Card),
    SelectCardById(String),
    SaveCard(Card),
    DeleteCard(String),
    DeleteCardFromForm(String),
    ClearAll,
    PopulateTestData,
    CancelForm,
    Search(String),
    SelectCard(usize),
}

pub struct App {
    cards: Vec<Card>,
    filtered_cards: Vec<Card>,
    editing_card: Option<Card>,
    show_form: bool,
    search_query: String,
    selected_index: Option<usize>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(AppMsg::LoadCards);
        Self {
            cards: Vec::new(),
            filtered_cards: Vec::new(),
            editing_card: None,
            show_form: false,
            search_query: String::new(),
            selected_index: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::LoadCards => {
                self.cards = CardStorage::load_cards();
                self.update_filtered_cards();
                true
            }
            AppMsg::AddCard => {
                self.editing_card = None;
                self.show_form = true;
                true
            }
            AppMsg::EditCard(card) => {
                self.editing_card = Some(card);
                self.show_form = true;
                true
            }
            AppMsg::SelectCardById(id) => {
                // Find the card in filtered_cards, select it, and open editor
                if let Some(index) = self.filtered_cards.iter().position(|c| c.id == id) {
                    self.selected_index = Some(index);
                    // Also open the editor
                    if let Some(card) = self.filtered_cards.get(index) {
                        self.editing_card = Some(card.clone());
                        self.show_form = true;
                    }
                }
                true
            }
            AppMsg::SaveCard(card) => {
                if self.editing_card.is_some() {
                    if let Ok(cards) = CardStorage::update_card(card) {
                        self.cards = cards;
                    }
                } else if let Ok(cards) = CardStorage::add_card(card) {
                    self.cards = cards;
                }
                self.show_form = false;
                self.editing_card = None;
                self.update_filtered_cards();
                true
            }
            AppMsg::DeleteCard(id) => {
                if let Ok(cards) = CardStorage::delete_card(&id) {
                    self.cards = cards;
                    self.update_filtered_cards();
                    self.selected_index = None;
                }
                true
            }
            AppMsg::DeleteCardFromForm(id) => {
                if let Ok(cards) = CardStorage::delete_card(&id) {
                    self.cards = cards;
                    self.update_filtered_cards();
                    self.selected_index = None;
                }
                self.show_form = false;
                self.editing_card = None;
                true
            }
            AppMsg::ClearAll => {
                if CardStorage::clear_all().is_ok() {
                    self.cards = Vec::new();
                    self.update_filtered_cards();
                    self.selected_index = None;
                }
                true
            }
            AppMsg::PopulateTestData => {
                // Clear existing data first
                let _ = CardStorage::clear_all();
                // Generate 100 fake cards
                let fake_cards = generate_fake_cards(100);
                if CardStorage::save_cards(&fake_cards).is_ok() {
                    self.cards = fake_cards;
                    self.update_filtered_cards();
                    self.selected_index = None;
                }
                true
            }
            AppMsg::CancelForm => {
                self.show_form = false;
                self.editing_card = None;
                true
            }
            AppMsg::Search(query) => {
                self.search_query = query;
                self.update_filtered_cards();
                self.selected_index = None;
                true
            }
            AppMsg::SelectCard(index) => {
                self.selected_index = Some(index);
                // Also open the editor
                if let Some(card) = self.filtered_cards.get(index) {
                    self.editing_card = Some(card.clone());
                    self.show_form = true;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_add = ctx.link().callback(|_| AppMsg::AddCard);
        let on_edit = ctx.link().callback(AppMsg::EditCard);
        let on_delete = ctx.link().callback(AppMsg::DeleteCard);
        let on_clear_all = ctx.link().callback(|_| AppMsg::ClearAll);
        let on_populate = ctx.link().callback(|_| AppMsg::PopulateTestData);
        let on_save = ctx.link().callback(AppMsg::SaveCard);
        let on_cancel = ctx.link().callback(|_| AppMsg::CancelForm);
        let on_search = ctx.link().callback(AppMsg::Search);
        let on_select = ctx.link().callback(AppMsg::SelectCard);
        let on_card_click = ctx.link().callback(AppMsg::SelectCardById);

        html! {
            <div class="app">
                <a class="github-corner" href="https://github.com/softwarewrighter/rolodex" target="_blank" aria-label="View source on GitHub">
                    <svg width="80" height="80" viewBox="0 0 250 250" aria-hidden="true">
                        <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z" fill="#151513"></path>
                        <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="#fff" class="octo-arm"></path>
                        <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C155.9,114.9 152.3,118.4 152.3,121.6 C152.3,125.6 152.4,137.4 152.4,142.8 C152.5,147.0 149.9,151.6 143.8,151.6 C142.7,151.6 141.4,151.6 140.0,151.6" fill="#fff" class="octo-body"></path>
                    </svg>
                </a>
                <header class="app-header">
                    <h1>{"Rolodex"}</h1>
                    <div class="header-actions">
                        <button class="btn-add" onclick={on_add}>{"+ Add Card"}</button>
                        <button class="btn-populate" onclick={on_populate}>{"Populate Test Data"}</button>
                        <button class="btn-clear" onclick={on_clear_all}>{"Clear All"}</button>
                    </div>
                </header>

                <main class="app-main">
                    <div class="sidebar">
                        <SearchBar
                            on_search={on_search}
                            value={self.search_query.clone()}
                        />
                        <CardList
                            cards={self.filtered_cards.clone()}
                            on_edit={on_edit}
                            on_delete={on_delete}
                            on_select={on_select.clone()}
                            selected_index={self.selected_index}
                        />
                    </div>

                    <div class="main-view">
                        <Rolodex3D
                            cards={self.filtered_cards.clone()}
                            selected_index={self.selected_index}
                            on_card_click={on_card_click}
                        />
                    </div>
                </main>

                <footer class="app-footer">
                    <div class="footer-content">
                        <span class="copyright">{"Copyright (c) 2025 Michael A Wright"}</span>
                        <span class="license">{"MIT License"}</span>
                        <span class="build-info">{format!(
                            "v{} | {} | {} | {}",
                            env!("CARGO_PKG_VERSION"),
                            option_env!("BUILD_HOST").unwrap_or("local"),
                            option_env!("BUILD_COMMIT").unwrap_or("dev"),
                            option_env!("BUILD_DATE").unwrap_or("unknown")
                        )}</span>
                    </div>
                </footer>

                {if self.show_form {
                    let on_delete_from_form = ctx.link().callback(AppMsg::DeleteCardFromForm);
                    html! {
                        <CardForm
                            card={self.editing_card.clone()}
                            on_save={on_save}
                            on_cancel={on_cancel}
                            on_delete={Some(on_delete_from_form)}
                        />
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }
}

impl App {
    fn update_filtered_cards(&mut self) {
        let mut filtered: Vec<Card> = self
            .cards
            .iter()
            .filter(|c| c.matches_search(&self.search_query))
            .cloned()
            .collect();
        // Sort alphabetically by name (case-insensitive)
        filtered.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        self.filtered_cards = filtered;
    }
}
