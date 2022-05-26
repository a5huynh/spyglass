use yew::prelude::*;

#[function_component(Tooltip)]
pub fn tooltip() -> Html {
    let styles = vec![
        "group-hover:block",
        "group-hover:text-neutral-400",
        "py-1",
        "px-2",
        // Positioning
        "-ml-16",
        "-mt-2",
        "rounded",
        "hidden",
        "absolute",
        "text-center",
        "bg-neutral-900",
        "text-sm",
    ];

    html! {
        <div class={styles}>
            {"Delete"}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DeleteButtonProps {
    pub doc_id: String,
}

#[function_component(DeleteButton)]
pub fn delete_btn(props: &DeleteButtonProps) -> Html {
    let onclick = {
        let doc_id = props.doc_id.clone();
        move |_| {
            log::info!("DELETING DOC {}", doc_id);
        }
    };

    html! {
        <div class="float-right pl-4 pr-0 h-28">
            <button
                {onclick}
                class="hover:text-red-600 text-neutral-600 group">
                <Tooltip />
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
            </button>
        </div>
    }
}
