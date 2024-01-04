use cfg_if::cfg_if;
use http::status::StatusCode;
use leptos::*;
use thiserror::Error;


#[component]
pub fn Header() -> impl IntoView {
    view! {
      <header class="bg-accent shadow-lg text-gray-300 body-font">
        <div class="container flex mx-auto px-5 py-8 sm:flex-row flex-col items-center">
          <a href="/" class="flex title-font font-medium items-center text-gray-900 mb-4 md:mb-0">
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

          <nav class="md:ml-auto flex flex-wrap items-center text-base justify-center">
            <a href="/login" class="mr-5 hover:text-white">Login</a>
          </nav>


          <div class="w-10 h-10 rounded-full inline-flex items-center justify-center bg-white text-gray-400">

            <svg fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="w-5 h-5" viewBox="0 0 24 24">
              <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"></path>
              <circle cx="12" cy="7" r="4"></circle>
            </svg>
          </div>

        </div>
      </header>
    }
}
