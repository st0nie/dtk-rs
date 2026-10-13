"""Audited native pointer contracts; never infer ownership from a method name.

Evidence is from apt source dtkwidget=6.7.47.1-1, dtkcore=6.7.47,
dtkgui=6.7.47 and qt6-base=6.8.0+dfsg-0deepin31. Paths below are relative
to those source packages. Native constructor/heap-copy ownership is emitted
separately by gen.py. A newly mapped pointer return must be added here after
reading its implementation, otherwise generation fails closed.
"""

# (class, C++ method): (contract, package/source path, implementation evidence)
# object_borrow: QPointer checked view, never a new deletion capability.
# opaque_static: process-owned non-QObject storage, scope checked, no deleter.
# opaque_transfer: unique heap allocation with a virtual/matching deleter.
# implementation_defined: foreign virtual implementation must supply its contract.
POINTER_RETURNS = {
    ("DAnchorsBase", "target"): ("object_borrow", "dtkwidget/src/widgets/danchors.cpp", "returns extendWidget->target(); widget is not transferred"),
    ("DAnchorsBase", "enhancedWidget"): ("object_borrow", "dtkwidget/src/widgets/danchors.cpp", "returns the existing extendWidget child"),
    ("DAnchorsBase", "anchors"): ("object_borrow", "dtkwidget/src/widgets/danchors.cpp", "returns this; no allocation or transfer"),
    ("DAnchorsBase", "fill"): ("object_borrow", "dtkwidget/src/widgets/danchors.cpp", "returns fill->target(); not an allocation"),
    ("DAnchorsBase", "centerIn"): ("object_borrow", "dtkwidget/src/widgets/danchors.cpp", "returns centerIn->target(); not an allocation"),
    ("DAnchorsBase", "getAnchorBaseByWidget"): ("object_borrow", "dtkwidget/src/widgets/danchors.cpp", "looks up getWidgetAnchorsBase; clearAnchors deletes it"),
    ("DArrowRectangle", "getContent"): ("object_borrow", "dtkwidget/src/widgets/darrowrectangle.cpp", "returns existing QPointer m_content"),
    ("DBaseLine", "leftLayout"): ("object_borrow", "dtkwidget/src/widgets/dbaseline.cpp", "returns existing m_leftLayout owned through the widget layout"),
    ("DBaseLine", "rightLayout"): ("object_borrow", "dtkwidget/src/widgets/dbaseline.cpp", "returns existing m_rightLayout owned through the widget layout"),
    ("DBoxWidget", "layout"): ("object_borrow", "dtkwidget/src/widgets/dboxwidget.cpp", "returns the installed layout"),
    ("DButtonBox", "checkedButton"): ("object_borrow", "dtkwidget/src/widgets/dbuttonbox.cpp", "returns QButtonGroup::checkedButton; no transfer"),
    ("DButtonBox", "button"): ("object_borrow", "dtkwidget/src/widgets/dbuttonbox.cpp", "returns QButtonGroup::button(id); no transfer"),
    ("DDialog", "getButton"): ("object_borrow", "dtkwidget/src/widgets/ddialog.cpp", "returns an existing buttonList entry; removal schedules deletion"),
    ("DDialog", "getContent"): ("object_borrow", "dtkwidget/src/widgets/ddialog.cpp", "returns an existing contentList entry; removal may schedule deletion"),
    ("DDrawer", "getContent"): ("object_borrow", "dtkwidget/src/widgets/ddrawer.cpp", "returns m_content; layout adopts the widget"),
    ("DDrawerGroup", "checkedExpand"): ("object_borrow", "dtkwidget/src/widgets/ddrawergroup.cpp", "returns checkedMap.first; the group does not adopt drawers"),
    ("DDrawerGroup", "expand"): ("object_borrow", "dtkwidget/src/widgets/ddrawergroup.cpp", "returns expandMap.value(id); the group does not adopt drawers"),
    ("DEnhancedWidget", "target"): ("object_borrow", "dtkwidget/src/widgets/denhancedwidget.cpp", "returns d->target; observer does not own its target"),
    ("DFileIconProvider", "globalProvider"): ("opaque_static", "dtkwidget/src/util/dfileiconprovider.cpp", "returns Q_GLOBAL_STATIC(DFileIconProvider, globalFIP); caller must not delete"),
    ("DFloatingWidget", "blurBackground"): ("object_borrow", "dtkwidget/src/widgets/dfloatingwidget.cpp", "returns existing background child; disabling queues its deletion"),
    ("DListView", "getHeaderWidget"): ("object_borrow", "dtkwidget/src/widgets/dlistview.cpp", "returns headerList entry; view/layout owns it"),
    ("DListView", "getFooterWidget"): ("object_borrow", "dtkwidget/src/widgets/dlistview.cpp", "returns footerList entry; view/layout owns it"),
    ("DListView", "takeHeaderWidget"): ("object_borrow", "dtkwidget/src/widgets/dlistview.cpp", "removes list/layout entry but leaves widget parented; an empty header container's deleteLater destroys it; NOT a transfer"),
    ("DListView", "takeFooterWidget"): ("object_borrow", "dtkwidget/src/widgets/dlistview.cpp", "removes list/layout entry but leaves widget parented; an empty footer container's deleteLater destroys it; NOT a transfer"),
    ("DLoadingIndicator", "widgetSource"): ("object_borrow", "dtkwidget/src/widgets/dloadingindicator.cpp", "returns widgetSource; QGraphicsProxyWidget adopts its widget"),
    ("DMessageManager", "instance"): ("object_borrow", "dtkwidget/src/widgets/dmessagemanager.cpp", "returns function-static manager; no caller ownership"),
    ("DPaletteHelper", "instance"): ("object_borrow", "dtkwidget/src/widgets/dpalettehelper.cpp", "returns native g_instance; cleanup is native-owned"),
    ("DPrintPreviewDialog", "createDialogSettingInfo"): ("opaque_transfer", "dtkwidget/src/widgets/dprintpreviewdialog.cpp", "loadInfo allocates derived setting info with new; rejected plugin results deleted internally; successful results released by caller; base destructor virtual"),
    ("DFeatureDisplayDialog", "removeItem"): ("object_transfer", "dtkwidget/src/widgets/dfeaturedisplaydialog.cpp", "removeOne releases retained identity without deletion; shim verifies membership and detaches its binding-added parent before returning a new owner"),
    ("DSettingsDialog", "widgetFactory"): ("object_borrow", "dtkwidget/src/widgets/dsettingsdialog.cpp", "returns existing widgetFactory child"),
    ("DStackWidget", "currentWidget"): ("object_borrow", "dtkwidget/src/widgets/dstackwidget.cpp", "returns currentWidget; pop/clear destroy or schedule widgets"),
    ("DStackWidget", "transition"): ("object_borrow", "dtkwidget/src/widgets/dstackwidget.cpp", "returns parented transition; replacement queues old deletion"),
    ("DStackWidget", "getWidgetByIndex"): ("object_borrow", "dtkwidget/src/widgets/dstackwidget.cpp", "returns widgetList[index]; bounds must be checked by shim"),
    ("DStyleHelper", "dstyle"): ("object_borrow", "dtkwidget/include/widgets/dstyle.h", "inline dynamic_cast of wstyle; helper neither allocates nor owns it"),
    ("DViewItemAction", "widget"): ("object_borrow", "dtkwidget/src/widgets/dstyleditemdelegate.cpp", "returns QPointer QWidget stored by setWidget; ownership unchanged"),
    ("DFontSizeManager", "instance"): ("opaque_static", "dtkwidget/src/widgets/dstyleoption.cpp", "returns address of function-static DFontSizeManager manager; caller must not delete"),
    ("DSwitchLineExpand", "header"): ("object_borrow", "dtkwidget/src/widgets/dswitchlineexpand.cpp", "returns existing header widget"),
    ("DTabBar", "tabButton"): ("object_borrow", "dtkwidget/src/widgets/dtabbar.cpp", "forwards QTabBar::tabButton; Qt owns inserted buttons"),
    ("DTitleBarToolInterface", "createView"): ("implementation_defined", "dtkwidget/include/widgets/dtitlebarsettings.h", "pure virtual; header does not specify borrowed vs transferred; cannot manufacture a unique owner"),
    ("DTitleBarSpacerInterface", "createPlaceholderView"): ("implementation_defined", "dtkwidget/include/widgets/dtitlebarsettings.h", "pure virtual; native implementations differ; caller must audit the foreign implementation"),
    ("DTitlebarSettings", "toolsEditPanel"): ("object_borrow", "dtkwidget/src/widgets/dtitlebarsettings.cpp", "returns impl()->toolsEditPanel; native titlebar settings controls its lifetime"),
    ("DWaterMarkHelper", "instance"): ("object_borrow", "dtkwidget/src/widgets/dwatermarkhelper.cpp", "returns Q_GLOBAL_STATIC helper; native post routine owns cleanup"),
    ("DGuiApplicationHelper", "instance"): ("object_borrow", "dtkgui/src/kernel/dguiapplicationhelper.cpp", "returns native global helper; staticCleanApplication post routine resets it"),
    ("DGuiApplicationHelper", "fontManager"): ("object_borrow", "dtkgui/src/kernel/dguiapplicationhelper.cpp", "returns Q_GLOBAL_STATIC(DFontManager, _globalFM); no transfer"),
}

# Audited implicit public value constructor needed by an exclusive output API.
# Value-initialize to avoid exposing its otherwise indeterminate bool field.
IMPLICIT_VALUE_CONSTRUCTORS = {
    "DStyleOptionFloatingWidget": ("dtkwidget/include/widgets/dstyleoption.h", "implicit public QStyleOption value constructor; value initialization zeroes noBackground"),
}

# Constructors which the header's default arguments do NOT make safe.
# DStyledItemDelegate immediately dereferences parent->viewport() in its body.
REQUIRED_PARENT_CONSTRUCTORS = {
    "DStyledItemDelegate": ("dtkwidget/src/widgets/dstyleditemdelegate.cpp", "constructor requires a live QAbstractItemView; nullptr default is dereferenced"),
}


def pointer_return_contract(class_name, method):
    try:
        return POINTER_RETURNS[class_name, method]
    except KeyError as error:
        raise ValueError(f"unaudited pointer return: {class_name}::{method}") from error
