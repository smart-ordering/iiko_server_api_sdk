# SDK 0.2 dependency migration

The SDK moves from reqwest 0.12.28 / quick-xml 0.38.4 to reqwest 0.13.5 /
quick-xml 0.42.0. This is a 0.2 release because the public `IikoError`
conversions and XML error variants refer to types from those dependencies.
Endpoint methods, DTO field names, and session ownership remain unchanged.

Rust 1.99.0 is pinned for development and CI. The committed Cargo.lock makes
the standalone development/test graph reproducible; applications consuming
the SDK still resolve dependencies through their own Cargo.lock.

## HTTP compatibility

Reqwest 0.13 changes default TLS and makes form/query support opt-in:
[official release notes](https://github.com/seanmonstar/reqwest/releases/tag/v0.13.0).
The SDK enables native TLS, form/query, JSON, charset decoding, HTTP/2 support,
and system-proxy support explicitly. The client selects the native TLS backend
and HTTP/1.1 explicitly. Earlier SDK reqwest 0.12 native TLS did not negotiate
HTTP/2 through ALPN; a consumer's combined reqwest features must not silently
change that protocol choice. Certificate and hostname validation remain enabled.

The default timeout of zero still means no client-wide deadline. Explicit
client deadlines, sequential request execution, the shared session cache,
one reauthentication/retry for eligible reads, and no application-level replay
of document mutations are preserved. `reports/productExpense` retains its
300-second request override and 64 MiB response limit. Other bounded reads
retain their existing byte limits and strict UTF-8 decoding.

## XML compatibility and intentional parser changes

Quick-xml 0.42 replaces byte-oriented event/name access with UTF-8 strings:
[official changelog](https://github.com/tafia/quick-xml/blob/v0.42.0/Changelog.md).
The internal read tree keeps its previous QName spelling, entity handling,
XML 1.1 text newline normalization, raw CDATA, and unescaped attribute values.
It does not adopt attribute whitespace normalization while adapting to the
new event API. Unknown fields and repeated children remain available.

Typed serde XML now honors the declared XML version. For implicit/explicit
XML 1.0, literal NEL (U+0085) and line separator (U+2028) are preserved;
quick-xml 0.38 previously normalized them to LF. Declared XML 1.1 retains that
normalization. Numeric character references are unaffected. Ordinary CR/CRLF,
Unicode names, outer string whitespace, empty/omitted fields, document numbering,
and ordered item arrays are covered by the shared before/after contract suite.

The typed serde parser also limits in-scope namespace bindings to 128 and rejects
excessive nesting. These are intentional rejection changes for pathological
input. SDK invoice tests cover the namespace limit and scope release.
The custom internal read tree retains its existing byte bound; it does not
inherit these typed parser namespace/depth limits.
Quick-xml 0.38's dependency branch is removed, including the versions affected
by [RUSTSEC-2026-0194](https://rustsec.org/advisories/RUSTSEC-2026-0194.html) and
[RUSTSEC-2026-0195](https://rustsec.org/advisories/RUSTSEC-2026-0195.html).
This does not constitute a repository-wide vulnerability audit.

The existing generic `Request<T>` uses a flattened map that the XML serializer
does not support; it was already unsupported on quick-xml 0.38. This migration
does not change that public helper. Contract tests use actual endpoint request
DTOs rather than claiming that the generic map was working.

## Test boundaries

Many historical files under tests/ perform real iiko reads and writes using
credentials loaded from .env. Do not run an unrestricted `cargo test` against
those credentials. CI compiles all targets but runs only library tests and
explicit synthetic contract targets:

```sh
cargo check --locked --all-targets
cargo test --locked --lib \
  --test incoming_invoice_numbering \
  --test xml_wire_contract \
  --test json_wire_contract \
  --test xml_namespace_security \
  --test http_auth_contract \
  --test http_timeouts_reports_contract \
  --test tls_transport_contract \
  -- --test-threads=1
```

Timing tests run serially so CPU contention does not consume their deadline headroom.

HTTP/TLS contract processes require proxy variables to be unset/empty and
NO_PROXY=127.0.0.1,localhost. OpenSSL is required; synthetic certificate keys
are generated in isolated temporary directories and removed with the server
processes. TLS tests verify SDK rejection/redaction and a separate trusted-CA
HTTP/1.1 control; they do not prove SDK trust against every production OS root
store or a deployed iiko integration. No live iiko request is needed for these
tests.

Runtime Tokio only enables sync. Test/example Tokio features are kept in
dev-dependencies. Unused anyhow, tokio-test, and once_cell dependencies are
removed. The read-only probe example preserves lowercase, zero-padded SHA-1
hash output after its development dependency moves to SHA-1 0.11.
