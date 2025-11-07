<div class="container mx-auto px-4 py-8 max-w-4xl">
    <h1 class="text-4xl font-bold mb-6 text-indigo-600">📚 Documentation</h1>

    <div class="bg-blue-50 border-l-4 border-blue-500 p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-2">Catch-All Route Demo</h2>
        <p class="text-gray-700 mb-4">
            This page uses a <span class="bg-gray-800 text-white px-2 py-1 rounded font-mono text-sm">[...slug]</span> catch-all parameter
            to match any path under /docs/.
        </p>
    </div>

    <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
        <h3 class="text-xl font-semibold mb-4 text-gray-800">Current Path Information</h3>

        <table class="w-full">
            <tr class="border-b">
                <td class="py-3 px-4 font-semibold bg-gray-50 w-1/3">Full Slug</td>
                <td class="py-3 px-4">
                    {if slug}
                        <code class="bg-gray-100 px-3 py-1 rounded text-sm">{slug}</code>
                    {else}
                        <span class="text-gray-400 italic">(empty - at /docs)</span>
                    {/if}
                </td>
            </tr>
            <tr class="border-b">
                <td class="py-3 px-4 font-semibold bg-gray-50">File Path</td>
                <td class="py-3 px-4">
                    <code class="bg-gray-100 px-3 py-1 rounded text-sm">pages/docs/[...slug].rhtml</code>
                </td>
            </tr>
            <tr>
                <td class="py-3 px-4 font-semibold bg-gray-50">Route Pattern</td>
                <td class="py-3 px-4">
                    <code class="bg-gray-100 px-3 py-1 rounded text-sm">/docs/*slug</code>
                </td>
            </tr>
        </table>
    </div>

    <div class="grid md:grid-cols-2 gap-6 mb-6">
        <div class="bg-gradient-to-br from-green-50 to-green-100 rounded-lg p-6 border border-green-200">
            <h4 class="font-semibold text-lg mb-3 text-green-800">✅ Example URLs That Match</h4>
            <ul class="space-y-2 text-sm">
                <li><a href="/docs" class="text-blue-600 hover:underline">/docs</a> → <code class="text-xs bg-white px-2 py-1 rounded">slug = ""</code></li>
                <li><a href="/docs/intro" class="text-blue-600 hover:underline">/docs/intro</a> → <code class="text-xs bg-white px-2 py-1 rounded">slug = "intro"</code></li>
                <li><a href="/docs/guide/getting-started" class="text-blue-600 hover:underline">/docs/guide/getting-started</a> → <code class="text-xs bg-white px-2 py-1 rounded">slug = "guide/getting-started"</code></li>
                <li><a href="/docs/api/v1/users" class="text-blue-600 hover:underline">/docs/api/v1/users</a> → <code class="text-xs bg-white px-2 py-1 rounded">slug = "api/v1/users"</code></li>
            </ul>
        </div>

        <div class="bg-gradient-to-br from-purple-50 to-purple-100 rounded-lg p-6 border border-purple-200">
            <h4 class="font-semibold text-lg mb-3 text-purple-800">🎯 Use Cases</h4>
            <ul class="space-y-2 text-sm text-gray-700">
                <li>• Documentation sites with nested pages</li>
                <li>• File browsers or directory listings</li>
                <li>• Blog archives (year/month/day/slug)</li>
                <li>• Wildcard API proxies</li>
                <li>• Multi-level category pages</li>
            </ul>
        </div>
    </div>

    <div class="bg-gray-800 text-gray-100 rounded-lg p-6 mb-6">
        <h4 class="font-semibold text-lg mb-3">📝 How It Works</h4>
        <div class="text-sm space-y-4">
            <div>
                <p class="text-gray-400 mb-2">1. Create a file with <code class="bg-gray-700 px-2 py-1 rounded">[...slug]</code> syntax:</p>
                <pre class="bg-gray-900 p-3 rounded overflow-x-auto"><code>pages/docs/[...slug].rhtml</code></pre>
            </div>
            <div>
                <p class="text-gray-400 mb-2">2. Access the slug parameter in your template:</p>
                <pre class="bg-gray-900 p-3 rounded overflow-x-auto"><code>&lt;p&gt;Current path: {slug}&lt;/p&gt;</code></pre>
            </div>
            <div>
                <p class="text-gray-400 mb-2">3. The slug contains all remaining path segments joined with "/"</p>
            </div>
        </div>
    </div>

    <div class="flex gap-4">
        <a href="/" class="inline-block bg-indigo-600 text-white px-6 py-2 rounded hover:bg-indigo-700 transition">
            ← Back to Home
        </a>
        <a href="/posts" class="inline-block bg-green-600 text-white px-6 py-2 rounded hover:bg-green-700 transition">
            Try Optional Parameters →
        </a>
    </div>
</div>
