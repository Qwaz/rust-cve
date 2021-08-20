# Rust CVE

## Preface

This is a list of CVEs for unsound APIs in the Rust standard library.
These bugs break Rust's memory safety guarantee
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

†: Reported by me
‡: Reported as part of [Rudra](https://github.com/sslab-gatech/Rudra) project

[tockos]: https://www.tockos.org/
[redleaf]: https://mars-research.github.io/redleaf

## CVE List

| CVE | Issue # | Title | Affected | RustSec |
| --- | ------- | ----- | -------- | ------- |

{%- for entry in cve %}
{{ entry|cve_entry }}
{%- endfor %}

## Backlog

These are soundness bugs that I plan to apply CVE IDs for.
Note that I focus on soundness bugs that arise from misuse of unsafe Rust code in the standard library,
so certain soundness bugs are not included in this list such as type-system bugs
(e.g., [#25860](https://github.com/rust-lang/rust/issues/25860))
or environmental bugs
(e.g., [#81996](https://github.com/rust-lang/rust/issues/81996)).
I'll still update the CVE list above if such bug get assigned a CVE ID.

| Issue # | Title | Affected | CVE ID Requested |
| ------- | ----- | -------- | ---------------- |

{%- for entry in backlog %}
{{ entry|backlog_entry }}
{%- endfor %}
