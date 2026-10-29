# Turkish interface

The Turkish catalog (`crates/ui-egui/src/i18n/tr.rs`) contains 1318 labels: the
shared ribbon catalog plus File view, dialog titles and fields, message boxes,
tooltips, sheet menus, chart and analysis tools, PivotTable fields, printer
controls and engine command labels. Untranslated text continues to fall back to
English. This covers all keys in the Japanese core and dialog catalogs; the
application also has hardcoded strings outside those catalogs.

Select **View → Interface language → Türkçe**. On a first visit with no saved
language preference, a `tr-TR` browser/system language selects Turkish. A saved
choice takes precedence; the web app stores it in `gridcraft.ui` in localStorage.
Changing the interface language does not translate workbook contents, sheet names
or command identifiers.

## Dialogs and command search

The shared dialog rendering now looks up previously hardcoded labels and formats
translated message templates without translating document titles or cell values.
Where one English word needs two Turkish ones, a context key keeps them apart:
Paste Special's arithmetic **Add** is "Topla", Solver's constraint **Add**
(`Solver|Add`) is "Ekle".

The command palette displays translated command names and menu paths, and accepts
Turkish label searches as well as canonical English names and command identifiers.
Interface searches respect the dotted/dotless `I` distinction; canonical identifiers
retain their language-independent case folding.

## Formulas

Turkish function names, descriptions and argument help are the subject of PR #1712;
this catalog does not add them. Formula identifiers and XLSX formulas remain
English. Typed Turkish workbook text, including the distinct letters `I`, `İ`, `ı`
and `i`, is preserved rather than case-folded or translated.

## Validation

- The i18n tests require every Japanese core and dialog key to have a nonempty
  Turkish translation with intact interpolation placeholders and no leaked
  `context|` prefix or unintended line break.
- Semantic samples cover save/discard, protect/unprotect, case-sensitive search,
  sheet deletion, context-dependent Fill and the two meanings of Add.
- Rendered-dialog regressions (`crates/ui-egui/tests/it/turkish_dialogs.rs`)
  exercise Find/Replace, save/discard, Format Cells, command search, the Insert
  Chart preview heading, the Protect Sheet permissions and Solver's Add button,
  while checking canonical tab ids and unchanged data.

## Remaining localization work

Hardcoded labels outside the translation catalog (for example the Macros dialog),
Turkish formula names and regional number/date formatting remain separate work.
PDF font/shaping limitations are unchanged; see
[localization parity](localization-parity.md).
