use crate::auth::{check_session_kind, SessionKind};
use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn ProtectedLayout(required: SessionKind, children: ChildrenFn) -> impl IntoView {
    let session = Resource::new(|| (), |_| check_session_kind());
    view! {
        <Suspense fallback=|| ()>
            {move || match session.get() {
                None => view! { <div/> }.into_any(),
                Some(Ok(kind)) if kind == required => children().into_any(),
                Some(Ok(SessionKind::None)) => view! { <LoginRequired/> }.into_any(),
                Some(_) => view! { <PermissionDenied/> }.into_any(),
            }}
        </Suspense>
    }
}

#[component]
pub fn ProtectedLayoutMulti(allowed: Vec<SessionKind>, children: ChildrenFn) -> impl IntoView {
    let session = Resource::new(|| (), |_| check_session_kind());
    view! {
        <Suspense fallback=|| ()>
            {move || match session.get() {
                None => view! { <div/> }.into_any(),
                Some(Ok(kind)) if allowed.contains(&kind) => children().into_any(),
                Some(Ok(SessionKind::None)) => view! { <LoginRequired/> }.into_any(),
                Some(_) => view! { <PermissionDenied/> }.into_any(),
            }}
        </Suspense>
    }
}

#[component]
pub fn PermissionDenied() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <section class="text-gray-600 body-font">
            <div class="container px-5 py-24 mx-auto text-center">
                <h1 class="sm:text-3xl text-2xl font-medium title-font mb-2 text-gray-900">
                    {t!(i18n, access_denied_title)}
                </h1>
                <p class="lg:w-1/2 w-full leading-relaxed text-gray-500 mx-auto">
                    {t!(i18n, access_denied_description)}
                </p>
                <a href="/" class="inline-flex mt-8 text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg">
                    {t!(i18n, access_denied_go_back)}
                </a>
            </div>
        </section>
    }
}

#[component]
pub fn LoginRequired() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <section class="text-gray-600 body-font">
            <div class="container px-5 py-24 mx-auto">
                <div class="flex flex-wrap w-full mb-20 flex-col items-center text-center">
                    <h1 class="sm:text-3xl text-2xl font-medium title-font mb-2 text-gray-900">
                        {t!(i18n, login_required_title)}
                    </h1>
                    <p class="lg:w-1/2 w-full leading-relaxed text-gray-500">
                        {t!(i18n, login_required_description)}
                    </p>
                </div>
                <div class="flex flex-wrap -m-4">
                    <div class="xl:w-1/3 md:w-1/2 p-4">
                        <div class="border border-gray-200 p-6 rounded-lg">
                            <div class="w-10 h-10 inline-flex items-center justify-center rounded-full bg-indigo-100 text-indigo-500 mb-4">
                                <svg fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="w-6 h-6" viewBox="0 0 24 24">
                                    <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"></path>
                                    <circle cx="12" cy="7" r="4"></circle>
                                </svg>
                            </div>
                            <h2 class="text-lg text-gray-900 font-medium title-font mb-2">{t!(i18n, login_required_people_title)}</h2>
                            <p class="leading-relaxed text-base">{t!(i18n, login_required_people_description)}</p>
                        </div>
                    </div>
                    <div class="xl:w-1/3 md:w-1/2 p-4">
                        <div class="border border-gray-200 p-6 rounded-lg">
                            <div class="w-10 h-10 inline-flex items-center justify-center rounded-full bg-indigo-100 text-indigo-500 mb-4">
                                <svg fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="w-6 h-6" viewBox="0 0 24 24">
                                    <path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1zM4 22v-7"></path>
                                </svg>
                            </div>
                            <h2 class="text-lg text-gray-900 font-medium title-font mb-2">{t!(i18n, login_required_location_title)}</h2>
                            <p class="leading-relaxed text-base">{t!(i18n, login_required_location_description)}</p>
                        </div>
                    </div>
                    <div class="xl:w-1/3 md:w-1/2 p-4">
                        <div class="border border-gray-200 p-6 rounded-lg">
                            <div class="w-10 h-10 inline-flex items-center justify-center rounded-full bg-indigo-100 text-indigo-500 mb-4">
                                <svg fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="w-6 h-6" viewBox="0 0 24 24">
                                    <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
                                </svg>
                            </div>
                            <h2 class="text-lg text-gray-900 font-medium title-font mb-2">{t!(i18n, login_required_privacy_title)}</h2>
                            <p class="leading-relaxed text-base">{t!(i18n, login_required_privacy_description)}</p>
                        </div>
                    </div>
                </div>
                <a href="/login" class="flex mx-auto mt-16 text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg">
                    {t!(i18n, login_required_login_button)}
                </a>
            </div>
        </section>
    }
}
