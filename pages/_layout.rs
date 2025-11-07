pub struct LayoutSlots {
    pub content: String,
    pub title: Option<String>,
    pub footer: Option<String>,
}

#[layout]
pub fn layout(slots: LayoutSlots) {
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{slots.title.unwrap_or("RHTML App".to_string())}</title>
  <script src="https://cdn.tailwindcss.com"></script>
</head>

<body class="bg-gray-50">
  <nav class="bg-blue-600 text-white p-4">
    <div class="container mx-auto">
      <a href="/" class="font-bold text-xl">RHTML App</a>
    </div>
  </nav>

  <main class="container mx-auto p-4">
    {slots.content}
  </main>

  <footer class="bg-gray-800 text-white p-4 mt-8">
    <div class="container mx-auto text-center">
      {slots.footer.unwrap_or("© 2024 RHTML".to_string())}
    </div>
  </footer>
</body>

</html>
}
