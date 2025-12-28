use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SearchBarProps {
    pub on_search: Callback<String>,
    pub value: String,
}

#[function_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
    let on_input = {
        let on_search = props.on_search.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            on_search.emit(input.value());
        })
    };

    html! {
        <div class="search-bar">
            <input
                type="text"
                placeholder="Search cards..."
                value={props.value.clone()}
                oninput={on_input}
                class="search-input"
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_search_bar_props() {
        // Test that SearchBarProps can be created properly
        let on_search = Callback::from(|_: String| {});
        let props = SearchBarProps {
            on_search,
            value: "test".to_string(),
        };
        assert_eq!(props.value, "test");
    }
}
