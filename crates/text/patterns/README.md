# Indic hyphenation patterns

The eleven `hyph-*.tex` files are unmodified upstream files from
[hyphenation/tex-hyphen](https://github.com/hyphenation/tex-hyphen/tree/5684c0f51c0b81133db2efbe60a408b4155a3ff5/hyph-utf8/tex/generic/hyph-utf8/patterns/tex),
pinned at commit `5684c0f51c0b81133db2efbe60a408b4155a3ff5`.

Hindi, Marathi, Gujarati, Bengali, Punjabi (Gurmukhi), Tamil, Telugu, Kannada,
Malayalam, Odia and Assamese: copyright (C) 2016 Santhosh Thottingal.
Vectnor uses these under their **MIT** license option; the full copyright
and permission notice is preserved at the top of each file. The alternative
LGPL/GPL licenses are not selected.

Sanskrit remains selectable for shaping and uses explicit user exceptions. No Sanskrit
pattern file is bundled, and unsupported languages do not fall back to Hindi or English rules.

The matcher reads only the contents of the TeX `\patterns{…}` block after
removing comments. Odd maximum weights permit breaks and even weights forbid
them (Liang patterns). Vectnor retains its existing fixed paragraph limits: words at least six Unicode
characters long, with at least three characters on each side of the break. Automatic points and preferences
exceptions are restricted to whole Unicode graphemes and actual shaping
cluster starts. Both composers consume the same points. The newer configurable Hyphen Limit and
Hyphenation Zone controls are not part of this adaptation.

Breaks after viramas are conservatively suppressed, including Tamil pulli
breaks permitted by the upstream patterns and Unicode grapheme segmentation.
This preserves consonant clusters even when the font subdivides them.
