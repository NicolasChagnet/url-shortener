pub const IDX_BDY: &str = r#"
    <!doctype html>
    <html>
        <head></head>
        <body>
            <form action="/add" method="post">
                <label for="to">
                    Enter a url:
                    <input type="text" name="to">
                </label>
                <input type="submit" value="Submit!">
            </form>
        </body>
    </html>
"#;

pub fn add_bdy(from: &str) -> String {
format!(r#"
    <!doctype html>
    <html>
        <head></head>
        <body>
            URL added: <a href="/{}">{}<br />
            <a href="/">Return home</a>
        </body>
    </html>
"#, from, from)
}