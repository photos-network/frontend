use crate::api::albums::MediaItemSummary;
use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn OwnerUploadBar(album_id: String, #[prop(optional)] album_name: String) -> impl IntoView {
    let i18n = use_i18n();
    let upload_url = format!("/albums/{}/upload", album_id);
    // "image/*,video/*" written without /* to avoid block-comment ambiguity in view!
    let accept = "image/jpeg,image/png,image/gif,image/webp,video/mp4,video/quicktime,video/x-msvideo";

    view! {
        <div class="mt-4 mb-2 p-3 bg-gray-50 rounded-xl border border-dashed border-gray-300 flex items-center gap-3">
            <form
                method="POST"
                action=upload_url
                enctype="multipart/form-data"
                class="flex items-center gap-2 flex-1 min-w-0"
            >

            {(!album_name.is_empty()).then(|| view! {
                <span class="text-xs text-gray-400 shrink-0 hidden sm:block">
                    {move || format!("{}: {}", t_string!(i18n, album_upload_to), album_name)}
                </span>
            })}
                <input
                    type="file"
                    name="file"
                    accept=accept
                    multiple
                    class="flex-1 min-w-0 text-sm text-gray-500 file:mr-3 file:py-1 file:px-3 file:rounded-lg file:border-0 file:text-xs file:font-medium file:bg-indigo-50 file:text-indigo-700 hover:file:bg-indigo-100 cursor-pointer"
                />
                <button
                    type="submit"
                    class="shrink-0 text-xs font-medium bg-indigo-600 text-white rounded-lg px-3 py-1.5 hover:bg-indigo-700 transition-colors"
                >
                    {t!(i18n, album_upload)}
                </button>
            </form>
        </div>
    }
}

#[component]
pub fn MediaCard(
    item: MediaItemSummary,
    is_owner: bool,
    on_delete: impl Fn() + 'static,
) -> impl IntoView {
    let i18n = use_i18n();
    let src = format!("/media/{}", item.uuid);
    let name = item.name.clone();

    view! {
        <div class="relative bg-white rounded-xl border border-gray-100 shadow-sm hover:shadow-md transition-shadow overflow-hidden group">
            <div class="aspect-square  overflow-hidden">
                <img
                    src=src
                    alt=name.clone()
                    class="w-full h-full object-cover"
                    loading="lazy"
                />
            </div>
            <div class="px-3 py-2 flex items-center justify-between gap-1">
                // <p class="text-xs text-gray-700 truncate flex-1">{name}</p>
                {if is_owner {
                    view! {
                        <button
                            class="shrink-0 text-xs text-red-400 hover:text-red-600 opacity-0 group-hover:opacity-100 transition-opacity"
                            on:click=move |_| on_delete()
                        >
                            {t!(i18n, album_delete_media)}
                        </button>
                    }.into_any()
                } else {
                    view! { <span/> }.into_any()
                }}
            </div>
        </div>
    }
}
