use leptos::*;
use leptos_router::*;
use crate::tailwind_form::TailwindForm;

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
pub fn Home() -> impl IntoView {
    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! {
      <div class="mx-auto max-w-7xl px-6 lg:px-8 mt-24">
        <h1 class="text-5xl font-semibold">Home</h1>
      </div>
    }
}

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
pub fn App() -> impl IntoView {
    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! {
      <div class="relative flex flex-col overflow-hidden bg-white">
        <Router>
          <div class="max-w-7xl w-full mx-auto">
            <nav class="border-gray-200 py-2">
              <div class="mx-auto flex flex-wrap items-center justify-between px-2">
                  <A exact=true href="" class="flex">
                    <span class="bg-slate-800 w-12 h-12 rounded-full mr-2">
                      <svg xmlns="http://www.w3.org/2000/svg" xml:space="preserve" id="Layer_1" x="0" y="0" style="enable-background:new 0 0 116 116" version="1.1" viewBox="0 0 116 116"><style id="style2">".st1{fill:#ef3b39}"</style><path id="path6" d="M81.9 15c3.7 0 6.9-2.1 8.5-5.1-5.3-3.6-11.3-6.3-17.6-8-.4 1.1-.6 2.2-.6 3.4 0 5.3 4.4 9.7 9.7 9.7z" class="st1"/><circle id="circle10" cx="58" cy="58" r="22.4" class="st1"/><path id="path12" d="M78.3 21c1.2.3 2.4.4 3.7.4 5.7 0 10.8-3 13.6-7.6-1.6-1.4-3.3-2.7-5.1-3.9-1.7 3-4.9 5.1-8.5 5.1-5.4 0-9.7-4.4-9.7-9.7 0-1.2.2-2.3.6-3.4-2.1-.5-4.2-1-6.3-1.3-.4 1.5-.7 3-.7 4.7 0 4.3 1.7 8.2 4.4 11-3.3 3.9-5.9 8.4-7.9 13.2-1.4-.2-2.9-.3-4.4-.3-15.9 0-28.8 12.9-28.8 28.8 0 13.2 8.9 24.3 21 27.7-3.9 9.9-11.5 18-21 22.6 2.3 1.3 4.6 2.5 7.1 3.4 9.4-5.7 16.7-14.5 20.4-25h1.4c15.9 0 28.8-12.9 28.8-28.8 0-12.2-7.6-22.6-18.2-26.8 1.7-4.1 4-7.9 6.9-11.2.8.5 1.7.8 2.7 1.1zm2.1 37c0 12.4-10.1 22.4-22.4 22.4-12.4 0-22.4-10.1-22.4-22.4 0-12.4 10.1-22.4 22.4-22.4 12.4 0 22.4 10 22.4 22.4z" style="fill:#fff"/></svg>
                    </span>
                    <span class="self-center text-lg font-semibold whitespace-nowrap">"leptos-playground"</span>
                  </A>
                  <button data-collapse-toggle="mobile-menu" type="button" class="md:hidden ml-3 text-gray-400 hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-300 rounded-lg inline-flex items-center justify-center" aria-controls="mobile-menu-2" aria-expanded="false">
                    <span class="sr-only">Open main menu</span>
                    <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
                    <svg class="hidden w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                  </button>
                  <div class="hidden md:block w-full md:w-auto" id="mobile-menu">
                    <ul class="flex-col md:flex-row flex md:space-x-8 mt-4 md:mt-0 md:text-sm md:font-medium">
                        <li>
                          <A exact=true href="" class="text-gray-700 hover:bg-gray-50 border-b border-gray-100 md:hover:bg-transparent md:border-0 block pl-3 pr-4 py-2 md:hover:text-blue-700 md:p-0">Home</A>
                        </li>
                        <li>
                          <A href="tailwind-form" class="text-gray-700 hover:bg-gray-50 border-b border-gray-100 md:hover:bg-transparent md:border-0 block pl-3 pr-4 py-2 md:hover:text-blue-700 md:p-0">TailwindCSS Form</A>
                        </li>
                    </ul>
                  </div>
              </div>
            </nav>
          </div>
          <main>
            <Routes>
              <Route path="" view=Home/>
              <Route path="tailwind-form" view=TailwindForm/>
            </Routes>
          </main>
        </Router>
      </div>
    }
}