# MemPalace

MemPalace 3.8.0 is retained historical memory and a temporal KG, not eight
clean current-checkout indexes. Production state must remain unchanged.

Identities, indexes, scores and results remain provider-local and are never
merged. See [capabilities](capabilities.md) for every tool/schema disposition,
[preparation](preparation.md) for ordered source hygiene, and
[validation](validation.md) for runnable evidence and remaining gaps.

## Vocabulary mapping

| MemPalace term | What it means | Our term (protocol / estate) |
| --- | --- | --- |
| palace | the on-disk memory store | memory store |
| wing | provider-local historical scope | historical source scope |
| room / taxonomy | classification bucket inside a wing | category |
| drawer | one stored memory record | memory record |
| filing | writing a record into the store | capture |
| mine | bulk-extract memories from files or conversations | ingest |
| diary | per-session summary entries | session summary (startup recall) |
| temporal KG fact / invalidate / supersede | time-versioned knowledge assertions | temporal facts and versions |
| tunnels / hallways | cross-memory links between records | no equivalent — not exposed |
| mesh / peers | multi-store synchronization | no equivalent — not exposed |

## State, scope and nonmutation policy

Native home `~/.mempalace` aliases its own provider directory under
`<workspace>/.maestro/state/providers/mempalace`. No mine, sync, prune,
consolidate, delete, repair or reindex was performed. Existing wing identities
remain distinct: hyphenated, underscored and wing-prefixed scopes can coexist.
Mining defaults normalize hyphens, explicit wings can preserve them, and diary
uses its own default. A wing is not automatically one canonical repository.

Immutable checkpointed SQLite metadata showed 18,243 drawers, including 9,834
source paths under graphify-out and 855 under snapshot/state trees. These are
historical provenance observations, not permission to delete or merge memory,
and not a live transactional count or retrieval-quality benchmark.

Native KG/logstream reads may initialize schema/WAL/storage. `--read-only`
refuses mutating tools but is not a no-filesystem-write sandbox. The
`memories_filed_away` tool acknowledges a checkpoint by unlinking its marker;
it is a mutation, never a health check. `sync.apply` defaults false but was not
called in production. Tool names/defaults do not override the no-write rule.

## Verified useful isolated workflows

With synthetic HOME plus explicit `--palace`, offline settings and scratch-only
storage, native KG add -> supersede -> as_of -> timeline -> invalidate -> query
preserved expected temporal versions. A date-only end includes that entire day;
an exact UTC instant is a different boundary. Invalid dates failed explicitly.
Native artifact put/get preserved exact bytes/hash; event append/list/wait/ack
and patch-submit worked on isolated storage. Patch-submit stores an artifact
and event; it does not apply a patch or prove distributed delivery.

`MEMPALACE_PALACE_PATH` alone does not isolate the KG/hook siblings. Use scratch
HOME and explicit --palace, private logs/runtime and no warmup/forwarding for
native tests. Actual stdio transport, Chroma retrieval quality, drawer writes,
mesh/hallways/tunnels and distributed behavior remain untested or blocked.

## Model and preparation hazards

Search/dedup can load all-MiniLM-L6-v2; it is unauthorized in this task. The
sidecar's dimension0 does not prove the actual 384-dimensional collections
are unusable. No embedding query/model load was executed.

Native query sanitizer can drop leading negation/exclusions from a long query;
review original vs cleaned semantics and bypass lossy rewrite. Transcript
strip_noise removes hook-looking text even inside code fences, so it must not
clean generic repository prose/code. Content/name/date validators and exact
plaintext normalization/chunk fixtures have narrower positive evidence; see
[preparation](preparation.md). Original code/inline terms/negation/structure and
source/wing/time provenance are retained, not silently rewritten.

The memory shim described elsewhere is design, not implemented capture. Native
coordination is a useful provider-local capability, not justification to fuse
provider results or treat historical assistant prose as source truth.
