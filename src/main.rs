use askama::Template;
use semver::Version;
use serde::Deserialize;

fn escape_md(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('#', "\\#")
        .replace('|', "&#124;")
}

mod filters {
    use reqwest::StatusCode;

    use crate::{escape_md, Report};

    pub fn cve_entry(report: &Report) -> askama::Result<String> {
        let affected = match (&report.introduced, &report.fixed) {
            (Some(from), Some(to)) => format!(">= {}, < {}", from, to),
            (Some(from), None) => format!(">= {}", from),
            (None, None) => String::from("TODO"),
            _ => panic!("fixed field is set without introduced field"),
        };

        let rustsec_url = format!("https://rustsec.org/advisories/{}.html", report.id);
        let response = reqwest::blocking::get(&rustsec_url).expect("Failed to get URL");
        let rustsec = if response.status() != StatusCode::OK {
            String::from("TODO")
        } else {
            format!("[link]({})", rustsec_url)
        };

        Ok(format!(
            "| [{id}]({id_url}) | [{issue}]({issue_url}) | {title} | {affected} | {rustsec} |\n",
            id = report.id,
            id_url = format!(
                "https://cve.mitre.org/cgi-bin/cvename.cgi?name={}",
                report.id
            ),
            issue = report.issue,
            issue_url = format!("https://github.com/rust-lang/rust/issues/{}", report.issue),
            title = escape_md(&report.title),
            affected = affected,
            rustsec = rustsec
        ))
    }
}

#[derive(Deserialize, Clone)]
pub struct Report {
    id: String,
    issue: usize,
    introduced: Option<Version>,
    fixed: Option<Version>,
    title: String,
}

#[derive(Template, Deserialize, Clone)]
#[template(path = "README.md")]
pub struct Data {
    cve: Vec<Report>,
}

fn main() {
    let data_str = std::fs::read_to_string("data.toml").expect("failed to open `data.toml`");
    let mut data: Data = toml::from_str(&data_str).expect("`data.toml` is malformed");
    data.cve.sort_by(|r1, r2| r1.id.cmp(&r2.id));

    let rendered = data.render().expect("Failed to render");
    std::fs::write("README.md", rendered).expect("failed to write `README.md`");
}
