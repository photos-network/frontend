use crate::api::albums::get_customer_albums;
use crate::components::album_card::AlbumCard;
use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn CustomerAlbumsPage() -> impl IntoView {
    let i18n = use_i18n();
    let albums = Resource::new(|| (), |_| get_customer_albums());

    view! {
            <div class="max-w-7xl mx-auto px-4 sm:px-6 py-8">
                <h1 class="text-xl font-semibold text-gray-800 mb-6">{t!(i18n, albums_title)}</h1>

                <Suspense fallback=move || view! {
                    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3">
                        {(0..10).map(|_| view! {
                            <div class="bg-white rounded-xl aspect-square animate-pulse"/>
                        }).collect_view()}
                    </div>
                }>
                    {move || match albums.get() {
                        None => view! { <div/> }.into_any(),
                        Some(Err(e)) => view! {
                            <div class="rounded-xl bg-red-50 border border-red-100 p-6 text-red-700">
                                <p class="font-medium text-sm">{t!(i18n, albums_load_error)}</p>
                                <p class="mt-1 text-xs font-mono opacity-70">{e.to_string()}</p>
                                <a href="/" class="mt-3 inline-block text-xs underline">
                                    {t!(i18n, albums_sign_in_again)}
                                </a>
                            </div>
                        }.into_any(),
                        Some(Ok(albums)) if albums.is_empty() => view! {
                            <div class="text-center py-24 text-gray-400">
                                <p class="text-base">{t!(i18n, albums_empty)}</p>
                            </div>
                        }.into_any(),
                        Some(Ok(albums)) => view! {
                            <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3">
                                {albums.into_iter().map(|album| view! {
                                    <AlbumCard album=album/>
                                }).collect_view()}
                            </div>
                        }.into_any(),
                    }}
                </Suspense>
            </div>
    }
}
