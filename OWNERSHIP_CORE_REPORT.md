# DTK6 core pointer/return ownership audit

Generated from explicit native contracts in tools/ownership.py.
Evidence: matching apt source dtkwidget 6.7.47.1-1, dtkcore/dtkgui 6.7.47; Qt 6.8.0+dfsg-0deepin31.
All mapped return signatures and emitted constructors appear below. No ownership is inferred from a method name.

| Native API | Native result | Rust ownership | Evidence |
|---|---|---|---|
| `DStandardPaths::writableLocation` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::standardLocations` | `Vec<String>` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::locate` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::locateAll` | `Vec<String>` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::findExecutable` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::setMode` | `()` | copied scalar / no result | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::homePath` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::homePath` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::path` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::path` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::paths` | `Vec<String>` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::filePath` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DStandardPaths::filePath` | `String` | owned Rust strings/container | shim conversion; `dstandardpaths.h` |
| `DSysInfo::isDeepin` | `bool` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::isDDE` | `bool` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::deepinType` | `i32` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::deepinVersion` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::deepinEdition` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::deepinCopyright` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::uosType` | `i32` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::uosEditionType` | `i32` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::spVersion` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::udpateVersion` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::majorVersion` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::minorVersion` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::buildVersion` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::distributionInfoPath` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::distributionInfoSectionName` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::distributionOrgLogo` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::operatingSystemName` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::productType` | `i32` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::productTypeString` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::productVersion` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::isCommunityEdition` | `bool` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::computerName` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::cpuModelName` | `String` | owned Rust strings/container | shim conversion; `dsysinfo.h` |
| `DSysInfo::memoryInstalledSize` | `i64` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::memoryTotalSize` | `i64` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::systemDiskSize` | `i64` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::uptime` | `i64` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DSysInfo::arch` | `i32` | copied scalar / no result | shim conversion; `dsysinfo.h` |
| `DConfig::setAppId` | `()` | copied scalar / no result | shim conversion; `dconfig.h` |
| `DConfig::backendName` | `String` | owned Rust strings/container | shim conversion; `dconfig.h` |
| `DConfig::keyList` | `Vec<String>` | owned Rust strings/container | shim conversion; `dconfig.h` |
| `DConfig::isValid` | `bool` | copied scalar / no result | shim conversion; `dconfig.h` |
| `DConfig::isDefaultValue` | `bool` | copied scalar / no result | shim conversion; `dconfig.h` |
| `DConfig::value` | `*mut QVariant` | owned heap copy | shim conversion; `dconfig.h` |
| `DConfig::value` Qt value inputs | `QVariant * fallback` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dconfig.h` native value/reference signature |
| `DConfig::setValue` | `()` | copied scalar / no result | shim conversion; `dconfig.h` |
| `DConfig::setValue` Qt value inputs | `QVariant * value` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dconfig.h` native value/reference signature |
| `DConfig::reset` | `()` | copied scalar / no result | shim conversion; `dconfig.h` |
| `DConfig::isReadOnly` | `bool` | copied scalar / no result | shim conversion; `dconfig.h` |
| `DConfig::name` | `String` | owned Rust strings/container | shim conversion; `dconfig.h` |
| `DConfig::subpath` | `String` | owned Rust strings/container | shim conversion; `dconfig.h` |
| `DSettings()` | `new DSettings` | constructor `Owned` | constructor; `dsettings.h` |
| `DSettings::keys` | `Vec<String>` | owned Rust strings/container | shim conversion; `dsettings.h` |
| `DSettings::value` | `*mut QVariant` | owned heap copy | shim conversion; `dsettings.h` |
| `DSettings::groupKeys` | `Vec<String>` | owned Rust strings/container | shim conversion; `dsettings.h` |
| `DSettings::getOption` | `*mut QVariant` | owned heap copy | shim conversion; `dsettings.h` |
| `DSettings::sync` | `()` | copied scalar / no result | shim conversion; `dsettings.h` |
| `DSettings::setOption` | `()` | copied scalar / no result | shim conversion; `dsettings.h` |
| `DSettings::setOption` Qt value inputs | `QVariant * value` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dsettings.h` native value/reference signature |
| `DSettings::reset` | `()` | copied scalar / no result | shim conversion; `dsettings.h` |
| `DLogManager::registerConsoleAppender` | `()` | copied scalar / no result | shim conversion; `LogManager.h` |
| `DLogManager::registerFileAppender` | `()` | copied scalar / no result | shim conversion; `LogManager.h` |
| `DLogManager::registerJournalAppender` | `()` | copied scalar / no result | shim conversion; `LogManager.h` |
| `DLogManager::getlogFilePath` | `String` | owned Rust strings/container | shim conversion; `LogManager.h` |
| `DLogManager::setlogFilePath` | `()` | copied scalar / no result | shim conversion; `LogManager.h` |
| `DLogManager::setLogFormat` | `()` | copied scalar / no result | shim conversion; `LogManager.h` |
| `DFileSystemWatcher()` | `new DFileSystemWatcher` | constructor `Owned` | constructor; `dfilesystemwatcher.h` |
| `DFileSystemWatcher::addPath` | `bool` | copied scalar / no result | shim conversion; `dfilesystemwatcher.h` |
| `DFileSystemWatcher::addPaths` | `Vec<String>` | owned Rust strings/container | shim conversion; `dfilesystemwatcher.h` |
| `DFileSystemWatcher::removePath` | `bool` | copied scalar / no result | shim conversion; `dfilesystemwatcher.h` |
| `DFileSystemWatcher::removePaths` | `Vec<String>` | owned Rust strings/container | shim conversion; `dfilesystemwatcher.h` |
| `DFileSystemWatcher::files` | `Vec<String>` | owned Rust strings/container | shim conversion; `dfilesystemwatcher.h` |
| `DFileSystemWatcher::directories` | `Vec<String>` | owned Rust strings/container | shim conversion; `dfilesystemwatcher.h` |
| `DBaseFileWatcher::fileUrl` | `QUrl` | by-value copy (cxx-qt-lib trivial type) | shim conversion; `dbasefilewatcher.h` |
| `DBaseFileWatcher::startWatcher` | `bool` | copied scalar / no result | shim conversion; `dbasefilewatcher.h` |
| `DBaseFileWatcher::stopWatcher` | `bool` | copied scalar / no result | shim conversion; `dbasefilewatcher.h` |
| `DBaseFileWatcher::restartWatcher` | `bool` | copied scalar / no result | shim conversion; `dbasefilewatcher.h` |
| `DBaseFileWatcher::setEnabledSubfileWatcher` | `()` | copied scalar / no result | shim conversion; `dbasefilewatcher.h` |
| `DBaseFileWatcher::setEnabledSubfileWatcher` Qt value inputs | `const QUrl &subfileUrl` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dbasefilewatcher.h` native value/reference signature |
| `DFileWatcherManager()` | `new DFileWatcherManager` | constructor `Owned` | constructor; `dfilewatchermanager.h` |
| `DFileWatcherManager::add` | `DFileWatcher*` | `object_borrow` | `dtkcore/src/filesystem/dfilewatchermanager.cpp`: new DFileWatcher(filePath, this) child kept in watchersMap; remove()/removeAll() deleteLater it |
| `DFileWatcherManager::remove` | `()` | copied scalar / no result | shim conversion; `dfilewatchermanager.h` |
| `DFileWatcherManager::removeAll` | `()` | copied scalar / no result | shim conversion; `dfilewatchermanager.h` |
| `DFileWatcherManager::watchedFiles` | `Vec<String>` | owned Rust strings/container | shim conversion; `dfilewatchermanager.h` |
