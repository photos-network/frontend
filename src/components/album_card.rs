use crate::api::albums::{get_album_media, AlbumStats, AlbumSummary};
use crate::components::access_panel::AlbumAccessPanel;
use crate::components::media_card::OwnerUploadBar;
use crate::i18n::*;
use leptos::html;
use leptos::prelude::*;

/// Grid card used on the customer albums page.
#[component]
pub fn AlbumCard(album: AlbumSummary) -> impl IntoView {
    let i18n = use_i18n();
    let album_id = album.album_id.clone();
    let media = Resource::new(move || album_id.clone(), |id| get_album_media(id));
    let name = album.name.clone();
    let heading = album.name.clone();
    let href = format!("/albums/{}", album.album_id);

    view! {
        <a href=href class="block bg-white rounded-xl border border-gray-100 shadow-sm hover:shadow-md transition-shadow overflow-hidden group">
            <div class="relative aspect-square bg-gradient-to-br from-gray-100 to-gray-200 overflow-hidden">
                <div class="absolute inset-0 flex items-center justify-center">
                    <svg class="w-10 h-10 text-gray-300" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 0 0 1.5-1.5V6a1.5 1.5 0 0 0-1.5-1.5H3.75A1.5 1.5 0 0 0 2.25 6v12a1.5 1.5 0 0 0 1.5 1.5Zm10.5-11.25h.008v.008h-.008V8.25Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z"/>
                    </svg>
                </div>
                <Suspense fallback=|| ()>
                    {move || {
                        let items = media.get().and_then(|r| r.ok()).unwrap_or_default();
                        let count = items.len();
                        let cover_src = items.into_iter().next().map(|item| format!("/media/{}", item.uuid));
                        let alt = name.clone();
                        view! {
                            <>
                                {cover_src.map(|src| view! {
                                    <img src=src alt=alt class="absolute inset-0 w-full h-full object-cover" loading="lazy"/>
                                })}
                                <span class="absolute bottom-2 right-2 bg-black/50 text-white text-xs px-1.5 py-0.5 rounded-md backdrop-blur-sm">
                                    {count} " " {t!(i18n, albums_items)}
                                </span>
                            </>
                        }
                    }}
                </Suspense>
            </div>
            <div class="px-3 py-2">
                <h2 class="text-sm font-medium text-gray-900 truncate">{heading}</h2>
            </div>
        </a>
    }
}

/// Full-width album section used on the dashboard.
/// Shows the album name, download button, optional access panel, and all images in a grid.
#[component]
pub fn AlbumSection(album: AlbumSummary, stats: Option<AlbumStats>, is_owner: bool) -> impl IntoView {
    let i18n = use_i18n();
    let album_id = StoredValue::new(album.album_id.clone());
    let media = Resource::new(move || album_id.get_value(), |id| get_album_media(id));
    let show_panel = RwSignal::new(false);
    let lightbox_idx = RwSignal::new(Option::<usize>::None);
    let overlay_ref = NodeRef::<html::Div>::new();
    let download_href = format!("/albums/{}/download", album.album_id);
    let album_name = album.name.clone();
    let downloading = RwSignal::new(false);

    // Flat list of media srcs derived from the resource — used by lightbox for navigation.
    let srcs = Signal::derive(move || {
        media
            .get()
            .and_then(|r| r.ok())
            .unwrap_or_default()
            .into_iter()
            .map(|item| format!("/media/{}", item.uuid))
            .collect::<Vec<_>>()
    });

    // Focus overlay on open so keyboard events are captured.
    Effect::new(move |_| {
        if lightbox_idx.get().is_some() {
            set_timeout(
                move || {
                    if let Some(el) = overlay_ref.get() {
                        let _ = el.focus();
                    }
                },
                std::time::Duration::ZERO,
            );
        }
    });

    view! {
        // Lightbox overlay
        {move || {
            let idx = lightbox_idx.get()?;
            let list = srcs.get();
            let src = list.get(idx)?.clone();
            let is_first = idx == 0;
            let is_last = idx + 1 >= list.len();
            Some(view! {
                <div
                    node_ref=overlay_ref
                    tabindex="0"
                    class="fixed inset-0 z-50 bg-black/90 flex items-center justify-center outline-none"
                    on:click=move |_| lightbox_idx.set(None)
                    on:keydown=move |ev| {
                        match ev.key().as_str() {
                            "Escape" => lightbox_idx.set(None),
                            "ArrowLeft" => lightbox_idx.update(|i| {
                                if let Some(idx) = i { if *idx > 0 { *idx -= 1; } }
                            }),
                            "ArrowRight" => {
                                let len = srcs.get().len();
                                lightbox_idx.update(|i| {
                                    if let Some(idx) = i { if *idx + 1 < len { *idx += 1; } }
                                });
                            }
                            _ => {}
                        }
                    }
                >
                    // Close button
                    <button
                        class="absolute top-4 right-4 text-white/60 hover:text-white text-4xl leading-none font-light"
                        on:click=move |ev| { ev.stop_propagation(); lightbox_idx.set(None); }
                    >
                        "×"
                    </button>
                    // Prev button
                    {(!is_first).then(|| view! {
                        <button
                            class="absolute left-4 top-1/2 -translate-y-1/2 text-white/60 hover:text-white text-5xl leading-none font-light px-3 py-4"
                            on:click=move |ev| {
                                ev.stop_propagation();
                                lightbox_idx.update(|i| {
                                    if let Some(idx) = i { if *idx > 0 { *idx -= 1; } }
                                });
                            }
                        >
                            "‹"
                        </button>
                    })}
                    // Next button
                    {(!is_last).then(|| view! {
                        <button
                            class="absolute right-4 top-1/2 -translate-y-1/2 text-white/60 hover:text-white text-5xl leading-none font-light px-3 py-4"
                            on:click=move |ev| {
                                ev.stop_propagation();
                                let len = srcs.get().len();
                                lightbox_idx.update(|i| {
                                    if let Some(idx) = i { if *idx + 1 < len { *idx += 1; } }
                                });
                            }
                        >
                            "›"
                        </button>
                    })}
                    // Image (stop propagation so click doesn't close lightbox)
                    <img
                        src=src
                        class="max-w-[90vw] max-h-[90vh] object-contain rounded shadow-2xl"
                        on:click=|ev| ev.stop_propagation()
                    />
                </div>
            })
        }}

        <div class="mb-10">
            // Header row: name · stats · download · access toggle
            <div class="flex items-center gap-3 mb-4">
                <h2 class="text-2xl font-semibold text-gray-900 flex-1 truncate">{album_name.clone()}</h2>
                {stats.map(|s| view! {
                    <span class="text-xs text-gray-400 shrink-0 hidden sm:block">
                        {s.total_views} " " {t!(i18n, dashboard_views)}
                        " · "
                        {s.unique_viewers} " " {t!(i18n, dashboard_viewers)}
                    </span>
                })}
                // rel="external" → browser handles download, not Leptos router.
                // on:click + RwSignal → Leptos handles the preparing state.
                <a
                    href=download_href.clone()
                    rel="external"
                    class="shrink-0 flex items-center gap-1 text-xs font-medium text-indigo-600 hover:text-indigo-800 transition-colors mr-4"
                    on:click=move |_| {
                        downloading.set(true);
                        set_timeout(move || downloading.set(false), std::time::Duration::from_secs(8));
                    }
                >
                    {move || if downloading.get() {
                        view! {
                            <>
                                <span class="inline-block w-3 h-3 border-2 border-current border-t-transparent rounded-full animate-spin"/>
                                " "
                                {t!(i18n, dashboard_download_preparing)}
                            </>
                        }.into_any()
                    } else {
                        view! {
                            <>
                                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5M16.5 12 12 16.5m0 0L7.5 12m4.5 4.5V3"/>
                                </svg>
                                {t!(i18n, dashboard_download_zip)}
                            </>
                        }.into_any()
                    }}
                </a>
                {is_owner.then(|| view! {
                    <button
                        class="shrink-0 text-xs font-medium text-gray-400 hover:text-gray-700 transition-colors"
                        on:click=move |_| show_panel.update(|v| *v = !*v)
                    >
                        {move || if show_panel.get() { "−" } else { t_string!(i18n, dashboard_access_codes) }}
                    </button>
                })}
            </div>

            // Access panel (owner only, toggleable)
            {move || (is_owner && show_panel.get()).then(|| view! {
                <div class="mb-4 bg-white rounded-xl border border-gray-100 shadow-sm">
                    <AlbumAccessPanel album_id=album_id.get_value()/>
                </div>
            })}

            // Upload bar (owner only)
            {is_owner.then(|| view! {
                <OwnerUploadBar album_id=album_id.get_value() album_name=album_name.clone()/>
            })}

            // Image grid
            <Suspense fallback=move || view! {
                <div class="grid gap-2" style="grid-template-columns: repeat(auto-fill, minmax(200px, 1fr))">
                    {(0..8).map(|_| view! {
                        <div class="aspect-square rounded-lg bg-gray-100 animate-pulse"/>
                    }).collect_view()}
                </div>
            }>
                {move || match media.get() {
                    None => view! { <div/> }.into_any(),
                    Some(Err(e)) => view! {
                        <p class="text-sm text-red-500 py-2">{e.to_string()}</p>
                    }.into_any(),
                    Some(Ok(items)) if items.is_empty() => view! {
                        <p class="text-sm text-gray-400 py-4">{t!(i18n, album_empty)}</p>
                    }.into_any(),
                    Some(Ok(items)) => view! {
                        <div class="grid gap-2" style="grid-template-columns: repeat(auto-fill, minmax(200px, 1fr))">
                            {items.into_iter().enumerate().map(|(idx, item)| {
                                let src = format!("/media/{}", item.uuid);
                                view! {
                                    <div
                                        class="aspect-square overflow-hidden rounded-lg cursor-zoom-in bg-gray-100"
                                        on:click=move |_| lightbox_idx.set(Some(idx))
                                    >
                                        <img
                                            src=src
                                            alt=""
                                            class="w-full h-full object-cover hover:scale-105 transition-transform duration-200"
                                            loading="lazy"
                                        />
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any(),
                }}
            </Suspense>
        </div>
    }
}
