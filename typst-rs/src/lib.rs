pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn render(str: String) -> String {
    markdown::to_html(&str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn html_test() {
        let html = render("# Hello World".to_string());
        assert_eq!(html, "<h1>Hello World</h1>")
    }
}

uniffi::include_scaffolding!("typst");
