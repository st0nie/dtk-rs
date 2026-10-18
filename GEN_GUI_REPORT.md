# DTK6 gui (selected classes) binding coverage report

classes: 7, methods generated: 166, skipped: 19

Qt ancestry: 0 widgets, 6 QObjects, 1 opaque types

## skip reasons
- unsupported param type: 12
- unsupported return type: 7

> Selected-class scope: only the listed classes/headers were scanned; this is NOT library-wide coverage.

## DGuiApplicationHelper — 34 methods generated, 4 skipped
- `static bool loadTranslator(const QString &fileName, const QString &appName, cons` ← unsupported param type: const QList<QLocale> &
- `static bool loadTranslator(const QString &fileName, const QList<QString> &transl` ← unsupported param type: const QList<QLocale> &
- `static bool loadTranslator(const QList<QLocale> &localeFallback = QList<QLocale>` ← unsupported param type: const QList<QLocale> &
- `static inline bool isCompactMode()` ← unsupported return type: inline bool

## DImageHandler — 37 methods generated, 0 skipped

## DFontManager — 20 methods generated, 0 skipped

## DNativeSettings — 3 methods generated, 1 skipped
- `QByteArrayList allKeys() const;` ← unsupported return type: QByteArrayList

## DPlatformTheme — 43 methods generated, 1 skipped
- `DPalette fetchPalette(const DPalette &base, bool *ok = nullptr) const;` ← unsupported param type: bool *

## DWindowManagerHelper — 11 methods generated, 9 skipped
- `static void setMotifFunctions(const QWindow *window, MotifFunctions hints);` ← unsupported param type: const QWindow *
- `static MotifFunctions setMotifFunctions(const QWindow *window, MotifFunctions hi` ← unsupported return type: MotifFunctions
- `static MotifFunctions getMotifFunctions(const QWindow *window);` ← unsupported return type: MotifFunctions
- `static void setMotifDecorations(const QWindow *window, MotifDecorations hints);` ← unsupported param type: const QWindow *
- `static MotifDecorations setMotifDecorations(const QWindow *window, MotifDecorati` ← unsupported return type: MotifDecorations
- `static MotifDecorations getMotifDecorations(const QWindow *window);` ← unsupported return type: MotifDecorations
- `static void setWmWindowTypes(QWindow *window, WmWindowTypes types);` ← unsupported param type: QWindow *
- `static void popupSystemWindowMenu(const QWindow *window);` ← unsupported param type: const QWindow *
- `QList<DForeignWindow*> currentWorkspaceWindows() const;` ← unsupported return type: QList<DForeignWindow*>

## DDesktopServices — 18 methods generated, 4 skipped
- `static bool showFolders(const QList<QUrl> &urls, const QString &startupId = QStr` ← unsupported param type: const QList<QUrl> &
- `static bool showFileItemProperties(const QList<QUrl> &urls, const QString &start` ← unsupported param type: const QList<QUrl> &
- `static bool showFileItems(const QList<QUrl> &urls, const QString &startupId = QS` ← unsupported param type: const QList<QUrl> &
- `static bool trash(const QList<QUrl> &urls);` ← unsupported param type: const QList<QUrl> &
