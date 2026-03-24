use crate::api::access_codes::{generate_album_code, get_album_codes, remove_album_code};
use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn AlbumAccessPanel(album_id: String) -> impl IntoView {
    let i18n = use_i18n();
    let version = RwSignal::new(0u32);
    let album_id = StoredValue::new(album_id);

    let codes = Resource::new(
        move || version.get(),
        move |_| get_album_codes(album_id.get_value()),
    );

    let generate_action = Action::new(move |display_name: &String| {
        let album = album_id.get_value();
        let name = display_name.clone();
        async move { generate_album_code(album, name).await }
    });

    let remove_action = Action::new(move |code: &String| {
        let album = album_id.get_value();
        let code = code.clone();
        async move { remove_album_code(album, code).await }
    });

    Effect::new(move |_| {
        if generate_action.version().get() > 0 {
            version.update(|v| *v += 1);
        }
    });
    Effect::new(move |_| {
        if remove_action.version().get() > 0 {
            version.update(|v| *v += 1);
        }
    });

    let new_name = RwSignal::new(String::new());
    let gen_error = RwSignal::new(Option::<String>::None);

    Effect::new(move |_| {
        if let Some(Err(e)) = generate_action.value().get() {
            gen_error.set(Some(e.to_string()));
        } else if generate_action.value().get().is_some() {
            gen_error.set(None);
        }
    });

    view! {
        <div class="mt-3 space-y-2 p-4">
            <Suspense fallback=|| view! { <p class="text-xs text-gray-300 py-1">"…"</p> }>
                {move || match codes.get() {
                    None => view! { <div/> }.into_any(),
                    Some(Err(e)) => view! {
                        <p class="text-xs text-red-400">{e.to_string()}</p>
                    }.into_any(),
                    Some(Ok(entries)) if entries.is_empty() => view! {
                        <p class="text-xs text-gray-300 py-1">{t!(i18n, dashboard_access_empty)}</p>
                    }.into_any(),
                    Some(Ok(entries)) => view! {
                        <ul class="space-y-1.5">
                            {entries.into_iter().map(|e| {
                                let code = e.access_code.clone();
                                let label = e.display_name.clone().unwrap_or_else(|| e.access_code.clone());
                                view! {
                                    <li class="flex items-center justify-between gap-2 rounded-lg px-2 py-1.5">
                                        <div class="min-w-0">
                                            <p class="text-xs font-medium text-gray-700 truncate">{label}</p>
                                            <p class="font-mono text-xs text-gray-400">{e.access_code.clone()}</p>
                                        </div>
                                        <button
                                            class="shrink-0 text-xs text-red-400 hover:text-red-600 transition-colors"
                                            on:click=move |_| { remove_action.dispatch(code.clone()); }
                                        >
                                            "×"
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any(),
                }}
            </Suspense>

            {move || gen_error.get().map(|e| view! {
                <p class="text-xs text-red-400">{e}</p>
            })}

            <div class="flex gap-1.5 pt-1">
                <input
                    type="text"
                    placeholder=move || t_string!(i18n, dashboard_display_name_placeholder)
                    class="flex-1 min-w-0 text-xs rounded-lg border border-gray-200 px-2 py-1.5 focus:border-indigo-300 focus:outline-none"
                    prop:value=move || new_name.get()
                    on:input=move |ev| new_name.set(event_target_value(&ev))
                />
                <button
                    class="shrink-0 text-xs rounded-lg bg-indigo-500 text-white px-2.5 py-1.5 hover:bg-indigo-600 transition-colors"
                    on:click=move |_| {
                        let name = new_name.get();
                        if !name.is_empty() {
                            generate_action.dispatch(name);
                            new_name.set(String::new());
                        }
                    }
                >
                    {t!(i18n, dashboard_generate_code)}
                </button>
            </div>
        </div>
    }
}
