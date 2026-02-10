/// Wrap body content in a full HTML page with Tailwind CDN
pub fn html_page(title: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Pact Web</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-50 min-h-screen">
    {nav}
    <main class="max-w-4xl mx-auto py-8 px-4">
        {body}
    </main>
</body>
</html>"#,
        title = title,
        nav = html_nav("Pact Web", &[("/", "Users"), ("/users/new", "New User")]),
        body = body,
    )
}

/// Top navigation bar
pub fn html_nav(title: &str, links: &[(&str, &str)]) -> String {
    let link_items: String = links
        .iter()
        .map(|(href, label)| {
            format!(
                r#"<a href="{}" class="text-gray-300 hover:text-white px-3 py-2 text-sm font-medium">{}</a>"#,
                href, label
            )
        })
        .collect();

    format!(
        r#"<nav class="bg-gray-800">
    <div class="max-w-4xl mx-auto px-4 py-3 flex items-center justify-between">
        <span class="text-white font-bold text-lg">{}</span>
        <div class="flex space-x-4">{}</div>
    </div>
</nav>"#,
        title, link_items
    )
}

/// Render a Tailwind-styled table
pub fn html_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let header_cells: String = headers
        .iter()
        .map(|h| format!(r#"<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{}</th>"#, h))
        .collect();

    let body_rows: String = rows
        .iter()
        .map(|row| {
            let cells: String = row
                .iter()
                .map(|cell| {
                    format!(
                        r#"<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{}</td>"#,
                        cell
                    )
                })
                .collect();
            format!("<tr class=\"hover:bg-gray-50\">{}</tr>", cells)
        })
        .collect();

    format!(
        r#"<div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 rounded-lg">
    <table class="min-w-full divide-y divide-gray-300">
        <thead class="bg-gray-50">
            <tr>{}</tr>
        </thead>
        <tbody class="divide-y divide-gray-200 bg-white">{}</tbody>
    </table>
</div>"#,
        header_cells, body_rows
    )
}

/// Render a Tailwind-styled form
pub fn html_form(action: &str, fields: &[(&str, &str, &str)]) -> String {
    let field_html: String = fields
        .iter()
        .map(|(name, label, input_type)| {
            format!(
                r#"<div class="mb-4">
    <label for="{name}" class="block text-sm font-medium text-gray-700 mb-1">{label}</label>
    <input type="{input_type}" name="{name}" id="{name}"
        class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm px-3 py-2 border"
        required>
</div>"#,
                name = name,
                label = label,
                input_type = input_type,
            )
        })
        .collect();

    format!(
        r#"<form method="POST" action="{}" class="bg-white shadow rounded-lg p-6 max-w-md">
    {}
    <button type="submit"
        class="w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 font-medium">
        Submit
    </button>
</form>"#,
        action, field_html
    )
}

/// Success/error alert
pub fn html_alert(kind: &str, message: &str) -> String {
    let (bg, border, text) = match kind {
        "success" => ("bg-green-50", "border-green-400", "text-green-700"),
        "error" => ("bg-red-50", "border-red-400", "text-red-700"),
        "warning" => ("bg-yellow-50", "border-yellow-400", "text-yellow-700"),
        _ => ("bg-blue-50", "border-blue-400", "text-blue-700"),
    };
    format!(
        r#"<div class="{} border-l-4 {} p-4 mb-4">
    <p class="{}">{}</p>
</div>"#,
        bg, border, text, message
    )
}
