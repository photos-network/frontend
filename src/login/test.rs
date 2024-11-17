use crate::api;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn Login() -> impl IntoView {
	let login = create_resource(
		api::fetch_api::<api::Login>(&api::login())
	);

	view! {
		<Suspense fallback=|| view! {  "Loading..." }>
		<Meta name="description" content=meta_description/>
            {move || login.get().map(|login| match login {
                None => view! { <div class="item-view">"Error loading login."</div> },
                Some(login) => view! {
					<h1>Login loaded</h1>
				}
		}
		}
		)
		}
        </Suspense>
	}
}
