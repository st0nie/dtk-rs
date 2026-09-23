# DTK6 widget binding coverage report

classes: 146, methods generated: 960, skipped: 228

Qt ancestry: 85 widgets, 26 QObjects, 35 opaque types

## skip reasons
- unsupported param type: 153
- unsupported return type: 71
- signature parse failed: 4

## DAboutDialog — 19 methods generated, 0 skipped

## DAbstractDialog — 10 methods generated, 0 skipped

## DAccessibilityChecker — 4 methods generated, 0 skipped

## DAlertControl — 10 methods generated, 0 skipped

## DAnchorsBase — 46 methods generated, 13 skipped
- `const DAnchorInfo *top() const;` ← unsupported return type: const DAnchorInfo *
- `const DAnchorInfo *bottom() const;` ← unsupported return type: const DAnchorInfo *
- `const DAnchorInfo *left() const;` ← unsupported return type: const DAnchorInfo *
- `const DAnchorInfo *right() const;` ← unsupported return type: const DAnchorInfo *
- `const DAnchorInfo *horizontalCenter() const;` ← unsupported return type: const DAnchorInfo *
- `const DAnchorInfo *verticalCenter() const;` ← unsupported return type: const DAnchorInfo *
- `bool isBinding(const DAnchorInfo *info) const;` ← unsupported param type: const DAnchorInfo *
- `bool setTop(const DAnchorInfo *top);` ← unsupported param type: const DAnchorInfo *
- `bool setBottom(const DAnchorInfo *bottom);` ← unsupported param type: const DAnchorInfo *
- `bool setLeft(const DAnchorInfo *left);` ← unsupported param type: const DAnchorInfo *
- `bool setRight(const DAnchorInfo *right);` ← unsupported param type: const DAnchorInfo *
- `bool setHorizontalCenter(const DAnchorInfo *horizontalCenter);` ← unsupported param type: const DAnchorInfo *
- `bool setVerticalCenter(const DAnchorInfo *verticalCenter);` ← unsupported param type: const DAnchorInfo *

## ArrowButtonIcon — 4 methods generated, 0 skipped

## DArrowButton — 3 methods generated, 0 skipped

## DArrowLineDrawer — 2 methods generated, 0 skipped

## DArrowRectangle — 39 methods generated, 0 skipped

## DBackgroundGroup — 6 methods generated, 2 skipped
- `void setBackgroundRole(QPalette::ColorRole role);` ← unsupported param type: QPalette::ColorRole
- `QPalette::ColorRole backgroundRole() const;` ← unsupported return type: QPalette::ColorRole

## DBaseLine — 8 methods generated, 0 skipped

## DBlurEffectWidget — 19 methods generated, 3 skipped
- `void setMaskPath(const QPainterPath &path);` ← unsupported param type: const QPainterPath &
- `void setSourceImage(const QImage &image, bool autoScale = true);` ← unsupported param type: const QImage &
- `void updateBlurSourceImage(const QRegion &ren);` ← unsupported param type: const QRegion &

## DBlurEffectGroup — 2 methods generated, 2 skipped
- `void setSourceImage(QImage image, int blurRadius = 35);` ← unsupported param type: QImage
- `void paint(QPainter *pa, DBlurEffectWidget *widget) const;` ← unsupported param type: QPainter *

## DBlurEffectWithBorderWidget — 0 methods generated, 0 skipped

## DBounceAnimation — 1 methods generated, 1 skipped
- `void setAnimationTarget(QAbstractScrollArea *w);` ← unsupported param type: QAbstractScrollArea *

## DBoxWidget — 3 methods generated, 2 skipped
- `QBoxLayout::Direction direction() const;` ← unsupported return type: QBoxLayout::Direction
- `void setDirection(QBoxLayout::Direction direction);` ← unsupported param type: QBoxLayout::Direction

## DHBoxWidget — 0 methods generated, 0 skipped

## DVBoxWidget — 0 methods generated, 0 skipped

## DButtonBoxButton — 7 methods generated, 1 skipped
- `void setIcon(QStyle::StandardPixmap iconType);` ← unsupported param type: QStyle::StandardPixmap

## DButtonBox — 7 methods generated, 2 skipped
- `void setButtonList(const QList<DButtonBoxButton*> &list, bool checkable);` ← unsupported param type: const QList<DButtonBoxButton*> &
- `QList<QAbstractButton*> buttonList() const;` ← unsupported return type: QList<QAbstractButton*>

## DCircleProgress — 10 methods generated, 2 skipped
- `QLabel *topLabel();` ← unsupported return type: QLabel *
- `QLabel *bottomLabel();` ← unsupported return type: QLabel *

## DClipEffectWidget — 2 methods generated, 2 skipped
- `QPainterPath clipPath() const;` ← unsupported return type: QPainterPath
- `void setClipPath(const QPainterPath &path);` ← unsupported param type: const QPainterPath &

## DColoredProgressBar — 1 methods generated, 2 skipped
- `void addThreshold(int threshold, QBrush brush);` ← unsupported param type: QBrush
- `QList<int> thresholds() const;` ← unsupported return type: QList<int>

## DComboBox — 1 methods generated, 1 skipped
- `virtual bool eventFilter(QObject *watched, QEvent *event) override;` ← unsupported param type: QObject *

## DCommandLinkButton — 1 methods generated, 0 skipped

## DCrumbTextFormat — 8 methods generated, 2 skipped
- `QBrush background() const;` ← unsupported return type: QBrush
- `void setBackground(const QBrush &background);` ← unsupported param type: const QBrush &

## DCrumbEdit — 12 methods generated, 5 skipped
- `bool insertCrumb(const DCrumbTextFormat &format, int pos = -1);` ← unsupported param type: const DCrumbTextFormat &
- `bool appendCrumb(const DCrumbTextFormat &format);` ← unsupported param type: const DCrumbTextFormat &
- `DCrumbTextFormat crumbTextFormat(const QString &text) const;` ← unsupported return type: DCrumbTextFormat
- `DCrumbTextFormat makeTextFormat() const;` ← unsupported return type: DCrumbTextFormat
- `DCrumbTextFormat makeTextFormat(CrumbType type) const;` ← unsupported return type: DCrumbTextFormat

## DDialog — 44 methods generated, 2 skipped
- `QList<QAbstractButton*> getButtons() const;` ← unsupported return type: QList<QAbstractButton*>
- `QList<QWidget*> getContents() const;` ← unsupported return type: QList<QWidget*>

## DDialogCloseButton — 0 methods generated, 0 skipped

## DDrawer — 9 methods generated, 1 skipped
- `void setAnimationEasingCurve(QEasingCurve curve);` ← unsupported param type: QEasingCurve

## DDrawerGroup — 7 methods generated, 1 skipped
- `QList<DDrawer *> expands() const;` ← unsupported return type: QList<DDrawer *>

## DEnhancedWidget — 4 methods generated, 0 skipped

## DFeatureItem — 6 methods generated, 0 skipped

## DFeatureDisplayDialog — 8 methods generated, 1 skipped
- `void addItems(QList<DFeatureItem*> items);` ← unsupported param type: QList<DFeatureItem*>

## DFileChooserEdit — 5 methods generated, 6 skipped
- `void setFileMode(QFileDialog::FileMode mode);` ← unsupported param type: QFileDialog::FileMode
- `QFileDialog::FileMode fileMode() const;` ← unsupported return type: QFileDialog::FileMode
- `void setDirectoryUrl(const QUrl &directory);` ← unsupported param type: const QUrl &
- `QUrl directoryUrl();` ← unsupported return type: QUrl
- `void setFileDialog(QFileDialog *fileDialog);` ← unsupported param type: QFileDialog *
- `QFileDialog *fileDialog() const;` ← unsupported return type: QFileDialog *

## DFileDialog — 6 methods generated, 2 skipped
- `void addComboBox(const QString &text, const DComboBoxOptions &options);` ← unsupported param type: const DComboBoxOptions &
- `void addLineEdit(const QString &text, const DLineEditOptions &options);` ← unsupported param type: const DLineEditOptions &

## DFileIconProvider — 1 methods generated, 2 skipped
- `QIcon icon(const QFileInfo &info) const;` ← unsupported param type: const QFileInfo &
- `QIcon icon(const QFileInfo &info, const QIcon &feedback) const;` ← unsupported param type: const QFileInfo &

## DFloatingButton — 0 methods generated, 0 skipped

## DFloatingMessage — 7 methods generated, 0 skipped

## DFloatingWidget — 7 methods generated, 0 skipped

## DFlowLayout — 18 methods generated, 8 skipped
- `void insertItem(int index, QLayoutItem *item);` ← unsupported param type: QLayoutItem *
- `void insertSpacerItem(int index, QSpacerItem *spacerItem);` ← unsupported param type: QSpacerItem *
- `void addSpacerItem(QSpacerItem *spacerItem);` ← unsupported param type: QSpacerItem *
- `void addItem(QLayoutItem *item);` ← unsupported param type: QLayoutItem *
- `QLayoutItem *itemAt(int index) const;` ← unsupported return type: QLayoutItem *
- `QLayoutItem *takeAt(int index);` ← unsupported return type: QLayoutItem *
- `Flow flow() const;` ← unsupported return type: Flow
- `void setFlow(Flow flow);` ← unsupported param type: Flow

## DFontComboBox — 3 methods generated, 4 skipped
- `void setWritingSystem(QFontDatabase::WritingSystem);` ← unsupported param type: QFontDatabase::
- `QFontDatabase::WritingSystem writingSystem() const;` ← unsupported return type: QFontDatabase::WritingSystem
- `void setFontFilters(QFontComboBox::FontFilters filters);` ← unsupported param type: QFontComboBox::FontFilters
- `QFontComboBox::FontFilters fontFilters() const;` ← unsupported return type: QFontComboBox::FontFilters

## DFrame — 2 methods generated, 0 skipped

## DHorizontalLine — 0 methods generated, 0 skipped

## DVerticalLine — 0 methods generated, 0 skipped

## DGraphicsClipEffect — 2 methods generated, 2 skipped
- `QPainterPath clipPath() const;` ← unsupported return type: QPainterPath
- `void setClipPath(const QPainterPath &clipPath);` ← unsupported param type: const QPainterPath &

## DGraphicsGlowEffect — 13 methods generated, 2 skipped
- `void draw(QPainter *painter);` ← unsupported param type: QPainter *
- `QRectF boundingRectFor(const QRectF &rect) const;` ← unsupported return type: QRectF

## DHeaderLine — 3 methods generated, 0 skipped

## DIconButton — 12 methods generated, 1 skipped
- `void setIcon(QStyle::StandardPixmap iconType);` ← unsupported param type: QStyle::StandardPixmap

## DImageViewer — 21 methods generated, 2 skipped
- `QImage image() const;` ← unsupported return type: QImage
- `void setImage(const QImage &image);` ← unsupported param type: const QImage &

## DIndeterminateProgressbar — 0 methods generated, 0 skipped

## DInputDialog — 38 methods generated, 4 skipped
- `static QString getText(QWidget *parent, const QString &title, const QString &mes` ← unsupported param type: bool *
- `static QString getItem(QWidget *parent, const QString &title, const QString &mes` ← unsupported param type: bool *
- `static int getInt(QWidget *parent, const QString &title, const QString &message,` ← unsupported param type: bool *
- `static double getDouble(QWidget *parent, const QString &title, const QString &me` ← unsupported param type: bool *

## DIpv4LineEdit — 9 methods generated, 0 skipped

## DKeySequenceEdit — 2 methods generated, 3 skipped
- `bool setKeySequence(const QKeySequence &keySequence);` ← unsupported param type: const QKeySequence &
- `QKeySequence keySequence();` ← unsupported return type: QKeySequence
- `QString getKeySequence(QKeySequence sequence);` ← unsupported param type: QKeySequence

## DLicenseDialog — 5 methods generated, 0 skipped

## DLineEdit — 30 methods generated, 3 skipped
- `QLineEdit *lineEdit() const;` ← unsupported return type: QLineEdit *
- `void setLeftWidgets(const QList<QWidget *> &list);` ← unsupported param type: const QList<QWidget *> &
- `void setRightWidgets(const QList<QWidget *> &list);` ← unsupported param type: const QList<QWidget *> &

## DVariantListModel — 0 methods generated, 5 skipped
- `int rowCount(const QModelIndex &parent = QModelIndex()) const;` ← unsupported param type: const QModelIndex &
- `QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const;` ← unsupported param type: const QModelIndex &
- `bool setData(const QModelIndex &index, const QVariant &value, int role);` ← unsupported param type: const QModelIndex &
- `bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex());` ← unsupported param type: const QModelIndex &
- `bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex());` ← unsupported param type: const QModelIndex &

## DListView — 28 methods generated, 3 skipped
- `State state() const;` ← unsupported return type: State
- `void setOrientation(QListView::Flow flow, bool wrapping);` ← unsupported param type: QListView::Flow
- `void edit(const QModelIndex &index);` ← unsupported param type: const QModelIndex &

## DLoadingIndicator — 19 methods generated, 3 skipped
- `QEasingCurve::Type aniEasingType() const;` ← unsupported return type: QEasingCurve::Type
- `void setAniEasingCurve(const QEasingCurve & easing);` ← unsupported param type: const QEasingCurve &
- `void setAniEasingType(QEasingCurve::Type aniEasingType);` ← unsupported param type: QEasingCurve::Type

## DMessageManager — 5 methods generated, 0 skipped

## DMPRISControl — 3 methods generated, 0 skipped

## DPageIndicator — 16 methods generated, 0 skipped

## DPaletteHelper — 2 methods generated, 2 skipped
- `DPalette palette(const QWidget *widget, const QPalette &base = QPalette()) const` ← unsupported return type: DPalette
- `void setPalette(QWidget *widget, const DPalette &palette);` ← unsupported param type: const DPalette &

## DPasswordEdit — 4 methods generated, 0 skipped

## DPictureSequenceView — 8 methods generated, 2 skipped
- `void setPictureSequence(const QString &srcFormat, const QPair<int, int> &range, ` ← unsupported param type: const QPair<int, int> &
- `void setPictureSequence(const QList<QPixmap> &sequence, const bool autoScale = f` ← unsupported param type: const QList<QPixmap> &

## DPlatformWindowHandle — 4 methods generated, 3 skipped
- `static bool setWindowBlurAreaByWM(QWidget *widget, const QVector<WMBlurArea> &ar` ← unsupported param type: const QVector<WMBlurArea> &
- `static bool setWindowBlurAreaByWM(QWidget *widget, const QList<QPainterPath> &pa` ← unsupported param type: const QList<QPainterPath> &
- `static bool setWindowWallpaperParaByWM(QWidget *widget, const QRect &area, Wallp` ← unsupported param type: WallpaperScaleMode

## ColorButton — 0 methods generated, 0 skipped

## ColorLabel — 3 methods generated, 1 skipped
- `QCursor pickColorCursor();` ← unsupported return type: QCursor

## ColorSlider — 1 methods generated, 0 skipped

## DPrintPickColorWidget — 6 methods generated, 0 skipped

## DPrintPreviewDialog — 13 methods generated, 2 skipped
- `virtual bool event(QEvent *event) override;` ← unsupported param type: QEvent *
- `bool eventFilter(QObject *watched, QEvent *event) override;` ← unsupported param type: QObject *

## DPrintPreviewSettingInfo — 1 methods generated, 0 skipped

## DPrintPreviewPrinterInfo — 0 methods generated, 0 skipped

## DPrintPreviewCopiesInfo — 0 methods generated, 0 skipped

## DPrintPreviewPageRangeInfo — 0 methods generated, 0 skipped

## DPrintPreviewOrientationInfo — 0 methods generated, 0 skipped

## DPrintPreviewPaperSizeInfo — 0 methods generated, 0 skipped

## DPrintPreviewPrintDuplexInfo — 0 methods generated, 0 skipped

## DPrintPreviewNUpPrintInfo — 0 methods generated, 0 skipped

## DPrintPreviewPageOrderInfo — 0 methods generated, 0 skipped

## DPrintPreviewColorModeInfo — 0 methods generated, 0 skipped

## DPrintPreviewPaperMarginsInfo — 0 methods generated, 0 skipped

## DPrintPreviewScalingInfo — 0 methods generated, 0 skipped

## DPrintPreviewWatermarkInfo — 0 methods generated, 0 skipped

## DPrintPreviewSettingInterface — 3 methods generated, 0 skipped

## DPrinter — 1 methods generated, 1 skipped
- `QList<const QPicture *> getPrinterPages();` ← unsupported return type: QList<const QPicture *>

## DPrintPreviewWidget — 48 methods generated, 5 skipped
- `void setPageRange(const QVector<int> &rangePages);` ← unsupported param type: const QVector<int> &
- `void setColorMode(const DPrinter::ColorMode &colorMode);` ← unsupported param type: const DPrinter::ColorMode &
- `DPrinter::ColorMode getColorMode();` ← unsupported return type: DPrinter::ColorMode
- `void setWaterMargImage(const QImage &image);` ← unsupported param type: const QImage &
- `void themeTypeChanged(DGuiApplicationHelper::ColorType themeType);` ← unsupported param type: DGuiApplicationHelper::ColorType

## DProgressBar — 2 methods generated, 0 skipped

## DSearchComboBox — 1 methods generated, 0 skipped

## DSearchEdit — 7 methods generated, 0 skipped

## DSettingsDialog — 6 methods generated, 2 skipped
- `void updateSettings(DTK_CORE_NAMESPACE::DSettings *settings);` ← unsupported param type: DTK_CORE_NAMESPACE::DSettings *
- `void updateSettings(const QByteArray &translateContext, DTK_CORE_NAMESPACE::DSet` ← unsupported param type: DTK_CORE_NAMESPACE::DSettings *

## DSettingsWidgetFactory — 0 methods generated, 7 skipped
- `QPair<QWidget*, QWidget*> createItem(QPointer<DTK_CORE_NAMESPACE::DSettingsOptio` ← signature parse failed
- `QPair<QWidget*, QWidget*> createItem(const QByteArray &translateContext, QPointe` ← signature parse failed
- `static QPair<QWidget*, QWidget*> createStandardItem(const QByteArray &translateC` ← signature parse failed
- `void registerWidget(const QString &viewType, std::function<WidgetCreateHandler> ` ← unsupported param type: std::function<WidgetCreateHandler>
- `void registerWidget(const QString &viewType, std::function<ItemCreateHandler> ha` ← unsupported param type: std::function<ItemCreateHandler>
- `QWidget *createWidget(QPointer<DTK_CORE_NAMESPACE::DSettingsOption> option);` ← unsupported param type: QPointer<DTK_CORE_NAMESPACE::DSettingsOption>
- `QWidget *createWidget(const QByteArray &translateContext, QPointer<DTK_CORE_NAME` ← unsupported param type: QPointer<DTK_CORE_NAMESPACE::DSettingsOption>

## DShadowLine — 1 methods generated, 0 skipped

## DShortcutEditLabel — 1 methods generated, 0 skipped

## DSimpleListItem — 1 methods generated, 2 skipped
- `virtual void drawBackground(QRect rect, QPainter *painter, int index, bool isSel` ← unsupported param type: QPainter *
- `virtual void drawForeground(QRect rect, QPainter *painter, int column, int index` ← unsupported param type: QPainter *

## DSimpleListView — 25 methods generated, 8 skipped
- `void setColumnTitleInfo(QList<QString> titles, QList<int> widths, int height);` ← unsupported param type: QList<QString>
- `void setColumnHideFlags(QList<bool> toggleHideFlags, int alwaysVisibleColumn=-1)` ← unsupported param type: QList<bool>
- `void setColumnSortingAlgorithms(QList<SortAlgorithm> *algorithms, int sortColumn` ← unsupported param type: QList<SortAlgorithm> *
- `void setSearchAlgorithm(SearchAlgorithm algorithm);` ← unsupported param type: SearchAlgorithm
- `void addItems(QList<DSimpleListItem*> items);` ← unsupported param type: QList<DSimpleListItem*>
- `void addSelections(QList<DSimpleListItem*> items, bool recordLastSelection=true)` ← unsupported param type: QList<DSimpleListItem*>
- `QList<DSimpleListItem*> getSelections();` ← unsupported return type: QList<DSimpleListItem*>
- `void refreshItems(QList<DSimpleListItem*> items);` ← unsupported param type: QList<DSimpleListItem*>

## DSizeModeHelper — 0 methods generated, 1 skipped
- `static inline T element(const T &t1, const T &t2)` ← unsupported return type: inline T

## DSlider — 22 methods generated, 3 skipped
- `QSlider *slider();` ← unsupported return type: QSlider *
- `void setMarkPositions(QList<int> list);` ← unsupported param type: QList<int>
- `QSlider::TickPosition tickPosition() const;` ← unsupported return type: QSlider::TickPosition

## SpecialSlider — 0 methods generated, 1 skipped
- `void paintEvent(QPaintEvent *ev);` ← unsupported param type: QPaintEvent *

## DSpinBox — 5 methods generated, 1 skipped
- `QLineEdit *lineEdit() const;` ← unsupported return type: QLineEdit *

## DDoubleSpinBox — 5 methods generated, 1 skipped
- `QLineEdit *lineEdit() const;` ← unsupported return type: QLineEdit *

## DSpinner — 4 methods generated, 0 skipped

## DAbstractStackWidgetTransition — 0 methods generated, 2 skipped
- `virtual void beginTransition(const TransitionInfo &info);` ← unsupported param type: const TransitionInfo &
- `virtual QVariantAnimation *animation() const;` ← unsupported return type: QVariantAnimation *

## DSlideStackWidgetTransition — 0 methods generated, 1 skipped
- `void beginTransition(const TransitionInfo &info);` ← unsupported param type: const TransitionInfo &

## DStackWidget — 14 methods generated, 2 skipped
- `QEasingCurve::Type animationType() const;` ← unsupported return type: QEasingCurve::Type
- `void setAnimationType(QEasingCurve::Type animationType);` ← unsupported param type: QEasingCurve::Type

## DStyle — 10 methods generated, 37 skipped
- `static QPair<QIcon::Mode, QIcon::State> toIconModeState(const QStyleOption *opti` ← signature parse failed
- `static DDciIcon::Mode toDciIconMode(const QStyleOption *option);` ← unsupported return type: DDciIcon::Mode
- `static DStyle::StyleState getState(const QStyleOption *option);` ← unsupported param type: const QStyleOption *
- `static void setRedPointVisible(QObject *object, bool visible);` ← unsupported param type: QObject *
- `static void setLineEditIconMargin(QObject *object, int margin);` ← unsupported param type: QObject *
- `static void drawPrimitive(const QStyle *style, DStyle::PrimitiveElement pe, cons` ← unsupported param type: const QStyle *
- `static void drawControl(const QStyle *style, DStyle::ControlElement ce, const QS` ← unsupported param type: const QStyle *
- `static int pixelMetric(const QStyle *style, DStyle::PixelMetric m, const QStyleO` ← unsupported param type: const QStyle *
- `static QRect subElementRect(const QStyle *style, DStyle::SubElement r, const QSt` ← unsupported param type: const QStyle *
- `static QSize sizeFromContents(const QStyle *style, DStyle::ContentsType ct, cons` ← unsupported param type: const QStyle *
- `static QIcon standardIcon(const QStyle *style, StandardPixmap st, const QStyleOp` ← unsupported param type: const QStyle *
- `inline void drawPrimitive(DStyle::PrimitiveElement pe, const QStyleOption *opt, ` ← unsupported param type: const QStyleOption *
- `inline void drawControl(DStyle::ControlElement ce, const QStyleOption *opt, QPai` ← unsupported param type: const QStyleOption *
- `inline int pixelMetric(DStyle::PixelMetric m, const QStyleOption *opt = nullptr,` ← unsupported param type: const QStyleOption *
- `inline QRect subElementRect(DStyle::SubElement r, const QStyleOption *opt, const` ← unsupported param type: const QStyleOption *
- `inline QSize sizeFromContents(DStyle::ContentsType ct, const QStyleOption *opt, ` ← unsupported param type: const QStyleOption *
- `inline QIcon standardIcon(DStyle::StandardPixmap st, const QStyleOption *opt = n` ← unsupported param type: const QStyleOption *
- `void drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption *opt, QPainte` ← unsupported param type: QStyle::PrimitiveElement
- `void drawControl(QStyle::ControlElement ce, const QStyleOption *opt, QPainter *p` ← unsupported param type: QStyle::ControlElement
- `int pixelMetric(QStyle::PixelMetric m, const QStyleOption *opt = nullptr, const ` ← unsupported param type: QStyle::PixelMetric
- `int styleHint(StyleHint sh, const QStyleOption *opt, const QWidget *w, QStyleHin` ← unsupported param type: StyleHint
- `QRect subElementRect(QStyle::SubElement r, const QStyleOption *opt, const QWidge` ← unsupported param type: QStyle::SubElement
- `QSize sizeFromContents(QStyle::ContentsType ct, const QStyleOption *opt, const Q` ← unsupported param type: QStyle::ContentsType
- `QIcon standardIcon(QStyle::StandardPixmap st, const QStyleOption *opt = nullptr,` ← unsupported param type: QStyle::StandardPixmap
- `QPixmap generatedIconPixmap(QIcon::Mode iconMode, const QPixmap &pixmap, const Q` ← unsupported param type: QIcon::Mode
- `QBrush generatedBrush(const QStyleOption *option, const QBrush &base, QPalette::` ← unsupported return type: QBrush
- `QBrush generatedBrush(StyleState state, const QStyleOption *option, const QBrush` ← unsupported return type: QBrush
- `virtual QBrush generatedBrush(StateFlags flags, const QBrush &base, QPalette::Co` ← unsupported return type: QBrush
- `QBrush generatedBrush(const QStyleOption *option, const QBrush &base, DPalette::` ← unsupported return type: QBrush
- `QBrush generatedBrush(StyleState state, const QStyleOption *option, const QBrush` ← unsupported return type: QBrush
- `virtual QBrush generatedBrush(StateFlags flags, const QBrush &base, DPalette::Co` ← unsupported return type: QBrush
- `static QSizeF viewItemTextLayout(QTextLayout &textLayout, int lineWidth);` ← unsupported return type: QSizeF
- `static QSize viewItemSize(const QStyle *style, const QStyleOptionViewItem *optio` ← unsupported param type: const QStyle *
- `static void viewItemLayout(const QStyle *style, const QStyleOptionViewItem *opt,` ← unsupported param type: const QStyle *
- `virtual void viewItemLayout(const QStyleOptionViewItem *opt, QRect *pixmapRect, ` ← unsupported param type: const QStyleOptionViewItem *
- `static QRect viewItemDrawText(const QStyle *style, QPainter *p, const QStyleOpti` ← unsupported param type: const QStyle *
- `virtual QRect viewItemDrawText(QPainter *p, const QStyleOptionViewItem *option, ` ← unsupported param type: QPainter *

## DStyleHelper — 1 methods generated, 14 skipped
- `inline DStyleHelper(const QStyle *style = QApplication::style());` ← unsupported return type: D
- `inline void setStyle(const QStyle *style);` ← unsupported param type: const QStyle *
- `inline const QStyle *style() const` ← unsupported return type: const QStyle *
- `inline QBrush generatedBrush(const QStyleOption *option, const QBrush &base, QPa` ← unsupported return type: QBrush
- `inline QBrush generatedBrush(const QStyleOption *option, const QBrush &base, QPa` ← unsupported return type: QBrush
- `inline QColor getColor(const QStyleOption *option, QPalette::ColorRole role) con` ← unsupported param type: const QStyleOption *
- `inline QColor getColor(const QStyleOption *option, const DPalette &palette, DPal` ← unsupported param type: const QStyleOption *
- `inline QColor getColor(const T *option, DPalette::ColorType type) const` ← unsupported param type: const T *
- `inline void drawPrimitive(DStyle::PrimitiveElement pe, const QStyleOption *opt, ` ← unsupported param type: const QStyleOption *
- `inline void drawControl(DStyle::ControlElement ce, const QStyleOption *opt, QPai` ← unsupported param type: const QStyleOption *
- `inline int pixelMetric(DStyle::PixelMetric m, const QStyleOption *opt = nullptr,` ← unsupported param type: const QStyleOption *
- `inline QRect subElementRect(DStyle::SubElement r, const QStyleOption *opt, const` ← unsupported param type: const QStyleOption *
- `inline QSize sizeFromContents(DStyle::ContentsType ct, const QStyleOption *opt, ` ← unsupported param type: const QStyleOption *
- `inline QIcon standardIcon(DStyle::StandardPixmap standardIcon, const QStyleOptio` ← unsupported param type: const QStyleOption *

## DStylePainter — 2 methods generated, 11 skipped
- `inline DStylePainter() : QPainter(), widget(nullptr), wstyle(nullptr);` ← unsupported return type: D
- `inline explicit DStylePainter(QWidget *w);` ← unsupported return type: D
- `inline DStylePainter(QPaintDevice *pd, QWidget *w);` ← unsupported return type: D
- `inline bool begin(QPaintDevice *pd, QWidget *w);` ← unsupported param type: QPaintDevice *
- `inline void drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption &opt);` ← unsupported param type: QStyle::PrimitiveElement
- `inline void drawPrimitive(DStyle::PrimitiveElement pe, const QStyleOption &opt);` ← unsupported param type: const QStyleOption &
- `inline void drawControl(QStyle::ControlElement ce, const QStyleOption &opt);` ← unsupported param type: QStyle::ControlElement
- `inline void drawControl(DStyle::ControlElement ce, const QStyleOption &opt);` ← unsupported param type: const QStyleOption &
- `inline void drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComp` ← unsupported param type: QStyle::ComplexControl
- `inline void drawItemText(const QRect &r, int flags, const QPalette &pal, bool en` ← unsupported param type: QPalette::ColorRole
- `inline QStyle *style() const;` ← unsupported return type: QStyle *

## DStyledIconEngine — 1 methods generated, 7 skipped
- `static void drawIcon(const QIcon &icon, QPainter *pa, const QRectF &rect);` ← unsupported param type: QPainter *
- `void bindDrawFun(DrawFun drawFun);` ← unsupported param type: DrawFun
- `QPixmap pixmap(const QSize &size, QIcon::Mode mode, QIcon::State state) override` ← unsupported param type: QIcon::Mode
- `void paint(QPainter *painter, const QPalette &palette, const QRectF &rect);` ← unsupported param type: QPainter *
- `void paint(QPainter *painter, const QRect &rect, QIcon::Mode mode, QIcon::State ` ← unsupported param type: QPainter *
- `QIconEngine *clone() const override;` ← unsupported return type: QIconEngine *
- `void setFrontRole(const QWidget* widget, QPalette::ColorRole role);` ← unsupported param type: QPalette::ColorRole

## DViewItemAction — 16 methods generated, 0 skipped

## DStyledItemDelegate — 8 methods generated, 3 skipped
- `void paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIn` ← unsupported param type: QPainter *
- `QSize sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) con` ← unsupported param type: const QStyleOptionViewItem &
- `void updateEditorGeometry(QWidget *editor, const QStyleOptionViewItem &option, c` ← unsupported param type: const QStyleOptionViewItem &

## DStandardItem — 12 methods generated, 5 skipped
- `void setActionList(Qt::Edge edge, const DViewItemActionList &list);` ← unsupported param type: const DViewItemActionList &
- `DViewItemActionList actionList(Qt::Edge edge) const;` ← unsupported return type: DViewItemActionList
- `void setTextActionList(const DViewItemActionList &list);` ← unsupported param type: const DViewItemActionList &
- `DViewItemActionList textActionList() const;` ← unsupported return type: DViewItemActionList
- `virtual QStandardItem *clone() const override;` ← unsupported return type: QStandardItem *

## DStyleOption — 2 methods generated, 0 skipped

## DStyleOptionButton — 1 methods generated, 0 skipped

## DStyleOptionButtonBoxButton — 0 methods generated, 0 skipped

## DStyleOptionLineEdit — 1 methods generated, 0 skipped

## DStyleOptionBackgroundGroup — 1 methods generated, 0 skipped

## DStyleOptionIcon — 0 methods generated, 0 skipped

## DStyleOptionIconV2 — 0 methods generated, 0 skipped

## DStyleOptionViewItem — 0 methods generated, 0 skipped

## DStyleOptionFloatingWidget — 0 methods generated, 0 skipped

## DFontSizeManager — 21 methods generated, 0 skipped

## DSwitchButton — 1 methods generated, 0 skipped

## DSwitchHeaderLine — 1 methods generated, 0 skipped

## DSwitchLineExpand — 3 methods generated, 0 skipped

## DTabBar — 65 methods generated, 1 skipped
- `QWindow *dragIconWindow() const;` ← unsupported return type: QWindow *

## DTabletWindowOptionButton — 1 methods generated, 0 skipped

## DTextEdit — 6 methods generated, 0 skipped

## DTickEffect — 6 methods generated, 0 skipped

## DTipLabel — 2 methods generated, 0 skipped

## DTitlebarToolBaseInterface — 3 methods generated, 0 skipped

## DTitleBarToolInterface — 1 methods generated, 0 skipped

## DTitleBarSpacerInterface — 2 methods generated, 0 skipped

## DTitlebarSettings — 1 methods generated, 1 skipped
- `bool initilize(QList<DTitlebarToolBaseInterface *> &tools, const QString &path);` ← unsupported param type: QList<DTitlebarToolBaseInterface *> &

## DToolButton — 2 methods generated, 0 skipped

## DToolTip — 8 methods generated, 1 skipped
- `static QString wrapToolTipText(QString text, QTextOption option);` ← unsupported param type: QTextOption

## DWarningButton — 0 methods generated, 0 skipped

## DWaterMarkHelper — 2 methods generated, 2 skipped
- `WaterMarkData data() const;` ← unsupported return type: WaterMarkData
- `void setData(const WaterMarkData &data);` ← unsupported param type: const WaterMarkData &

## WaterMarkData — 22 methods generated, 2 skipped
- `QImage image() const;` ← unsupported return type: QImage
- `void setImage(const QImage &image);` ← unsupported param type: const QImage &

## DWaterMarkWidget — 0 methods generated, 2 skipped
- `const WaterMarkData &data();` ← unsupported return type: const WaterMarkData &
- `void setData(const WaterMarkData &data);` ← unsupported param type: const WaterMarkData &

## DWaterProgress — 5 methods generated, 0 skipped

## DWindowCloseButton — 1 methods generated, 0 skipped

## DWindowMaxButton — 3 methods generated, 0 skipped

## DWindowMinButton — 1 methods generated, 0 skipped

## DWindowOptionButton — 1 methods generated, 0 skipped

## DWindowQuitFullButton — 1 methods generated, 0 skipped
