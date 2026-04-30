use crate::api::auth::{account_login, customer_login, AccountLogin, CustomerLogin};
use crate::i18n::*;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn LoginForm() -> impl IntoView {
    let i18n = use_i18n();
    let active_tab = RwSignal::new(false);

    let customer_action = ServerAction::<CustomerLogin>::new();
    let account_action = ServerAction::<AccountLogin>::new();

    let customer_response = customer_action.value();
    let account_response = account_action.value();

    let session_version = use_context::<RwSignal<u32>>().unwrap_or(RwSignal::new(0));
    let navigate_customer = use_navigate();
    let navigate_account = use_navigate();

    Effect::new(move |_| {
        if customer_response.get().and_then(|r| r.ok()).and_then(|r| r.jwt_token).is_some() {
            session_version.update(|v| *v += 1);
            navigate_customer("/albums", Default::default());
        }
    });

    Effect::new(move |_| {
        if account_response.get().and_then(|r| r.ok()).and_then(|r| r.jwt_token).is_some() {
            session_version.update(|v| *v += 1);
            navigate_account("/albums", Default::default());
        }
    });

    let error_msg = move || {
        if active_tab.get() {
            account_response.get().and_then(|r| match r {
                Ok(resp) => resp.error,
                Err(e) => Some(e.to_string()),
            })
        } else {
            customer_response.get().and_then(|r| match r {
                Ok(resp) => resp.error,
                Err(e) => Some(e.to_string()),
            })
        }
    };

    // suppress unused import warnings on WASM side
    let _ = customer_login;
    let _ = account_login;

    view! {
        <div class="container mx-auto px-5 py-8 sm:flex-row flex-col items-center">
            <div class="lg:w-1/2 md:w-2/3 mx-auto">
                <div class="bg-white rounded-2xl shadow-lg px-8 py-10">
                    <div class="mb-8 text-center">
                        <h1 class="text-2xl font-bold text-gray-900">{t!(i18n, login_title)}</h1>
                    </div>

                    <div class="flex border-b border-gray-200 mb-6">
                        <button
                            type="button"
                            class=move || if !active_tab.get() {
                                "flex-1 py-2 text-sm font-medium text-indigo-600 border-b-2 border-indigo-500 focus:outline-none"
                            } else {
                                "flex-1 py-2 text-sm font-medium text-gray-500 hover:text-gray-700 focus:outline-none"
                            }
                            on:click=move |_| active_tab.set(false)
                        >
                            {t!(i18n, login_tab_access_code)}
                        </button>
                        <button
                            type="button"
                            class=move || if active_tab.get() {
                                "flex-1 py-2 text-sm font-medium text-indigo-600 border-b-2 border-indigo-500 focus:outline-none"
                            } else {
                                "flex-1 py-2 text-sm font-medium text-gray-500 hover:text-gray-700 focus:outline-none"
                            }
                            on:click=move |_| active_tab.set(true)
                        >
                            {t!(i18n, login_tab_account)}
                        </button>
                    </div>

                    <Show when=move || error_msg().is_some()>
                        <div class="mb-6 rounded-lg bg-red-50 border border-red-200 p-4 text-sm text-red-700">
                            {move || error_msg().unwrap_or_default()}
                        </div>
                    </Show>

                    <Show when=move || !active_tab.get()>
                        <ActionForm action=customer_action>
                            <div class="space-y-5">
                                <div>
                                    <label for="access_code" class="block text-sm font-medium text-gray-700 leading-7 mb-1">
                                        {t!(i18n, login_access_code_label)}
                                    </label>
                                    <input
                                        id="access_code"
                                        type="text"
                                        name="access_code"
                                        placeholder=move || t_string!(i18n, login_access_code_placeholder)
                                        autocomplete="username"
                                        required
                                        class="w-full bg-gray-100 bg-opacity-50 rounded border border-gray-300 focus:border-indigo-500 focus:bg-white focus:ring-2 focus:ring-indigo-200 text-base outline-none text-gray-700 py-2 px-3 leading-8 transition-colors duration-200 ease-in-out"
                                    />
                                </div>
                                <button
                                    type="submit"
                                    class="w-full text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-300 rounded-lg py-2.5 text-sm font-medium transition-colors disabled:opacity-60"
                                    disabled=move || customer_action.pending().get()
                                >
                                    {move || if customer_action.pending().get() {
                                        t_string!(i18n, login_signing_in)
                                    } else {
                                        t_string!(i18n, login_sign_in)
                                    }}
                                </button>
                            </div>
                        </ActionForm>
                    </Show>

                    <Show when=move || active_tab.get()>
                        <ActionForm action=account_action>
                            <div class="space-y-5">
                                <div>
                                    <label for="email" class="block text-sm font-medium text-gray-700 leading-7 mb-1">
                                        {t!(i18n, login_email_label)}
                                    </label>
                                    <input
                                        id="email"
                                        type="email"
                                        name="email"
                                        placeholder=move || t_string!(i18n, login_email_placeholder)
                                        autocomplete="email"
                                        required
                                        class="w-full rounded-lg border border-gray-300 bg-gray-50 focus:border-indigo-500 focus:bg-white focus:ring-2 focus:ring-indigo-200 text-sm outline-none text-gray-700 py-2 px-3 transition-colors"
                                    />
                                </div>
                                <div>
                                    <label for="password" class="block text-sm font-medium text-gray-700 leading-7 mb-1">
                                        {t!(i18n, login_password_label)}
                                    </label>
                                    <input
                                        id="password"
                                        type="password"
                                        name="password"
                                        placeholder=move || t_string!(i18n, login_password_placeholder)
                                        autocomplete="current-password"
                                        required
                                        class="w-full rounded-lg border border-gray-300 bg-gray-50 focus:border-indigo-500 focus:bg-white focus:ring-2 focus:ring-indigo-200 text-sm outline-none text-gray-700 py-2 px-3 transition-colors"
                                    />
                                </div>
                                <button
                                    type="submit"
                                    class="w-full text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-300 rounded-lg py-2.5 text-sm font-medium transition-colors disabled:opacity-60"
                                    disabled=move || account_action.pending().get()
                                >
                                    {move || if account_action.pending().get() {
                                        t_string!(i18n, login_signing_in)
                                    } else {
                                        t_string!(i18n, login_sign_in)
                                    }}
                                </button>
                                <div class="pt-4 mt-2 border-t border-gray-200 text-center">
                                    <span class="text-sm text-gray-500">{t!(i18n, login_no_account)}</span>
                                    {" "}
                                    <a href="/register" class="text-sm text-indigo-600 hover:underline">{t!(i18n, login_register_link)}</a>
                                </div>
                            </div>
                        </ActionForm>
                    </Show>
                </div>
            </div>
        </div>
    }
}
