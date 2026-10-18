# DTK6 widget binding coverage report

classes: 146, methods generated: 1043, skipped: 146

Qt ancestry: 85 widgets, 26 QObjects, 35 opaque types

## skip reasons
- unsupported param type: 79
- unsupported return type: 48
- retained raw pointer: 18
- required native parent: 1

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

## DBackgroundGroup — 8 methods generated, 0 skipped

## DBaseLine — 8 methods generated, 0 skipped

## DBlurEffectWidget — 21 methods generated, 1 skipped
- `void updateBlurSourceImage(const QRegion &ren);` ← unsupported param type: const QRegion &

## DBlurEffectGroup — 4 methods generated, 0 skipped

## DBlurEffectWithBorderWidget — 0 methods generated, 0 skipped

## DBounceAnimation — 1 methods generated, 1 skipped
- `void setAnimationTarget(QAbstractScrollArea *w);` ← unsupported param type: QAbstractScrollArea *

## DBoxWidget — 3 methods generated, 2 skipped
- `QBoxLayout::Direction direction() const;` ← unsupported return type: QBoxLayout::Direction
- `void setDirection(QBoxLayout::Direction direction);` ← unsupported param type: QBoxLayout::Direction

## DHBoxWidget — 0 methods generated, 0 skipped

## DVBoxWidget — 0 methods generated, 0 skipped

## DButtonBoxButton — 8 methods generated, 0 skipped

## DButtonBox — 7 methods generated, 2 skipped
- `void setButtonList(const QList<DButtonBoxButton*> &list, bool checkable);` ← unsupported param type: const QList<DButtonBoxButton*> &
- `QList<QAbstractButton*> buttonList() const;` ← unsupported return type: QList<QAbstractButton*>

## DCircleProgress — 10 methods generated, 2 skipped
- `QLabel *topLabel();` ← unsupported return type: QLabel *
- `QLabel *bottomLabel();` ← unsupported return type: QLabel *

## DClipEffectWidget — 4 methods generated, 0 skipped

## DColoredProgressBar — 3 methods generated, 0 skipped

## DComboBox — 1 methods generated, 1 skipped
- `virtual bool eventFilter(QObject *watched, QEvent *event) override;` ← unsupported param type: QObject *

## DCommandLinkButton — 1 methods generated, 0 skipped

## DCrumbTextFormat — 10 methods generated, 0 skipped

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

## DFileChooserEdit — 7 methods generated, 4 skipped
- `void setFileMode(QFileDialog::FileMode mode);` ← unsupported param type: QFileDialog::FileMode
- `QFileDialog::FileMode fileMode() const;` ← unsupported return type: QFileDialog::FileMode
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

## DGraphicsClipEffect — 4 methods generated, 0 skipped

## DGraphicsGlowEffect — 15 methods generated, 0 skipped

## DHeaderLine — 3 methods generated, 0 skipped

## DIconButton — 13 methods generated, 0 skipped

## DImageViewer — 23 methods generated, 0 skipped

## DIndeterminateProgressbar — 0 methods generated, 0 skipped

## DInputDialog — 38 methods generated, 4 skipped
- `static QString getText(QWidget *parent, const QString &title, const QString &mes` ← unsupported param type: bool *
- `static QString getItem(QWidget *parent, const QString &title, const QString &mes` ← unsupported param type: bool *
- `static int getInt(QWidget *parent, const QString &title, const QString &message,` ← unsupported param type: bool *
- `static double getDouble(QWidget *parent, const QString &title, const QString &me` ← unsupported param type: bool *

## DIpv4LineEdit — 9 methods generated, 0 skipped

## DKeySequenceEdit — 5 methods generated, 0 skipped

## DLicenseDialog — 5 methods generated, 0 skipped

## DLineEdit — 30 methods generated, 3 skipped
- `QLineEdit *lineEdit() const;` ← unsupported return type: QLineEdit *
- `void setLeftWidgets(const QList<QWidget *> &list);` ← unsupported param type: const QList<QWidget *> &
- `void setRightWidgets(const QList<QWidget *> &list);` ← unsupported param type: const QList<QWidget *> &

## DVariantListModel — 5 methods generated, 0 skipped

## DListView — 29 methods generated, 2 skipped
- `State state() const;` ← unsupported return type: State
- `void setOrientation(QListView::Flow flow, bool wrapping);` ← unsupported param type: QListView::Flow

## DLoadingIndicator — 21 methods generated, 1 skipped
- `void setAniEasingCurve(const QEasingCurve & easing);` ← unsupported param type: const QEasingCurve &

## DMessageManager — 5 methods generated, 0 skipped

## DMPRISControl — 3 methods generated, 0 skipped

## DPageIndicator — 16 methods generated, 0 skipped

## DPaletteHelper — 4 methods generated, 0 skipped

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

## DPrintPreviewWidget — 50 methods generated, 3 skipped
- `void setColorMode(const DPrinter::ColorMode &colorMode);` ← unsupported param type: const DPrinter::ColorMode &
- `DPrinter::ColorMode getColorMode();` ← unsupported return type: DPrinter::ColorMode
- `void themeTypeChanged(DGuiApplicationHelper::ColorType themeType);` ← unsupported param type: DGuiApplicationHelper::ColorType

## DProgressBar — 2 methods generated, 0 skipped

## DSearchComboBox — 1 methods generated, 0 skipped

## DSearchEdit — 7 methods generated, 0 skipped

## DSettingsDialog — 6 methods generated, 2 skipped
- `void updateSettings(DTK_CORE_NAMESPACE::DSettings *settings);` ← unsupported param type: DTK_CORE_NAMESPACE::DSettings *
- `void updateSettings(const QByteArray &translateContext, DTK_CORE_NAMESPACE::DSet` ← unsupported param type: DTK_CORE_NAMESPACE::DSettings *

## DSettingsWidgetFactory — 0 methods generated, 7 skipped
- `void registerWidget(const QString &viewType, std::function<WidgetCreateHandler> ` ← unsupported param type: std::function<WidgetCreateHandler>
- `void registerWidget(const QString &viewType, std::function<ItemCreateHandler> ha` ← unsupported param type: std::function<ItemCreateHandler>
- `QWidget *createWidget(QPointer<DTK_CORE_NAMESPACE::DSettingsOption> option);` ← unsupported param type: QPointer<DTK_CORE_NAMESPACE::DSettingsOption>
- `QWidget *createWidget(const QByteArray &translateContext, QPointer<DTK_CORE_NAME` ← unsupported param type: QPointer<DTK_CORE_NAMESPACE::DSettingsOption>
- `QPair<QWidget*, QWidget*> createItem(QPointer<DTK_CORE_NAMESPACE::DSettingsOptio` ← unsupported return type: QPair<QWidget*, QWidget*>
- `QPair<QWidget*, QWidget*> createItem(const QByteArray &translateContext, QPointe` ← unsupported return type: QPair<QWidget*, QWidget*>
- `static QPair<QWidget*, QWidget*> createStandardItem(const QByteArray &translateC` ← unsupported return type: QPair<QWidget*, QWidget*>

## DShadowLine — 1 methods generated, 0 skipped

## DShortcutEditLabel — 1 methods generated, 0 skipped

## DSimpleListItem — 3 methods generated, 0 skipped

## DSimpleListView — 26 methods generated, 7 skipped
- `void setColumnHideFlags(QList<bool> toggleHideFlags, int alwaysVisibleColumn=-1)` ← unsupported param type: QList<bool>
- `void setColumnSortingAlgorithms(QList<SortAlgorithm> *algorithms, int sortColumn` ← unsupported param type: QList<SortAlgorithm> *
- `void setSearchAlgorithm(SearchAlgorithm algorithm);` ← unsupported param type: SearchAlgorithm
- `void addItems(QList<DSimpleListItem*> items);` ← unsupported param type: QList<DSimpleListItem*>
- `void addSelections(QList<DSimpleListItem*> items, bool recordLastSelection=true)` ← unsupported param type: QList<DSimpleListItem*>
- `QList<DSimpleListItem*> getSelections();` ← unsupported return type: QList<DSimpleListItem*>
- `void refreshItems(QList<DSimpleListItem*> items);` ← unsupported param type: QList<DSimpleListItem*>

## DSizeModeHelper — 0 methods generated, 1 skipped
- `static inline T element(const T &t1, const T &t2)` ← unsupported return type: inline T

## DSlider — 24 methods generated, 1 skipped
- `QSlider *slider();` ← unsupported return type: QSlider *

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

## DStackWidget — 16 methods generated, 0 skipped

## DStyle — 35 methods generated, 12 skipped
- `static QPair<QIcon::Mode, QIcon::State> toIconModeState(const QStyleOption *opti` ← unsupported return type: QPair<QIcon::Mode, QIcon::State>
- `static DDciIcon::Mode toDciIconMode(const QStyleOption *option);` ← unsupported return type: DDciIcon::Mode
- `static void setRedPointVisible(QObject *object, bool visible);` ← unsupported param type: QObject *
- `static void setLineEditIconMargin(QObject *object, int margin);` ← unsupported param type: QObject *
- `int styleHint(StyleHint sh, const QStyleOption *opt, const QWidget *w, QStyleHin` ← unsupported param type: StyleHint
- `virtual QBrush generatedBrush(StateFlags flags, const QBrush &base, QPalette::Co` ← unsupported param type: StateFlags
- `QBrush generatedBrush(const QStyleOption *option, const QBrush &base, DPalette::` ← unsupported param type: DPalette::ColorGroup
- `QBrush generatedBrush(StyleState state, const QStyleOption *option, const QBrush` ← unsupported param type: DPalette::ColorGroup
- `virtual QBrush generatedBrush(StateFlags flags, const QBrush &base, DPalette::Co` ← unsupported param type: StateFlags
- `static QSizeF viewItemTextLayout(QTextLayout &textLayout, int lineWidth);` ← unsupported param type: QTextLayout &
- `static void viewItemLayout(const QStyle *style, const QStyleOptionViewItem *opt,` ← unsupported param type: QRect *
- `virtual void viewItemLayout(const QStyleOptionViewItem *opt, QRect *pixmapRect, ` ← unsupported param type: QRect *

## DStyleHelper — 1 methods generated, 14 skipped
- `inline DStyleHelper(const QStyle *style = QApplication::style());` ← unsupported return type: D
- `inline void setStyle(const QStyle *style);` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline const QStyle *style() const` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline QBrush generatedBrush(const QStyleOption *option, const QBrush &base, QPa` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline QBrush generatedBrush(const QStyleOption *option, const QBrush &base, QPa` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline QColor getColor(const QStyleOption *option, QPalette::ColorRole role) con` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline QColor getColor(const QStyleOption *option, const DPalette &palette, DPal` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline QColor getColor(const T *option, DPalette::ColorType type) const` ← unsupported param type: const T *
- `inline void drawPrimitive(DStyle::PrimitiveElement pe, const QStyleOption *opt, ` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline void drawControl(DStyle::ControlElement ce, const QStyleOption *opt, QPai` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline int pixelMetric(DStyle::PixelMetric m, const QStyleOption *opt = nullptr,` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline QRect subElementRect(DStyle::SubElement r, const QStyleOption *opt, const` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline QSize sizeFromContents(DStyle::ContentsType ct, const QStyleOption *opt, ` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor
- `inline QIcon standardIcon(DStyle::StandardPixmap standardIcon, const QStyleOptio` ← retained raw pointer: dstyle.h DStyleHelper keeps `const QStyle *m_style` raw and has no scope-bound constructor

## DStylePainter — 3 methods generated, 10 skipped
- `inline DStylePainter() : QPainter(), widget(nullptr), wstyle(nullptr);` ← unsupported return type: D
- `inline explicit DStylePainter(QWidget *w);` ← unsupported return type: D
- `inline DStylePainter(QPaintDevice *pd, QWidget *w);` ← unsupported return type: D
- `inline bool begin(QPaintDevice *pd, QWidget *w);` ← unsupported param type: QPaintDevice *
- `inline void drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption &opt);` ← retained raw pointer: dstyle.h DStylePainter keeps raw widget/style pointers and has no safe constructor
- `inline void drawPrimitive(DStyle::PrimitiveElement pe, const QStyleOption &opt);` ← retained raw pointer: dstyle.h DStylePainter keeps raw widget/style pointers and has no safe constructor
- `inline void drawControl(QStyle::ControlElement ce, const QStyleOption &opt);` ← retained raw pointer: dstyle.h DStylePainter keeps raw widget/style pointers and has no safe constructor
- `inline void drawControl(DStyle::ControlElement ce, const QStyleOption &opt);` ← retained raw pointer: dstyle.h DStylePainter keeps raw widget/style pointers and has no safe constructor
- `inline void drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComp` ← unsupported param type: const QStyleOptionComplex &
- `inline QStyle *style() const;` ← retained raw pointer: dstyle.h DStylePainter keeps raw widget/style pointers and has no safe constructor

## DStyledIconEngine — 5 methods generated, 3 skipped
- `void bindDrawFun(DrawFun drawFun);` ← unsupported param type: DrawFun
- `QIconEngine *clone() const override;` ← unsupported return type: QIconEngine *
- `void setFrontRole(const QWidget* widget, QPalette::ColorRole role);` ← retained raw pointer: dtkwidget dstyle.cpp stores `m_widget = widget` without a QPointer; later paint dereferences it

## DViewItemAction — 16 methods generated, 0 skipped

## DStyledItemDelegate — 11 methods generated, 1 skipped
- `DStyledItemDelegate(parent = nullptr)` ← required native parent: dtkwidget/src/widgets/dstyleditemdelegate.cpp: constructor requires a live QAbstractItemView; nullptr default is dereferenced

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

## DWaterMarkHelper — 4 methods generated, 0 skipped

## WaterMarkData — 24 methods generated, 0 skipped

## DWaterMarkWidget — 2 methods generated, 0 skipped

## DWaterProgress — 5 methods generated, 0 skipped

## DWindowCloseButton — 1 methods generated, 0 skipped

## DWindowMaxButton — 3 methods generated, 0 skipped

## DWindowMinButton — 1 methods generated, 0 skipped

## DWindowOptionButton — 1 methods generated, 0 skipped

## DWindowQuitFullButton — 1 methods generated, 0 skipped
