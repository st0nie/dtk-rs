# DTK6 core (selected classes) binding coverage report

classes: 9, methods generated: 80, skipped: 23

Qt ancestry: 0 widgets, 6 QObjects, 3 opaque types

## skip reasons
- unsupported param type: 12
- unsupported return type: 11

> Selected-class scope: only the listed classes/headers were scanned; this is NOT library-wide coverage.

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

## DSettings — 7 methods generated, 8 skipped
- `void setBackend(DSettingsBackend *backend = nullptr);` ← unsupported param type: DSettingsBackend *
- `static QPointer<DSettings> fromJson(const QByteArray &json);` ← unsupported return type: QPointer<DSettings>
- `static QPointer<DSettings> fromJsonFile(const QString &filepath);` ← unsupported return type: QPointer<DSettings>
- `QJsonObject meta() const;` ← unsupported return type: QJsonObject
- `QList<QPointer<DSettingsOption>> options() const;` ← unsupported return type: QList<QPointer<DSettingsOption>>
- `QPointer<DSettingsOption> option(const QString &key) const;` ← unsupported return type: QPointer<DSettingsOption>
- `QList<QPointer<DSettingsGroup>> groups() const;` ← unsupported return type: QList<QPointer<DSettingsGroup>>
- `QPointer<DSettingsGroup> group(const QString &key) const;` ← unsupported return type: QPointer<DSettingsGroup>

## DLogManager — 6 methods generated, 0 skipped

## DFileSystemWatcher — 6 methods generated, 0 skipped

## DBaseFileWatcher — 5 methods generated, 2 skipped
- `static bool ghostSignal(const QUrl &targetUrl, SignalType1 signal, const QUrl &a` ← unsupported param type: SignalType1
- `static bool ghostSignal(const QUrl &targetUrl, SignalType2 signal, const QUrl &a` ← unsupported param type: SignalType2

## DFileWatcher — 0 methods generated, 0 skipped

## DFileWatcherManager — 4 methods generated, 0 skipped
