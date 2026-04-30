use crate::api::albums::get_album_media;
use crate::api::media::{check_album_ownership, delete_album_media};
use crate::auth::{check_session_kind, SessionKind};
use crate::components::media_card::{MediaCard, OwnerUploadBar};
use crate::i18n::*;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn CustomerAlbumDetailPage() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();
    let album_id = move || params.read().get("id").unwrap_or_default();

    let refresh = RwSignal::new(0u32);
    let media = Resource::new(
        move || (album_id(), refresh.get()),
        |(id, _)| get_album_media(id),
    );

    let session = Resource::new(|| (), |_| check_session_kind());
    let ownership = Resource::new(album_id, |id| check_album_ownership(id));

    view! {
        
            <div class="max-w-7xl mx-auto px-4 sm:px-6 py-8">
                <a href="/albums" class="text-sm text-indigo-600 hover:underline">
                    {t!(i18n, album_back)}
                </a>

                <Suspense fallback=|| view! { <div/> }>
                    {move || {
                        let kind = session.get().and_then(|r| r.ok()).unwrap_or(SessionKind::Customer);
                        let is_owner = matches!(kind, SessionKind::Account | SessionKind::Admin)
                            && ownership.get().and_then(|r| r.ok()).unwrap_or(false);

                        if is_owner {
                            view! { <OwnerUploadBar album_id=album_id() album_name=String::new()/> }.into_any()
                        } else {
                            view! { <div/> }.into_any()
                        }
                    }}
                </Suspense>

                <Suspense fallback=move || view! {
                    <div class="mt-6 grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3">
                        {(0..10).map(|_| view! {
                            <div class="bg-white rounded-xl aspect-square animate-pulse"/>
                        }).collect_view()}
                    </div>
                }>
                    {move || {
                        let kind = session.get().and_then(|r| r.ok()).unwrap_or(SessionKind::Customer);
                        let is_owner = matches!(kind, SessionKind::Account | SessionKind::Admin)
                            && ownership.get().and_then(|r| r.ok()).unwrap_or(false);
                        let aid = album_id();

                        match media.get() {
                            None => view! { <div/> }.into_any(),
                            Some(Err(e)) => view! {
                                <div class="mt-6 rounded-xl bg-red-50 border border-red-100 p-6 text-red-700">
                                    <p class="font-medium text-sm">{t!(i18n, album_load_error)}</p>
                                    <p class="mt-1 text-xs font-mono opacity-70">{e.to_string()}</p>
                                    <a href="/" class="mt-3 inline-block text-xs underline">
                                        {t!(i18n, album_sign_in_again)}
                                    </a>
                                </div>
                            }.into_any(),
                            Some(Ok(items)) if items.is_empty() => view! {
                                <div class="mt-6 text-center py-24 text-gray-400">
                                    <p class="text-base">{t!(i18n, album_empty)}</p>
                                </div>
                            }.into_any(),
                            Some(Ok(items)) => view! {
                                <div class="mt-6 grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3">
                                    {items.into_iter().map(|item| {
                                        let media_id = item.uuid.clone();
                                        let album = aid.clone();
                                        view! {
                                            <MediaCard
                                                item=item
                                                is_owner=is_owner
                                                on_delete=move || {
                                                    let a = album.clone();
                                                    let m = media_id.clone();
                                                    leptos::task::spawn_local(async move {
                                                        let _ = delete_album_media(a, m).await;
                                                    });
                                                    refresh.update(|v| *v += 1);
                                                }
                                            />
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any(),
                        }
                    }}
                </Suspense>
            </div>
        
    }
}
