# DTK6 widget pointer/return ownership audit

Generated from explicit native contracts in tools/ownership.py.
Evidence: matching apt source dtkwidget 6.7.47.1-1, dtkcore/dtkgui 6.7.47; Qt 6.8.0+dfsg-0deepin31.
All mapped return signatures and emitted constructors appear below. No ownership is inferred from a method name.

| Native API | Native result | Rust ownership | Evidence |
|---|---|---|---|
| `DAboutDialog()` | `new DAboutDialog` | constructor `Owned` | constructor; `daboutdialog.h` |
| `DAboutDialog::windowTitle` | `String` | owned Rust strings/container | shim conversion; `daboutdialog.h` |
| `DAboutDialog::productName` | `String` | owned Rust strings/container | shim conversion; `daboutdialog.h` |
| `DAboutDialog::version` | `String` | owned Rust strings/container | shim conversion; `daboutdialog.h` |
| `DAboutDialog::description` | `String` | owned Rust strings/container | shim conversion; `daboutdialog.h` |
| `DAboutDialog::companyLogo` | `*mut QPixmap` | owned heap copy | shim conversion; `daboutdialog.h` |
| `DAboutDialog::websiteName` | `String` | owned Rust strings/container | shim conversion; `daboutdialog.h` |
| `DAboutDialog::websiteLink` | `String` | owned Rust strings/container | shim conversion; `daboutdialog.h` |
| `DAboutDialog::license` | `String` | owned Rust strings/container | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setLicenseEnabled` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setWindowTitle` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setProductIcon` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setProductIcon` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `daboutdialog.h` native value/reference signature |
| `DAboutDialog::setProductName` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setVersion` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setDescription` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setCompanyLogo` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setCompanyLogo` Qt value inputs | `QPixmap * companyLogo` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `daboutdialog.h` native value/reference signature |
| `DAboutDialog::setWebsiteName` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setWebsiteLink` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setAcknowledgementVisible` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAboutDialog::setLicense` | `()` | copied scalar / no result | shim conversion; `daboutdialog.h` |
| `DAbstractDialog()` | `new DAbstractDialog` | constructor `Owned` | constructor; `dabstractdialog.h` |
| `DAbstractDialog::displayPosition` | `i32` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::move` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::move` Qt value inputs | `QPoint * pos` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dabstractdialog.h` native value/reference signature |
| `DAbstractDialog::move` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::setGeometry` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::setGeometry` Qt value inputs | `QRect * rect` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dabstractdialog.h` native value/reference signature |
| `DAbstractDialog::setGeometry` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::moveToCenter` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::moveToTopRight` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::moveToCenterByRect` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::moveToCenterByRect` Qt value inputs | `QRect * rect` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dabstractdialog.h` native value/reference signature |
| `DAbstractDialog::moveToTopRightByRect` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAbstractDialog::moveToTopRightByRect` Qt value inputs | `QRect * rect` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dabstractdialog.h` native value/reference signature |
| `DAbstractDialog::setDisplayPosition` | `()` | copied scalar / no result | shim conversion; `dabstractdialog.h` |
| `DAccessibilityChecker()` | `new DAccessibilityChecker` | constructor `Owned` | constructor; `daccessibilitychecker.h` |
| `DAccessibilityChecker::setOutputFormat` | `()` | copied scalar / no result | shim conversion; `daccessibilitychecker.h` |
| `DAccessibilityChecker::outputFormat` | `i32` | copied scalar / no result | shim conversion; `daccessibilitychecker.h` |
| `DAccessibilityChecker::check` | `bool` | copied scalar / no result | shim conversion; `daccessibilitychecker.h` |
| `DAccessibilityChecker::start` | `()` | copied scalar / no result | shim conversion; `daccessibilitychecker.h` |
| `DAlertControl::setAlert` | `()` | copied scalar / no result | shim conversion; `dalertcontrol.h` |
| `DAlertControl::isAlert` | `bool` | copied scalar / no result | shim conversion; `dalertcontrol.h` |
| `DAlertControl::setAlertColor` | `()` | copied scalar / no result | shim conversion; `dalertcontrol.h` |
| `DAlertControl::setAlertColor` Qt value inputs | `QColor * c` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dalertcontrol.h` native value/reference signature |
| `DAlertControl::alertColor` | `*mut QColor` | owned heap copy | shim conversion; `dalertcontrol.h` |
| `DAlertControl::defaultAlertColor` | `*mut QColor` | owned heap copy | shim conversion; `dalertcontrol.h` |
| `DAlertControl::setMessageAlignment` | `()` | copied scalar / no result | shim conversion; `dalertcontrol.h` |
| `DAlertControl::messageAlignment` | `i32` | copied scalar / no result | shim conversion; `dalertcontrol.h` |
| `DAlertControl::showAlertMessage` | `()` | copied scalar / no result | shim conversion; `dalertcontrol.h` |
| `DAlertControl::showAlertMessage` | `()` | copied scalar / no result | shim conversion; `dalertcontrol.h` |
| `DAlertControl::showAlertMessage` parameters | `QWidget*` | borrow | native `dalertcontrol.h` implementation: alert follows widget; native QObject connections/parented alert |
| `DAlertControl::hideAlertMessage` | `()` | copied scalar / no result | shim conversion; `dalertcontrol.h` |
| `DAnchorsBase::target` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/danchors.cpp`: returns extendWidget->target(); widget is not transferred |
| `DAnchorsBase::enhancedWidget` | `DEnhancedWidget*` | `object_borrow` | `dtkwidget/src/widgets/danchors.cpp`: returns the existing extendWidget child |
| `DAnchorsBase::enabled` | `bool` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::anchors` | `DAnchorsBase*` | `object_borrow` | `dtkwidget/src/widgets/danchors.cpp`: returns this; no allocation or transfer |
| `DAnchorsBase::fill` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/danchors.cpp`: returns fill->target(); not an allocation |
| `DAnchorsBase::centerIn` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/danchors.cpp`: returns centerIn->target(); not an allocation |
| `DAnchorsBase::margins` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::topMargin` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::bottomMargin` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::leftMargin` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::rightMargin` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::horizontalCenterOffset` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::verticalCenterOffset` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::alignWhenCentered` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::errorCode` | `i32` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::errorString` | `String` | owned Rust strings/container | shim conversion; `danchors.h` |
| `DAnchorsBase::setAnchor` | `bool` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setAnchor` parameters | `QWidget*, QWidget*` | retained borrow | native `danchors.h` implementation: anchor/observer relations; clearAnchors deletes the observer, not widgets |
| `DAnchorsBase::clearAnchors` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::clearAnchors` parameters | `QWidget*` | borrow; deletes observer | native `danchors.h` implementation: widget stays owned by its original parent/owner |
| `DAnchorsBase::getAnchorBaseByWidget` | `DAnchorsBase*` | `object_borrow` | `dtkwidget/src/widgets/danchors.cpp`: looks up getWidgetAnchorsBase; clearAnchors deletes it |
| `DAnchorsBase::getAnchorBaseByWidget` parameters | `QWidget*` | borrow | native `danchors.h` implementation: lookup only |
| `DAnchorsBase::setEnabled` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setAnchor` | `bool` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setAnchor` parameters | `QWidget*` | retained borrow | native `danchors.h` implementation: anchor/observer relations; clearAnchors deletes the observer, not widgets |
| `DAnchorsBase::setFill` | `bool` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setFill` parameters | `QWidget*` | retained borrow | native `danchors.h` implementation: fill target is not adopted |
| `DAnchorsBase::setCenterIn` | `bool` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setCenterIn` parameters | `QWidget*` | retained borrow | native `danchors.h` implementation: center target is not adopted |
| `DAnchorsBase::setFill` | `bool` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setFill` parameters | `DAnchorsBase*` | retained borrow | native `danchors.h` implementation: fill target is not adopted |
| `DAnchorsBase::setCenterIn` | `bool` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setCenterIn` parameters | `DAnchorsBase*` | retained borrow | native `danchors.h` implementation: center target is not adopted |
| `DAnchorsBase::setMargins` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setTopMargin` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setBottomMargin` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setLeftMargin` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setRightMargin` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setHorizontalCenterOffset` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setVerticalCenterOffset` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setAlignWhenCentered` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setTop` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setBottom` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setLeft` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setRight` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setHorizontalCenter` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::setVerticalCenter` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::moveTop` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::moveBottom` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::moveLeft` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::moveRight` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::moveHorizontalCenter` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::moveVerticalCenter` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::moveCenter` | `()` | copied scalar / no result | shim conversion; `danchors.h` |
| `DAnchorsBase::moveCenter` Qt value inputs | `QPoint * arg` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `danchors.h` native value/reference signature |
| `ArrowButtonIcon()` | `new ArrowButtonIcon` | constructor `Owned` | constructor; `darrowbutton.h` |
| `ArrowButtonIcon::setArrowDirection` | `()` | copied scalar / no result | shim conversion; `darrowbutton.h` |
| `ArrowButtonIcon::setButtonState` | `()` | copied scalar / no result | shim conversion; `darrowbutton.h` |
| `ArrowButtonIcon::arrowDirection` | `i32` | copied scalar / no result | shim conversion; `darrowbutton.h` |
| `ArrowButtonIcon::buttonState` | `i32` | copied scalar / no result | shim conversion; `darrowbutton.h` |
| `DArrowButton()` | `new DArrowButton` | constructor `Owned` | constructor; `darrowbutton.h` |
| `DArrowButton::setArrowDirection` | `()` | copied scalar / no result | shim conversion; `darrowbutton.h` |
| `DArrowButton::arrowDirection` | `i32` | copied scalar / no result | shim conversion; `darrowbutton.h` |
| `DArrowButton::buttonState` | `i32` | copied scalar / no result | shim conversion; `darrowbutton.h` |
| `DArrowLineDrawer()` | `new DArrowLineDrawer` | constructor `Owned` | constructor; `darrowlinedrawer.h` |
| `DArrowLineDrawer::setTitle` | `()` | copied scalar / no result | shim conversion; `darrowlinedrawer.h` |
| `DArrowLineDrawer::setExpand` | `()` | copied scalar / no result | shim conversion; `darrowlinedrawer.h` |
| `DArrowRectangle::radius` | `i32` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::radiusForceEnabled` | `bool` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::arrowHeight` | `i32` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::arrowWidth` | `i32` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::arrowX` | `i32` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::arrowY` | `i32` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::margin` | `i32` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::borderWidth` | `i32` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::borderColor` | `*mut QColor` | owned heap copy | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::backgroundColor` | `*mut QColor` | owned heap copy | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::arrowDirection` | `i32` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setRadius` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setRadiusForceEnable` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setArrowHeight` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setArrowWidth` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setArrowX` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setArrowY` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setMargin` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setBorderWidth` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setBorderColor` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setBorderColor` Qt value inputs | `QColor * borderColor` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `darrowrectangle.h` native value/reference signature |
| `DArrowRectangle::setBackgroundColor` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setBackgroundColor` Qt value inputs | `QColor * backgroundColor` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `darrowrectangle.h` native value/reference signature |
| `DArrowRectangle::setBackgroundColor` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setArrowDirection` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setWidth` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setHeight` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::show` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setContent` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setContent` parameters | `QWidget*` | Qt adoption | native `darrowrectangle.h` implementation: QPointer content and installed widget layout |
| `DArrowRectangle::getContent` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/darrowrectangle.cpp`: returns existing QPointer m_content |
| `DArrowRectangle::resizeWithContent` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::move` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::getFixedSize` | `*mut QSize` | owned heap copy | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::shadowXOffset` | `f64` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::shadowYOffset` | `f64` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::shadowBlurRadius` | `f64` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setShadowBlurRadius` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setShadowXOffset` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setShadowYOffset` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setLeftRightRadius` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DArrowRectangle::setRadiusArrowStyleEnable` | `()` | copied scalar / no result | shim conversion; `darrowrectangle.h` |
| `DBackgroundGroup()` | `new DBackgroundGroup` | constructor `Owned` | constructor; `dbackgroundgroup.h` |
| `DBackgroundGroup::itemMargins` | `*mut QMargins` | owned heap copy | shim conversion; `dbackgroundgroup.h` |
| `DBackgroundGroup::useWidgetBackground` | `bool` | copied scalar / no result | shim conversion; `dbackgroundgroup.h` |
| `DBackgroundGroup::setLayout` | `()` | copied scalar / no result | shim conversion; `dbackgroundgroup.h` |
| `DBackgroundGroup::setLayout` parameters | `QLayout*` | Qt adoption | native `dbackgroundgroup.h` implementation: QWidget::setLayout; only valid layout handoff |
| `DBackgroundGroup::setItemMargins` | `()` | copied scalar / no result | shim conversion; `dbackgroundgroup.h` |
| `DBackgroundGroup::setItemMargins` Qt value inputs | `QMargins * itemMargins` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dbackgroundgroup.h` native value/reference signature |
| `DBackgroundGroup::setItemSpacing` | `()` | copied scalar / no result | shim conversion; `dbackgroundgroup.h` |
| `DBackgroundGroup::setUseWidgetBackground` | `()` | copied scalar / no result | shim conversion; `dbackgroundgroup.h` |
| `DBaseLine()` | `new DBaseLine` | constructor `Owned` | constructor; `dbaseline.h` |
| `DBaseLine::setLeftContent` | `()` | copied scalar / no result | shim conversion; `dbaseline.h` |
| `DBaseLine::setLeftContent` parameters | `QWidget*` | Qt adoption | native `dbaseline.h` implementation: layout removes QWidgetItems, NOT their widgets, then adopts new content |
| `DBaseLine::setRightContent` | `()` | copied scalar / no result | shim conversion; `dbaseline.h` |
| `DBaseLine::setRightContent` parameters | `QWidget*` | Qt adoption | native `dbaseline.h` implementation: layout removes QWidgetItems, NOT their widgets, then adopts new content |
| `DBaseLine::leftLayout` | `QBoxLayout*` | `object_borrow` | `dtkwidget/src/widgets/dbaseline.cpp`: returns existing m_leftLayout owned through the widget layout |
| `DBaseLine::rightLayout` | `QBoxLayout*` | `object_borrow` | `dtkwidget/src/widgets/dbaseline.cpp`: returns existing m_rightLayout owned through the widget layout |
| `DBaseLine::setLeftMargin` | `()` | copied scalar / no result | shim conversion; `dbaseline.h` |
| `DBaseLine::setRightMargin` | `()` | copied scalar / no result | shim conversion; `dbaseline.h` |
| `DBaseLine::leftMargin` | `i32` | copied scalar / no result | shim conversion; `dbaseline.h` |
| `DBaseLine::rightMargin` | `i32` | copied scalar / no result | shim conversion; `dbaseline.h` |
| `DBlurEffectWidget()` | `new DBlurEffectWidget` | constructor `Owned` | constructor; `dblureffectwidget.h` |
| `DBlurEffectWidget::radius` | `i32` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::mode` | `i32` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::blendMode` | `i32` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::blurRectXRadius` | `i32` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::blurRectYRadius` | `i32` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::isFull` | `bool` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::blurEnabled` | `bool` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::maskColor` | `*mut QColor` | owned heap copy | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::maskAlpha` | `u8` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setMaskPath` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setMaskPath` Qt value inputs | `QPainterPath * path` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dblureffectwidget.h` native value/reference signature |
| `DBlurEffectWidget::setSourceImage` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setSourceImage` Qt value inputs | `QImage * image` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dblureffectwidget.h` native value/reference signature |
| `DBlurEffectWidget::setRadius` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setMode` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setBlendMode` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setBlurRectXRadius` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setBlurRectYRadius` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setMaskAlpha` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setMaskColor` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setMaskColor` Qt value inputs | `QColor * maskColor` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dblureffectwidget.h` native value/reference signature |
| `DBlurEffectWidget::setMaskColor` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setFull` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectWidget::setBlurEnabled` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectGroup()` | `new DBlurEffectGroup` | unique `OwnedOpaque` | constructor; `dblureffectwidget.h` |
| `DBlurEffectGroup::setSourceImage` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectGroup::setSourceImage` Qt value inputs | `QImage * image` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dblureffectwidget.h` native value/reference signature |
| `DBlurEffectGroup::addWidget` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectGroup::addWidget` parameters | `DBlurEffectWidget*` | retained borrow | native `dblureffectwidget.h` implementation: widget destructor removes itself from group; group destructor clears widget group pointers |
| `DBlurEffectGroup::addWidget` Qt value inputs | `QPoint * offset` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dblureffectwidget.h` native value/reference signature |
| `DBlurEffectGroup::removeWidget` | `()` | copied scalar / no result | shim conversion; `dblureffectwidget.h` |
| `DBlurEffectGroup::removeWidget` parameters | `DBlurEffectWidget*` | borrow; releases relation | native `dblureffectwidget.h` implementation: removes map entry and clears widget group pointer |
| `DBlurEffectWithBorderWidget()` | `new DBlurEffectWithBorderWidget` | constructor `Owned` | constructor; `dblureffectwithborderwidget.h` |
| `DBounceAnimation()` | `new DBounceAnimation` | constructor `Owned` | constructor; `dbounceanimation.h` |
| `DBounceAnimation::setAniMationEnable` | `()` | copied scalar / no result | shim conversion; `dbounceanimation.h` |
| `DBoxWidget::layout` | `QBoxLayout*` | `object_borrow` | `dtkwidget/src/widgets/dboxwidget.cpp`: returns the installed layout |
| `DBoxWidget::addWidget` | `()` | copied scalar / no result | shim conversion; `dboxwidget.h` |
| `DBoxWidget::addWidget` parameters | `QWidget*` | Qt adoption | native `dboxwidget.h` implementation: QBoxLayout owns item and reparents widget to containing widget |
| `DBoxWidget::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dboxwidget.h` |
| `DHBoxWidget()` | `new DHBoxWidget` | constructor `Owned` | constructor; `dboxwidget.h` |
| `DVBoxWidget()` | `new DVBoxWidget` | constructor `Owned` | constructor; `dboxwidget.h` |
| `DButtonBoxButton::setIcon` | `()` | copied scalar / no result | shim conversion; `dbuttonbox.h` |
| `DButtonBoxButton::setIcon` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dbuttonbox.h` native value/reference signature |
| `DButtonBoxButton::setIcon` | `()` | copied scalar / no result | shim conversion; `dbuttonbox.h` |
| `DButtonBoxButton::setIcon` | `()` | copied scalar / no result | shim conversion; `dbuttonbox.h` |
| `DButtonBoxButton::setIcon` Qt value inputs | `DDciIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dbuttonbox.h` native value/reference signature |
| `DButtonBoxButton::dciIcon` | `*mut DDciIcon` | owned heap copy | shim conversion; `dbuttonbox.h` |
| `DButtonBoxButton::iconSize` | `*mut QSize` | owned heap copy | shim conversion; `dbuttonbox.h` |
| `DButtonBoxButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dbuttonbox.h` |
| `DButtonBoxButton::minimumSizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dbuttonbox.h` |
| `DButtonBox()` | `new DButtonBox` | constructor `Owned` | constructor; `dbuttonbox.h` |
| `DButtonBox::orientation` | `i32` | copied scalar / no result | shim conversion; `dbuttonbox.h` |
| `DButtonBox::setOrientation` | `()` | copied scalar / no result | shim conversion; `dbuttonbox.h` |
| `DButtonBox::checkedButton` | `QAbstractButton*` | `object_borrow` | `dtkwidget/src/widgets/dbuttonbox.cpp`: returns QButtonGroup::checkedButton; no transfer |
| `DButtonBox::button` | `QAbstractButton*` | `object_borrow` | `dtkwidget/src/widgets/dbuttonbox.cpp`: returns QButtonGroup::button(id); no transfer |
| `DButtonBox::setId` | `()` | copied scalar / no result | shim conversion; `dbuttonbox.h` |
| `DButtonBox::setId` parameters | `QAbstractButton*` | borrow | native `dbuttonbox.h` implementation: button must already be a member |
| `DButtonBox::id` | `i32` | copied scalar / no result | shim conversion; `dbuttonbox.h` |
| `DButtonBox::id` parameters | `QAbstractButton*` | borrow | native `dbuttonbox.h` implementation: QButtonGroup lookup; no transfer |
| `DButtonBox::checkedId` | `i32` | copied scalar / no result | shim conversion; `dbuttonbox.h` |
| `DCircleProgress()` | `new DCircleProgress` | constructor `Owned` | constructor; `dcircleprogress.h` |
| `DCircleProgress::value` | `i32` | copied scalar / no result | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::setValue` | `()` | copied scalar / no result | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::text` | `String` | owned Rust strings/container | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::setText` | `()` | copied scalar / no result | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::backgroundColor` | `*mut QColor` | owned heap copy | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::setBackgroundColor` | `()` | copied scalar / no result | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::setBackgroundColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dcircleprogress.h` native value/reference signature |
| `DCircleProgress::chunkColor` | `*mut QColor` | owned heap copy | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::setChunkColor` | `()` | copied scalar / no result | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::setChunkColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dcircleprogress.h` native value/reference signature |
| `DCircleProgress::lineWidth` | `i32` | copied scalar / no result | shim conversion; `dcircleprogress.h` |
| `DCircleProgress::setLineWidth` | `()` | copied scalar / no result | shim conversion; `dcircleprogress.h` |
| `DClipEffectWidget::margins` | `*mut QMargins` | owned heap copy | shim conversion; `dclipeffectwidget.h` |
| `DClipEffectWidget::clipPath` | `*mut QPainterPath` | owned heap copy | shim conversion; `dclipeffectwidget.h` |
| `DClipEffectWidget::setMargins` | `()` | copied scalar / no result | shim conversion; `dclipeffectwidget.h` |
| `DClipEffectWidget::setMargins` Qt value inputs | `QMargins * margins` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dclipeffectwidget.h` native value/reference signature |
| `DClipEffectWidget::setClipPath` | `()` | copied scalar / no result | shim conversion; `dclipeffectwidget.h` |
| `DClipEffectWidget::setClipPath` Qt value inputs | `QPainterPath * path` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dclipeffectwidget.h` native value/reference signature |
| `DColoredProgressBar()` | `new DColoredProgressBar` | constructor `Owned` | constructor; `dcoloredprogressbar.h` |
| `DColoredProgressBar::addThreshold` | `()` | copied scalar / no result | shim conversion; `dcoloredprogressbar.h` |
| `DColoredProgressBar::addThreshold` Qt value inputs | `QBrush * brush` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dcoloredprogressbar.h` native value/reference signature |
| `DColoredProgressBar::removeThreshold` | `()` | copied scalar / no result | shim conversion; `dcoloredprogressbar.h` |
| `DComboBox()` | `new DComboBox` | constructor `Owned` | constructor; `dcombobox.h` |
| `DComboBox::showPopup` | `()` | copied scalar / no result | shim conversion; `dcombobox.h` |
| `DCommandLinkButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dcommandlinkbutton.h` |
| `DCrumbTextFormat()` | `new DCrumbTextFormat` | unique `OwnedOpaque` | constructor; `dcrumbedit.h` |
| `DCrumbTextFormat::tagColor` | `*mut QColor` | owned heap copy | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::setTagColor` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::setTagColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dcrumbedit.h` native value/reference signature |
| `DCrumbTextFormat::text` | `String` | owned Rust strings/container | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::setText` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::textColor` | `*mut QColor` | owned heap copy | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::setTextColor` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::setTextColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dcrumbedit.h` native value/reference signature |
| `DCrumbTextFormat::background` | `*mut QBrush` | owned heap copy | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::setBackground` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::setBackground` Qt value inputs | `QBrush * background` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dcrumbedit.h` native value/reference signature |
| `DCrumbTextFormat::backgroundRadius` | `i32` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbTextFormat::setBackgroundRadius` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit()` | `new DCrumbEdit` | constructor `Owned` | constructor; `dcrumbedit.h` |
| `DCrumbEdit::insertCrumb` | `bool` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::appendCrumb` | `bool` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::containCrumb` | `bool` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::crumbList` | `Vec<String>` | owned Rust strings/container | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::dualClickMakeCrumb` | `bool` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::crumbReadOnly` | `bool` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::crumbRadius` | `i32` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::splitter` | `String` | owned Rust strings/container | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::setCrumbReadOnly` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::setCrumbRadius` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::setSplitter` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DCrumbEdit::setDualClickMakeCrumb` | `()` | copied scalar / no result | shim conversion; `dcrumbedit.h` |
| `DDialog()` | `new DDialog` | constructor `Owned` | constructor; `ddialog.h` |
| `DDialog::getButtonIndexByText` | `i32` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::buttonCount` | `i32` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::contentCount` | `i32` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::getButton` | `QAbstractButton*` | `object_borrow` | `dtkwidget/src/widgets/ddialog.cpp`: returns an existing buttonList entry; removal schedules deletion |
| `DDialog::getContent` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/ddialog.cpp`: returns an existing contentList entry; removal may schedule deletion |
| `DDialog::title` | `String` | owned Rust strings/container | shim conversion; `ddialog.h` |
| `DDialog::message` | `String` | owned Rust strings/container | shim conversion; `ddialog.h` |
| `DDialog::icon` | `*mut QIcon` | owned heap copy | shim conversion; `ddialog.h` |
| `DDialog::textFormat` | `i32` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::onButtonClickedClose` | `bool` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setContentLayoutContentsMargins` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setContentLayoutContentsMargins` Qt value inputs | `QMargins * margins` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `ddialog.h` native value/reference signature |
| `DDialog::contentLayoutContentsMargins` | `*mut QMargins` | owned heap copy | shim conversion; `ddialog.h` |
| `DDialog::closeButtonVisible` | `bool` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::addButton` | `i32` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::addButtons` | `i32` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::insertButton` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::insertButton` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::insertButton` parameters | `QAbstractButton*` | Qt adoption | native `ddialog.h` implementation: button layout/parent and buttonList; duplicate membership forbidden |
| `DDialog::insertButtons` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::removeButton` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::removeButton` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::removeButton` parameters | `QAbstractButton*` | borrow; may delete | native `ddialog.h` implementation: removes native list/layout entries; deletion policy is native |
| `DDialog::removeButtonByText` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::clearButtons` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setDefaultButton` | `bool` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setDefaultButton` | `bool` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setDefaultButton` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setDefaultButton` parameters | `QAbstractButton*` | retained borrow | native `ddialog.h` implementation: must select a live button owned by this dialog |
| `DDialog::addContent` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::addContent` parameters | `QWidget*` | Qt adoption | native `ddialog.h` implementation: contentList/layout; reject duplicate/cyclic handoffs |
| `DDialog::insertContent` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::insertContent` parameters | `QWidget*` | Qt adoption | native `ddialog.h` implementation: contentList/layout; reject duplicate/cyclic handoffs |
| `DDialog::removeContent` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::removeContent` parameters | `QWidget*` | borrow; optional native delete | native `ddialog.h` implementation: deletes only if requested; native list removal precedes deletion |
| `DDialog::clearContents` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setSpacing` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::addSpacing` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::insertSpacing` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::clearSpacing` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setButtonText` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setButtonIcon` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setButtonIcon` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `ddialog.h` native value/reference signature |
| `DDialog::setTitle` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setWordWrapTitle` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setMessage` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setWordWrapMessage` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setIcon` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setIcon` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `ddialog.h` native value/reference signature |
| `DDialog::setTextFormat` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setOnButtonClickedClose` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::setCloseButtonVisible` | `()` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialog::exec` | `i32` | copied scalar / no result | shim conversion; `ddialog.h` |
| `DDialogCloseButton()` | `new DDialogCloseButton` | constructor `Owned` | constructor; `ddialogclosebutton.h` |
| `DDrawer()` | `new DDrawer` | constructor `Owned` | constructor; `ddrawer.h` |
| `DDrawer::setHeader` | `()` | copied scalar / no result | shim conversion; `ddrawer.h` |
| `DDrawer::setHeader` parameters | `QWidget*` | Qt adoption | native `ddrawer.h` implementation: header layout and raw m_header; replacement does not delete old widget |
| `DDrawer::setContent` | `()` | copied scalar / no result | shim conversion; `ddrawer.h` |
| `DDrawer::setContent` parameters | `QWidget*` | Qt adoption | native `ddrawer.h` implementation: content layout and raw m_content; replacement does not delete old widget |
| `DDrawer::getContent` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/ddrawer.cpp`: returns m_content; layout adopts the widget |
| `DDrawer::setHeaderHeight` | `()` | copied scalar / no result | shim conversion; `ddrawer.h` |
| `DDrawer::setExpand` | `()` | copied scalar / no result | shim conversion; `ddrawer.h` |
| `DDrawer::expand` | `bool` | copied scalar / no result | shim conversion; `ddrawer.h` |
| `DDrawer::setAnimationDuration` | `()` | copied scalar / no result | shim conversion; `ddrawer.h` |
| `DDrawer::setSeparatorVisible` | `()` | copied scalar / no result | shim conversion; `ddrawer.h` |
| `DDrawer::setExpandedSeparatorVisible` | `()` | copied scalar / no result | shim conversion; `ddrawer.h` |
| `DDrawerGroup()` | `new DDrawerGroup` | constructor `Owned` | constructor; `ddrawergroup.h` |
| `DDrawerGroup::checkedExpand` | `DDrawer*` | `object_borrow` | `dtkwidget/src/widgets/ddrawergroup.cpp`: returns checkedMap.first; the group does not adopt drawers |
| `DDrawerGroup::expand` | `DDrawer*` | `object_borrow` | `dtkwidget/src/widgets/ddrawergroup.cpp`: returns expandMap.value(id); the group does not adopt drawers |
| `DDrawerGroup::addExpand` | `()` | copied scalar / no result | shim conversion; `ddrawergroup.h` |
| `DDrawerGroup::addExpand` parameters | `DDrawer*` | retained borrow | native `ddrawergroup.h` implementation: native raw expandMap/checkedMap; destroyed-member cleanup required |
| `DDrawerGroup::setId` | `()` | copied scalar / no result | shim conversion; `ddrawergroup.h` |
| `DDrawerGroup::setId` parameters | `DDrawer*` | borrow | native `ddrawergroup.h` implementation: member required; avoid duplicate native connections |
| `DDrawerGroup::removeExpand` | `()` | copied scalar / no result | shim conversion; `ddrawergroup.h` |
| `DDrawerGroup::removeExpand` parameters | `DDrawer*` | borrow; releases relation | native `ddrawergroup.h` implementation: native remove does not disconnect/clear checkedMap; shim must repair |
| `DDrawerGroup::checkedId` | `i32` | copied scalar / no result | shim conversion; `ddrawergroup.h` |
| `DDrawerGroup::id` | `i32` | copied scalar / no result | shim conversion; `ddrawergroup.h` |
| `DDrawerGroup::id` parameters | `DDrawer*` | borrow | native `ddrawergroup.h` implementation: reject absent member before native keys().at(-1) |
| `DEnhancedWidget::target` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/denhancedwidget.cpp`: returns d->target; observer does not own its target |
| `DEnhancedWidget::enabled` | `bool` | copied scalar / no result | shim conversion; `denhancedwidget.h` |
| `DEnhancedWidget::setTarget` | `()` | copied scalar / no result | shim conversion; `denhancedwidget.h` |
| `DEnhancedWidget::setTarget` parameters | `QWidget*` | retained borrow | native `denhancedwidget.h` implementation: native raw target; observer does not adopt; clear before target lifetime ends |
| `DEnhancedWidget::setEnabled` | `()` | copied scalar / no result | shim conversion; `denhancedwidget.h` |
| `DFeatureItem()` | `new DFeatureItem` | constructor `Owned` | constructor; `dfeaturedisplaydialog.h` |
| `DFeatureItem::icon` | `*mut QIcon` | owned heap copy | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureItem::setIcon` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureItem::setIcon` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfeaturedisplaydialog.h` native value/reference signature |
| `DFeatureItem::name` | `String` | owned Rust strings/container | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureItem::setName` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureItem::description` | `String` | owned Rust strings/container | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureItem::setDescription` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureDisplayDialog()` | `new DFeatureDisplayDialog` | constructor `Owned` | constructor; `dfeaturedisplaydialog.h` |
| `DFeatureDisplayDialog::setTitle` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureDisplayDialog::addItem` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureDisplayDialog::addItem` parameters | `DFeatureItem*` | native ownership transfer without QObject parent | native `dfeaturedisplaydialog.h` implementation: m_featureItems retains item; dialog destructor/clearItems schedules its deletion; original owner must relinquish deletion |
| `DFeatureDisplayDialog::removeItem` | `DFeatureItem*` | `object_transfer` | `dtkwidget/src/widgets/dfeaturedisplaydialog.cpp`: removeOne releases retained identity without deletion; shim verifies membership and detaches its binding-added parent before returning a new owner |
| `DFeatureDisplayDialog::removeItem` parameters | `DFeatureItem*` | ownership transfer back to caller | native `dfeaturedisplaydialog.h` implementation: removeOne does not delete/reparent item; only verified membership may produce a fresh owner |
| `DFeatureDisplayDialog::clearItems` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureDisplayDialog::setLinkButtonVisible` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureDisplayDialog::setLinkUrl` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureDisplayDialog::show` | `()` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFeatureDisplayDialog::isEmpty` | `bool` | copied scalar / no result | shim conversion; `dfeaturedisplaydialog.h` |
| `DFileChooserEdit()` | `new DFileChooserEdit` | constructor `Owned` | constructor; `dfilechooseredit.h` |
| `DFileChooserEdit::setNameFilters` | `()` | copied scalar / no result | shim conversion; `dfilechooseredit.h` |
| `DFileChooserEdit::nameFilters` | `Vec<String>` | owned Rust strings/container | shim conversion; `dfilechooseredit.h` |
| `DFileChooserEdit::setDirectoryUrl` | `()` | copied scalar / no result | shim conversion; `dfilechooseredit.h` |
| `DFileChooserEdit::setDirectoryUrl` Qt value inputs | `QUrl * directory` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfilechooseredit.h` native value/reference signature |
| `DFileChooserEdit::directoryUrl` | `*mut QUrl` | owned heap copy | shim conversion; `dfilechooseredit.h` |
| `DFileChooserEdit::setDialogDisplayPosition` | `()` | copied scalar / no result | shim conversion; `dfilechooseredit.h` |
| `DFileChooserEdit::dialogDisplayPosition` | `i32` | copied scalar / no result | shim conversion; `dfilechooseredit.h` |
| `DFileChooserEdit::initDialog` | `()` | copied scalar / no result | shim conversion; `dfilechooseredit.h` |
| `DFileDialog()` | `new DFileDialog` | constructor `Owned` | constructor; `dfiledialog.h` |
| `DFileDialog::addComboBox` | `()` | copied scalar / no result | shim conversion; `dfiledialog.h` |
| `DFileDialog::addLineEdit` | `()` | copied scalar / no result | shim conversion; `dfiledialog.h` |
| `DFileDialog::setAllowMixedSelection` | `()` | copied scalar / no result | shim conversion; `dfiledialog.h` |
| `DFileDialog::getComboBoxValue` | `String` | owned Rust strings/container | shim conversion; `dfiledialog.h` |
| `DFileDialog::getLineEditValue` | `String` | owned Rust strings/container | shim conversion; `dfiledialog.h` |
| `DFileDialog::setVisible` | `()` | copied scalar / no result | shim conversion; `dfiledialog.h` |
| `DFileIconProvider()` | `new DFileIconProvider` | unique `OwnedOpaque` | constructor; `dfileiconprovider.h` |
| `DFileIconProvider::globalProvider` | `DFileIconProvider*` | `opaque_static` | `dtkwidget/src/util/dfileiconprovider.cpp`: returns Q_GLOBAL_STATIC(DFileIconProvider, globalFIP); caller must not delete |
| `DFloatingButton()` | `new DFloatingButton` | constructor `Owned` | constructor; `dfloatingbutton.h` |
| `DFloatingMessage()` | `new DFloatingMessage` | constructor `Owned` | constructor; `dfloatingmessage.h` |
| `DFloatingMessage::messageType` | `i32` | copied scalar / no result | shim conversion; `dfloatingmessage.h` |
| `DFloatingMessage::setIcon` | `()` | copied scalar / no result | shim conversion; `dfloatingmessage.h` |
| `DFloatingMessage::setIcon` Qt value inputs | `QIcon * ico` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfloatingmessage.h` native value/reference signature |
| `DFloatingMessage::setIcon` | `()` | copied scalar / no result | shim conversion; `dfloatingmessage.h` |
| `DFloatingMessage::setIcon` Qt value inputs | `DDciIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfloatingmessage.h` native value/reference signature |
| `DFloatingMessage::setMessage` | `()` | copied scalar / no result | shim conversion; `dfloatingmessage.h` |
| `DFloatingMessage::setWidget` | `()` | copied scalar / no result | shim conversion; `dfloatingmessage.h` |
| `DFloatingMessage::setWidget` parameters | `QWidget*` | Qt adoption | native `dfloatingmessage.h` implementation: content layout; deletes old layout item, not widget |
| `DFloatingMessage::setDuration` | `()` | copied scalar / no result | shim conversion; `dfloatingmessage.h` |
| `DFloatingMessage::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dfloatingmessage.h` |
| `DFloatingWidget()` | `new DFloatingWidget` | constructor `Owned` | constructor; `dfloatingwidget.h` |
| `DFloatingWidget::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dfloatingwidget.h` |
| `DFloatingWidget::setWidget` | `()` | copied scalar / no result | shim conversion; `dfloatingwidget.h` |
| `DFloatingWidget::setWidget` parameters | `QWidget*` | Qt adoption | native `dfloatingwidget.h` implementation: layout manages content; removes layout items before insertion |
| `DFloatingWidget::setFramRadius` | `()` | copied scalar / no result | shim conversion; `dfloatingwidget.h` |
| `DFloatingWidget::initStyleOption` | `()` | copied scalar / no result | shim conversion; `dfloatingwidget.h` |
| `DFloatingWidget::initStyleOption` parameters | `DStyleOptionFloatingWidget*` | exclusive output borrow | native `dfloatingwidget.h` implementation: non-const option pointer is populated; Rust requires &mut OwnedOpaque<T>, not replaceable &mut borrowed T |
| `DFloatingWidget::blurBackgroundIsEnabled` | `bool` | copied scalar / no result | shim conversion; `dfloatingwidget.h` |
| `DFloatingWidget::blurBackground` | `DBlurEffectWidget*` | `object_borrow` | `dtkwidget/src/widgets/dfloatingwidget.cpp`: returns existing background child; disabling queues its deletion |
| `DFloatingWidget::setBlurBackgroundEnabled` | `()` | copied scalar / no result | shim conversion; `dfloatingwidget.h` |
| `DFlowLayout()` | `new DFlowLayout` | constructor `Owned` | constructor; `dflowlayout.h` |
| `DFlowLayout::insertWidget` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::insertWidget` parameters | `QWidget*` | Qt adoption | native `dflowlayout.h` implementation: addChildWidget + owned QWidgetItemV2 |
| `DFlowLayout::insertLayout` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::insertLayout` parameters | `QLayout*` | Qt adoption | native `dflowlayout.h` implementation: reject already-parented/duplicate/cyclic layout before addChildLayout + insertItem |
| `DFlowLayout::insertSpacing` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::insertStretch` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::addSpacing` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::addStretch` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::hasHeightForWidth` | `bool` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::heightForWidth` | `i32` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::count` | `i32` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::minimumSize` | `*mut QSize` | owned heap copy | shim conversion; `dflowlayout.h` |
| `DFlowLayout::setGeometry` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::setGeometry` Qt value inputs | `QRect * rect` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dflowlayout.h` native value/reference signature |
| `DFlowLayout::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dflowlayout.h` |
| `DFlowLayout::expandingDirections` | `i32` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::horizontalSpacing` | `i32` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::verticalSpacing` | `i32` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::setHorizontalSpacing` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::setVerticalSpacing` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFlowLayout::setSpacing` | `()` | copied scalar / no result | shim conversion; `dflowlayout.h` |
| `DFontComboBox()` | `new DFontComboBox` | constructor `Owned` | constructor; `dfontcombobox.h` |
| `DFontComboBox::currentFont` | `*mut QFont` | owned heap copy | shim conversion; `dfontcombobox.h` |
| `DFontComboBox::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dfontcombobox.h` |
| `DFontComboBox::setCurrentFont` | `()` | copied scalar / no result | shim conversion; `dfontcombobox.h` |
| `DFontComboBox::setCurrentFont` Qt value inputs | `QFont * f` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dfontcombobox.h` native value/reference signature |
| `DFrame()` | `new DFrame` | constructor `Owned` | constructor; `dframe.h` |
| `DFrame::setFrameRounded` | `()` | copied scalar / no result | shim conversion; `dframe.h` |
| `DFrame::setBackgroundRole` | `()` | copied scalar / no result | shim conversion; `dframe.h` |
| `DHorizontalLine()` | `new DHorizontalLine` | constructor `Owned` | constructor; `dframe.h` |
| `DVerticalLine()` | `new DVerticalLine` | constructor `Owned` | constructor; `dframe.h` |
| `DGraphicsClipEffect()` | `new DGraphicsClipEffect` | constructor `Owned` | constructor; `dgraphicsclipeffect.h` |
| `DGraphicsClipEffect::margins` | `*mut QMargins` | owned heap copy | shim conversion; `dgraphicsclipeffect.h` |
| `DGraphicsClipEffect::clipPath` | `*mut QPainterPath` | owned heap copy | shim conversion; `dgraphicsclipeffect.h` |
| `DGraphicsClipEffect::setMargins` | `()` | copied scalar / no result | shim conversion; `dgraphicsclipeffect.h` |
| `DGraphicsClipEffect::setMargins` Qt value inputs | `QMargins * margins` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dgraphicsclipeffect.h` native value/reference signature |
| `DGraphicsClipEffect::setClipPath` | `()` | copied scalar / no result | shim conversion; `dgraphicsclipeffect.h` |
| `DGraphicsClipEffect::setClipPath` Qt value inputs | `QPainterPath * clipPath` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dgraphicsclipeffect.h` native value/reference signature |
| `DGraphicsGlowEffect()` | `new DGraphicsGlowEffect` | constructor `Owned` | constructor; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::setOffset` | `()` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::setXOffset` | `()` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::xOffset` | `f64` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::setYOffset` | `()` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::yOffset` | `f64` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::setDistance` | `()` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::distance` | `f64` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::setBlurRadius` | `()` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::blurRadius` | `f64` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::setColor` | `()` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::setColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dgraphicsgloweffect.h` native value/reference signature |
| `DGraphicsGlowEffect::color` | `*mut QColor` | owned heap copy | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::opacity` | `f64` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DGraphicsGlowEffect::setOpacity` | `()` | copied scalar / no result | shim conversion; `dgraphicsgloweffect.h` |
| `DHeaderLine()` | `new DHeaderLine` | constructor `Owned` | constructor; `dheaderline.h` |
| `DHeaderLine::setTitle` | `()` | copied scalar / no result | shim conversion; `dheaderline.h` |
| `DHeaderLine::setContent` | `()` | copied scalar / no result | shim conversion; `dheaderline.h` |
| `DHeaderLine::setContent` parameters | `QWidget*` | Qt adoption | native `dheaderline.h` implementation: delegates to DBaseLine layout handoff |
| `DHeaderLine::title` | `String` | owned Rust strings/container | shim conversion; `dheaderline.h` |
| `DIconButton()` | `new DIconButton` | constructor `Owned` | constructor; `diconbutton.h` |
| `DIconButton::setIcon` | `()` | copied scalar / no result | shim conversion; `diconbutton.h` |
| `DIconButton::setIcon` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `diconbutton.h` native value/reference signature |
| `DIconButton::setIcon` | `()` | copied scalar / no result | shim conversion; `diconbutton.h` |
| `DIconButton::setIcon` | `()` | copied scalar / no result | shim conversion; `diconbutton.h` |
| `DIconButton::setIcon` Qt value inputs | `DDciIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `diconbutton.h` native value/reference signature |
| `DIconButton::dciIcon` | `*mut DDciIcon` | owned heap copy | shim conversion; `diconbutton.h` |
| `DIconButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `diconbutton.h` |
| `DIconButton::minimumSizeHint` | `*mut QSize` | owned heap copy | shim conversion; `diconbutton.h` |
| `DIconButton::iconSize` | `*mut QSize` | owned heap copy | shim conversion; `diconbutton.h` |
| `DIconButton::isFlat` | `bool` | copied scalar / no result | shim conversion; `diconbutton.h` |
| `DIconButton::setEnabledCircle` | `()` | copied scalar / no result | shim conversion; `diconbutton.h` |
| `DIconButton::enabledCircle` | `bool` | copied scalar / no result | shim conversion; `diconbutton.h` |
| `DIconButton::setNewNotification` | `()` | copied scalar / no result | shim conversion; `diconbutton.h` |
| `DIconButton::setFlat` | `()` | copied scalar / no result | shim conversion; `diconbutton.h` |
| `DImageViewer()` | `new DImageViewer` | constructor `Owned` | constructor; `dimageviewer.h` |
| `DImageViewer::image` | `*mut QImage` | owned heap copy | shim conversion; `dimageviewer.h` |
| `DImageViewer::setImage` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::setImage` Qt value inputs | `QImage * image` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimageviewer.h` native value/reference signature |
| `DImageViewer::fileName` | `String` | owned Rust strings/container | shim conversion; `dimageviewer.h` |
| `DImageViewer::setFileName` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::scaleFactor` | `f64` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::setScaleFactor` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::scaleImage` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::autoFitImage` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::fitToWidget` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::fitNormalSize` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::rotateClockwise` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::rotateCounterclockwise` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::rotateAngle` | `i32` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::resetRotateAngle` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::clear` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::centerOn` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::visibleImageRect` | `*mut QRect` | owned heap copy | shim conversion; `dimageviewer.h` |
| `DImageViewer::scaleAtPoint` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::scaleAtPoint` Qt value inputs | `QPoint * pos` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dimageviewer.h` native value/reference signature |
| `DImageViewer::beginCropImage` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::endCropImage` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::resetCropImage` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::setCropAspectRatio` | `()` | copied scalar / no result | shim conversion; `dimageviewer.h` |
| `DImageViewer::cropImageRect` | `*mut QRect` | owned heap copy | shim conversion; `dimageviewer.h` |
| `DIndeterminateProgressbar()` | `new DIndeterminateProgressbar` | constructor `Owned` | constructor; `dindeterminateprogressbar.h` |
| `DInputDialog()` | `new DInputDialog` | constructor `Owned` | constructor; `dinputdialog.h` |
| `DInputDialog::setInputMode` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::inputMode` | `i32` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setTextValue` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::textValue` | `String` | owned Rust strings/container | shim conversion; `dinputdialog.h` |
| `DInputDialog::setTextEchoMode` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::textEchoMode` | `i32` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setComboBoxEditable` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::isComboBoxEditable` | `bool` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setComboBoxItems` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::comboBoxItems` | `Vec<String>` | owned Rust strings/container | shim conversion; `dinputdialog.h` |
| `DInputDialog::setComboBoxCurrentIndex` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::comboBoxCurrentIndex` | `i32` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setIntValue` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::intValue` | `i32` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setIntMinimum` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::intMinimum` | `i32` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setIntMaximum` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::intMaximum` | `i32` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setIntRange` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setIntStep` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::intStep` | `i32` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setDoubleValue` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::doubleValue` | `f64` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setDoubleMinimum` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::doubleMinimum` | `f64` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setDoubleMaximum` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::doubleMaximum` | `f64` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setDoubleRange` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setDoubleDecimals` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::doubleDecimals` | `i32` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setOkButtonText` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::okButtonText` | `String` | owned Rust strings/container | shim conversion; `dinputdialog.h` |
| `DInputDialog::setOkButtonEnabled` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::okButtonIsEnabled` | `bool` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::setCancelButtonText` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::cancelButtonText` | `String` | owned Rust strings/container | shim conversion; `dinputdialog.h` |
| `DInputDialog::setTextAlert` | `()` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DInputDialog::isTextAlert` | `bool` | copied scalar / no result | shim conversion; `dinputdialog.h` |
| `DIpv4LineEdit()` | `new DIpv4LineEdit` | constructor `Owned` | constructor; `dipv4lineedit.h` |
| `DIpv4LineEdit::displayText` | `String` | owned Rust strings/container | shim conversion; `dipv4lineedit.h` |
| `DIpv4LineEdit::cursorPosition` | `i32` | copied scalar / no result | shim conversion; `dipv4lineedit.h` |
| `DIpv4LineEdit::alignment` | `i32` | copied scalar / no result | shim conversion; `dipv4lineedit.h` |
| `DIpv4LineEdit::hasAcceptableInput` | `bool` | copied scalar / no result | shim conversion; `dipv4lineedit.h` |
| `DIpv4LineEdit::isReadOnly` | `bool` | copied scalar / no result | shim conversion; `dipv4lineedit.h` |
| `DIpv4LineEdit::setCursorPosition` | `()` | copied scalar / no result | shim conversion; `dipv4lineedit.h` |
| `DIpv4LineEdit::setReadOnly` | `()` | copied scalar / no result | shim conversion; `dipv4lineedit.h` |
| `DIpv4LineEdit::setSelection` | `()` | copied scalar / no result | shim conversion; `dipv4lineedit.h` |
| `DIpv4LineEdit::selectAll` | `()` | copied scalar / no result | shim conversion; `dipv4lineedit.h` |
| `DKeySequenceEdit()` | `new DKeySequenceEdit` | constructor `Owned` | constructor; `dkeysequenceedit.h` |
| `DKeySequenceEdit::clear` | `()` | copied scalar / no result | shim conversion; `dkeysequenceedit.h` |
| `DKeySequenceEdit::setKeySequence` | `bool` | copied scalar / no result | shim conversion; `dkeysequenceedit.h` |
| `DKeySequenceEdit::setKeySequence` Qt value inputs | `QKeySequence * keySequence` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dkeysequenceedit.h` native value/reference signature |
| `DKeySequenceEdit::keySequence` | `*mut QKeySequence` | owned heap copy | shim conversion; `dkeysequenceedit.h` |
| `DKeySequenceEdit::ShortcutDirection` | `()` | copied scalar / no result | shim conversion; `dkeysequenceedit.h` |
| `DKeySequenceEdit::getKeySequence` | `String` | owned Rust strings/container | shim conversion; `dkeysequenceedit.h` |
| `DKeySequenceEdit::getKeySequence` Qt value inputs | `QKeySequence * sequence` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dkeysequenceedit.h` native value/reference signature |
| `DLicenseDialog()` | `new DLicenseDialog` | constructor `Owned` | constructor; `dlicensedialog.h` |
| `DLicenseDialog::setContent` | `()` | copied scalar / no result | shim conversion; `dlicensedialog.h` |
| `DLicenseDialog::setFile` | `()` | copied scalar / no result | shim conversion; `dlicensedialog.h` |
| `DLicenseDialog::setLicenseSearchPath` | `()` | copied scalar / no result | shim conversion; `dlicensedialog.h` |
| `DLicenseDialog::load` | `bool` | copied scalar / no result | shim conversion; `dlicensedialog.h` |
| `DLicenseDialog::isValid` | `bool` | copied scalar / no result | shim conversion; `dlicensedialog.h` |
| `DLineEdit()` | `new DLineEdit` | constructor `Owned` | constructor; `dlineedit.h` |
| `DLineEdit::setPlaceholderText` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setAlert` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::isAlert` | `bool` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::showAlertMessage` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::showAlertMessage` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::showAlertMessage` parameters | `QWidget*` | borrow | native `dlineedit.h` implementation: alert follows widget; no caller ownership transfer |
| `DLineEdit::setAlertMessageAlignment` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::alertMessageAlignment` | `i32` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::hideAlertMessage` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setLeftWidgetsVisible` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setRightWidgetsVisible` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setClearButtonEnabled` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::isClearButtonEnabled` | `bool` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setText` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::text` | `String` | owned Rust strings/container | shim conversion; `dlineedit.h` |
| `DLineEdit::clear` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::echoMode` | `i32` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setEchoMode` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setContextMenuPolicy` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::speechToTextIsEnabled` | `bool` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setSpeechToTextEnabled` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::textToSpeechIsEnabled` | `bool` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setTextToSpeechEnabled` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::textToTranslateIsEnabled` | `bool` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setTextToTranslateEnabled` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::copyEnabled` | `bool` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setCopyEnabled` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::cutEnabled` | `bool` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setCutEnabled` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::pasteEnabled` | `bool` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DLineEdit::setPasteEnabled` | `()` | copied scalar / no result | shim conversion; `dlineedit.h` |
| `DVariantListModel()` | `new DVariantListModel` | constructor `Owned` | constructor; `dlistview.h` |
| `DListView()` | `new DListView` | constructor `Owned` | constructor; `dlistview.h` |
| `DListView::getHeaderWidget` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dlistview.cpp`: returns headerList entry; view/layout owns it |
| `DListView::getFooterWidget` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dlistview.cpp`: returns footerList entry; view/layout owns it |
| `DListView::isActiveRect` | `bool` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::isActiveRect` Qt value inputs | `QRect * rect` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dlistview.h` native value/reference signature |
| `DListView::isVisualRect` | `bool` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::isVisualRect` Qt value inputs | `QRect * rect` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dlistview.h` native value/reference signature |
| `DListView::count` | `i32` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::orientation` | `i32` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::setModel` | `()` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::setModel` parameters | `QAbstractItemModel*` | retained borrow | native `dlistview.h` implementation: QAbstractItemView::setModel does not transfer QObject ownership; Qt tracks model destruction |
| `DListView::minimumSizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dlistview.h` |
| `DListView::backgroundType` | `i32` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::itemMargins` | `*mut QMargins` | owned heap copy | shim conversion; `dlistview.h` |
| `DListView::itemSize` | `*mut QSize` | owned heap copy | shim conversion; `dlistview.h` |
| `DListView::addItem` | `bool` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::addItem` Qt value inputs | `QVariant * data` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dlistview.h` native value/reference signature |
| `DListView::addItems` | `bool` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::addItems` Qt value inputs | `QVariantList * datas` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dlistview.h` native value/reference signature |
| `DListView::insertItem` | `bool` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::insertItem` Qt value inputs | `QVariant * data` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dlistview.h` native value/reference signature |
| `DListView::insertItems` | `bool` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::insertItems` Qt value inputs | `QVariantList * datas` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dlistview.h` native value/reference signature |
| `DListView::removeItem` | `bool` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::removeItems` | `bool` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::addHeaderWidget` | `i32` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::addHeaderWidget` parameters | `QWidget*` | Qt adoption | native `dlistview.h` implementation: view headerList/layout; take leaves widget parented; empty header container's deferred deletion destroys it |
| `DListView::removeHeaderWidget` | `()` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::takeHeaderWidget` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dlistview.cpp`: removes list/layout entry but leaves widget parented; an empty header container's deleteLater destroys it; NOT a transfer |
| `DListView::addFooterWidget` | `i32` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::addFooterWidget` parameters | `QWidget*` | Qt adoption | native `dlistview.h` implementation: view footerList/layout; take leaves widget parented; empty footer container's deferred deletion destroys it |
| `DListView::removeFooterWidget` | `()` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::takeFooterWidget` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dlistview.cpp`: removes list/layout entry but leaves widget parented; an empty footer container's deleteLater destroys it; NOT a transfer |
| `DListView::setBackgroundType` | `()` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::setItemMargins` | `()` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::setItemMargins` Qt value inputs | `QMargins * itemMargins` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dlistview.h` native value/reference signature |
| `DListView::setItemSize` | `()` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::setItemSize` Qt value inputs | `QSize * itemSize` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dlistview.h` native value/reference signature |
| `DListView::setItemSpacing` | `()` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DListView::setItemRadius` | `()` | copied scalar / no result | shim conversion; `dlistview.h` |
| `DLoadingIndicator()` | `new DLoadingIndicator` | constructor `Owned` | constructor; `dloadingindicator.h` |
| `DLoadingIndicator::backgroundColor` | `*mut QColor` | owned heap copy | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::loading` | `bool` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::widgetSource` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dloadingindicator.cpp`: returns widgetSource; QGraphicsProxyWidget adopts its widget |
| `DLoadingIndicator::imageSource` | `*mut QPixmap` | owned heap copy | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::aniDuration` | `i32` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::smooth` | `bool` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::direction` | `i32` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::rotate` | `f64` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::start` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::stop` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::setLoading` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::setAniDuration` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::setBackgroundColor` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::setBackgroundColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dloadingindicator.h` native value/reference signature |
| `DLoadingIndicator::setRotate` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::setRotate` Qt value inputs | `QVariant * angle` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dloadingindicator.h` native value/reference signature |
| `DLoadingIndicator::setWidgetSource` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::setWidgetSource` parameters | `QWidget*` | proxy adoption | native `dloadingindicator.h` implementation: QGraphicsProxyWidget shares destruction with embedded widget, not necessarily QObject parenting; raw widgetSource requires repair |
| `DLoadingIndicator::setImageSource` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::setImageSource` Qt value inputs | `QPixmap * imageSource` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dloadingindicator.h` native value/reference signature |
| `DLoadingIndicator::setSmooth` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DLoadingIndicator::setDirection` | `()` | copied scalar / no result | shim conversion; `dloadingindicator.h` |
| `DMessageManager::instance` | `DMessageManager*` | `object_borrow` | `dtkwidget/src/widgets/dmessagemanager.cpp`: returns function-static manager; no caller ownership |
| `DMessageManager::sendMessage` | `()` | copied scalar / no result | shim conversion; `dmessagemanager.h` |
| `DMessageManager::sendMessage` parameters | `QWidget*, DFloatingMessage*` | Qt adoption / borrow parent | native `dmessagemanager.h` implementation: floating message becomes content-layout child; parent argument is borrowed |
| `DMessageManager::sendMessage` | `()` | copied scalar / no result | shim conversion; `dmessagemanager.h` |
| `DMessageManager::sendMessage` parameters | `QWidget*` | Qt adoption / borrow parent | native `dmessagemanager.h` implementation: floating message becomes content-layout child; parent argument is borrowed |
| `DMessageManager::sendMessage` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dmessagemanager.h` native value/reference signature |
| `DMessageManager::sendMessage` | `()` | copied scalar / no result | shim conversion; `dmessagemanager.h` |
| `DMessageManager::sendMessage` parameters | `QWidget*` | Qt adoption / borrow parent | native `dmessagemanager.h` implementation: floating message becomes content-layout child; parent argument is borrowed |
| `DMessageManager::sendMessage` Qt value inputs | `DDciIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dmessagemanager.h` native value/reference signature |
| `DMessageManager::setContentMargens` | `bool` | copied scalar / no result | shim conversion; `dmessagemanager.h` |
| `DMessageManager::setContentMargens` parameters | `QWidget*` | borrow | native `dmessagemanager.h` implementation: changes existing content layout; no transfer |
| `DMessageManager::setContentMargens` Qt value inputs | `QMargins * margins` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dmessagemanager.h` native value/reference signature |
| `DMPRISControl()` | `new DMPRISControl` | constructor `Owned` | constructor; `dmpriscontrol.h` |
| `DMPRISControl::isWorking` | `bool` | copied scalar / no result | shim conversion; `dmpriscontrol.h` |
| `DMPRISControl::setPictureVisible` | `()` | copied scalar / no result | shim conversion; `dmpriscontrol.h` |
| `DMPRISControl::setPictureSize` | `()` | copied scalar / no result | shim conversion; `dmpriscontrol.h` |
| `DMPRISControl::setPictureSize` Qt value inputs | `QSize * size` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dmpriscontrol.h` native value/reference signature |
| `DPageIndicator()` | `new DPageIndicator` | constructor `Owned` | constructor; `dpageindicator.h` |
| `DPageIndicator::pageCount` | `i32` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setPageCount` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::nextPage` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::previousPage` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setCurrentPage` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::currentPageIndex` | `i32` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::pointColor` | `*mut QColor` | owned heap copy | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setPointColor` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setPointColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dpageindicator.h` native value/reference signature |
| `DPageIndicator::secondaryPointColor` | `*mut QColor` | owned heap copy | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setSecondaryPointColor` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setSecondaryPointColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dpageindicator.h` native value/reference signature |
| `DPageIndicator::pointRadius` | `i32` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setPointRadius` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::secondaryPointRadius` | `i32` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setSecondaryPointRadius` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::pointDistance` | `i32` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPageIndicator::setPointDistance` | `()` | copied scalar / no result | shim conversion; `dpageindicator.h` |
| `DPaletteHelper::instance` | `DPaletteHelper*` | `object_borrow` | `dtkwidget/src/widgets/dpalettehelper.cpp`: returns native g_instance; cleanup is native-owned |
| `DPaletteHelper::palette` | `*mut DPalette` | owned heap copy | shim conversion; `dpalettehelper.h` |
| `DPaletteHelper::palette` parameters | `QWidget*` | borrow | native `dpalettehelper.h` implementation: returns owned palette copy |
| `DPaletteHelper::palette` Qt value inputs | `QPalette * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dpalettehelper.h` native value/reference signature |
| `DPaletteHelper::setPalette` | `()` | copied scalar / no result | shim conversion; `dpalettehelper.h` |
| `DPaletteHelper::setPalette` parameters | `QWidget*` | borrow | native `dpalettehelper.h` implementation: copies palette value into widget/native palette state |
| `DPaletteHelper::setPalette` Qt value inputs | `DPalette * palette` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dpalettehelper.h` native value/reference signature |
| `DPaletteHelper::resetPalette` | `()` | copied scalar / no result | shim conversion; `dpalettehelper.h` |
| `DPaletteHelper::resetPalette` parameters | `QWidget*` | borrow | native `dpalettehelper.h` implementation: resets widget palette, no ownership transfer |
| `DPasswordEdit()` | `new DPasswordEdit` | constructor `Owned` | constructor; `dpasswordedit.h` |
| `DPasswordEdit::isEchoMode` | `bool` | copied scalar / no result | shim conversion; `dpasswordedit.h` |
| `DPasswordEdit::setEchoMode` | `()` | copied scalar / no result | shim conversion; `dpasswordedit.h` |
| `DPasswordEdit::setEchoButtonIsVisible` | `()` | copied scalar / no result | shim conversion; `dpasswordedit.h` |
| `DPasswordEdit::echoButtonIsVisible` | `bool` | copied scalar / no result | shim conversion; `dpasswordedit.h` |
| `DPictureSequenceView()` | `new DPictureSequenceView` | constructor `Owned` | constructor; `dpicturesequenceview.h` |
| `DPictureSequenceView::setPictureSequence` | `()` | copied scalar / no result | shim conversion; `dpicturesequenceview.h` |
| `DPictureSequenceView::play` | `()` | copied scalar / no result | shim conversion; `dpicturesequenceview.h` |
| `DPictureSequenceView::pause` | `()` | copied scalar / no result | shim conversion; `dpicturesequenceview.h` |
| `DPictureSequenceView::stop` | `()` | copied scalar / no result | shim conversion; `dpicturesequenceview.h` |
| `DPictureSequenceView::speed` | `i32` | copied scalar / no result | shim conversion; `dpicturesequenceview.h` |
| `DPictureSequenceView::setSpeed` | `()` | copied scalar / no result | shim conversion; `dpicturesequenceview.h` |
| `DPictureSequenceView::singleShot` | `bool` | copied scalar / no result | shim conversion; `dpicturesequenceview.h` |
| `DPictureSequenceView::setSingleShot` | `()` | copied scalar / no result | shim conversion; `dpicturesequenceview.h` |
| `DPlatformWindowHandle::enableDXcbForWindow` | `()` | copied scalar / no result | shim conversion; `dplatformwindowhandle.h` |
| `DPlatformWindowHandle::enableDXcbForWindow` parameters | `QWidget*` | borrow | native `dplatformwindowhandle.h` implementation: platform property/configuration, not QWidget ownership |
| `DPlatformWindowHandle::enableDXcbForWindow` | `()` | copied scalar / no result | shim conversion; `dplatformwindowhandle.h` |
| `DPlatformWindowHandle::enableDXcbForWindow` parameters | `QWidget*` | borrow | native `dplatformwindowhandle.h` implementation: platform property/configuration, not QWidget ownership |
| `DPlatformWindowHandle::isEnabledDXcb` | `bool` | copied scalar / no result | shim conversion; `dplatformwindowhandle.h` |
| `DPlatformWindowHandle::isEnabledDXcb` parameters | `QWidget*` | borrow | native `dplatformwindowhandle.h` implementation: platform property lookup |
| `DPlatformWindowHandle::setWindowSurfaceType` | `()` | copied scalar / no result | shim conversion; `dplatformwindowhandle.h` |
| `ColorLabel::getColor` | `*mut QColor` | owned heap copy | shim conversion; `dprintpickcolorwidget.h` |
| `ColorLabel::setHue` | `()` | copied scalar / no result | shim conversion; `dprintpickcolorwidget.h` |
| `ColorLabel::pickColor` | `()` | copied scalar / no result | shim conversion; `dprintpickcolorwidget.h` |
| `ColorLabel::pickColor` Qt value inputs | `QPoint * pos` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpickcolorwidget.h` native value/reference signature |
| `ColorSlider()` | `new ColorSlider` | constructor `Owned` | constructor; `dprintpickcolorwidget.h` |
| `ColorSlider::getColor` | `*mut QColor` | owned heap copy | shim conversion; `dprintpickcolorwidget.h` |
| `DPrintPickColorWidget()` | `new DPrintPickColorWidget` | constructor `Owned` | constructor; `dprintpickcolorwidget.h` |
| `DPrintPickColorWidget::initUI` | `()` | copied scalar / no result | shim conversion; `dprintpickcolorwidget.h` |
| `DPrintPickColorWidget::initConnection` | `()` | copied scalar / no result | shim conversion; `dprintpickcolorwidget.h` |
| `DPrintPickColorWidget::setRgbEdit` | `()` | copied scalar / no result | shim conversion; `dprintpickcolorwidget.h` |
| `DPrintPickColorWidget::setRgbEdit` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpickcolorwidget.h` native value/reference signature |
| `DPrintPickColorWidget::convertColor` | `()` | copied scalar / no result | shim conversion; `dprintpickcolorwidget.h` |
| `DPrintPickColorWidget::convertColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpickcolorwidget.h` native value/reference signature |
| `DPrintPickColorWidget::slotColorPick` | `()` | copied scalar / no result | shim conversion; `dprintpickcolorwidget.h` |
| `DPrintPickColorWidget::slotEditColor` | `()` | copied scalar / no result | shim conversion; `dprintpickcolorwidget.h` |
| `DPrintPreviewDialog()` | `new DPrintPreviewDialog` | constructor `Owned` | constructor; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::setPluginMimeData` | `()` | copied scalar / no result | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::setPluginMimeData` Qt value inputs | `QVariant * mimeData` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpreviewdialog.h` native value/reference signature |
| `DPrintPreviewDialog::pluginMimeData` | `*mut QVariant` | owned heap copy | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::setCurrentPlugin` | `bool` | copied scalar / no result | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::currentPlugin` | `String` | owned Rust strings/container | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::availablePlugins` | `Vec<String>` | owned Rust strings/container | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::setDocName` | `()` | copied scalar / no result | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::docName` | `String` | owned Rust strings/container | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::setPrintFromPath` | `bool` | copied scalar / no result | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::printFromPath` | `String` | owned Rust strings/container | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::setAsynPreview` | `bool` | copied scalar / no result | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::isAsynPreview` | `bool` | copied scalar / no result | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::createDialogSettingInfo` | `DPrintPreviewSettingInfo*` | `opaque_transfer` | `dtkwidget/src/widgets/dprintpreviewdialog.cpp`: loadInfo allocates derived setting info with new; rejected plugin results deleted internally; successful results released by caller; base destructor virtual |
| `DPrintPreviewDialog::updateDialogSettingInfo` | `()` | copied scalar / no result | shim conversion; `dprintpreviewdialog.h` |
| `DPrintPreviewDialog::updateDialogSettingInfo` parameters | `DPrintPreviewSettingInfo*` | borrow | native `dprintpreviewdialog.h` implementation: reads populated derived setting info synchronously; does not retain/delete successful caller result |
| `DPrintPreviewSettingInfo::type` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewPrinterInfo()` | `new DPrintPreviewPrinterInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewCopiesInfo()` | `new DPrintPreviewCopiesInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewPageRangeInfo()` | `new DPrintPreviewPageRangeInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewOrientationInfo()` | `new DPrintPreviewOrientationInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewPaperSizeInfo()` | `new DPrintPreviewPaperSizeInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewPrintDuplexInfo()` | `new DPrintPreviewPrintDuplexInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewNUpPrintInfo()` | `new DPrintPreviewNUpPrintInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewPageOrderInfo()` | `new DPrintPreviewPageOrderInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewColorModeInfo()` | `new DPrintPreviewColorModeInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewPaperMarginsInfo()` | `new DPrintPreviewPaperMarginsInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewScalingInfo()` | `new DPrintPreviewScalingInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewWatermarkInfo()` | `new DPrintPreviewWatermarkInfo` | unique `OwnedOpaque` | constructor; `dprintpreviewsettinginfo.h` |
| `DPrintPreviewSettingInterface::name` | `String` | owned Rust strings/container | shim conversion; `dprintpreviewsettinginterface.h` |
| `DPrintPreviewSettingInterface::settingFilter` | `bool` | copied scalar / no result | shim conversion; `dprintpreviewsettinginterface.h` |
| `DPrintPreviewSettingInterface::settingFilter` parameters | `DPrintPreviewSettingInfo*` | foreign implementation borrow | native `dprintpreviewsettinginterface.h` implementation: virtual plugin may mutate info; caller must uphold foreign implementation contract |
| `DPrintPreviewSettingInterface::settingFilter` Qt value inputs | `QVariant * mimeData` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpreviewsettinginterface.h` native value/reference signature |
| `DPrintPreviewSettingInterface::settingStatus` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewsettinginterface.h` |
| `DPrintPreviewSettingInterface::settingStatus` Qt value inputs | `QVariant * mimeData` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpreviewsettinginterface.h` native value/reference signature |
| `DPrinter()` | `new DPrinter` | unique `OwnedOpaque` | constructor; `dprintpreviewwidget.h` |
| `DPrinter::setPreviewMode` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setVisible` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setPageRange` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setPageRangeALL` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setPageRangeMode` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::pageRangeMode` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::pagesCount` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::currentPage` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::turnPageAble` | `bool` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setOrientation` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setScale` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::getScale` | `f64` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::updateView` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::updateWaterMark` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::refreshBegin` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::refreshEnd` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMarkType` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMargImage` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMargImage` Qt value inputs | `QImage * image` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpreviewwidget.h` native value/reference signature |
| `DPrintPreviewWidget::setWaterMarkRotate` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMarkScale` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMarkOpacity` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setConfidentialWaterMark` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setDraftWaterMark` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setSampleWaterMark` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setCustomWaterMark` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setTextWaterMark` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMarkFont` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMarkFont` Qt value inputs | `QFont * font` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpreviewwidget.h` native value/reference signature |
| `DPrintPreviewWidget::waterMarkColor` | `*mut QColor` | owned heap copy | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMarkColor` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setWaterMarkColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dprintpreviewwidget.h` native value/reference signature |
| `DPrintPreviewWidget::setWaterMarkLayout` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setImposition` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::imposition` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setOrder` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::order` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setPrintFromPath` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::printFromPath` | `String` | owned Rust strings/container | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setPrintMode` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setAsynPreview` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::isAsynPreview` | `bool` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::isPageByPage` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::targetPageCount` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::originPageCount` | `i32` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::printerColorModel` | `String` | owned Rust strings/container | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::updatePreview` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::turnFront` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::turnBack` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::turnBegin` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::turnEnd` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::setCurrentPage` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DPrintPreviewWidget::print` | `()` | copied scalar / no result | shim conversion; `dprintpreviewwidget.h` |
| `DProgressBar()` | `new DProgressBar` | constructor `Owned` | constructor; `dprogressbar.h` |
| `DProgressBar::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dprogressbar.h` |
| `DProgressBar::minimumSizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dprogressbar.h` |
| `DSearchComboBox()` | `new DSearchComboBox` | constructor `Owned` | constructor; `dsearchcombobox.h` |
| `DSearchComboBox::setEditable` | `()` | copied scalar / no result | shim conversion; `dsearchcombobox.h` |
| `DSearchEdit()` | `new DSearchEdit` | constructor `Owned` | constructor; `dsearchedit.h` |
| `DSearchEdit::setPlaceHolder` | `()` | copied scalar / no result | shim conversion; `dsearchedit.h` |
| `DSearchEdit::placeHolder` | `String` | owned Rust strings/container | shim conversion; `dsearchedit.h` |
| `DSearchEdit::clear` | `()` | copied scalar / no result | shim conversion; `dsearchedit.h` |
| `DSearchEdit::clearEdit` | `()` | copied scalar / no result | shim conversion; `dsearchedit.h` |
| `DSearchEdit::isVoiceInput` | `bool` | copied scalar / no result | shim conversion; `dsearchedit.h` |
| `DSearchEdit::setPlaceholderText` | `()` | copied scalar / no result | shim conversion; `dsearchedit.h` |
| `DSearchEdit::placeholderText` | `String` | owned Rust strings/container | shim conversion; `dsearchedit.h` |
| `DSettingsDialog()` | `new DSettingsDialog` | constructor `Owned` | constructor; `dsettingsdialog.h` |
| `DSettingsDialog::widgetFactory` | `DSettingsWidgetFactory*` | `object_borrow` | `dtkwidget/src/widgets/dsettingsdialog.cpp`: returns existing widgetFactory child |
| `DSettingsDialog::groupIsVisible` | `bool` | copied scalar / no result | shim conversion; `dsettingsdialog.h` |
| `DSettingsDialog::setResetVisible` | `()` | copied scalar / no result | shim conversion; `dsettingsdialog.h` |
| `DSettingsDialog::scrollToGroup` | `()` | copied scalar / no result | shim conversion; `dsettingsdialog.h` |
| `DSettingsDialog::setIcon` | `()` | copied scalar / no result | shim conversion; `dsettingsdialog.h` |
| `DSettingsDialog::setIcon` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dsettingsdialog.h` native value/reference signature |
| `DSettingsDialog::setGroupVisible` | `()` | copied scalar / no result | shim conversion; `dsettingsdialog.h` |
| `DSettingsWidgetFactory()` | `new DSettingsWidgetFactory` | constructor `Owned` | constructor; `dsettingswidgetfactory.h` |
| `DShadowLine()` | `new DShadowLine` | constructor `Owned` | constructor; `dshadowline.h` |
| `DShadowLine::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dshadowline.h` |
| `DShortcutEditLabel()` | `new DShortcutEditLabel` | constructor `Owned` | constructor; `dshortcutedit.h` |
| `DShortcutEditLabel::setEchoState` | `()` | copied scalar / no result | shim conversion; `dshortcutedit.h` |
| `DSimpleListItem::sameAs` | `bool` | copied scalar / no result | shim conversion; `dsimplelistitem.h` |
| `DSimpleListItem::sameAs` parameters | `DSimpleListItem*` | foreign implementation borrow | native `dsimplelistitem.h` implementation: abstract QObject item with QPointer view; no safe constructor/getter exists |
| `DSimpleListView::setRowHeight` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::setClipRadius` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::removeItem` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::removeItem` parameters | `DSimpleListItem*` | borrow; native item removal | native `dsimplelistview.h` implementation: abstract QObject item borrow; native removeOne does not transfer/delete ownership; no safe source of foreign concrete items exists |
| `DSimpleListView::clearItems` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::clearSelections` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::search` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::setSingleSelect` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::keepSelectWhenClickBlank` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::selectAllItems` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::selectFirstItem` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::selectLastItem` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::selectNextItem` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::selectPrevItem` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::shiftSelectPageDown` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::shiftSelectPageUp` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::shiftSelectToEnd` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::shiftSelectToHome` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::shiftSelectToNext` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::shiftSelectToPrev` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::scrollPageDown` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::scrollPageUp` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::ctrlScrollPageDown` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::ctrlScrollPageUp` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::ctrlScrollToEnd` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSimpleListView::ctrlScrollToHome` | `()` | copied scalar / no result | shim conversion; `dsimplelistview.h` |
| `DSlider()` | `new DSlider` | constructor `Owned` | constructor; `dslider.h` |
| `DSlider::orientation` | `i32` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setLeftIcon` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setLeftIcon` Qt value inputs | `QIcon * left` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dslider.h` native value/reference signature |
| `DSlider::setRightIcon` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setRightIcon` Qt value inputs | `QIcon * right` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dslider.h` native value/reference signature |
| `DSlider::setIconSize` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setIconSize` Qt value inputs | `QSize * size` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dslider.h` native value/reference signature |
| `DSlider::setMinimum` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::minimum` | `i32` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setValue` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::value` | `i32` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setPageStep` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::pageStep` | `i32` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setMaximum` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::maximum` | `i32` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setLeftTicks` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setRightTicks` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setAboveTicks` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setBelowTicks` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setMouseWheelEnabled` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setTipValue` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dslider.h` |
| `DSlider::setHandleVisible` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::handleVisible` | `bool` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSlider::setEnabledAcrossStyle` | `()` | copied scalar / no result | shim conversion; `dslider.h` |
| `DSpinBox()` | `new DSpinBox` | constructor `Owned` | constructor; `dspinbox.h` |
| `DSpinBox::isAlert` | `bool` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DSpinBox::showAlertMessage` | `()` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DSpinBox::showAlertMessage` | `()` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DSpinBox::showAlertMessage` parameters | `QWidget*` | borrow | native `dspinbox.h` implementation: alert follows follower widget |
| `DSpinBox::setEnabledEmbedStyle` | `()` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DSpinBox::setAlert` | `()` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DDoubleSpinBox()` | `new DDoubleSpinBox` | constructor `Owned` | constructor; `dspinbox.h` |
| `DDoubleSpinBox::isAlert` | `bool` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DDoubleSpinBox::showAlertMessage` | `()` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DDoubleSpinBox::showAlertMessage` | `()` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DDoubleSpinBox::showAlertMessage` parameters | `QWidget*` | borrow | native `dspinbox.h` implementation: alert follows follower widget |
| `DDoubleSpinBox::setEnabledEmbedStyle` | `()` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DDoubleSpinBox::setAlert` | `()` | copied scalar / no result | shim conversion; `dspinbox.h` |
| `DSpinner()` | `new DSpinner` | constructor `Owned` | constructor; `dspinner.h` |
| `DSpinner::isPlaying` | `bool` | copied scalar / no result | shim conversion; `dspinner.h` |
| `DSpinner::start` | `()` | copied scalar / no result | shim conversion; `dspinner.h` |
| `DSpinner::stop` | `()` | copied scalar / no result | shim conversion; `dspinner.h` |
| `DSpinner::setBackgroundColor` | `()` | copied scalar / no result | shim conversion; `dspinner.h` |
| `DSpinner::setBackgroundColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dspinner.h` native value/reference signature |
| `DSlideStackWidgetTransition()` | `new DSlideStackWidgetTransition` | constructor `Owned` | constructor; `dstackwidget.h` |
| `DStackWidget()` | `new DStackWidget` | constructor `Owned` | constructor; `dstackwidget.h` |
| `DStackWidget::busy` | `bool` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::depth` | `i32` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::currentIndex` | `i32` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::currentWidget` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dstackwidget.cpp`: returns currentWidget; pop/clear destroy or schedule widgets |
| `DStackWidget::transition` | `DAbstractStackWidgetTransition*` | `object_borrow` | `dtkwidget/src/widgets/dstackwidget.cpp`: returns parented transition; replacement queues old deletion |
| `DStackWidget::animationDuration` | `i32` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::pushWidget` | `i32` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::pushWidget` parameters | `QWidget*` | Qt adoption | native `dstackwidget.h` implementation: raw widgetList + parent; reject duplicate widgets |
| `DStackWidget::insertWidget` | `()` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::insertWidget` parameters | `QWidget*` | Qt adoption | native `dstackwidget.h` implementation: raw widgetList + parent; reject duplicate/cyclic handoffs and invalid index |
| `DStackWidget::popWidget` | `()` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::popWidget` parameters | `QWidget*` | borrow; optional native delete | native `dstackwidget.h` implementation: no transfer to Rust; bound count to remaining stack entries |
| `DStackWidget::clear` | `()` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::indexOf` | `i32` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::indexOf` parameters | `QWidget*` | borrow | native `dstackwidget.h` implementation: lookup only |
| `DStackWidget::getWidgetByIndex` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dstackwidget.cpp`: returns widgetList[index]; bounds must be checked by shim |
| `DStackWidget::setTransition` | `()` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStackWidget::setTransition` parameters | `DAbstractStackWidgetTransition*` | Qt adoption | native `dstackwidget.h` implementation: parents new transition, schedules old deletion; self replacement must be a no-op |
| `DStackWidget::setAnimationDuration` | `()` | copied scalar / no result | shim conversion; `dstackwidget.h` |
| `DStyle()` | `new DStyle` | constructor `Owned` | constructor; `dstyle.h` |
| `DStyle::adjustColor` | `*mut QColor` | owned heap copy | shim conversion; `dstyle.h` |
| `DStyle::adjustColor` Qt value inputs | `QColor * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyle.h` native value/reference signature |
| `DStyle::blendColor` | `*mut QColor` | owned heap copy | shim conversion; `dstyle.h` |
| `DStyle::blendColor` Qt value inputs | `QColor * substrate, QColor * superstratum` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyle.h` native value/reference signature |
| `DStyle::setFocusRectVisible` | `()` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStyle::setFocusRectVisible` parameters | `QWidget*` | borrow | native `dstyle.h` implementation: widget style property |
| `DStyle::setFrameRadius` | `()` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStyle::setFrameRadius` parameters | `QWidget*` | borrow | native `dstyle.h` implementation: widget style property |
| `DStyle::setUncheckedItemIndicatorVisible` | `()` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStyle::setUncheckedItemIndicatorVisible` parameters | `QWidget*` | borrow | native `dstyle.h` implementation: widget style property |
| `DStyle::setShortcutUnderlineVisible` | `()` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStyle::shortcutUnderlineVisible` | `bool` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStyle::setMenuKeyboardSearchDisabled` | `()` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStyle::isMenuKeyboardSearchDisabled` | `bool` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStyle::standardPalette` | `*mut QPalette` | owned heap copy | shim conversion; `dstyle.h` |
| `DStyleHelper::dstyle` | `DStyle*` | `object_borrow` | `dtkwidget/include/widgets/dstyle.h`: inline dynamic_cast of wstyle; helper neither allocates nor owns it |
| `DStylePainter::begin` | `bool` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStylePainter::begin` parameters | `QWidget*` | callback-scoped device borrow | native `dstyle.h` implementation: QPainter paint device lifetime; this opaque class has no safe constructor/getter |
| `DStylePainter::drawItemPixmap` | `()` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DStylePainter::drawItemPixmap` Qt value inputs | `QRect * r, QPixmap * pixmap` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyle.h` native value/reference signature |
| `DStyledIconEngine::setIconName` | `()` | copied scalar / no result | shim conversion; `dstyle.h` |
| `DViewItemAction()` | `new DViewItemAction` | constructor `Owned` | constructor; `dstyleditemdelegate.h` |
| `DViewItemAction::alignment` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::iconSize` | `*mut QSize` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::maximumSize` | `*mut QSize` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::clickAreaMargins` | `*mut QMargins` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::setClickAreaMargins` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::setClickAreaMargins` Qt value inputs | `QMargins * margins` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleditemdelegate.h` native value/reference signature |
| `DViewItemAction::setTextColorRole` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::setTextColorRole` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::textColorType` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::textColorRole` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::setFontSize` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::font` | `*mut QFont` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::isClickable` | `bool` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::setWidget` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::setWidget` parameters | `QWidget*` | retained borrow | native `dstyleditemdelegate.h` implementation: stores QPointer QWidget and hides it; no transfer |
| `DViewItemAction::widget` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dstyleditemdelegate.cpp`: returns QPointer QWidget stored by setWidget; ownership unchanged |
| `DViewItemAction::setDciIcon` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DViewItemAction::setDciIcon` Qt value inputs | `DDciIcon * dciIcon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleditemdelegate.h` native value/reference signature |
| `DViewItemAction::dciIcon` | `*mut DDciIcon` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DStyledItemDelegate::backgroundType` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStyledItemDelegate::margins` | `*mut QMargins` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DStyledItemDelegate::itemSize` | `*mut QSize` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DStyledItemDelegate::spacing` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStyledItemDelegate::setBackgroundType` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStyledItemDelegate::setMargins` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStyledItemDelegate::setMargins` Qt value inputs | `QMargins * margins` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleditemdelegate.h` native value/reference signature |
| `DStyledItemDelegate::setItemSize` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStyledItemDelegate::setItemSize` Qt value inputs | `QSize * itemSize` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleditemdelegate.h` native value/reference signature |
| `DStyledItemDelegate::setItemSpacing` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::setTextColorRole` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::setTextColorRole` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::textColorType` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::textColorRole` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::setBackgroundRole` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::setBackgroundRole` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::backgroundType` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::backgroundRole` | `i32` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::setFontSize` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::font` | `*mut QFont` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::setDciIcon` | `()` | copied scalar / no result | shim conversion; `dstyleditemdelegate.h` |
| `DStandardItem::setDciIcon` Qt value inputs | `DDciIcon * dciIcon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleditemdelegate.h` native value/reference signature |
| `DStandardItem::dciIcon` | `*mut DDciIcon` | owned heap copy | shim conversion; `dstyleditemdelegate.h` |
| `DStyleOption::init` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DStyleOption::init` parameters | `QWidget*` | borrow | native `dstyleoption.h` implementation: copies option state; no caller ownership transfer |
| `DStyleOption::init` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DStyleOption::init` parameters | `QWidget*` | borrow | native `dstyleoption.h` implementation: copies option state; no caller ownership transfer |
| `DStyleOptionButton()` | `new DStyleOptionButton` | unique `OwnedOpaque` | constructor; `dstyleoption.h` |
| `DStyleOptionButton::init` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DStyleOptionButton::init` parameters | `QWidget*` | borrow | native `dstyleoption.h` implementation: copies option state from widget |
| `DStyleOptionLineEdit::init` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DStyleOptionLineEdit::init` parameters | `QWidget*` | borrow | native `dstyleoption.h` implementation: copies option state from widget |
| `DStyleOptionBackgroundGroup::init` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DStyleOptionBackgroundGroup::init` parameters | `QWidget*` | borrow | native `dstyleoption.h` implementation: copies option state from widget |
| `DStyleOptionFloatingWidget()` | `new DStyleOptionFloatingWidget` | unique `OwnedOpaque` | constructor; `dstyleoption.h` |
| `DFontSizeManager::instance` | `DFontSizeManager*` | `opaque_static` | `dtkwidget/src/widgets/dstyleoption.cpp`: returns address of function-static DFontSizeManager manager; caller must not delete |
| `DFontSizeManager::bind` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::bind` parameters | `QWidget*` | retained borrow | native `dstyleoption.h` implementation: native binder map removes widgets on QObject::destroyed |
| `DFontSizeManager::bind` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::bind` parameters | `QWidget*` | retained borrow | native `dstyleoption.h` implementation: native binder map removes widgets on QObject::destroyed |
| `DFontSizeManager::unbind` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::unbind` parameters | `QWidget*` | borrow; releases relation | native `dstyleoption.h` implementation: removes widget from all binder lists |
| `DFontSizeManager::fontPixelSize` | `u16` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::setFontPixelSize` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::setFontGenericPixelSize` | `()` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::get` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::get` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::get` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::get` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t1` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t1` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t2` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t2` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t3` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t3` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t4` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t4` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t5` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t5` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t6` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t6` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t7` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t7` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t8` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t8` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t9` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t9` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t10` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t10` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::t11` | `*mut QFont` | owned heap copy | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::t11` Qt value inputs | `QFont * base` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DFontSizeManager::fontPixelSize` | `i32` | copied scalar / no result | shim conversion; `dstyleoption.h` |
| `DFontSizeManager::fontPixelSize` Qt value inputs | `QFont * font` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dstyleoption.h` native value/reference signature |
| `DSwitchButton()` | `new DSwitchButton` | constructor `Owned` | constructor; `dswitchbutton.h` |
| `DSwitchButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dswitchbutton.h` |
| `DSwitchHeaderLine()` | `new DSwitchHeaderLine` | constructor `Owned` | constructor; `dswitchlineexpand.h` |
| `DSwitchHeaderLine::setExpand` | `()` | copied scalar / no result | shim conversion; `dswitchlineexpand.h` |
| `DSwitchLineExpand()` | `new DSwitchLineExpand` | constructor `Owned` | constructor; `dswitchlineexpand.h` |
| `DSwitchLineExpand::setTitle` | `()` | copied scalar / no result | shim conversion; `dswitchlineexpand.h` |
| `DSwitchLineExpand::setExpand` | `()` | copied scalar / no result | shim conversion; `dswitchlineexpand.h` |
| `DSwitchLineExpand::header` | `DBaseLine*` | `object_borrow` | `dtkwidget/src/widgets/dswitchlineexpand.cpp`: returns existing header widget |
| `DTabBar()` | `new DTabBar` | constructor `Owned` | constructor; `dtabbar.h` |
| `DTabBar::setTabMinimumSize` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabMinimumSize` Qt value inputs | `QSize * size` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::setTabMaximumSize` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabMaximumSize` Qt value inputs | `QSize * size` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::visibleAddButton` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::shape` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setShape` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::addTab` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::addTab` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::addTab` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::insertTab` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::insertTab` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::insertTab` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::removeTab` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::moveTab` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::isTabEnabled` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabEnabled` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::tabText` | `String` | owned Rust strings/container | shim conversion; `dtabbar.h` |
| `DTabBar::setTabText` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::tabIcon` | `*mut QIcon` | owned heap copy | shim conversion; `dtabbar.h` |
| `DTabBar::setTabIcon` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabIcon` Qt value inputs | `QIcon * icon` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::elideMode` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setElideMode` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabToolTip` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::tabToolTip` | `String` | owned Rust strings/container | shim conversion; `dtabbar.h` |
| `DTabBar::setTabWhatsThis` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::tabWhatsThis` | `String` | owned Rust strings/container | shim conversion; `dtabbar.h` |
| `DTabBar::setTabData` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabData` Qt value inputs | `QVariant * data` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::tabData` | `*mut QVariant` | owned heap copy | shim conversion; `dtabbar.h` |
| `DTabBar::tabRect` | `*mut QRect` | owned heap copy | shim conversion; `dtabbar.h` |
| `DTabBar::tabAt` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::tabAt` Qt value inputs | `QPoint * pos` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::currentIndex` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::count` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setDrawBase` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::drawBase` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::iconSize` | `*mut QSize` | owned heap copy | shim conversion; `dtabbar.h` |
| `DTabBar::setIconSize` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setIconSize` Qt value inputs | `QSize * size` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::usesScrollButtons` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setUsesScrollButtons` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::tabsClosable` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabsClosable` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabButton` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabButton` parameters | `QWidget*` | Qt adoption | native `dtabbar.h` implementation: QTabBar owns/replaces tab button widgets |
| `DTabBar::tabButton` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dtabbar.cpp`: forwards QTabBar::tabButton; Qt owns inserted buttons |
| `DTabBar::selectionBehaviorOnRemove` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setSelectionBehaviorOnRemove` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::expanding` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setExpanding` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::isMovable` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setMovable` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::isDragable` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setDragable` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::documentMode` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setDocumentMode` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::autoHide` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setAutoHide` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::changeCurrentOnDrag` | `bool` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setChangeCurrentOnDrag` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::startDragDistance` | `i32` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::maskColor` | `*mut QColor` | owned heap copy | shim conversion; `dtabbar.h` |
| `DTabBar::flashColor` | `*mut QColor` | owned heap copy | shim conversion; `dtabbar.h` |
| `DTabBar::setEnabledEmbedStyle` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setTabLabelAlignment` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setCurrentIndex` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setVisibleAddButton` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setStartDragDistance` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setMaskColor` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setMaskColor` Qt value inputs | `QColor * maskColor` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::setFlashColor` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::setFlashColor` Qt value inputs | `QColor * flashColor` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtabbar.h` native value/reference signature |
| `DTabBar::startDrag` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabBar::stopDrag` | `()` | copied scalar / no result | shim conversion; `dtabbar.h` |
| `DTabletWindowOptionButton()` | `new DTabletWindowOptionButton` | constructor `Owned` | constructor; `dtabletwindowoptionbutton.h` |
| `DTabletWindowOptionButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dtabletwindowoptionbutton.h` |
| `DTextEdit()` | `new DTextEdit` | constructor `Owned` | constructor; `dtextedit.h` |
| `DTextEdit::speechToTextIsEnabled` | `bool` | copied scalar / no result | shim conversion; `dtextedit.h` |
| `DTextEdit::setSpeechToTextEnabled` | `()` | copied scalar / no result | shim conversion; `dtextedit.h` |
| `DTextEdit::textToSpeechIsEnabled` | `bool` | copied scalar / no result | shim conversion; `dtextedit.h` |
| `DTextEdit::setTextToSpeechEnabled` | `()` | copied scalar / no result | shim conversion; `dtextedit.h` |
| `DTextEdit::textToTranslateIsEnabled` | `bool` | copied scalar / no result | shim conversion; `dtextedit.h` |
| `DTextEdit::setTextToTranslateEnabled` | `()` | copied scalar / no result | shim conversion; `dtextedit.h` |
| `DTickEffect::play` | `()` | copied scalar / no result | shim conversion; `dtickeffect.h` |
| `DTickEffect::stop` | `()` | copied scalar / no result | shim conversion; `dtickeffect.h` |
| `DTickEffect::pause` | `()` | copied scalar / no result | shim conversion; `dtickeffect.h` |
| `DTickEffect::resume` | `()` | copied scalar / no result | shim conversion; `dtickeffect.h` |
| `DTickEffect::setDirection` | `()` | copied scalar / no result | shim conversion; `dtickeffect.h` |
| `DTickEffect::setFixedPixelMove` | `()` | copied scalar / no result | shim conversion; `dtickeffect.h` |
| `DTipLabel()` | `new DTipLabel` | constructor `Owned` | constructor; `dtiplabel.h` |
| `DTipLabel::show` | `()` | copied scalar / no result | shim conversion; `dtiplabel.h` |
| `DTipLabel::show` Qt value inputs | `QPoint * pos` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtiplabel.h` native value/reference signature |
| `DTipLabel::setForegroundRole` | `()` | copied scalar / no result | shim conversion; `dtiplabel.h` |
| `DTitlebarToolBaseInterface::id` | `String` | owned Rust strings/container | shim conversion; `dtitlebarsettings.h` |
| `DTitlebarToolBaseInterface::description` | `String` | owned Rust strings/container | shim conversion; `dtitlebarsettings.h` |
| `DTitlebarToolBaseInterface::iconName` | `String` | owned Rust strings/container | shim conversion; `dtitlebarsettings.h` |
| `DTitleBarToolInterface::createView` | `QWidget*` | `implementation_defined` | `dtkwidget/include/widgets/dtitlebarsettings.h`: pure virtual; header does not specify borrowed vs transferred; cannot manufacture a unique owner |
| `DTitleBarSpacerInterface::createPlaceholderView` | `QWidget*` | `implementation_defined` | `dtkwidget/include/widgets/dtitlebarsettings.h`: pure virtual; native implementations differ; caller must audit the foreign implementation |
| `DTitleBarSpacerInterface::size` | `i32` | copied scalar / no result | shim conversion; `dtitlebarsettings.h` |
| `DTitlebarSettings::toolsEditPanel` | `QWidget*` | `object_borrow` | `dtkwidget/src/widgets/dtitlebarsettings.cpp`: returns impl()->toolsEditPanel; native titlebar settings controls its lifetime |
| `DToolButton()` | `new DToolButton` | constructor `Owned` | constructor; `dtoolbutton.h` |
| `DToolButton::setAlignment` | `()` | copied scalar / no result | shim conversion; `dtoolbutton.h` |
| `DToolButton::alignment` | `i32` | copied scalar / no result | shim conversion; `dtoolbutton.h` |
| `DToolTip::setToolTipTextFormat` | `()` | copied scalar / no result | shim conversion; `dtooltip.h` |
| `DToolTip::toolTipTextFormat` | `i32` | copied scalar / no result | shim conversion; `dtooltip.h` |
| `DToolTip::setToolTipShowMode` | `()` | copied scalar / no result | shim conversion; `dtooltip.h` |
| `DToolTip::setToolTipShowMode` parameters | `QWidget*` | borrow | native `dtooltip.h` implementation: widget property |
| `DToolTip::toolTipShowMode` | `i32` | copied scalar / no result | shim conversion; `dtooltip.h` |
| `DToolTip::toolTipShowMode` parameters | `QWidget*` | borrow | native `dtooltip.h` implementation: widget property lookup |
| `DToolTip::needUpdateToolTip` | `bool` | copied scalar / no result | shim conversion; `dtooltip.h` |
| `DToolTip::needUpdateToolTip` parameters | `QWidget*` | borrow | native `dtooltip.h` implementation: widget property lookup |
| `DToolTip::setShowToolTip` | `()` | copied scalar / no result | shim conversion; `dtooltip.h` |
| `DToolTip::setShowToolTip` parameters | `QWidget*` | borrow | native `dtooltip.h` implementation: widget property |
| `DToolTip::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dtooltip.h` |
| `DToolTip::show` | `()` | copied scalar / no result | shim conversion; `dtooltip.h` |
| `DToolTip::show` Qt value inputs | `QPoint * pos` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dtooltip.h` native value/reference signature |
| `DWarningButton()` | `new DWarningButton` | constructor `Owned` | constructor; `dwarningbutton.h` |
| `DWaterMarkHelper::instance` | `DWaterMarkHelper*` | `object_borrow` | `dtkwidget/src/widgets/dwatermarkhelper.cpp`: returns Q_GLOBAL_STATIC helper; native post routine owns cleanup |
| `DWaterMarkHelper::registerWidget` | `()` | copied scalar / no result | shim conversion; `dwatermarkhelper.h` |
| `DWaterMarkHelper::registerWidget` parameters | `QWidget*` | retained borrow | native `dwatermarkhelper.h` implementation: native QObject destruction cleanup; watermark child overlay |
| `DWaterMarkHelper::data` | `*mut WaterMarkData` | owned heap copy | shim conversion; `dwatermarkhelper.h` |
| `DWaterMarkHelper::setData` | `()` | copied scalar / no result | shim conversion; `dwatermarkhelper.h` |
| `DWaterMarkHelper::setData` parameters | `WaterMarkData*` | value copy | native `dwatermarkhelper.h` implementation: copies WaterMarkData; does not retain pointer to Rust-owned copy |
| `WaterMarkData()` | `new WaterMarkData` | unique `OwnedOpaque` | constructor; `dwatermarkwidget.h` |
| `WaterMarkData::type` | `i32` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setType` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::layout` | `i32` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setLayout` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::scaleFactor` | `f64` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setScaleFactor` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::spacing` | `i32` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setSpacing` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::lineSpacing` | `i32` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setLineSpacing` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::text` | `String` | owned Rust strings/container | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setText` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::font` | `*mut QFont` | owned heap copy | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setFont` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setFont` Qt value inputs | `QFont * font` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dwatermarkwidget.h` native value/reference signature |
| `WaterMarkData::color` | `*mut QColor` | owned heap copy | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setColor` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setColor` Qt value inputs | `QColor * color` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dwatermarkwidget.h` native value/reference signature |
| `WaterMarkData::rotation` | `f64` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setRotation` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::opacity` | `f64` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setOpacity` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::image` | `*mut QImage` | owned heap copy | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setImage` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setImage` Qt value inputs | `QImage * image` | call borrow of independently owned Qt value; no QObject ownership transfer | explicit value mapping; `dwatermarkwidget.h` native value/reference signature |
| `WaterMarkData::grayScale` | `bool` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `WaterMarkData::setGrayScale` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `DWaterMarkWidget()` | `new DWaterMarkWidget` | constructor `Owned` | constructor; `dwatermarkwidget.h` |
| `DWaterMarkWidget::data` | `*mut WaterMarkData` | owned heap copy | shim conversion; `dwatermarkwidget.h` |
| `DWaterMarkWidget::setData` | `()` | copied scalar / no result | shim conversion; `dwatermarkwidget.h` |
| `DWaterMarkWidget::setData` parameters | `WaterMarkData*` | value copy | native `dwatermarkwidget.h` implementation: copies WaterMarkData into native value state |
| `DWaterProgress()` | `new DWaterProgress` | constructor `Owned` | constructor; `dwaterprogress.h` |
| `DWaterProgress::value` | `i32` | copied scalar / no result | shim conversion; `dwaterprogress.h` |
| `DWaterProgress::start` | `()` | copied scalar / no result | shim conversion; `dwaterprogress.h` |
| `DWaterProgress::stop` | `()` | copied scalar / no result | shim conversion; `dwaterprogress.h` |
| `DWaterProgress::setValue` | `()` | copied scalar / no result | shim conversion; `dwaterprogress.h` |
| `DWaterProgress::setTextVisible` | `()` | copied scalar / no result | shim conversion; `dwaterprogress.h` |
| `DWindowCloseButton()` | `new DWindowCloseButton` | constructor `Owned` | constructor; `dwindowclosebutton.h` |
| `DWindowCloseButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dwindowclosebutton.h` |
| `DWindowMaxButton()` | `new DWindowMaxButton` | constructor `Owned` | constructor; `dwindowmaxbutton.h` |
| `DWindowMaxButton::isMaximized` | `bool` | copied scalar / no result | shim conversion; `dwindowmaxbutton.h` |
| `DWindowMaxButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dwindowmaxbutton.h` |
| `DWindowMaxButton::setMaximized` | `()` | copied scalar / no result | shim conversion; `dwindowmaxbutton.h` |
| `DWindowMinButton()` | `new DWindowMinButton` | constructor `Owned` | constructor; `dwindowminbutton.h` |
| `DWindowMinButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dwindowminbutton.h` |
| `DWindowOptionButton()` | `new DWindowOptionButton` | constructor `Owned` | constructor; `dwindowoptionbutton.h` |
| `DWindowOptionButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dwindowoptionbutton.h` |
| `DWindowQuitFullButton()` | `new DWindowQuitFullButton` | constructor `Owned` | constructor; `dwindowquitfullbutton.h` |
| `DWindowQuitFullButton::sizeHint` | `*mut QSize` | owned heap copy | shim conversion; `dwindowquitfullbutton.h` |
