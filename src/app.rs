use crate::auth::guards::{ProtectedLayout, ProtectedLayoutMulti};
use crate::auth::{check_session_kind, SessionKind};
use crate::footer::Footer;
use crate::header::Header;
use crate::pages::admin::AdminOverview;
use crate::pages::album_detail::CustomerAlbumDetailPage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::login::LoginForm;
use crate::i18n::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_navigate,
    path,
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Shared counter: bump it to force the session resource in Header to refetch.
    let session_version = RwSignal::new(0u32);
    provide_context(session_version);

    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Title text="Photos.network"/>

        <I18nContextProvider>
        <Router>
            <Header/>
            <main>
                <div class="container px-5 py-24 mx-auto">
                <Routes fallback=NotFound>
                    <Route path=StaticSegment("admin") view=move || view! {
                        <ProtectedLayout required=SessionKind::Admin>
                            <AdminOverview/>
                        </ProtectedLayout>
                    }/>
                    <Route path=StaticSegment("") view=StartPage/>
                    <Route path=StaticSegment("login") view=LoginForm/>
                    <Route path=StaticSegment("dashboard") view=move || view! {
                        <ProtectedLayoutMulti allowed=vec![SessionKind::Customer, SessionKind::Account, SessionKind::Admin]>
                            <DashboardPage/>
                        </ProtectedLayoutMulti>
                    }/>
                    <Route path=StaticSegment("albums") view=AlbumsRedirect/>
                    <Route path=path!("/albums/:id") view=move || view! {
                        <ProtectedLayoutMulti allowed=vec![SessionKind::Customer, SessionKind::Account, SessionKind::Admin]>
                            <CustomerAlbumDetailPage/>
                        </ProtectedLayoutMulti>
                    }/>
                </Routes>
                </div>
            </main>
            <Footer/>
        </Router>
        </I18nContextProvider>
    }
}


#[component]
fn StartPage() -> impl IntoView {
    let navigate = use_navigate();
    let session = Resource::new(|| (), |_| check_session_kind());

    Effect::new(move |_| {
        if let Some(Ok(kind)) = session.get() {
            let path = match kind {
                SessionKind::Admin => "/dashboard",
                SessionKind::Account => "/dashboard",
                SessionKind::Customer => "/dashboard",
                SessionKind::None => "/login",
            };
            navigate(path, Default::default());
        }
    });

    view! { <div/> }
}


/// Redirects `/albums` to `/dashboard` so old bookmarks keep working.
#[component]
fn AlbumsRedirect() -> impl IntoView {
    let navigate = use_navigate();
    Effect::new(move |_| {
        navigate("/dashboard", Default::default());
    });
    view! { <div/> }
}

#[component]
fn NotFound() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <main class="ui main container mx-auto">
            <h1>{t!(i18n, not_found)}</h1>
        </main>
    }
}
