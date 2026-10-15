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
| `DGuiApplicationHelper::isXWindowPlatform` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::isTabletEnvironment` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::isSpecialEffectsEnvironment` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::setAttribute` | `()` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
| `DGuiApplicationHelper::testAttribute` | `bool` | copied scalar / no result | shim conversion; `dguiapplicationhelper.h` |
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
