# DTK6 core pilot binding coverage report

classes: 2, methods generated: 41, skipped: 8

Qt ancestry: 0 widgets, 0 QObjects, 2 opaque types

## skip reasons
- unsupported param type: 5
- unsupported return type: 2
- signature parse failed: 1

> Pilot scope: only the listed classes/headers were scanned; this is NOT library-wide coverage.

## DStandardPaths — 13 methods generated, 0 skipped

## DSysInfo — 28 methods generated, 8 skipped
- `static QPair<QString, QString> distributionOrgWebsite(OrgType type = Distributio` ← signature parse failed
- `static QString deepinTypeDisplayName(const QLocale &locale = QLocale::system());` ← unsupported param type: const QLocale &
- `static QString uosProductTypeName(const QLocale &locale = QLocale::system());` ← unsupported param type: const QLocale &
- `static QString uosSystemName(const QLocale &locale = QLocale::system());` ← unsupported param type: const QLocale &
- `static QString uosEditionName(const QLocale &locale = QLocale::system());` ← unsupported param type: const QLocale &
- `static QString distributionOrgName(OrgType type = Distribution, const QLocale &l` ← unsupported param type: const QLocale &
- `static QDateTime bootTime();` ← unsupported return type: QDateTime
- `static QDateTime shutdownTime();` ← unsupported return type: QDateTime
