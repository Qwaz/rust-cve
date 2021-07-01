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
| [CVE-2015-20001](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2015-20001) | [25842](https://github.com/rust-lang/rust/issues/25842) | Panic safety violation in BinaryHeap | >= 1.0.0, < 1.2.0 | [link](https://rustsec.org/advisories/CVE-2015-20001.html) |
| [CVE-2017-20004](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-20004) | [41622](https://github.com/rust-lang/rust/issues/41622) | MutexGuard\<Cell\<i32\>\> must not be Sync | >= 1.0.0, < 1.19.0 | TODO |
| [CVE-2018-1000657](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-1000657) | [44800](https://github.com/rust-lang/rust/issues/44800) | Buffer overflow vulnerability in VecDeque::reserve\(\) | >= 1.3.0, < 1.22.0 | [link](https://rustsec.org/advisories/CVE-2018-1000657.html) |
| [CVE-2018-1000810](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-1000810) | [54399](https://github.com/rust-lang/rust/issues/54399) | Buffer overflow vulnerability in str::repeat\(\) | >= 1.26.0, < 1.29.1 | [link](https://rustsec.org/advisories/CVE-2018-1000810.html) |
| [CVE-2018-25008](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-25008) | [51780](https://github.com/rust-lang/rust/issues/51780) | Insufficient synchronization in `Arc::get_mut` | >= 1.3.0, < 1.29.0 | TODO |
| [CVE-2019-1010299](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-1010299) | [53566](https://github.com/rust-lang/rust/issues/53566) | vec_deque::Iter has unsound Debug implementation | >= 1.18.0, < 1.30.0 | TODO |
| [CVE-2019-12083](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-12083) | [60787](https://github.com/rust-lang/rust/issues/60787) | Memory safety vulnerabilities arising from `Error::type_id` | >= 1.34.0, < 1.34.2 | [link](https://rustsec.org/advisories/CVE-2019-12083.html) |
| [CVE-2020-36317](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-36317) | [78498](https://github.com/rust-lang/rust/issues/78498) | String::retain allows safely creating invalid strings when abusing panic | >= 1.26.0, < 1.49.0 | [link](https://rustsec.org/advisories/CVE-2020-36317.html) |
| [CVE-2020-36318](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-36318) | [79808](https://github.com/rust-lang/rust/issues/79808) | VecDeque::make_contiguous may duplicate the contained elements | >= 1.48.0, < 1.49.0 | [link](https://rustsec.org/advisories/CVE-2020-36318.html) |
| [CVE-2020-36323](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-36323) | [80335](https://github.com/rust-lang/rust/issues/80335) | API soundness issue in join\(\) implementation of \[Borrow\<str\>\] | >= 1.28.0, < 1.52.0 | TODO |
| [CVE-2021-28875](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-28875) | [80894](https://github.com/rust-lang/rust/issues/80894) | Logic bug in Read can cause buffer overflow in read_to_end\(\) | >= 1.20.0, < 1.50.0 | [link](https://rustsec.org/advisories/CVE-2021-28875.html) |
| [CVE-2021-28876](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-28876) | [81740](https://github.com/rust-lang/rust/issues/81740) | Panic safety issue in Zip specialization | >= 1.14.0, < 1.52.0 | [link](https://rustsec.org/advisories/CVE-2021-28876.html) |
| [CVE-2021-28877](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-28877) | [80670](https://github.com/rust-lang/rust/issues/80670) | TrustedRandomAaccess specialization composes incorrectly for nested iter::Zips | >= 1.11.0, < 1.51.0 | [link](https://rustsec.org/advisories/CVE-2021-28877.html) |
| [CVE-2021-28878](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-28878) | [82291](https://github.com/rust-lang/rust/issues/82291) | Zip may call __iterator_get_unchecked twice with the same index | >= 1.14.0, < 1.52.0 | [link](https://rustsec.org/advisories/CVE-2021-28878.html) |
| [CVE-2021-28879](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-28879) | [82282](https://github.com/rust-lang/rust/issues/82282) | Zip can cause buffer overflow when a consumed Zip iterator is used again | >= 1.14.0, < 1.52.0 | [link](https://rustsec.org/advisories/CVE-2021-28879.html) |
| [CVE-2021-31162](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-31162) | [83618](https://github.com/rust-lang/rust/issues/83618) | Double free in Vec::from_iter specialization when drop panics | >= 1.48.0, < 1.52.0 | TODO |


## Backlog

I believe these bugs also deserve CVE numbers,
but they are not reported to CVE authority (e.g., MITRE) yet.