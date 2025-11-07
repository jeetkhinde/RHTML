<!-- Demo page for request context features -->
<div class="container mx-auto p-8">
    <h1 class="text-4xl font-bold mb-8">Request Context Demo</h1>

    <!-- Request Info -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4">Request Information</h2>
        <div class="space-y-2">
            <p><strong>Method:</strong> {request_method}</p>
            <p><strong>Path:</strong> {request_path}</p>
            <p><strong>Is GET:</strong> {is_get}</p>
            <p><strong>Is POST:</strong> {is_post}</p>
            <p><strong>Accepts JSON:</strong> {accepts_json}</p>
        </div>
    </div>

    <!-- Query Parameters -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4">Query Parameters</h2>
        <p class="text-gray-600 mb-4">Try: <code>/request-demo?name=John&age=25&active=true</code></p>

        <div r-if="query_name">
            <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded">
                <p><strong>Name:</strong> {query_name}</p>
                <p r-if="query_age"><strong>Age:</strong> {query_age}</p>
                <p r-if="query_active"><strong>Active:</strong> {query_active}</p>
            </div>
        </div>
        <div r-else>
            <p class="text-gray-500">No query parameters found. Add <code>?name=value</code> to the URL.</p>
        </div>
    </div>

    <!-- Form Handling -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4">Form Submission</h2>

        <div r-if="form_username">
            <div class="bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded mb-4">
                <p><strong>Form submitted!</strong></p>
                <p><strong>Username:</strong> {form_username}</p>
                <p r-if="form_email"><strong>Email:</strong> {form_email}</p>
                <p r-if="form_message"><strong>Message:</strong> {form_message}</p>
            </div>
        </div>

        <form method="post" action="/request-demo" class="space-y-4">
            <div>
                <label for="username" class="block text-sm font-medium text-gray-700">Username</label>
                <input
                    type="text"
                    name="username"
                    id="username"
                    required
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                />
            </div>

            <div>
                <label for="email" class="block text-sm font-medium text-gray-700">Email</label>
                <input
                    type="email"
                    name="email"
                    id="email"
                    required
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                />
            </div>

            <div>
                <label for="message" class="block text-sm font-medium text-gray-700">Message</label>
                <textarea
                    name="message"
                    id="message"
                    rows="3"
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                ></textarea>
            </div>

            <button
                type="submit"
                class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
                Submit Form
            </button>
        </form>
    </div>

    <!-- Content Negotiation -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4">Content Negotiation</h2>
        <p class="text-gray-600 mb-4">
            This page supports content negotiation. Try requesting with different Accept headers:
        </p>

        <div class="bg-gray-100 p-4 rounded font-mono text-sm space-y-2">
            <p><strong>HTML (default):</strong></p>
            <code class="block">curl http://localhost:3000/request-demo</code>

            <p class="mt-4"><strong>JSON:</strong></p>
            <code class="block">curl -H "Accept: application/json" http://localhost:3000/request-demo</code>

            <p class="mt-4"><strong>Or use query parameter:</strong></p>
            <code class="block">curl http://localhost:3000/request-demo?api=true</code>
        </div>
    </div>

    <!-- API Examples -->
    <div class="bg-white rounded-lg shadow p-6">
        <h2 class="text-2xl font-semibold mb-4">API Examples</h2>
        <p class="text-gray-600 mb-4">Test with curl commands:</p>

        <div class="space-y-4">
            <div>
                <p class="font-semibold">GET with query params (JSON response):</p>
                <code class="block bg-gray-100 p-2 rounded text-sm">
                    curl -H "Accept: application/json" "http://localhost:3000/request-demo?name=Alice&age=30"
                </code>
            </div>

            <div>
                <p class="font-semibold">POST with form data (JSON response):</p>
                <code class="block bg-gray-100 p-2 rounded text-sm">
                    curl -X POST -H "Accept: application/json" \<br/>
                    &nbsp;&nbsp;-d "username=bob&email=bob@example.com" \<br/>
                    &nbsp;&nbsp;http://localhost:3000/request-demo
                </code>
            </div>

            <div>
                <p class="font-semibold">POST with JSON data:</p>
                <code class="block bg-gray-100 p-2 rounded text-sm">
                    curl -X POST -H "Content-Type: application/json" \<br/>
                    &nbsp;&nbsp;-H "Accept: application/json" \<br/>
                    &nbsp;&nbsp;-d '{"username":"charlie","email":"charlie@example.com"}' \<br/>
                    &nbsp;&nbsp;http://localhost:3000/request-demo
                </code>
            </div>
        </div>
    </div>

    <!-- Back to Home -->
    <div class="mt-8">
        <a href="/" class="text-blue-600 hover:text-blue-800 underline">← Back to Home</a>
    </div>
</div>

<style>
code {
    background-color: #f3f4f6;
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
}
</style>
