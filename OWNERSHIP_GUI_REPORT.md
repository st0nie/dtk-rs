# DTK6 gui pointer/return ownership audit

Generated from explicit native contracts in tools/ownership.py.
Evidence: matching apt source dtkwidget 6.7.47.1-1, dtkcore/dtkgui 6.7.47; Qt 6.8.0+dfsg-0deepin31.
All mapped return signatures and emitted constructors appear below. No ownership is inferred from a method name.

| Native API | Native result | Rust ownership | Evidence |
|---|---|---|---|
| `DGuiApplicationHelper::instance` | `DGuiApplicationHelper*` | `object_borrow` | `dtkgui/src/kernel/dguiapplicationhelper.cpp`: returns native global helper; staticCleanApplication post routine resets it |
| `DGuiApplicationHelper::adjustColor` | `QColor` | by-value copy (cxx-qt-lib trivial type) | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::adjustColor` Qt value inputs | `const QColor &base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dguiapplicationhelper.h` native value/reference signature |
| `DGuiApplicationHelper::adjustColor` | `*mut QImage` | owned heap copy | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::adjustColor` Qt value inputs | `QImage * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dguiapplicationhelper.h` native value/reference signature |
| `DGuiApplicationHelper::blendColor` | `QColor` | by-value copy (cxx-qt-lib trivial type) | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::blendColor` Qt value inputs | `const QColor &substrate, const QColor &superstratum` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dguiapplicationhelper.h` native value/reference signature |
| `DGuiApplicationHelper::standardPalette` | `*mut DPalette` | owned heap copy | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::generatePaletteColor` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::generatePaletteColor` Qt value inputs | `DPalette * base` | exclusive mutable value borrow; no QObject ownership transfer | explicit value mapping; `dguiapplicationhelper.h` native value/reference signature |
| `DGuiApplicationHelper::generatePalette` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::generatePalette` Qt value inputs | `DPalette * base` | exclusive mutable value borrow; no QObject ownership transfer | explicit value mapping; `dguiapplicationhelper.h` native value/reference signature |
| `DGuiApplicationHelper::fetchPalette` | `*mut DPalette` | owned heap copy | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::fetchPalette` parameters | `DPlatformTheme*` | borrow | native `dguiapplicationhelper.h` implementation: dtkgui dguiapplicationhelper.cpp: reads theme->themeName()/fetchPalette during the call; not retained |
| `DGuiApplicationHelper::isXWindowPlatform` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::isTabletEnvironment` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::isSpecialEffectsEnvironment` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::setAttribute` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::testAttribute` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::systemTheme` | `DPlatformTheme*` | `object_borrow` | `dtkgui/src/kernel/dguiapplicationhelper.cpp`: returns d->systemTheme = new DPlatformTheme(0, q): child of the helper |
| `DGuiApplicationHelper::applicationTheme` | `DPlatformTheme*` | `object_borrow` | `dtkgui/src/kernel/dguiapplicationhelper.cpp`: returns d->appTheme (systemTheme, or a lazily created child DPlatformTheme(windowLeader, systemTheme)) |
| `DGuiApplicationHelper::applicationPalette` | `*mut DPalette` | owned heap copy | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::applicationPalette` | `*mut DPalette` | owned heap copy | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::setApplicationPalette` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::setApplicationPalette` Qt value inputs | `DPalette * palette` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dguiapplicationhelper.h` native value/reference signature |
| `DGuiApplicationHelper::fontManager` | `DFontManager*` | `object_borrow` | `dtkgui/src/kernel/dguiapplicationhelper.cpp`: returns Q_GLOBAL_STATIC(DFontManager, _globalFM); no transfer |
| `DGuiApplicationHelper::toColorType` | `i32` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::toColorType` Qt value inputs | `const QColor &color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dguiapplicationhelper.h` native value/reference signature |
| `DGuiApplicationHelper::toColorType` | `i32` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::toColorType` Qt value inputs | `QPalette * palette` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dguiapplicationhelper.h` native value/reference signature |
| `DGuiApplicationHelper::themeType` | `i32` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::paletteType` | `i32` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::setSingleInstance` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::setSingleInstanceInterval` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::userManualPaths` | `Vec<String>` | owned Rust strings/container | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::hasUserManual` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::setPaletteType` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::handleHelpAction` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::openUrl` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::sizeMode` | `i32` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::setSizeMode` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::resetSizeMode` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DImageHandler()` | `new DImageHandler` | constructor `Owned` | constructor; `dimagehandler.h` |
| `DImageHandler::setFileName` | `()` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::fileName` | `String` | owned Rust strings/container | shim conversion; `dimagehandler.h` |
| `DImageHandler::readImage` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::thumbnail` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::thumbnail` Qt value inputs | `const QSize &size` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::imageFormat` | `String` | owned Rust strings/container | shim conversion; `dimagehandler.h` |
| `DImageHandler::imageSize` | `QSize` | by-value copy (cxx-qt-lib trivial type) | shim conversion; `dimagehandler.h` |
| `DImageHandler::findAllMetaData` | `Vec<String>` | owned Rust strings/container | shim conversion; `dimagehandler.h` |
| `DImageHandler::clearCache` | `()` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::saveImage` | `bool` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::saveImage` | `bool` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::saveImage` Qt value inputs | `QImage * image` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::rotateImage` | `bool` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::rotateImage` Qt value inputs | `QImage * image` | exclusive mutable value borrow; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::rotateImageFile` | `bool` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::isReadable` | `bool` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::isWriteable` | `bool` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::isRotatable` | `bool` | copied scalar / no result | shim conversion; `dimagehandler.h` |
| `DImageHandler::lastError` | `String` | owned Rust strings/container | shim conversion; `dimagehandler.h` |
| `DImageHandler::supportFormats` | `Vec<String>` | owned Rust strings/container | shim conversion; `dimagehandler.h` |
| `DImageHandler::detectImageFormat` | `String` | owned Rust strings/container | shim conversion; `dimagehandler.h` |
| `DImageHandler::oldColorFilter` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::oldColorFilter` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::warmColorFilter` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::warmColorFilter` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::coolColorFilter` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::coolColorFilter` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::grayScaleColorFilter` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::grayScaleColorFilter` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::antiColorFilter` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::antiColorFilter` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::metalColorFilter` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::metalColorFilter` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::bilateralFilter` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::bilateralFilter` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::contourExtraction` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::contourExtraction` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::binaryzation` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::binaryzation` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::grayScale` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::grayScale` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::laplaceSharpen` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::laplaceSharpen` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::sobelEdgeDetector` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::sobelEdgeDetector` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::changeLightAndContrast` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::changeLightAndContrast` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::changeBrightness` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::changeBrightness` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::changeTransparency` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::changeTransparency` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::changeStauration` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::changeStauration` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::replacePointColor` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::replacePointColor` Qt value inputs | `QImage * img, const QColor &oldColor, const QColor &newColor` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::flipHorizontal` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::flipHorizontal` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DImageHandler::flipVertical` | `*mut QImage` | owned heap copy | shim conversion; `dimagehandler.h` |
| `DImageHandler::flipVertical` Qt value inputs | `QImage * img` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimagehandler.h` native value/reference signature |
| `DFontManager()` | `new DFontManager` | constructor `Owned` | constructor; `dfontmanager.h` |
| `DFontManager::fontPixelSize` | `i32` | copied scalar / no result | shim conversion; `dfontmanager.h` |
| `DFontManager::setFontPixelSize` | `()` | copied scalar / no result | shim conversion; `dfontmanager.h` |
| `DFontManager::fontPixelSize` | `i32` | copied scalar / no result | shim conversion; `dfontmanager.h` |
| `DFontManager::fontPixelSize` Qt value inputs | `QFont * font` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfontmanager.h` native value/reference signature |
| `DFontManager::get` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::get` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfontmanager.h` native value/reference signature |
| `DFontManager::get` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::get` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfontmanager.h` native value/reference signature |
| `DFontManager::get` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::baseFont` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::setBaseFont` | `()` | copied scalar / no result | shim conversion; `dfontmanager.h` |
| `DFontManager::setBaseFont` Qt value inputs | `QFont * font` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfontmanager.h` native value/reference signature |
| `DFontManager::resetBaseFont` | `()` | copied scalar / no result | shim conversion; `dfontmanager.h` |
| `DFontManager::t1` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t2` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t3` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t4` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t5` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t6` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t7` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t8` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t9` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t10` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DFontManager::t11` | `*mut QFont` | owned heap copy | shim conversion; `dfontmanager.h` |
| `DNativeSettings::isValid` | `bool` | copied scalar / no result | shim conversion; `dnativesettings.h` |
| `DNativeSettings::getSetting` | `*mut QVariant` | owned heap copy | shim conversion; `dnativesettings.h` |
| `DNativeSettings::setSetting` | `()` | copied scalar / no result | shim conversion; `dnativesettings.h` |
| `DNativeSettings::setSetting` Qt value inputs | `QVariant * value` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dnativesettings.h` native value/reference signature |
| `DPlatformTheme::isValid` | `bool` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::parentTheme` | `DPlatformTheme*` | `object_borrow` | `dtkgui/src/kernel/dplatformtheme.cpp`: returns d->parent, the QObject parent theme; nullptr for the system theme |
| `DPlatformTheme::setFallbackProperty` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::palette` | `*mut DPalette` | owned heap copy | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setPalette` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setPalette` Qt value inputs | `DPalette * palette` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dplatformtheme.h` native value/reference signature |
| `DPlatformTheme::cursorBlinkTime` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::cursorBlinkTimeout` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::cursorBlink` | `bool` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::doubleClickDistance` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::doubleClickTime` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::dndDragThreshold` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::windowRadius` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::windowRadius` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::themeName` | `String` | owned Rust strings/container | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::iconThemeName` | `String` | owned Rust strings/container | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::soundThemeName` | `String` | owned Rust strings/container | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::fontName` | `String` | owned Rust strings/container | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::monoFontName` | `String` | owned Rust strings/container | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::fontPointSize` | `f64` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::gtkFontName` | `String` | owned Rust strings/container | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::activeColor` | `QColor` | by-value copy (cxx-qt-lib trivial type) | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::darkActiveColor` | `QColor` | by-value copy (cxx-qt-lib trivial type) | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::isValidPalette` | `bool` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::sizeMode` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::scrollBarPolicy` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setCursorBlinkTime` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setCursorBlinkTimeout` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setCursorBlink` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setDoubleClickDistance` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setDoubleClickTime` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setDndDragThreshold` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setThemeName` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setIconThemeName` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setSoundThemeName` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setFontName` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setMonoFontName` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setFontPointSize` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setGtkFontName` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setActiveColor` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setActiveColor` Qt value inputs | `const QColor &activeColor` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dplatformtheme.h` native value/reference signature |
| `DPlatformTheme::setDarkActiveColor` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setDarkActiveColor` Qt value inputs | `const QColor &activeColor` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dplatformtheme.h` native value/reference signature |
| `DPlatformTheme::dotsPerInch` | `i32` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setDotsPerInch` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DPlatformTheme::setWindowRadius` | `()` | copied scalar / no result | shim conversion; `dplatformtheme.h` |
| `DWindowManagerHelper::instance` | `DWindowManagerHelper*` | `object_borrow` | `dtkgui/src/kernel/dwindowmanagerhelper.cpp`: returns Q_GLOBAL_STATIC wmhGlobal (treelandWMHGlobal on Wayland); never deleted by callers |
| `DWindowManagerHelper::setWmClassName` | `()` | copied scalar / no result | shim conversion; `dwindowmanagerhelper.h` |
| `DWindowManagerHelper::hasBlurWindow` | `bool` | copied scalar / no result | shim conversion; `dwindowmanagerhelper.h` |
| `DWindowManagerHelper::hasComposite` | `bool` | copied scalar / no result | shim conversion; `dwindowmanagerhelper.h` |
| `DWindowManagerHelper::hasNoTitlebar` | `bool` | copied scalar / no result | shim conversion; `dwindowmanagerhelper.h` |
| `DWindowManagerHelper::hasWallpaperEffect` | `bool` | copied scalar / no result | shim conversion; `dwindowmanagerhelper.h` |
| `DWindowManagerHelper::windowManagerNameString` | `String` | owned Rust strings/container | shim conversion; `dwindowmanagerhelper.h` |
| `DWindowManagerHelper::windowManagerName` | `i32` | copied scalar / no result | shim conversion; `dwindowmanagerhelper.h` |
| `DWindowManagerHelper::windowFromPoint` | `u32` | copied scalar / no result | shim conversion; `dwindowmanagerhelper.h` |
| `DWindowManagerHelper::windowFromPoint` Qt value inputs | `const QPoint &p` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dwindowmanagerhelper.h` native value/reference signature |
| `DDesktopServices::showFolder` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::showFolder` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::showFolder` Qt value inputs | `const QUrl &url` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `ddesktopservices.h` native value/reference signature |
| `DDesktopServices::showFileItemProperty` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::showFileItemProperty` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::showFileItemProperty` Qt value inputs | `const QUrl &url` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `ddesktopservices.h` native value/reference signature |
| `DDesktopServices::showFileItem` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::showFileItem` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::showFileItem` Qt value inputs | `const QUrl &url` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `ddesktopservices.h` native value/reference signature |
| `DDesktopServices::trash` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::trash` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::trash` Qt value inputs | `const QUrl &url` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `ddesktopservices.h` native value/reference signature |
| `DDesktopServices::playSystemSoundEffect` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::playSystemSoundEffect` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::previewSystemSoundEffect` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::previewSystemSoundEffect` | `bool` | copied scalar / no result | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::getNameByEffectType` | `String` | owned Rust strings/container | shim conversion; `ddesktopservices.h` |
| `DDesktopServices::errorMessage` | `String` | owned Rust strings/container | shim conversion; `ddesktopservices.h` |
