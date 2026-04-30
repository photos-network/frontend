use crate::api::admin::{
    get_admin_albums, get_admin_users, get_album_access_list, grant_album_access,
    revoke_album_access, AdminAlbumEntry,
};
use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn AdminOverview() -> impl IntoView {
    let i18n = use_i18n();
    let active_tab = RwSignal::new(0u8);

    view! {
        <main class="ui main container mx-auto px-4 py-8">
            <h1 class="text-2xl font-bold text-gray-900 mb-6">{t!(i18n, admin_title)}</h1>

            <div class="flex space-x-2 mb-6 border-b border-gray-200">
                <button
                    class=move || if active_tab.get() == 0 {
                        "px-4 py-2 font-medium text-blue-600 border-b-2 border-blue-600"
                    } else {
                        "px-4 py-2 font-medium text-gray-500 hover:text-gray-700"
                    }
                    on:click=move |_| active_tab.set(0)
                >
                    {t!(i18n, admin_tab_albums)}
                </button>
                <button
                    class=move || if active_tab.get() == 1 {
                        "px-4 py-2 font-medium text-blue-600 border-b-2 border-blue-600"
                    } else {
                        "px-4 py-2 font-medium text-gray-500 hover:text-gray-700"
                    }
                    on:click=move |_| active_tab.set(1)
                >
                    {t!(i18n, admin_tab_users)}
                </button>
            </div>

            {move || if active_tab.get() == 1 {
                view! { <UsersTab/> }.into_any()
            } else {
                view! { <AlbumsTab/> }.into_any()
            }}
        </main>
    }
}

#[component]
fn UsersTab() -> impl IntoView {
    let i18n = use_i18n();
    let users = Resource::new(|| (), |_| get_admin_users());

    view! {
        <Suspense fallback=move || view! {
            <p class="text-gray-500">{t!(i18n, admin_loading)}</p>
        }>
            {move || match users.get() {
                None => view! { <div/> }.into_any(),
                Some(Err(e)) => view! {
                    <div class="rounded-lg bg-red-50 border border-red-200 p-4 text-red-700">
                        <p>{e.to_string()}</p>
                    </div>
                }.into_any(),
                Some(Ok(users)) if users.is_empty() => view! {
                    <p class="text-gray-500">{t!(i18n, admin_no_users)}</p>
                }.into_any(),
                Some(Ok(users)) => view! {
                    <div class="bg-white rounded-xl border border-gray-100 shadow-sm divide-y divide-gray-50">
                        {users.into_iter().map(|u| view! {
                            <div class="flex items-center gap-3 px-4 py-3">
                                <div class="w-8 h-8 rounded-full bg-indigo-100 flex items-center justify-center text-indigo-700 text-xs font-semibold shrink-0">
                                    {u.email.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| "?".to_string())}
                                </div>
                                <div class="min-w-0 flex-1">
                                    <p class="text-sm font-medium text-gray-900 truncate">
                                        {u.display_name.unwrap_or_else(|| u.email.clone())}
                                    </p>
                                    <p class="text-xs text-gray-400 truncate">{u.email}</p>
                                </div>
                                <p class="text-xs font-mono text-gray-300 shrink-0 hidden sm:block">{u.account_id}</p>
                            </div>
                        }).collect_view()}
                    </div>
                }.into_any(),
            }}
        </Suspense>
    }
}

#[component]
fn AlbumsTab() -> impl IntoView {
    let i18n = use_i18n();
    let albums = Resource::new(|| (), |_| get_admin_albums());

    view! {
        <Suspense fallback=move || view! {
            <p class="text-gray-500">{t!(i18n, admin_loading)}</p>
        }>
            {move || match albums.get() {
                None => view! { <div/> }.into_any(),
                Some(Err(e)) => view! {
                    <div class="rounded-lg bg-red-50 border border-red-200 p-4 text-red-700">
                        <p>{e.to_string()}</p>
                    </div>
                }.into_any(),
                Some(Ok(albums)) if albums.is_empty() => view! {
                    <p class="text-gray-500">{t!(i18n, admin_no_albums)}</p>
                }.into_any(),
                Some(Ok(albums)) => view! {
                    <div class="space-y-3">
                        {albums.into_iter().map(|album| view! {
                            <AlbumRow album=album/>
                        }).collect_view()}
                    </div>
                }.into_any(),
            }}
        </Suspense>
    }
}

#[component]
fn AlbumRow(album: AdminAlbumEntry) -> impl IntoView {
    let i18n = use_i18n();
    let expanded = RwSignal::new(false);
    let album_id = album.album_id.clone();
    let album_id_for_access = album.album_id.clone();
    let album_id_for_grant = album.album_id.clone();
    let access_version = RwSignal::new(0u32);

    let access_list = Resource::new(
        move || (album_id_for_access.clone(), access_version.get()),
        |(id, _)| get_album_access_list(id),
    );

    let grant_account_id = RwSignal::new(String::new());
    let grant_role = RwSignal::new("viewer".to_string());

    let album_id_grant = album_id_for_grant.clone();
    let grant_action = Action::new(move |_: &()| {
        let aid = album_id_grant.clone();
        let account_id = grant_account_id.get();
        let role = grant_role.get();
        async move { grant_album_access(aid, account_id, role).await }
    });

    let revoke_action = Action::new(move |(album_id, account_id): &(String, String)| {
        let aid = album_id.clone();
        let acct = account_id.clone();
        async move { revoke_album_access(aid, acct).await }
    });

    Effect::new(move |_| {
        if grant_action.version().get() > 0 {
            access_version.update(|v| *v += 1);
            grant_account_id.set(String::new());
        }
    });
    Effect::new(move |_| {
        if revoke_action.version().get() > 0 {
            access_version.update(|v| *v += 1);
        }
    });

    view! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-4">
            <div class="flex items-start justify-between">
                <div>
                    <h3 class="font-semibold text-gray-900">{album.name.clone()}</h3>
                    <p class="text-sm text-gray-500 mt-0.5">"Owner: " {album.owner.clone()}</p>
                    {album.description.map(|d| view! {
                        <p class="text-sm text-gray-400 mt-1">{d}</p>
                    })}
                </div>
                <button
                    class="ml-4 text-sm text-blue-600 hover:text-blue-800 font-medium"
                    on:click=move |_| expanded.update(|e| *e = !*e)
                >
                    {move || if expanded.get() { "Hide".into_any() } else { t!(i18n, admin_access_section).into_any() }}
                </button>
            </div>

            {move || if expanded.get() {
                let revoke_action_inner = revoke_action.clone();
                let album_id_revoke = album_id.clone();
                view! {
                    <div class="mt-4 border-t border-gray-100 pt-4">
                        <h4 class="text-sm font-medium text-gray-700 mb-2">
                            {t!(i18n, admin_access_section)}
                        </h4>
                        <Suspense fallback=move || view! {
                            <p class="text-sm text-gray-400">{t!(i18n, admin_loading)}</p>
                        }>
                            {move || {
                                let revoke = revoke_action_inner.clone();
                                let alb_id = album_id_revoke.clone();
                                match access_list.get() {
                                    None => view! { <div/> }.into_any(),
                                    Some(Err(e)) => view! {
                                        <p class="text-sm text-red-600">{e.to_string()}</p>
                                    }.into_any(),
                                    Some(Ok(entries)) if entries.is_empty() => view! {
                                        <p class="text-sm text-gray-400">{t!(i18n, admin_access_empty)}</p>
                                    }.into_any(),
                                    Some(Ok(entries)) => view! {
                                        <ul class="space-y-1 mb-3">
                                            {entries.into_iter().map(|entry| {
                                                let alb = alb_id.clone();
                                                let acct = entry.account_id.clone();
                                                let revoke_clone = revoke.clone();
                                                view! {
                                                    <li class="flex items-center justify-between text-sm py-1">
                                                        <span class="font-mono text-gray-700">{entry.account_id.clone()}</span>
                                                        <span class="text-gray-400 mx-2">{entry.role.clone()}</span>
                                                        <button
                                                            class="text-red-500 hover:text-red-700 text-xs font-medium"
                                                            on:click=move |_| {
                                                                revoke_clone.dispatch((alb.clone(), acct.clone()));
                                                            }
                                                        >
                                                            {t!(i18n, admin_revoke_access)}
                                                        </button>
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    }.into_any(),
                                }
                            }}
                        </Suspense>
                        <div class="flex items-center space-x-2 mt-3">
                            <input
                                type="text"
                                class="flex-1 border border-gray-300 rounded px-2 py-1 text-sm"
                                placeholder=move || t_string!(i18n, admin_account_id_placeholder)
                                prop:value=move || grant_account_id.get()
                                on:input=move |ev| grant_account_id.set(event_target_value(&ev))
                            />
                            <select
                                class="border border-gray-300 rounded px-2 py-1 text-sm"
                                on:change=move |ev| grant_role.set(event_target_value(&ev))
                            >
                                <option value="viewer">{t!(i18n, admin_role_viewer)}</option>
                                <option value="owner">{t!(i18n, admin_role_owner)}</option>
                            </select>
                            <button
                                class="bg-blue-600 text-white px-3 py-1 rounded text-sm hover:bg-blue-700"
                                on:click=move |_| { grant_action.dispatch(()); }
                            >
                                {t!(i18n, admin_grant_access)}
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div/> }.into_any()
            }}
        </div>
    }
}
