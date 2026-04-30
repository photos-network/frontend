use crate::i18n::*;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[server]
async fn check_customer_session() -> Result<bool, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let has_token = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .map(|cookies| {
            cookies.split(';').any(|pair| {
                let mut parts = pair.trim().splitn(2, '=');
                parts.next().map(|n| n.trim()) == Some("customer_token")
                    && parts.next().map(|v| !v.trim().is_empty()).unwrap_or(false)
            })
        })
        .unwrap_or(false);

    Ok(has_token)
}

#[server]
async fn customer_logout() -> Result<(), ServerFnError> {
    use axum::http::{header, HeaderValue};
    use leptos_axum::ResponseOptions;

    let response_options = expect_context::<ResponseOptions>();
    let cookie = "customer_token=; Path=/; SameSite=Lax; Max-Age=0";
    if let Ok(val) = HeaderValue::from_str(cookie) {
        response_options.append_header(header::SET_COOKIE, val);
    }

    Ok(())
}

#[component]
pub fn Header() -> impl IntoView {
    let i18n = use_i18n();
    let session_version = use_context::<RwSignal<u32>>().unwrap_or(RwSignal::new(0));
    let session = Resource::new(move || session_version.get(), |_| check_customer_session());
    let logout_action = ServerAction::<CustomerLogout>::new();
    let navigate = use_navigate();

    Effect::new(move |_| {
        if logout_action.value().get().and_then(|r| r.ok()).is_some() {
            session_version.update(|v| *v += 1);
            navigate("/", Default::default());
        }
    });

    view! {
      <header class="bg-accent shadow-lg text-gray-300 body-font">
        <div class="container flex flex-nowrap mx-auto px-5 py-8 items-center">
          <a href="/" class="flex grow title-font font-medium items-center text-gray-900 mb-0">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-10 h-10 text-white p-2 bg-white rounded-full" width="40" height="40" viewBox="0 0 2008 2008" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;">
              <g id="außen" transform="matrix(1,0,0,1,-328.484,-623.52)">
                <path d="M1094.51,2405.45L1332.42,2268.1L1570.33,2405.45L1332.42,2542.81L1094.51,2405.45ZM1245.66,2218.01L1007.75,2355.36L864.368,2272.58L864.368,1997.87L1245.66,2218.01ZM1800.48,2272.58L1657.09,2355.36L1419.18,2218.01L1800.48,1997.87L1800.48,2272.58ZM2125.14,2085.13L1887.23,2222.49L1887.23,1947.78L2125.14,1810.42L2125.14,2085.13ZM539.701,1810.42L777.61,1947.78L777.61,2222.49L539.701,2085.13L539.701,1810.42ZM2125.14,1544.67L2125.14,1710.24L1887.23,1847.6L1887.23,1407.32L2125.14,1544.67ZM777.61,1847.6L539.701,1710.24L539.701,1544.67L777.61,1407.32L777.61,1847.6ZM777.61,1032.42L777.61,1307.14L539.701,1444.49L539.701,1169.78L777.61,1032.42ZM2125.14,1169.78L2125.14,1444.49L1887.23,1307.14L1887.23,1032.42L2125.14,1169.78ZM1245.66,1036.9L864.368,1257.05L864.368,982.333L1007.75,899.549L1245.66,1036.9ZM1800.48,982.333L1800.48,1257.05L1419.18,1036.9L1657.09,899.549L1800.48,982.333ZM1570.33,849.459L1332.42,986.816L1094.51,849.459L1332.42,712.102L1570.33,849.459Z" style="fill:rgb(2,0,50);"/>
              </g>
              <g id="außen1" serif:id="außen" transform="matrix(1,0,0,1,-328.484,-623.52)">
                <path d="M1800.48,1357.23L1800.48,1897.69L1332.42,2167.92L864.368,1897.69L864.368,1357.23L1332.42,1087L1800.48,1357.23Z" style="fill:rgb(113,108,255);"/>
              </g>
            </svg>
            <span class="ml-3 text-xl">Photos.network</span>
          </a>

            <Suspense fallback=|| ()>
              {move || {
                let logged_in = session.get().and_then(|r| r.ok()).unwrap_or(false);
                if logged_in {
                  view! {
                    <span class="inline">
                      <ActionForm action=logout_action>
                        <button
                          type="submit"
                          class="ml-4 hover:text-white cursor-pointer bg-transparent border-0 text-base text-gray-300"
                        >
                          {t!(i18n, header_logout)}
                        </button>
                      </ActionForm>
                    </span>
                  }.into_any()
                } else {
                  view! {
                  }.into_any()
                }
              }}
            </Suspense>

            <button
              type="button"
              title="Switch language"
              class="text-xl ml-4 leading-none hover:scale-110 transition-transform cursor-pointer bg-transparent border-0 pl-4"
              on:click=move |_| {
                if i18n.get_locale() == Locale::en {
                  i18n.set_locale(Locale::de);
                } else {
                  i18n.set_locale(Locale::en);
                }
              }
            >
              {move || if i18n.get_locale() == Locale::en { "🇩🇪" } else { "🇬🇧" }}
            </button>


        </div>
      </header>
    }
}
