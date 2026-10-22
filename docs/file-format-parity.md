# File format parity

> **Last reviewed:** 2026-10-22 · **Last updated:** 2026-10-22 · **Change:** minor (multi-artboard and linked AI text recovery; scores unchanged) · **Target:** public documentation and owner requirements

Every format Illustrator opens, places, saves or exports, against Vectnor's format table (`FORMATS`,
`OPEN_EXTS`, `PLACE_EXTS` in `crates/engine/src/cmd/fileio/mod.rs`). Illustrator's list comes from its public
documentation (helpx "file formats" and Save/Export pages), not from its installed bundle: the clean-room rule
forbids reading the install. Part of [target-app-parity.md](target-app-parity.md).

**Fidelity** is estimated unless a test is named. Test sources: unit and property tests per crate
(`crates/svg/tests`, `crates/pdf/tests`, `crates/format/tests/prop_format.rs`), import fuzzing
(`crates/engine/tests/import_fuzz.rs`), the Affinity corpus (`cargo xtask corpus --affinity`, 51 pinned public
files, CI job `affinity-corpus.yml`). There is **no real-world Illustrator corpus**: test inputs are written
from the public specifications, and users' own `.ai` files are used locally only.

## Summary

| | Illustrator | Vectnor | Notes |
|---|---:|---:|---|
| Formats opened or placed | ~30 | 22 | missing: FreeHand, CorelDRAW, DWG, HEIC/HEIF, JPEG 2000, PICT, PCX, Pixar, RTF/Word, PS |
| Formats saved (Save As) | 6 (AI, AIT, PDF, EPS, SVG, SVGZ) | 6 + native | `.ai` written as PDF-compatible only (no Illustrator editing data) |
| Formats exported | ~16 | 17 | missing: DWG, 3D (OBJ/USDA/glTF); extra: GIF/PNG-8 palette export, DXF up to 2018, layered PSD |
| **Overall (weighted by use)** | | **~75%** | `.ai` and PDF dominate the weighting |

## The main format: `.ai`

The format that decides beta. Illustrator's `.ai` is a PDF (or, for old files, PostScript) plus Illustrator's
private editing data. Vectnor:

- **Reads** `.ai`/`.ait` editing data first when the reader reports no omitted art or text, without
  importing or comparing the optional PDF drawing. This preserves the native artboard even when the
  PDF page includes bleed or margins. Explicit page, non-default crop, outline or layer-off options
  use the PDF path. Incomplete editing data retains page-assisted recovery and loss warnings.
  Ordinary PDF files keep page-first import. Opens Illustrator EPS through its own PostScript
  Level 3 interpreter. A `.ai` in PostScript form (versions 3 to 8, and what Rhino and other CAD apps
  write) is its own editing data: it opens through the layers its program has, compared with its page
  when the program runs, from the layers alone when it names its prolog without including it (#1027).
- **Writes** a PDF-compatible `.ai` (Save As › `ai`): Illustrator opens it as plain art. It never writes
  Illustrator's private editing data, which is undocumented.
- **Known failures in users' files:** layer structure flattened or multiplied (#951: 109 layers instead of
  14; #868: round trip with Illustrator 2018 loses layers and groups), guides and non-printing construction
  lines lost (#779), some files 4× slower to open since the editing data is read whole (#758), live effects,
  brushes and symbols come in as their drawn look (#637; pattern fills are read since #1025); legacy `.ai`
  without the prolog opens since #1027, its legacy type (`To` … `TO`) not read yet.
- Estimate: **~65%** for real exchange with Illustrator users (open: ~75%; save: PDF-compatible only).
  Closing the open-side issues: 10–20 h with users' files.

### Illustrator's private editing data

the app's own editing copy of a file (an EPS after its `%%EOF`, a `.ai`'s `AIPrivateData` streams, Zstandard or zlib, the EPS's in ASCII85) is not documented as a whole. Vectnor reads its structure: layers and sublayers (names, order, visibility, lock, printing, preview, dimming, colour), groups as they were nested, compound paths, clipping groups and layer clipping, object names, hidden and locked objects, fills and strokes (grey, CMYK, RGB, named and spot colours, global process colours as global swatches at their tint), linear and radial gradients, pattern fills with their patterns as pattern swatches (#1025), opacity, blend modes, isolation and knockout, embedded images with their alpha channel, guides, and every artboard where it is. An object with several fills or strokes, effects or a brush comes in as its drawn look (a group named after it). Type is made from the file's text document, whole and editable, shown or not: point type, area type (a story in several frames as threaded type) and type on a path, with its characters, fonts, size, leading, tracking, scaling, baseline shift, fill and stroke (grey, RGB or CMYK), alignment, indents and paragraph spacing; the page's drawing of it (often in pieces) is left out. Where the text document can't say (a frame of another kind, or a document that doesn't agree with the page), type that shows comes from the page into its text object's place (no group around it), and hidden type is left out with a note. Kerning, underline and the other character and paragraph options aren't read yet. A `.ai` saved without PDF compatibility (a placeholder page) opens from its editing data alone, its point type, area type and type on a path made from the text document. Symbols, pattern fills whose pattern it can't read, placed files and anything else it doesn't read make the file come in as its page (on a layer that doesn't show, they are left out of that layer), with a warning saying what. For EPS and page-assisted recovery, the layers' art is also compared with the page (by lightness, so colours written as their CMYK equivalents compare alike, and by whether a pixel is inked at all); the page is used when they differ by more than 5%, where one draws an object the other doesn't (away from type, whose lines may be laid out afresh), and when an EPS whose page is its art's box has art printed outside it (#735). `textAs: "outlines"` opens a `.ai` whose type shows from its PDF part, which has the outlines; `editingData: false` opens any of them as its page or PDF part only, for print pipelines (#735). A file whose import left something out (hidden text, art or layers) is never written over by an export or Save As to its own path unless `acknowledgeLoss: true` (`convert --replace-lossy`), so the original keeps what the document lacks. Where a `.ai` can't be read that way (or its layers are turned off in the import options), we read its PDF-compatible part, so Illustrator-only live objects arrive as appearance, and plain groups arrive ungrouped: that part draws each object on its own and marks only layers, clipping groups and groups with opacity, blending or a mask (those come back). Art outside the artboards isn't in that part at all (Illustrator's PDF part leaves it out), so it doesn't open; the import says so in a note (#472). That part also has no paragraphs: area type is written line by line, and comes back as area type only where its font is installed and lays the lines out with the same breaks, else as a point type object per line (#508). Art off the page that a PDF does draw opens on the pasteboard.

### Original text-frame recovery during artwork fallback

If unsupported artwork forces the PDF fallback, Vectnor separately reads the original AI
text stories and frame identities. Complete, unambiguous matches restore one editable text
object per original frame, with its paragraph breaks, area/point type and character runs.
Separate original frames stay separate. This uses the original editing data, rather than
joining nearby lines into an assumed paragraph.

Recovery is bounded and conservative: matching complete artboard sets, contiguous fragments
within each frame's graphics context, matching complete text and orientation, and unique ownership.
Complete linked stories restore every original frame and link atomically. Unread source frames,
ambiguous/mismatched text, different crops or missing artboards, path-type fallback, masks or
intervening artwork retain their existing page objects. Direct native AI import continues
to handle supported threaded/path type. Original-frame recovery requires readable editing
data; PDF-only text cannot reliably reveal the original boxes. Reopen the original `.ai`
to benefit; earlier `.vcnr` conversions do not retain the discarded AI frame metadata.
See [tests and limitations](vectnor/AI_IMPORT.md#original-text-frames--2026-10-22).

### Local AI opening check — 2026-10-22

A user-supplied AI file exposed an editing-data rejection caused by a PDF page 36 pt wider and
taller than its native artboard. Direct editing-data import recovers its A4 artboard, groups,
compound paths and four editable text objects. Three fonts remain missing at the final bundled-app check and use fallback fonts;
this does not establish fidelity for unsupported AI features or other files. The supplied file,
its decoded data and generated artwork remain local and are never committed. Synthetic tests
cover contradictory page artwork, differing page sizes, AI templates, explicit PDF options,
page-assisted text recovery and hostile editing data. See [verification](vectnor/AI_IMPORT.md).

Public behavior reference: [saving artwork](https://helpx.adobe.com/illustrator/using/saving-artwork.html).
No Adobe application, installation or embedded prolog was used as implementation source.

## Format by format

| Format | Illustrator | Ours: open/place | Ours: save/export | Fidelity and tests |
|---|---|---|---|---|
| Vectnor `.vcnr` / `.vctemplate` | — | yes | yes | lossless JSON, versioned, save down to v1; property-tested round trips (`format/tests/prop_format.rs`) |
| Illustrator `.ai` | open, save | yes (PDF-compatible + editing-data structure) | PDF-compatible only | see above; no real-file corpus |
| Illustrator template `.ait` | open, save | yes | no (save as `.vctemplate`) | |
| PDF | open, place, save | yes (layers, masks, editable text, security) | yes (presets, PDF/X-1a/3/4, layers, marks and bleed, ICC, fast web view, 1.3+) | fuzzed import; PDF text placement issues in some files (#722) |
| EPS | open, place, save | yes (own PostScript L3 interpreter, Illustrator EPS editing data structure) | yes (L2/L3, TIFF previews) | CID-keyed CJK type and CCITTFax images not read; #505 |
| PostScript `.ps` | open | no | via Print to PostScript | |
| SVG / SVGZ | open, place, save | yes | yes (SVG Options, Preserve Editing, symbols, filters, rich text) | strongest format; tests in `crates/svg/tests` |
| DXF | open, export | yes | yes (R12–2018) | |
| DWG | open, export | no | no | no open spec; DXF instead |
| EMF / WMF | open, export | yes | yes | |
| PSD | open, place, export | yes (merged image, every mode and depth) | yes (layered, max editability) | placing as layers (Photoshop Import Options) missing |
| PSB | open | yes (merged image) | no | |
| PNG / JPEG / GIF / BMP / TIFF / Targa | open, export | yes (Targa: export only) | yes | Targa import missing |
| WebP | export (Export for Screens) | yes | yes (lossless only; lossy written lossless with a warning) | |
| HEIC / HEIF | open, place | no | no | |
| JPEG 2000 | open, place | no | no | |
| PICT, PCX, Pixar PXR | open, place (legacy) | no | no | rarely used |
| FreeHand `.fh*` | open (legacy) | no | no | |
| CorelDRAW `.cdr` | open | no | no | |
| Text `.txt` | place, export | place (Text Import Options) | yes | |
| RTF / Word `.doc`/`.docx` | place | no | no | |
| CSS | export (CSS Properties) | — | yes | |
| OBJ / USDA / glTF | 3D export | — | no | needs 3D (gap G3) |
| Affinity `.af`, `.afdesign`, `.afpub` | no | yes (beyond Illustrator) | no (#665) | 51 pinned files, [affinity-validation.md](affinity-validation.md) |
| Clipboard: PDF, SVG, PNG, EMF, text | yes | yes | yes | copied files paste as art |

## Reviewed upstream hardening

EPS `arc`/`arcn` normalize huge angles in constant time, preserving bounded sweeps and current-point behavior. Closed DXF SPLINE exports carry the closed bit as well as the planar bit. Original synthetic tests cover both; circle/arc fitting and other listed format gaps remain. [Audit](vectnor/UPSTREAM_AUDIT.md).

## Local correctness follow-up — 2026-10-13

Generated RGB ICC profiles have aligned tag offsets and still decode to the same colour space (#1589);
no profile files are imported. macOS saves use plain fsync when a full flush is unsupported, with injected
failure/cleanup tests (#1625); actual SMB verification is pending. Chart CSV keeps exact numeric values
rather than display-rounded values (#1485). Native `.vcnr`/`vectnor` identity is unchanged. See the
[upstream audit](vectnor/UPSTREAM_AUDIT.md). No format score change is claimed.

## Local file-safety follow-up — 2026-10-17

File Info dates change only when saving succeeds, including background saves with intervening edits.
Replacement staging is owner-only on Unix, retains target permissions and preserves the original on
failure. Export prefixes/suffixes remain filename text inside the export folder. Embedded DIB PNG/JPEG
headers are size-checked before pixel decode. `ruzstd` 0.9.1 fixes the documented decompression bomb.
[Source pins and platform limits](vectnor/UPSTREAM_AUDIT.md). Native format identity and scores are unchanged.

## Local format follow-up — 2026-10-21

Graph CSV preserves quoted delimiters/newlines and huge axis labels. SVG title/description filters XML controls and CSS font-family names are escaped/quoted. CLI/MCP raster scale leaves vector DXF drawing units intact. Checked mesh dimensions, polygon counts and a global EMF tile budget bound malformed work. These do not establish general XML/importer parity. [Pinned PR audit](vectnor/UPSTREAM_AUDIT.md). Scores unchanged.

## Local EPS and palette follow-up — 2026-10-22

EPS readline treats CRLF as one terminator, preserving counted metadata and raw/buffered payloads. Huge finite number formatting avoids accidental inf; numeric range validation is separate. CSS palette gradient stops retain fractional percentages. Synthetic fixtures only. CDR proposals #3123/#2890 remain deferred pending full parser/fuzz/file review, so the existing unsupported CDR row remains accurate. [Pinned audit](vectnor/UPSTREAM_AUDIT.md). Scores unchanged.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-22 | minor | Supplied two-artboard AI: recover translated and linked original text frames, identity point matrices, saved first baselines and script positions; 143 fragments become 10 text objects locally. Scores unchanged. [Evidence](vectnor/AI_IMPORT.md#multi-artboard-linked-frames--2026-10-22). |
| 2026-10-22 | minor | Recover original AI text frames during artwork fallback: multiline and mixed-style text stays together; ambiguous ownership stays unchanged. Scores unchanged. [Evidence](vectnor/AI_IMPORT.md#original-text-frames--2026-10-22). |
| 2026-10-22 | minor | Prefer complete AI editing data before its optional PDF drawing; local real-file check and synthetic margin/option/hostile-input coverage. Scores unchanged. [Evidence](vectnor/AI_IMPORT.md). |
| 2026-10-22 | minor | Nineteen reviewed upstream scopes for ordered Indic/AltGr input, threaded text, brush references, clipping, EPS/CSS, exact-size shapes, Recent tabs and panel scrolling. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-21 | minor | Thirty-two reviewed local upstream adaptations: Indic caret/joiners, style resets, selection, bounded mesh/EMF processing, CSV/SVG and raster-scale export correctness. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-17 | minor | Eighteen reviewed upstream adaptations for save/export safety, bounded processing, automation framing and editing; scores unchanged. [Audit](vectnor/UPSTREAM_AUDIT.md). No upload. |
| 2026-10-13 | minor | Reviewed upstream correctness pass: twenty updated adaptations and two alignment dependencies; English interface/Indic text retained; scores unchanged. See the [audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-12 | minor | Nine reviewed upstream fixes adapted: Shape Builder seams/cutting, EPS angle hangs, renderer safety, saved views, MCP no-replay, stroke visibility, DXF closed splines and stable generated sources. Pagenor structure/provenance conventions aligned; scores unchanged. |
| 2026-10-12 | minor | Native document extension changed to `.vcnr`; save/open filters, file associations, recovery, examples and documentation agree. Stable `vectnor` format header and parity scores unchanged. |
| 2026-10-11 | minor | `.ai` in PostScript form opens through its layers, also without its prolog (#1027) |
| 2026-10-11 | minor | `.ai` editing data: global process colours and pattern fills read, so they no longer drop sublayers (#1025) |
| 2026-10-10 | major | First checklist; moved the `.ai` editing-data scope from the ROADMAP's "Out of scope" here; added the user-reported `.ai` failures |
