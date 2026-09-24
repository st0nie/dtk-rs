# DTK6 core pilot binding coverage report

classes: 3, methods generated: 52, skipped: 13

Qt ancestry: 0 widgets, 1 QObjects, 2 opaque types

## skip reasons
- unsupported param type: 9
- unsupported return type: 4

> Pilot scope: only the listed classes/headers were scanned; this is NOT library-wide coverage.

## DStandardPaths — 13 methods generated, 0 skipped

## DSysInfo — 28 methods generated, 8 skipped
- `static QString deepinTypeDisplayName(const QLocale &locale = QLocale::system());` ← unsupported param type: const QLocale &
- `static QString uosProductTypeName(const QLocale &locale = QLocale::system());` ← unsupported param type: const QLocale &
- `static QString uosSystemName(const QLocale &locale = QLocale::system());` ← unsupported param type: const QLocale &
- `static QString uosEditionName(const QLocale &locale = QLocale::system());` ← unsupported param type: const QLocale &
- `static QString distributionOrgName(OrgType type = Distribution, const QLocale &l` ← unsupported param type: const QLocale &
- `static QPair<QString, QString> distributionOrgWebsite(OrgType type = Distributio` ← unsupported return type: QPair<QString, QString>
- `static QDateTime bootTime();` ← unsupported return type: QDateTime
- `static QDateTime shutdownTime();` ← unsupported return type: QDateTime

## DConfig — 11 methods generated, 5 skipped
- `static DConfig *create(const QString &appId, const QString &name, const QString ` ← unsupported param type: QObject *
- `static DConfig *create(DConfigBackend *backend, const QString &appId, const QStr` ← unsupported param type: DConfigBackend *
- `static DConfig *createGeneric(const QString &name, const QString &subpath = QStr` ← unsupported param type: QObject *
- `static DConfig *createGeneric(DConfigBackend *backend, const QString &name, cons` ← unsupported param type: DConfigBackend *
- `static QThread *globalThread();` ← unsupported return type: QThread *
