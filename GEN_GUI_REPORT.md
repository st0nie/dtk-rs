# DTK6 gui pilot binding coverage report

classes: 2, methods generated: 66, skipped: 9

Qt ancestry: 0 widgets, 2 QObjects, 0 opaque types

## skip reasons
- unsupported param type: 5
- unsupported return type: 4

> Pilot scope: only the listed classes/headers were scanned; this is NOT library-wide coverage.

## DGuiApplicationHelper — 29 methods generated, 9 skipped
- `static void generatePaletteColor(DPalette &base, QPalette::ColorRole role, Color` ← unsupported param type: QPalette::ColorRole
- `static DPalette fetchPalette(const DPlatformTheme *theme);` ← unsupported param type: const DPlatformTheme *
- `DPlatformTheme *systemTheme() const;` ← unsupported return type: DPlatformTheme *
- `DPlatformTheme *applicationTheme() const;` ← unsupported return type: DPlatformTheme *
- `const DFontManager *fontManager() const;` ← unsupported return type: const DFontManager *
- `static bool loadTranslator(const QString &fileName, const QString &appName, cons` ← unsupported param type: const QList<QLocale> &
- `static bool loadTranslator(const QString &fileName, const QList<QString> &transl` ← unsupported param type: const QList<QString> &
- `static bool loadTranslator(const QList<QLocale> &localeFallback = QList<QLocale>` ← unsupported param type: const QList<QLocale> &
- `static inline bool isCompactMode()` ← unsupported return type: inline bool

## DImageHandler — 37 methods generated, 0 skipped
