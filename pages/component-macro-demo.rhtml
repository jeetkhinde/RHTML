// Component Macro System Demo
//
// This file demonstrates the new #[component] macro for defining
// public and private components with proper visibility handling.
//
// Public Components: Accessible via ?partial=name query parameter
// Private Components: File-scoped, used only within this template

// ===== PUBLIC COMPONENTS (accessible via HTTP) =====

// Public Component 1: Analytics Widget
// Access: GET /component-macro-demo?partial=analytics
// This component shows statistics and is accessible from external requests
partial analytics(props: &PartialProps<()>) {
    <div id="analytics" class="bg-white rounded-lg shadow-lg p-6">
        <h3 class="text-lg font-bold mb-4">Analytics Dashboard</h3>
        <div class="grid grid-cols-3 gap-4">
            <div class="text-center">
                <div class="text-3xl font-bold text-blue-600">1,234</div>
                <div class="text-sm text-gray-600">Total Users</div>
            </div>
            <div class="text-center">
                <div class="text-3xl font-bold text-green-600">892</div>
                <div class="text-sm text-gray-600">Active Users</div>
            </div>
            <div class="text-center">
                <div class="text-3xl font-bold text-purple-600">156</div>
                <div class="text-sm text-gray-600">New This Month</div>
            </div>
        </div>
    </div>
}

// Public Component 2: Status Indicator
// Access: GET /component-macro-demo?partial=status_indicator
// Shows current system status
partial status_indicator(props: &PartialProps<()>) {
    <div class="bg-green-50 border-l-4 border-green-500 p-4 rounded">
        <div class="flex items-center gap-2">
            <div class="w-3 h-3 rounded-full bg-green-500"></div>
            <div>
                <h4 class="font-semibold text-green-800">System Status</h4>
                <p class="text-sm text-green-700">All systems operational</p>
            </div>
        </div>
    </div>
}

// Public Component 3: Feature Card
// Access: GET /component-macro-demo?partial=feature_card
// Displays a single feature/capability
partial feature_card(props: &PartialProps<()>) {
    <div class="bg-white rounded-lg border border-gray-200 p-5 hover:shadow-md transition-shadow">
        <div class="text-2xl mb-2">✨</div>
        <h4 class="font-bold text-gray-800 mb-2">Component System</h4>
        <p class="text-sm text-gray-600 mb-3">
            The new #[component] macro makes it easy to define reusable UI components
            with clear public/private visibility.
        </p>
        <a href="/component-macro-guide" class="text-blue-600 hover:text-blue-800 text-sm font-semibold">
            Learn More →
        </a>
    </div>
}

// ===== PRIVATE COMPONENTS (file-scoped only) =====

// Private Component 1: User Card
// This component is only used within this template
// NOT accessible via HTTP query parameters
fn user_card(props: &PartialProps<()>) {
    <div class="bg-white rounded-lg shadow p-4 mb-4">
        <div class="flex items-center gap-4">
            <div class="w-12 h-12 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold">
                JD
            </div>
            <div class="flex-1">
                <h4 class="font-semibold text-gray-800">John Doe</h4>
                <p class="text-sm text-gray-500">john@example.com</p>
            </div>
            <span class="px-3 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded-full">
                Active
            </span>
        </div>
    </div>
}

// Private Component 2: Stat Item
// Reusable private component for displaying statistics
fn stat_item(props: &PartialProps<()>) {
    <div class="p-4 bg-gray-50 rounded border border-gray-200">
        <div class="text-2xl font-bold text-blue-600">42</div>
        <div class="text-sm text-gray-600 mt-1">Items Processed</div>
    </div>
}

// Private Component 3: Badge
// Used multiple times within public components
fn badge(props: &PartialProps<()>) {
    <span class="inline-block px-2 py-1 bg-blue-100 text-blue-800 text-xs font-semibold rounded">
        New
    </span>
}

// ===== MAIN PAGE =====

WebPage {
    <div class="min-h-screen bg-gray-50 p-8">
        <div class="max-w-4xl mx-auto">
            <h1 class="text-4xl font-bold mb-2">Component Macro System</h1>
            <p class="text-gray-600 mb-8">
                Demonstration of the new #[component] macro for public and private components
            </p>

            <!-- Public Components Section -->
            <section class="mb-12">
                <h2 class="text-2xl font-bold mb-4">Public Components</h2>
                <p class="text-gray-600 mb-6">
                    These components are accessible via HTTP query parameters (e.g., ?partial=analytics)
                </p>

                <div class="space-y-6">
                    <div>
                        <h3 class="text-lg font-semibold mb-2">Analytics Widget</h3>
                        <analytics />
                    </div>

                    <div>
                        <h3 class="text-lg font-semibold mb-2">Status Indicator</h3>
                        <status_indicator />
                    </div>

                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <h3 class="text-lg font-semibold mb-2">Feature Cards</h3>
                            <feature_card />
                        </div>
                        <div>
                            <feature_card />
                        </div>
                    </div>
                </div>
            </section>

            <!-- Private Components Section -->
            <section class="mb-12">
                <h2 class="text-2xl font-bold mb-4">Private Components</h2>
                <p class="text-gray-600 mb-6">
                    These components are file-scoped and NOT accessible via HTTP
                </p>

                <div class="space-y-6">
                    <div>
                        <h3 class="text-lg font-semibold mb-3">User Cards (Private Component)</h3>
                        <div class="space-y-3">
                            <user_card />
                            <user_card />
                            <user_card />
                        </div>
                    </div>

                    <div>
                        <h3 class="text-lg font-semibold mb-3">Stats Grid (Private Component)</h3>
                        <div class="grid grid-cols-3 gap-4">
                            <stat_item />
                            <stat_item />
                            <stat_item />
                        </div>
                    </div>
                </div>
            </section>

            <!-- Usage Examples -->
            <section class="bg-white rounded-lg shadow p-6">
                <h2 class="text-2xl font-bold mb-4">Usage Guide</h2>

                <div class="space-y-4">
                    <div>
                        <h3 class="font-bold text-gray-800 mb-2">Accessing Public Components</h3>
                        <div class="bg-gray-100 p-3 rounded font-mono text-sm">
                            GET /component-macro-demo?partial=analytics
                        </div>
                    </div>

                    <div>
                        <h3 class="font-bold text-gray-800 mb-2">With HTMX</h3>
                        <div class="bg-gray-100 p-3 rounded font-mono text-sm overflow-x-auto">
                            &lt;div hx-get="/component-macro-demo?partial=analytics"
                                 hx-trigger="load"&gt;
                                Loading...
                            &lt;/div&gt;
                        </div>
                    </div>

                    <div class="bg-blue-50 border-l-4 border-blue-500 p-4 rounded">
                        <h4 class="font-bold text-blue-900 mb-2">Key Points</h4>
                        <ul class="text-sm text-blue-800 space-y-1">
                            <li>✓ Public components start with <code>pub fn</code> - accessible via HTTP</li>
                            <li>✓ Private components (no <code>pub</code>) - file-scoped only</li>
                            <li>✓ Access via <code>?partial=component_name</code> query parameter</li>
                            <li>✓ Great for HTMX dynamic updates and SPA interactions</li>
                        </ul>
                    </div>
                </div>
            </section>

            <!-- Documentation Link -->
            <div class="mt-8 text-center">
                <p class="text-gray-600 mb-4">
                    For more information, see the
                    <a href="/COMPONENT_MACRO_GUIDE.md" class="text-blue-600 hover:text-blue-800 font-semibold">
                        Component Macro Guide
                    </a>
                </p>
            </div>
        </div>
    </div>
}
