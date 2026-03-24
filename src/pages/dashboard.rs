use crate::api::albums::{get_admin_users_detailed, get_dashboard_albums, get_owned_album_stats, AlbumStats};
use crate::auth::{check_session_kind, SessionKind};
use crate::components::album_card::AlbumSection;
use crate::components::user_card::AdminUserSection;
use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let i18n = use_i18n();
    let albums = Resource::new(|| (), |_| get_dashboard_albums());
    let stats = Resource::new(|| (), |_| get_owned_album_stats());
    let admin_users = Resource::new(|| (), |_| get_admin_users_detailed());
    let session = Resource::new(|| (), |_| check_session_kind());

    view! {
            <div class="max-w-7xl mx-auto px-4 sm:px-6">

                <Suspense fallback=move || view! {
                    <div class="space-y-10">
                        {(0..2).map(|_| view! {
                            <div>
                                <div class="h-5 bg-gray-200 rounded w-40 mb-4 animate-pulse"/>
                                <div class="grid gap-2" style="grid-template-columns: repeat(auto-fill, minmax(200px, 1fr))">
                                    {(0..6).map(|_| view! {
                                        <div class="aspect-square rounded-lg bg-gray-100 animate-pulse"/>
                                    }).collect_view()}
                                </div>
                            </div>
                        }).collect_view()}
                    </div>
                }>
                    {move || {
                        let session_kind = session.get().and_then(|r| r.ok()).unwrap_or(SessionKind::Customer);
                        let is_account = matches!(session_kind, SessionKind::Account | SessionKind::Admin);

                        let stats_map: std::collections::HashMap<String, AlbumStats> = if is_account {
                            stats.get().and_then(|r| r.ok()).unwrap_or_default()
                                .into_iter().map(|s| (s.album_id.clone(), s)).collect()
                        } else {
                            std::collections::HashMap::new()
                        };

                        match albums.get() {
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
                                <div class="space-y-10">
                                    {albums.into_iter().map(|album| {
                                        let album_stats = stats_map.get(&album.album_id).cloned();
                                        let is_owner = is_account && album_stats.is_some();
                                        view! {
                                            <AlbumSection album=album stats=album_stats is_owner=is_owner/>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any(),
                        }
                    }}
                </Suspense>

                <Suspense fallback=move || view! { <div/> }>
                    {move || {
                        let users = admin_users.get().and_then(|r| r.ok()).unwrap_or_default();
                        if users.is_empty() {
                            view! { <div/> }.into_any()
                        } else {
                            view! { <AdminUserSection users=users/> }.into_any()
                        }
                    }}
                </Suspense>
            </div>
    }
}
