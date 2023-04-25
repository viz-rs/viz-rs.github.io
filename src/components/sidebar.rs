use leptos::*;

#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
    view! { cx,
        <aside class="fixed z-35 flex flex-col p-5 gap-4 sidebar top-4.375rem bottom-0">
            <section>
                <h3 class="py-1 text-4 font-medium">"Get Started"</h3>
                <ul class="text-3.5">
                    <li>
                        <a href="/docs/0.4.x/guide/introduction" class="inline-block py-1 font-normal transition-colors hover:op100 op100 text-yellow-600">"Introduction"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/guide/quick-start" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Quick Start"</a>
                    </li>
                </ul>
            </section>
            <section>
                <h3 class="py-1 text-4 font-medium">"Concepts"</h3>
                <ul class="text-3.5">
                    <li>
                        <a href="/docs/0.4.x/concepts/requests-and-responses" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Request &amp; Response"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/concepts/handler" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Handler"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/concepts/middleware" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Middleware"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/concepts/routing" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Routing"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/concepts/extractors" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Extractors"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/concepts/server" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Server"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/concepts/error-handling" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Error Handling"</a>
                    </li>
                </ul>
            </section>
            <section>
                <h3 class="py-1 text-4 font-medium">"Built-in"</h3>
                <ul class="text-3.5">
                    <li>
                        <a href="/docs/0.4.x/built-ins/handlers" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Handlers"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/built-ins/middleware" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Middleware"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/built-ins/extractors" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Extractors"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/built-ins/tls" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"TLS"</a>
                    </li>
                </ul>
            </section>
            <section>
                <h3 class="py-1 text-4 font-medium">"Extra Topics"</h3>
                <ul class="text-3.5">
                    <li>
                        <a href="/docs/0.4.x/extra-topics/benchmarks" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Benchmarks"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/extra-topics/templates" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Templates"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/extra-topics/examples" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Examples"</a>
                    </li>
                    <li>
                        <a href="/docs/0.4.x/extra-topics/showcase" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Showcase"</a>
                    </li>
                </ul>
            </section>
            <section>
                <h3 class="py-1 text-4 font-medium">"Others"</h3>
                <ul class="text-3.5">
                    <li>
                        <a href="/docs/0.4.x/others/sponsor" class="inline-block py-1 font-normal transition-colors hover:op100 op61.8">"Sponsor"</a>
                    </li>
                </ul>
            </section>
        </aside>
    }
}
