# Rust CVE

## Preface

This is a list of CVEs for unsound APIs in the Rust standard library.
All of these bugs break Rust's memory safety guarantee
and lead to security issues when triggered.
Fortunately, they are context-sensitive library APIs
that are not usually used in a way that the bugs can be triggered.
Many of them require very specific interaction to trigger
(e.g., partially consume an iterator and `zip()` it with another iterator)
that is not likely to appear in their daily usage.

Yet, we can't say for sure that there is no code out there using these APIs in a bug-triggering way.
Moreover, certain applications such as [TockOS][tockos] and [RedLeaf][redleaf] that use Rust's type system as an isolation mechanism can be easily attacked with these bugs.
Hence, it is important to signal the existence of these bugs,
and I found that issuing a CVE number and creating a RustSec advisory is the most effective way to do so.

[tockos]: https://www.tockos.org/
[redleaf]: https://mars-research.github.io/redleaf

## CVE List

| CVE | Issue # | Title | Affected | RustSec |
| --- | ------- | ----- | -------- | ------- |
{% for entry in cve %}{{ entry|cve_entry }}{% endfor %}

## Backlog

I believe these bugs also deserve CVE numbers,
but they are not reported to CVE authority (e.g., MITRE) yet.
