use crate::api::albums::AccountWithAlbums;
use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn AdminUserSection(users: Vec<AccountWithAlbums>) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="mt-12">
            <div class="text-2xl font-semibold text-gray-700 mb-4 uppercase tracking-wide">
                {t!(i18n, dashboard_user_management)}
            </div>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {users.into_iter().map(|user| {
                    let i18n = i18n;
                    let initial = user.display_name.as_ref()
                        .or(Some(&user.email))
                        .and_then(|s| s.chars().next())
                        .map(|c| c.to_uppercase().to_string())
                        .unwrap_or_else(|| "?".to_string());
                    let display = user.display_name.clone().unwrap_or_else(|| user.email.clone());
                    let email = user.email.clone();
                    let last_login = user.last_login_at.clone()
                        .unwrap_or_else(|| t_string!(i18n, dashboard_never_logged_in).to_string());
                    let is_admin = user.is_admin;
                    let albums = user.albums.clone();

                    view! {
                        <div class="bg-white rounded-xl border border-gray-100 shadow-sm p-4 flex flex-col gap-3">
                            <div class="flex items-center gap-3">
                                <div class="w-9 h-9 rounded-full bg-indigo-100 flex items-center justify-center text-indigo-700 font-semibold text-sm shrink-0">
                                    {initial}
                                </div>
                                <div class="min-w-0">
                                    <div class="flex items-center gap-2 flex-wrap">
                                        <span class="text-sm font-medium text-gray-900 truncate">{display}</span>
                                        {if is_admin {
                                            view! {
                                                <span class="rounded-full bg-indigo-50 px-2 py-0.5 text-xs font-semibold text-indigo-600">
                                                    {t!(i18n, dashboard_admin_badge)}
                                                </span>
                                            }.into_any()
                                        } else {
                                            view! { <span/> }.into_any()
                                        }}
                                    </div>
                                    <p class="text-xs text-gray-400 truncate">{email}</p>
                                </div>
                            </div>
                            <div class="text-xs text-gray-400">
                                {t!(i18n, dashboard_last_login)}
                                {": "}
                                <span class="text-gray-600">{last_login}</span>
                            </div>
                            {if albums.is_empty() {
                                view! {
                                    <p class="text-xs text-gray-300">{t!(i18n, dashboard_no_albums_assigned)}</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="flex flex-wrap gap-1">
                                        {albums.into_iter().map(|album| {
                                            let i18n = i18n;
                                            let album_name = album.album_name.clone();
                                            let role = album.role.clone();
                                            let chip = if album.role == "owner" {
                                                "rounded-md bg-emerald-50 px-2 py-0.5 text-xs font-medium text-emerald-700"
                                            } else {
                                                "rounded-md bg-sky-50 px-2 py-0.5 text-xs font-medium text-sky-700"
                                            };
                                            view! {
                                                <span class=chip>{move || format!("{} · {}",
                                                    album_name,
                                                    if role == "owner" {
                                                        t_string!(i18n, dashboard_role_owner)
                                                    } else {
                                                        t_string!(i18n, dashboard_role_viewer)
                                                    }
                                                )}</span>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }}
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
