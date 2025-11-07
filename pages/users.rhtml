// Named Partials Example - All user-related partials in one file!

// Named partial: Stats
// URL: /users?partial=Stats
partial Stats(props: &PartialProps<()>) {
    <div class="bg-white rounded-lg shadow-lg p-6">
        <h2 class="text-2xl font-bold mb-4">User Statistics</h2>
        <div class="grid grid-cols-3 gap-4">
            <div class="text-center p-4 bg-blue-50 rounded">
                <div class="text-3xl font-bold text-blue-600">1,234</div>
                <div class="text-sm text-gray-600">Total Users</div>
            </div>
            <div class="text-center p-4 bg-green-50 rounded">
                <div class="text-3xl font-bold text-green-600">892</div>
                <div class="text-sm text-gray-600">Active Users</div>
            </div>
            <div class="text-center p-4 bg-purple-50 rounded">
                <div class="text-3xl font-bold text-purple-600">156</div>
                <div class="text-sm text-gray-600">New This Month</div>
            </div>
        </div>
    </div>
}

// Named partial: ActiveUsers
// URL: /users?partial=ActiveUsers
partial ActiveUsers(props: &PartialProps<()>) {
    <div class="bg-white rounded-lg shadow-lg p-6">
        <h2 class="text-2xl font-bold mb-4">Active Users</h2>
        <div class="space-y-3">
            <div class="flex items-center gap-3 p-3 hover:bg-gray-50 rounded">
                <div class="w-10 h-10 rounded-full bg-blue-500 flex items-center justify-center text-white font-bold">
                    JD
                </div>
                <div>
                    <div class="font-semibold">John Doe</div>
                    <div class="text-sm text-gray-500">john@example.com</div>
                </div>
                <span class="ml-auto px-2 py-1 text-xs bg-green-100 text-green-800 rounded">
                    Online
                </span>
            </div>
            <div class="flex items-center gap-3 p-3 hover:bg-gray-50 rounded">
                <div class="w-10 h-10 rounded-full bg-purple-500 flex items-center justify-center text-white font-bold">
                    AS
                </div>
                <div>
                    <div class="font-semibold">Alice Smith</div>
                    <div class="text-sm text-gray-500">alice@example.com</div>
                </div>
                <span class="ml-auto px-2 py-1 text-xs bg-green-100 text-green-800 rounded">
                    Online
                </span>
            </div>
            <div class="flex items-center gap-3 p-3 hover:bg-gray-50 rounded">
                <div class="w-10 h-10 rounded-full bg-orange-500 flex items-center justify-center text-white font-bold">
                    BJ
                </div>
                <div>
                    <div class="font-semibold">Bob Johnson</div>
                    <div class="text-sm text-gray-500">bob@example.com</div>
                </div>
                <span class="ml-auto px-2 py-1 text-xs bg-green-100 text-green-800 rounded">
                    Online
                </span>
            </div>
        </div>
    </div>
}

// Named partial: RecentActivity
// URL: /users?partial=RecentActivity
partial RecentActivity(props: &PartialProps<()>) {
    <div class="bg-white rounded-lg shadow-lg p-6">
        <h2 class="text-2xl font-bold mb-4">Recent Activity</h2>
        <div class="space-y-4">
            <div class="flex items-start gap-3">
                <div class="w-2 h-2 mt-2 rounded-full bg-blue-500"></div>
                <div>
                    <div class="font-semibold">John Doe logged in</div>
                    <div class="text-sm text-gray-500">2 minutes ago</div>
                </div>
            </div>
            <div class="flex items-start gap-3">
                <div class="w-2 h-2 mt-2 rounded-full bg-green-500"></div>
                <div>
                    <div class="font-semibold">Alice Smith created a project</div>
                    <div class="text-sm text-gray-500">15 minutes ago</div>
                </div>
            </div>
            <div class="flex items-start gap-3">
                <div class="w-2 h-2 mt-2 rounded-full bg-purple-500"></div>
                <div>
                    <div class="font-semibold">Bob Johnson updated profile</div>
                    <div class="text-sm text-gray-500">1 hour ago</div>
                </div>
            </div>
            <div class="flex items-start gap-3">
                <div class="w-2 h-2 mt-2 rounded-full bg-orange-500"></div>
                <div>
                    <div class="font-semibold">Charlie Brown joined</div>
                    <div class="text-sm text-gray-500">3 hours ago</div>
                </div>
            </div>
        </div>
    </div>
}

// Full page (optional) - used when accessing /users without ?partial=
slot! {
    title: "User Management",
    footer: "User Dashboard"
}

#[webpage]
fn page(props: PageProps) {
  <div class="container mx-auto p-8">
    <h1 class="text-4xl font-bold mb-8">User Management Dashboard</h1>

    <p class="text-gray-600 mb-8">
      This page demonstrates named partials. All user-related fragments are in a single file!
    </p>

    <!-- Load partials dynamically -->
    <div class="grid grid-cols-1 gap-6">
      <!-- Stats Section -->
      <div id="stats-section">
        <button
          hx-get="/users?partial=Stats"
          hx-target="#stats-section"
          hx-swap="innerHTML"
          class="mb-4 bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
          Load User Stats
        </button>
      </div>

      <!-- Active Users Section -->
      <div id="active-users-section">
        <button
          hx-get="/users?partial=ActiveUsers"
          hx-target="#active-users-section"
          hx-swap="innerHTML"
          class="mb-4 bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700">
          Load Active Users
        </button>
      </div>

      <!-- Recent Activity Section -->
      <div id="activity-section">
        <button
          hx-get="/users?partial=RecentActivity"
          hx-target="#activity-section"
          hx-swap="innerHTML"
          class="mb-4 bg-purple-600 text-white px-4 py-2 rounded hover:bg-purple-700">
          Load Recent Activity
        </button>
      </div>
    </div>

    <!-- API Examples -->
    <div class="mt-12 bg-gray-50 rounded-lg p-6">
      <h2 class="text-2xl font-semibold mb-4">Direct Partial URLs</h2>
      <div class="space-y-2 font-mono text-sm">
        <div>
          <a href="/users?partial=Stats" class="text-blue-600 hover:underline">
            /users?partial=Stats
          </a>
        </div>
        <div>
          <a href="/users?partial=ActiveUsers" class="text-blue-600 hover:underline">
            /users?partial=ActiveUsers
          </a>
        </div>
        <div>
          <a href="/users?partial=RecentActivity" class="text-blue-600 hover:underline">
            /users?partial=RecentActivity
          </a>
        </div>
      </div>
    </div>

    <!-- Back to Home -->
    <div class="mt-8">
      <a href="/" class="text-blue-600 hover:text-blue-800 underline">← Back to Home</a>
    </div>
  </div>
}
