use askama::Template;
use semver::Version;
use serde::Deserialize;

mod filters {
    use reqwest::StatusCode;
    use semver::Version;

    use crate::{BacklogItem, CveItem};

    fn escape_md(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('<', "\\<")
            .replace('>', "\\>")
            .replace('#', "\\#")
            .replace('|', "&#124;")
    }

    fn affected_string(introduced: &Option<Version>, fixed: &Option<Version>) -> String {
        match (introduced, fixed) {
            (Some(from), Some(to)) => format!(">= {}, < {}", from, to),
            (Some(from), None) => format!(">= {}", from),
            (None, None) => String::from("TODO"),
            _ => panic!("fixed field is set without introduced field"),
        }
    }

    pub fn cve_entry(entry: &CveItem) -> askama::Result<String> {
        let affected = affected_string(&entry.introduced, &entry.fixed);

        let rustsec_url = format!("https://rustsec.org/advisories/{}.html", entry.id);
        let response = reqwest::blocking::get(&rustsec_url).expect("Failed to get URL");
        let rustsec = if response.status() != StatusCode::OK {
            String::from("TODO")
        } else {
            format!("[link]({})", rustsec_url)
        };

        Ok(format!(
            "| [{id}]({id_url}) | [{issue}{found_by_symbol}]({issue_url}) | {title} | {affected} | {rustsec} |",
            id = entry.id,
            id_url = format!(
                "https://cve.mitre.org/cgi-bin/cvename.cgi?name={}",
                entry.id
            ),
            issue = entry.issue,
            issue_url = format!("https://github.com/rust-lang/rust/issues/{}", entry.issue),
            found_by_symbol = entry.found_by.symbol(),
            title = escape_md(&entry.title),
            affected = affected,
            rustsec = rustsec
        ))
    }

    pub fn backlog_entry(entry: &BacklogItem) -> askama::Result<String> {
        let affected = affected_string(&entry.introduced, &entry.fixed);

        Ok(format!(
            "| [{issue}{found_by_symbol}]({issue_url}) | {title} | {affected} | {applied} |",
            issue = entry.issue,
            issue_url = format!("https://github.com/rust-lang/rust/issues/{}", entry.issue),
            found_by_symbol = entry.found_by.symbol(),
            title = escape_md(&entry.title),
            affected = affected,
            applied = if entry.applied { "Yes" } else { "No" }
        ))
    }
}

#[derive(Deserialize, Clone, Copy)]
pub enum FoundBy {
    Someone,
    Myself,
    Rudra,
}

impl Default for FoundBy {
    fn default() -> Self {
        FoundBy::Someone
    }
}

impl FoundBy {
    fn symbol(self) -> &'static str {
        match self {
            FoundBy::Someone => "",
            FoundBy::Myself => "†",
            FoundBy::Rudra => "‡",
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct CveItem {
    id: String,
    issue: usize,
    #[serde(default)]
    found_by: FoundBy,
    introduced: Option<Version>,
    fixed: Option<Version>,
    title: String,
}

#[derive(Deserialize, Clone)]
pub struct BacklogItem {
    issue: usize,
    #[serde(default)]
    found_by: FoundBy,
    introduced: Option<Version>,
    fixed: Option<Version>,
    title: String,
    applied: bool,
}

#[derive(Template, Deserialize, Clone)]
#[template(path = "README.md")]
pub struct Data {
    cve: Vec<CveItem>,
    backlog: Vec<BacklogItem>,
}

fn main() {
    let data_str = std::fs::read_to_string("data.toml").expect("failed to open `data.toml`");
    let mut data: Data = toml::from_str(&data_str).expect("`data.toml` is malformed");

    data.cve.sort_by(|r1, r2| r1.id.cmp(&r2.id));
    data.backlog.sort_by(|r1, r2| r1.issue.cmp(&r2.issue));

    let rendered = data.render().expect("Failed to render");
    std::fs::write("README.md", rendered).expect("failed to write `README.md`");

    println!("Successfully generated README.md");
}
