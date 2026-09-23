#include "dtk_shim.h"
#include "relay.h"
#include "dtk-sys/src/lib.rs.h" // Rust callback declarations (dtk_cb0/dtk_cb_i32)

#include <QCoreApplication>
#include <QGuiApplication>
#include <QApplication>
#include <QHBoxLayout>
#include <QInputMethod>
#include <QClipboard>
#include <QProxyStyle>
#include <DGuiApplicationHelper>
#include <DPalette>

// The distro's DFontSizeManager forwarding header is broken (it only includes
// dstyleoption.h); declare the two symbols we use directly — they live in
// libdtk6widget (verified mangled names match this declaration).
namespace Dtk { namespace Widget {
class DFontSizeManager {
public:
    enum SizeType { T1, T2, T3, T4, T5, T6, T7, T8, T9, T10 };
    static DFontSizeManager *instance();
    int fontPixelSize(SizeType type) const;
};
}}
#include <QScrollBar>
#include <QStyleOptionTab>
#include <QToolButton>
#include <QTextOption>
#include <QShortcut>
#include <QKeySequence>
#include <QKeyEvent>
#include <QMouseEvent>
#include <QWheelEvent>
#include <QInputMethodEvent>
#include <QHeaderView>
#include <QStyle>
#include <QProgressBar>
#include <QTableWidgetItem>
#include <cstdio>
#include <string>
#include <utility>
#include <vector>

namespace dtkrs {

std::unique_ptr<QObjectGuard> guard_object(QObject *object) {
    return std::make_unique<QObjectGuard>(object);
}
std::unique_ptr<QObjectGuard> guard_widget(QWidget *widget) {
    return guard_object(widget); // C++ adjusts QWidget* to its QObject base
}
bool guard_alive(const QObjectGuard &guard) { return !guard.watched.isNull(); }

// Release a Rust closure as soon as the QObject that can invoke it is destroyed.
// No receiver/context is needed: Qt removes the connection with its sender.
static void dispose_on_destroy(QObject *owner, size_t cb_id) {
    QObject::connect(owner, &QObject::destroyed, [cb_id] { dtk_cb_dispose(cb_id); });
}

// DApplication subclass: event() intercepts QEvent::Quit and asks the Rust guard
class DtkAppEx : public DApplication {
public:
    DtkAppEx(int &argc, char **argv, size_t guard_id)
        : DApplication(argc, argv), m_guard_id(guard_id) {
        dispose_on_destroy(this, guard_id);
    }

    bool event(QEvent *e) override {
        if (e->type() == QEvent::Quit && m_guard_id && !dtk_cb_guard(m_guard_id))
            return true; // guard refused -> swallow; the Rust side schedules its own retry
        return DApplication::event(e);
    }

private:
    size_t m_guard_id;
};

// DMainWindow subclass: forwards showEvent/closeEvent to Rust
class DtkMainWindowEx : public DMainWindow {
public:
    DtkMainWindowEx(size_t show_id, size_t close_id) : m_show_id(show_id), m_close_id(close_id) {
        dispose_on_destroy(this, show_id);
        dispose_on_destroy(this, close_id);
    }

protected:
    void showEvent(QShowEvent *e) override {
        if (m_show_id)
            dtk_cb0(m_show_id);
        DMainWindow::showEvent(e);
    }
    void closeEvent(QCloseEvent *e) override {
        if (m_close_id && !dtk_cb_guard(m_close_id)) {
            e->ignore();
            return;
        }
        DMainWindow::closeEvent(e);
    }

private:
    size_t m_show_id, m_close_id;
};

// generic paint delegate: paints the default style background first (so the
// cell background/selection matches DTK theme, identical to the default column),
// then forwards overlay painting (icon + text) to Rust. Mirrors the C++
// AppNameDelegate pattern: drawControl(CE_ItemViewItem) with icon/text cleared.
class RustDelegate : public QStyledItemDelegate {
public:
    RustDelegate(size_t cb_id, QObject *parent) : QStyledItemDelegate(parent), m_cb_id(cb_id) {
        dispose_on_destroy(this, cb_id);
    }

    void paint(QPainter *p, const QStyleOptionViewItem &opt, const QModelIndex &idx) const override {
        // Paint the default item background/selection via the widget style so
        // it stays consistent with the default-rendered columns (DTK theme).
        QStyleOptionViewItem base(opt);
        initStyleOption(&base, idx);
        base.icon = {};
        base.text.clear();
        QStyle *style = base.widget ? base.widget->style() : QApplication::style();
        style->drawControl(QStyle::CE_ItemViewItem, &base, p, base.widget);

        // Rust callback draws the overlay (icon + text) on top.
        dtk_cb_paint(m_cb_id, p, const_cast<QModelIndex *>(&idx), opt.rect.x(), opt.rect.y(),
                     opt.rect.width(), opt.rect.height(), static_cast<int32_t>(opt.state));
    }

private:
    size_t m_cb_id;
};

rust::String to_rust_string(const QString &s) {
    QByteArray utf8 = s.toUtf8();
    return rust::String(utf8.constData(), utf8.size());
}

QString from_rust_str(rust::Str s) {
    return QString::fromUtf8(s.data(), static_cast<qsizetype>(s.size()));
}

rust::Vec<rust::String> to_rust_string_vec(const QStringList &qsl) {
    rust::Vec<rust::String> out;
    for (const QString &qs : qsl) {
        QByteArray u8 = qs.toUtf8();
        out.push_back(rust::String(u8.constData(), static_cast<size_t>(u8.size())));
    }
    return out;
}

QStringList to_qstringlist(rust::Vec<rust::String> v) {
    QStringList out;
    out.reserve(static_cast<qsizetype>(v.size()));
    for (const auto &s : v)
        out.append(QString::fromUtf8(s.data(), static_cast<qsizetype>(s.size())));
    return out;
}

// ---- DtkPaintWidget: user-drawn widget; paint + all input events forward to Rust ----
// Tab label drawing like deepin-terminal's TermTabStyle: centered text, elided to
// width-30 so the label never slides under the close button (no fade shadow).
class DtkTabLabelStyle : public QProxyStyle {
public:
    using QProxyStyle::QProxyStyle;
    void drawControl(ControlElement element, const QStyleOption *option, QPainter *painter,
                     const QWidget *widget) const override {
        if (element == CE_TabBarTabLabel) {
            if (const auto *tab = qstyleoption_cast<const QStyleOptionTab *>(option)) {
                QTextOption textOption;
                textOption.setAlignment(Qt::AlignCenter);
                // mirrors deepin-terminal's TermTabStyle exactly: app font at
                // DFontSizeManager::T6 pixel size, Medium weight, palette
                // HighlightedText (selected) / TextTitle (normal)
                QFont textFont = QApplication::font();
                textFont.setPixelSize(Dtk::Widget::DFontSizeManager::instance()->fontPixelSize(Dtk::Widget::DFontSizeManager::T6));
                textFont.setWeight(QFont::Medium);
                painter->setFont(textFont);
                const bool selected = tab->state & State_Selected;
                const Dtk::Gui::DPalette pa = Dtk::Gui::DGuiApplicationHelper::instance()->standardPalette(
                    Dtk::Gui::DGuiApplicationHelper::instance()->themeType());
                painter->setPen(selected ? pa.color(QPalette::HighlightedText)
                                         : pa.color(Dtk::Gui::DPalette::TextTitle));
                const QFontMetrics fm(textFont);
                const QString elided = fm.elidedText(tab->text, Qt::ElideRight,
                                                     tab->rect.width() - 30, Qt::TextShowMnemonic);
                painter->drawText(tab->rect, elided, textOption);
                return;
            }
        }
        QProxyStyle::drawControl(element, option, painter, widget);
    }
};

class DtkPaintWidget : public QWidget {
public:
    DtkPaintWidget(size_t cb_id, QWidget *parent) : QWidget(parent), m_cb_id(cb_id) {
        dispose_on_destroy(this, cb_id);
        setFocusPolicy(Qt::StrongFocus);
        setAttribute(Qt::WA_InputMethodEnabled);
        setMouseTracking(true);
    }

    // IME candidate window anchors here (set from Rust each frame)
    QRect m_imeRect;

    // Tab/Backtab are eaten by the focus framework before keyPressEvent; intercept here
    bool event(QEvent *e) override {
        if (e->type() == QEvent::KeyPress) {
            auto *ke = static_cast<QKeyEvent *>(e);
            if (ke->key() == Qt::Key_Tab || ke->key() == Qt::Key_Backtab) {
                keyPressEvent(ke);
                return true;
            }
        }
        return QWidget::event(e);
    }

protected:
    void paintEvent(QPaintEvent *) override {
        QPainter p(this);
        dtk_cb_pw_paint(m_cb_id, this, &p, width(), height());
    }
    void keyPressEvent(QKeyEvent *e) override {
        dtk_cb_pw_key(m_cb_id, e->key(), static_cast<int32_t>(e->modifiers()),
                      to_rust_string(e->text()), true, e->isAutoRepeat());
    }
    void keyReleaseEvent(QKeyEvent *e) override {
        dtk_cb_pw_key(m_cb_id, e->key(), static_cast<int32_t>(e->modifiers()),
                      to_rust_string(e->text()), false, e->isAutoRepeat());
    }
    void mousePressEvent(QMouseEvent *e) override { forward_mouse(e, 0); }
    void mouseReleaseEvent(QMouseEvent *e) override { forward_mouse(e, 1); }
    void mouseMoveEvent(QMouseEvent *e) override { forward_mouse(e, 2); }
    void mouseDoubleClickEvent(QMouseEvent *e) override { forward_mouse(e, 3); }
    void wheelEvent(QWheelEvent *e) override {
        auto pos = e->position();
        dtk_cb_pw_wheel(m_cb_id, e->angleDelta().y(), static_cast<int32_t>(pos.x()),
                        static_cast<int32_t>(pos.y()), static_cast<int32_t>(e->modifiers()));
    }
    void inputMethodEvent(QInputMethodEvent *e) override {
        dtk_cb_pw_ime(m_cb_id, to_rust_string(e->commitString()),
                      to_rust_string(e->preeditString()));
        e->accept();
    }
    QVariant inputMethodQuery(Qt::InputMethodQuery q) const override {
        switch (q) {
        case Qt::ImCursorRectangle:
        case Qt::ImAnchorRectangle:
            return m_imeRect;
        case Qt::ImFont:
            return font();
        default:
            return {};
        }
    }
    void resizeEvent(QResizeEvent *e) override {
        dtk_cb_pw_resize(m_cb_id, e->size().width(), e->size().height());
    }
    void focusInEvent(QFocusEvent *) override { dtk_cb_pw_focus(m_cb_id, true); }
    void focusOutEvent(QFocusEvent *) override { dtk_cb_pw_focus(m_cb_id, false); }

private:
    void forward_mouse(QMouseEvent *e, int32_t kind) {
        // during a drag button() is NoButton; buttons() carries the held one
        auto btn = e->button();
        if (btn == Qt::NoButton && kind == 2) {
            btn = e->buttons() & Qt::LeftButton   ? Qt::LeftButton
                : e->buttons() & Qt::MiddleButton ? Qt::MiddleButton
                : e->buttons() & Qt::RightButton  ? Qt::RightButton
                                                  : Qt::NoButton;
        }
        dtk_cb_pw_mouse(m_cb_id, kind, static_cast<int32_t>(btn),
                        static_cast<int32_t>(e->pos().x()), static_cast<int32_t>(e->pos().y()),
                        static_cast<int32_t>(e->modifiers()));
    }
    size_t m_cb_id;
};

// ---- DApplication ----
// QApplication requires argc AND argv to outlive the app (Qt6 stores int& argc):
// static storage, filled on first call (QApplication is a singleton anyway)
static std::pair<int &, char **> make_argv(rust::Str name, rust::Vec<uint8_t> bytes,
                                           rust::Vec<size_t> lengths) {
    static std::vector<std::string> store;
    static std::vector<char *> argv;
    static int argc;
    if (!store.empty()) // QApplication singleton: second call reuses the first argv
        return {argc, argv.data()};
    size_t offset = 0;
    for (const size_t length : lengths) {
        if (length > bytes.size() - offset) // malformed FFI input; never read outside the buffer
            break;
        if (length)
            store.emplace_back(reinterpret_cast<const char *>(bytes.data() + offset), length);
        else
            store.emplace_back(); // an empty argv entry is valid and must be kept
        offset += length;
    }
    if (store.empty())
        store.emplace_back(name.data(), name.size());
    for (auto &s : store)
        argv.push_back(s.data());
    argv.push_back(nullptr); // Qt expects argv[argc] == nullptr
    argc = static_cast<int>(store.size());
    return {argc, argv.data()};
}

DApplication *application_new(rust::Str name, rust::Vec<uint8_t> bytes,
                              rust::Vec<size_t> lengths) {
    if (QCoreApplication::instance())
        return nullptr; // Qt permits only one QApplication per process at a time
    auto [argc, argv] = make_argv(name, std::move(bytes), std::move(lengths));
    auto *app = new DApplication(argc, argv);
    app->setApplicationName(from_rust_str(name));
    return app;
}

bool application_is_gui() {
    return qobject_cast<QApplication *>(QCoreApplication::instance()) != nullptr;
}
int32_t application_exec(DApplication *app) { return app->exec(); }
void application_quit() { QCoreApplication::quit(); }

DApplication *application_new_ex(rust::Str name, rust::Vec<uint8_t> bytes,
                                 rust::Vec<size_t> lengths, size_t quit_guard_id) {
    if (QCoreApplication::instance())
        return nullptr;
    auto [argc, argv] = make_argv(name, std::move(bytes), std::move(lengths));
    auto *app = new DtkAppEx(argc, argv, quit_guard_id);
    app->setApplicationName(from_rust_str(name));
    return app;
}

void application_set_quit_on_last_window_closed(bool quit) {
    QGuiApplication::setQuitOnLastWindowClosed(quit);
}
void application_set_application_display_name(rust::Str name) {
    QGuiApplication::setApplicationDisplayName(from_rust_str(name));
}
bool application_load_translator(DApplication *app) { return app->loadTranslator(); }
bool application_has_arg(rust::Str arg) {
    return QCoreApplication::arguments().contains(from_rust_str(arg));
}

// ---- QAction / QMenu ----
QAction *action_new(rust::Str text, QObject *parent) {
    return new QAction(from_rust_str(text), parent);
}
void action_set_enabled(QAction *action, bool enabled) { action->setEnabled(enabled); }
bool action_is_enabled(QAction *action) { return action->isEnabled(); }
void action_set_visible(QAction *action, bool visible) { action->setVisible(visible); }
bool action_is_visible(QAction *action) { return action->isVisible(); }
void action_set_text(QAction *action, rust::Str text) { action->setText(from_rust_str(text)); }
rust::String action_text(QAction *action) { return to_rust_string(action->text()); }
void action_trigger(QAction *action) { action->trigger(); }
QMenu *menu_new(QWidget *parent) { return new QMenu(parent); }
void menu_add_existing_action(QMenu *menu, QAction *action) { menu->addAction(action); }
void menu_add_action_cb(QMenu *m, rust::Str text, size_t cb_id) {
    QAction *a = m->addAction(from_rust_str(text));
    dispose_on_destroy(a, cb_id);
    QObject::connect(a, &QAction::triggered, m, [cb_id] { dtk_cb0(cb_id); });
}
void menu_add_separator(QMenu *m) { m->addSeparator(); }
void menu_popup(QMenu *m, QWidget *ref, int32_t x, int32_t y) {
    m->popup(ref->mapToGlobal(QPoint(x, y)));
}

// ---- QWidget common ----
void widget_show(QWidget *w) { w->show(); }
void widget_set_mouse_tracking(QWidget *w, bool on) { w->setMouseTracking(on); }
void widget_hide(QWidget *w) { w->hide(); }
void widget_resize(QWidget *w, int32_t w_px, int32_t h_px) { w->resize(w_px, h_px); }
int32_t widget_width(QWidget *w) { return w->width(); }
int32_t widget_height(QWidget *w) { return w->height(); }
void widget_set_enabled(QWidget *w, bool on) { w->setEnabled(on); }
void widget_set_window_title(QWidget *w, rust::Str title) { w->setWindowTitle(from_rust_str(title)); }
void widget_set_window_icon(QWidget *w, QIcon *icon) { w->setWindowIcon(*icon); }
void widget_set_fixed_size(QWidget *w, int32_t w_px, int32_t h_px) { w->setFixedSize(w_px, h_px); }
void widget_raise(QWidget *w) { w->raise(); }
void widget_update(QWidget *w) { w->update(); }
void widget_set_focus(QWidget *w) { w->setFocus(); }
void widget_move(QWidget *w, int x, int y) { w->move(x, y); }
void tabbar_install_style(QWidget *tb) { tb->setStyle(new DtkTabLabelStyle); }
// DTabBar's internal QBoxLayout (and the titlebar container above it) relayouts
// lazily; after add/remove/setTabText a paint can land on the stale geometry
// (tabs AlignHCenter'd in an over-wide cell = blank space both sides, 1 frame).
// Flush every layout up the parent chain synchronously right after a mutation.
// Qt's scroll decision latches: once the scroll buttons show, their width keeps
// "needed" true forever. Hiding the QToolButtons re-runs DTK's eventFilter, which
// hides its mirror buttons and zeroes the spacers. Qt re-shows them on the next
// layoutTabs if scrolling is really needed, so this is safe to call any time.
void tabbar_unlatch_scroll_buttons(QWidget *tb) {
    // DTK mirrors Qt's scroll buttons via its eventFilter. During startup layout
    // transients the filter can miss a Hide, leaving DTK's mirror visible while
    // Qt's button is hidden; the mirror's layout spacers then stay sized (a gap
    // before the first tab). Directly hiding the mirror does not stick, so drive
    // Qt's button through one show/hide cycle: DTK's own filter then hides the
    // mirror and zeroes the spacers. Only act on a real mismatch, so genuine
    // overflow scrolling is never touched.
    const struct {
        const char *qtBtn;
        const char *dtkBtn;
    } pairs[] = {{"ScrollLeftButton", "leftButton"}, {"ScrollRightButton", "rightButton"}};
    for (const auto &p : pairs) {
        auto *qt = tb->findChild<QToolButton *>(QLatin1String(p.qtBtn));
        auto *dtk = tb->findChild<QWidget *>(QLatin1String(p.dtkBtn));
        // only repair the actually-observed stuck state (mirror on, Qt off);
        // never touch a genuinely-visible scroll button
        if (qt && dtk && dtk->isVisible() && !qt->isVisible()) {
            qt->show(); // -> DTK filter shows its mirror (already visible; no-op)
            qt->hide(); // -> DTK filter hides its mirror and refreshSpacers zeroes it
        }
    }
}
void tabbar_flush_layout(QWidget *tb) {
    // DTK's setTabMinimumSize/MaximumSize only flags layoutDirty; the widget
    // sizeHint caches stay stale, so the inner tab strip keeps its old (smaller)
    // width and Qt's scroll-button state latches on. Refresh geometry hints up
    // front so the relayout below works with current sizes and Qt unlatches its
    // scroll buttons through the normal resize path.
    tb->updateGeometry();
    const auto descendants = tb->findChildren<QWidget *>();
    for (QWidget *child : descendants) {
        child->updateGeometry();
    }
    for (QWidget *w = tb; w; w = w->parentWidget()) {
        if (QLayout *lay = w->layout()) {
            lay->invalidate();
            lay->activate();
        }
    }
    // resync last: once the layouts above have settled, Qt's scroll buttons hold
    // their final state; cycling now cannot be undone by a later activation
    tabbar_unlatch_scroll_buttons(tb);
    if (QLayout *lay = tb->layout()) {
        lay->invalidate();
        lay->activate();
    }
}
void widget_set_parent(QWidget *child, QWidget *parent) { child->setParent(parent); }
QWidget *scrollbar_new(QWidget *parent) { return new QScrollBar(Qt::Vertical, parent); }
void scrollbar_set_range(QWidget *sb, int minimum, int maximum) {
    static_cast<QScrollBar *>(sb)->setRange(minimum, maximum);
}
int scrollbar_maximum(QWidget *sb) { return static_cast<QScrollBar *>(sb)->maximum(); }
void scrollbar_set_value(QWidget *sb, int v) { static_cast<QScrollBar *>(sb)->setValue(v); }
int scrollbar_value(QWidget *sb) { return static_cast<QScrollBar *>(sb)->value(); }
void scrollbar_set_page_step(QWidget *sb, int v) { static_cast<QScrollBar *>(sb)->setPageStep(v); }
void paint_widget_set_ime_rect(QWidget *w, int x, int y, int width, int height) {
    if (auto *pw = dynamic_cast<DtkPaintWidget *>(w)) {
        pw->m_imeRect = QRect(x, y, width, height);
        QGuiApplication::inputMethod()->update(Qt::ImCursorRectangle);
    }
}
void main_window_titlebar_set_tabbar(QWidget *w, QWidget *tabbar) {
    auto *mw = qobject_cast<DMainWindow *>(w);
    if (!mw || !mw->titlebar()) {
        return;
    }
    // mirrors deepin-terminal's TitleBar: zero-margin hbox, vertically centered
    auto *container = new QWidget;
    auto *lay = new QHBoxLayout(container);
    lay->setContentsMargins(0, 0, 0, 0);
    lay->setSpacing(0);
    lay->addWidget(tabbar, 0, Qt::AlignVCenter);
    mw->titlebar()->setCustomWidget(container, false);
}
void main_window_titlebar_add_widget(QWidget *w, QWidget *child) {
    if (auto *mw = qobject_cast<DMainWindow *>(w)) {
        if (auto *tb = mw->titlebar()) {
            tb->addWidget(child, Qt::AlignLeft);
        }
    }
}
void widget_set_titlebar_icon(QWidget *w, QIcon *icon) {
    if (auto *mw = qobject_cast<DMainWindow *>(w)) {
        if (auto *tb = mw->titlebar()) {
            tb->setIcon(*icon);
        }
    }
}
bool app_popup_active() {
    return QApplication::activeModalWidget() != nullptr || QApplication::activePopupWidget() != nullptr;
}
uint32_t app_palette_window_rgb() {
    const QColor c = QApplication::palette().color(QPalette::Window);
    return static_cast<uint32_t>((c.red() << 16) | (c.green() << 8) | c.blue());
}
// DPalette::Highlight from the application palette (theme accent color);
// deepin-terminal's split divider is exactly this (termpage.cpp setSplitStyle)
uint32_t app_palette_highlight_rgb() {
    const auto pa = Dtk::Gui::DGuiApplicationHelper::instance()->applicationPalette();
    const QColor c = pa.color(Dtk::Gui::DPalette::Highlight);
    return static_cast<uint32_t>((c.red() << 16) | (c.green() << 8) | c.blue());
}
void widget_activate_window(QWidget *w) { w->activateWindow(); }
void widget_close(QWidget *w) { w->close(); }
bool widget_is_visible(QWidget *w) { return w->isVisible(); }
void widget_set_focus_policy(QWidget *w, int32_t policy) {
    w->setFocusPolicy(static_cast<Qt::FocusPolicy>(policy));
}
void widget_set_font(QWidget *w, QFont *font) { w->setFont(*font); }
QPalette *widget_palette(QWidget *w) { return new QPalette(w->palette()); }
void widget_set_palette(QWidget *w, QPalette *pal) { w->setPalette(*pal); }
void object_delete_later(QObject *o) { o->deleteLater(); }

// ---- QProgressBar common ----
void progressbar_set_value(QWidget *w, int32_t value) {
    static_cast<QProgressBar *>(w)->setValue(value);
}

rust::String line_edit_text(QWidget *w) {
    return to_rust_string(static_cast<QLineEdit *>(w)->text());
}
void progressbar_set_range(QWidget *w, int32_t minimum, int32_t maximum) {
    static_cast<QProgressBar *>(w)->setRange(minimum, maximum);
}
int32_t progressbar_value(QWidget *w) { return static_cast<QProgressBar *>(w)->value(); }

// ---- DMainWindow ----
DMainWindow *mainwindow_new() { return new DMainWindow; }
DMainWindow *mainwindow_new_ex(size_t show_cb_id, size_t close_cb_id) {
    return new DtkMainWindowEx(show_cb_id, close_cb_id);
}
DTitlebar *mainwindow_titlebar(DMainWindow *w) { return w->titlebar(); }
void mainwindow_set_central_widget(DMainWindow *w, QWidget *central) { w->setCentralWidget(central); }
QWidget *mainwindow_take_central_widget(DMainWindow *w) { return w->takeCentralWidget(); }
void mainwindow_set_window_radius(DMainWindow *w, int32_t radius) { w->setWindowRadius(radius); }
void mainwindow_set_enable_blur(DMainWindow *w, bool enable) { w->setEnableBlurWindow(enable); }

// ---- DTitlebar ----
void titlebar_set_title(DTitlebar *tb, rust::Str title) { tb->setTitle(from_rust_str(title)); }
void titlebar_set_icon(DTitlebar *tb, const QIcon &icon) { tb->setIcon(icon); }

// ---- QIcon ----
QIcon *icon_from_theme(rust::Str name) { return new QIcon(QIcon::fromTheme(from_rust_str(name))); }
QIcon *icon_from_theme_fallback(rust::Str name, QIcon *fallback) {
    return new QIcon(QIcon::fromTheme(from_rust_str(name), *fallback));
}
QIcon *icon_from_file(rust::Str path) { return new QIcon(from_rust_str(path)); }

// ---- DLabel ----
DLabel *label_new(rust::Str text) { return new DLabel(from_rust_str(text)); }
void label_set_text(DLabel *l, rust::Str text) { l->setText(from_rust_str(text)); }
void label_set_word_wrap(DLabel *l, bool wrap) { l->setWordWrap(wrap); }
void label_set_alignment(DLabel *l, int32_t alignment) {
    l->setAlignment(Qt::Alignment::fromInt(alignment));
}
void label_set_pixmap(DLabel *l, QPixmap *pm) { l->setPixmap(*pm); }

// ---- buttons ----
DSuggestButton *suggest_button_new(rust::Str text) {
    auto *b = new DSuggestButton;
    b->setText(from_rust_str(text));
    return b;
}
DPushButton *push_button_new(rust::Str text) { return new DPushButton(from_rust_str(text)); }
void button_set_text(DPushButton *b, rust::Str text) { b->setText(from_rust_str(text)); }
void button_click(DPushButton *b) { b->click(); }

// ---- QMessageBox (DMessageBox typedef) ----
QMessageBox *qmessagebox_new() { return new QMessageBox; }
QMessageBox *qmessagebox_new_with(int32_t icon, rust::Str title, rust::Str text, int32_t buttons, QWidget *parent) {
    return new QMessageBox(static_cast<QMessageBox::Icon>(icon), from_rust_str(title), from_rust_str(text),
                           static_cast<QMessageBox::StandardButtons>(buttons), parent);
}
void qmessagebox_set_text(QMessageBox *mb, rust::Str text) { mb->setText(from_rust_str(text)); }
void qmessagebox_set_icon(QMessageBox *mb, int32_t icon) { mb->setIcon(static_cast<QMessageBox::Icon>(icon)); }
void qmessagebox_set_standard_buttons(QMessageBox *mb, int32_t buttons) {
    mb->setStandardButtons(static_cast<QMessageBox::StandardButtons>(buttons));
}
void qmessagebox_set_informative_text(QMessageBox *mb, rust::Str text) { mb->setInformativeText(from_rust_str(text)); }
void qmessagebox_set_detailed_text(QMessageBox *mb, rust::Str text) { mb->setDetailedText(from_rust_str(text)); }
DPushButton *qmessagebox_add_button_text(QMessageBox *mb, rust::Str text, int32_t role) {
    return mb->addButton(from_rust_str(text), static_cast<QMessageBox::ButtonRole>(role));
}
DPushButton *qmessagebox_add_button_standard(QMessageBox *mb, int32_t button) {
    return mb->addButton(static_cast<QMessageBox::StandardButton>(button));
}
void qmessagebox_set_default_button(QMessageBox *mb, int32_t button) {
    mb->setDefaultButton(static_cast<QMessageBox::StandardButton>(button));
}
int32_t qmessagebox_exec(QMessageBox *mb) { return static_cast<int32_t>(mb->exec()); }
int32_t qmessagebox_clicked_button(QMessageBox *mb) {
    return static_cast<int32_t>(mb->standardButton(mb->clickedButton()));
}
rust::String qmessagebox_text(QMessageBox *mb) { return to_rust_string(mb->text()); }
int32_t qmessagebox_information(QWidget *parent, rust::Str title, rust::Str text, int32_t buttons, int32_t default_button) {
    return static_cast<int32_t>(QMessageBox::information(parent, from_rust_str(title), from_rust_str(text),
        static_cast<QMessageBox::StandardButtons>(buttons), static_cast<QMessageBox::StandardButton>(default_button)));
}
int32_t qmessagebox_warning(QWidget *parent, rust::Str title, rust::Str text, int32_t buttons, int32_t default_button) {
    return static_cast<int32_t>(QMessageBox::warning(parent, from_rust_str(title), from_rust_str(text),
        static_cast<QMessageBox::StandardButtons>(buttons), static_cast<QMessageBox::StandardButton>(default_button)));
}
int32_t qmessagebox_critical(QWidget *parent, rust::Str title, rust::Str text, int32_t buttons, int32_t default_button) {
    return static_cast<int32_t>(QMessageBox::critical(parent, from_rust_str(title), from_rust_str(text),
        static_cast<QMessageBox::StandardButtons>(buttons), static_cast<QMessageBox::StandardButton>(default_button)));
}
int32_t qmessagebox_question(QWidget *parent, rust::Str title, rust::Str text, int32_t buttons, int32_t default_button) {
    return static_cast<int32_t>(QMessageBox::question(parent, from_rust_str(title), from_rust_str(text),
        static_cast<QMessageBox::StandardButtons>(buttons), static_cast<QMessageBox::StandardButton>(default_button)));
}
void qmessagebox_about(QWidget *parent, rust::Str title, rust::Str text) {
    QMessageBox::about(parent, from_rust_str(title), from_rust_str(text));
}

// ---- layouts ----
QVBoxLayout *vbox_new(QWidget *parent) { return new QVBoxLayout(parent); }
QHBoxLayout *hbox_new(QWidget *parent) { return new QHBoxLayout(parent); }
QWidget *widget_new(QWidget *parent) { return new QWidget(parent); }
void layout_add_widget(QLayout *l, QWidget *w) { l->addWidget(w); }
void layout_add_widget_ex(QLayout *l, QWidget *w, int32_t stretch, int32_t alignment) {
    // ponytail: stretch/alignment only exist on QBoxLayout; other layouts fall back to plain add
    if (auto *box = qobject_cast<QBoxLayout *>(l))
        box->addWidget(w, stretch, Qt::Alignment::fromInt(alignment));
    else
        l->addWidget(w);
}
void layout_add_stretch(QLayout *l, int32_t stretch) {
    if (auto *box = qobject_cast<QBoxLayout *>(l))
        box->addStretch(stretch);
}
void layout_add_layout(QLayout *l, QLayout *child) {
    // ponytail: QLayout::addItem() leaves child layouts inactive/zero-height; QBoxLayout::addLayout works
    if (auto *box = qobject_cast<QBoxLayout *>(l))
        box->addLayout(child);
    else
        l->addItem(child);
}
void layout_set_spacing(QLayout *l, int32_t spacing) { l->setSpacing(spacing); }
void layout_set_contents_margins(QLayout *l, int32_t l_, int32_t t, int32_t r, int32_t b) {
    l->setContentsMargins(l_, t, r, b);
}

// ---- QTableWidget ----
QTableWidget *table_new(int32_t rows, int32_t cols) { return new QTableWidget(rows, cols); }
void table_set_column_count(QTableWidget *t, int32_t cols) { t->setColumnCount(cols); }
void table_set_row_count(QTableWidget *t, int32_t rows) { t->setRowCount(rows); }
void table_set_horizontal_header_labels(QTableWidget *t, rust::Str joined) {
    t->setHorizontalHeaderLabels(from_rust_str(joined).split(QLatin1Char('|')));
}
void table_set_cell_text(QTableWidget *t, int32_t row, int32_t col, rust::Str text) {
    if (auto *item = t->item(row, col))
        item->setText(from_rust_str(text)); // preserve UserRole, icon, foreground, etc.
    else
        t->setItem(row, col, new QTableWidgetItem(from_rust_str(text)));
}
void table_set_cell_data(QTableWidget *t, int32_t row, int32_t col, int64_t data) {
    if (auto *item = t->item(row, col))
        item->setData(Qt::UserRole, QVariant::fromValue<qint64>(data));
}
int64_t table_cell_data(QTableWidget *t, int32_t row, int32_t col) {
    if (auto *item = t->item(row, col))
        return item->data(Qt::UserRole).toLongLong();
    return 0;
}
rust::String table_cell_text(QTableWidget *t, int32_t row, int32_t col) {
    if (auto *item = t->item(row, col))
        return to_rust_string(item->text());
    return rust::String();
}
int32_t table_current_row(QTableWidget *t) { return t->currentRow(); }
int32_t table_row_count(QTableWidget *t) { return t->rowCount(); }
void table_select_rows_readonly(QTableWidget *t) {
    t->setSelectionBehavior(QAbstractItemView::SelectRows);
    t->setEditTriggers(QAbstractItemView::NoEditTriggers);
    t->setSelectionMode(QAbstractItemView::SingleSelection);
}
void table_header_stretch_last(QTableWidget *t, bool stretch) {
    t->horizontalHeader()->setStretchLastSection(stretch);
}
void table_set_column_width(QTableWidget *t, int32_t col, int32_t width) {
    t->setColumnWidth(col, width);
}

// ---- QTableWidget extras ----
QTableWidgetItem *table_item(QTableWidget *t, int32_t row, int32_t col) { return t->item(row, col); }
void table_select_row(QTableWidget *t, int32_t row) { t->selectRow(row); }
void table_hide_headers(QTableWidget *t, bool horizontal, bool vertical) {
    if (horizontal)
        t->horizontalHeader()->hide();
    if (vertical)
        t->verticalHeader()->hide();
}
void table_set_section_resize_mode(QTableWidget *t, int32_t col, int32_t mode) {
    t->horizontalHeader()->setSectionResizeMode(col, static_cast<QHeaderView::ResizeMode>(mode));
}
void table_set_vertical_header_default_section_size(QTableWidget *t, int32_t size) {
    t->verticalHeader()->setDefaultSectionSize(size);
}
void table_set_show_grid(QTableWidget *t, bool show) { t->setShowGrid(show); }
void table_set_frame_shape(QTableWidget *t, int32_t shape) {
    t->setFrameShape(static_cast<QFrame::Shape>(shape));
}
void table_set_icon_size(QTableWidget *t, int32_t w, int32_t h) { t->setIconSize(QSize(w, h)); }
void table_set_delegate_for_column(QTableWidget *t, int32_t col, QStyledItemDelegate *delegate) {
    t->setItemDelegateForColumn(col, delegate);
}

// ---- QAbstractItemModel ----
QStandardItemModel *standard_item_model_new(QObject *parent) {
    return new QStandardItemModel(parent);
}
void standard_item_model_set_column_count(QStandardItemModel *model, int32_t count) {
    model->setColumnCount(count);
}
QAbstractItemModel *standard_item_model_as_model(QStandardItemModel *model) {
    return static_cast<QAbstractItemModel *>(model);
}
void model_set_parent(QAbstractItemModel *model, QObject *parent) { model->setParent(parent); }
int32_t model_row_count(QAbstractItemModel *model) { return model->rowCount(); }
int32_t model_column_count(QAbstractItemModel *model) { return model->columnCount(); }
QVariant *model_data(QAbstractItemModel *model, int32_t row, int32_t col, int32_t role) {
    const QModelIndex index = model->index(row, col);
    return new QVariant(index.isValid() ? model->data(index, role) : QVariant());
}
bool model_set_data(QAbstractItemModel *model, int32_t row, int32_t col, QVariant *data, int32_t role) {
    const QModelIndex index = model->index(row, col);
    return index.isValid() && model->setData(index, *data, role);
}
bool model_insert_rows(QAbstractItemModel *model, int32_t row, int32_t count) {
    return model->insertRows(row, count);
}
bool model_remove_rows(QAbstractItemModel *model, int32_t row, int32_t count) {
    return model->removeRows(row, count);
}

// ---- QTableWidgetItem ----
void item_set_icon(QTableWidgetItem *it, QIcon *icon) { it->setIcon(*icon); }
void item_set_text_alignment(QTableWidgetItem *it, int32_t alignment) {
    it->setTextAlignment(Qt::Alignment::fromInt(alignment));
}
void item_set_foreground(QTableWidgetItem *it, QColor *color) { it->setForeground(*color); }
void item_set_data_string(QTableWidgetItem *it, int32_t role, rust::Str value) {
    it->setData(role, from_rust_str(value));
}
rust::String item_data_string(QTableWidgetItem *it, int32_t role) {
    return to_rust_string(it->data(role).toString());
}
void item_set_data_bool(QTableWidgetItem *it, int32_t role, bool value) { it->setData(role, value); }
bool item_data_bool(QTableWidgetItem *it, int32_t role) { return it->data(role).toBool(); }

// ---- value types ----
QVariant *variant_new() { return new QVariant; }
QVariant *variant_from_bool(bool value) { return new QVariant(value); }
QVariant *variant_from_i64(int64_t value) {
    return new QVariant(QVariant::fromValue<qint64>(value));
}
QVariant *variant_from_f64(double value) { return new QVariant(value); }
QVariant *variant_from_string(rust::Str value) { return new QVariant(from_rust_str(value)); }
QVariant *variant_clone(QVariant *value) { return new QVariant(*value); }
bool variant_is_valid(QVariant *value) { return value->isValid(); }
bool variant_to_bool(QVariant *value) { return value->toBool(); }
int64_t variant_to_i64(QVariant *value, bool *ok) { return value->toLongLong(ok); }
rust::String variant_to_string(QVariant *value) { return to_rust_string(value->toString()); }
void variant_delete(QVariant *value) { delete value; }
QVariantList *variant_list_new() { return new QVariantList; }
QVariantList *variant_list_clone(QVariantList *list) { return new QVariantList(*list); }
void variant_list_push(QVariantList *list, QVariant *value) { list->append(*value); }
size_t variant_list_len(QVariantList *list) { return static_cast<size_t>(list->size()); }
QVariant *variant_list_get(QVariantList *list, size_t index) {
    if (index >= static_cast<size_t>(list->size()))
        return nullptr;
    return new QVariant(list->at(static_cast<qsizetype>(index)));
}
void variant_list_delete(QVariantList *list) { delete list; }
QColor *color_new_rgb(int32_t r, int32_t g, int32_t b, int32_t a) { return new QColor(r, g, b, a); }
int32_t color_rgba(QColor *c) { return static_cast<int32_t>(c->rgba()); }
void color_delete(QColor *c) { delete c; }
QFont *font_new() { return new QFont; }
void font_set_point_size(QFont *f, int32_t size) { f->setPointSize(size); }
void font_set_bold(QFont *f, bool bold) { f->setBold(bold); }
void font_set_italic(QFont *f, bool italic) { f->setItalic(italic); }

int32_t fontmetrics_height(QFont *f) { return QFontMetrics(*f).height(); }
int32_t fontmetrics_ascent(QFont *f) { return QFontMetrics(*f).ascent(); }
int32_t fontmetrics_max_width(QFont *f) { return QFontMetrics(*f).horizontalAdvance(QLatin1Char('M')); }
int32_t fontmetrics_advance(QFont *f, rust::Str text) { return QFontMetrics(*f).horizontalAdvance(from_rust_str(text)); }
void font_force_integer_metrics(QFont *f) {
    // Qt6 removed QFont::ForceIntegerMetrics; full hinting snaps advances to
    // integer pixels, which is what a cell grid needs
    f->setHintingPreference(QFont::PreferFullHinting);
}

void font_set_monospace(QFont *f) {
    f->setStyleHint(QFont::TypeWriter);
    f->setFamily(QStringLiteral("monospace"));
}
void font_set_family(QFont *f, rust::Str name) { f->setFamily(from_rust_str(name)); }
void font_delete(QFont *f) { delete f; }
QPalette *palette_new() { return new QPalette; }
void palette_set_color(QPalette *pal, int32_t group, int32_t role, QColor *color) {
    pal->setColor(static_cast<QPalette::ColorGroup>(group), static_cast<QPalette::ColorRole>(role), *color);
}
QColor *palette_color(QPalette *pal, int32_t group, int32_t role) {
    return new QColor(pal->color(static_cast<QPalette::ColorGroup>(group),
                                 static_cast<QPalette::ColorRole>(role)));
}
void palette_delete(QPalette *pal) { delete pal; }
QPixmap *pixmap_new(rust::Str path) { return new QPixmap(from_rust_str(path)); }
void pixmap_delete(QPixmap *pm) { delete pm; }

// ---- QMargins ----
QMargins *q_margins_new(int32_t left, int32_t top, int32_t right, int32_t bottom) {
    return new QMargins(left, top, right, bottom);
}
void margins_delete(QMargins *m) { delete m; }

// ---- DDciIcon (dtkgui) ----
DDciIcon *ddci_icon_new() { return new DDciIcon; }
DDciIcon *ddci_icon_from_file(rust::Str path) {
    return new DDciIcon(QString::fromUtf8(path.data(), static_cast<qsizetype>(path.size())));
}
void ddci_icon_delete(DDciIcon *i) { delete i; }
QPixmap *standard_icon_pixmap(QWidget *w, int32_t icon, int32_t size) {
    return new QPixmap(w->style()->standardIcon(static_cast<QStyle::StandardPixmap>(icon)).pixmap(size, size));
}
QSize *size_new(int32_t w, int32_t h) { return new QSize(w, h); }
void size_delete(QSize *s) { delete s; }
QPoint *point_new(int32_t x, int32_t y) { return new QPoint(x, y); }
void point_delete(QPoint *p) { delete p; }
QRect *rect_new(int32_t x, int32_t y, int32_t w, int32_t h) { return new QRect(x, y, w, h); }
void rect_delete(QRect *r) { delete r; }
int32_t rect_width(QRect *r) { return r->width(); }
int32_t rect_height(QRect *r) { return r->height(); }
int32_t rect_x(QRect *r) { return r->x(); }
int32_t rect_y(QRect *r) { return r->y(); }
void icon_delete(QIcon *icon) { delete icon; }

// ---- QSocketNotifier ----
QSocketNotifier *socket_notifier_new(int32_t fd, QObject *parent) {
    return new QSocketNotifier(fd, QSocketNotifier::Read, parent);
}
void socket_notifier_set_enabled(QSocketNotifier *n, bool on) { n->setEnabled(on); }

// ---- generic paint delegate ----
QStyledItemDelegate *rust_delegate_new(size_t paint_cb_id, QObject *parent) {
    return new RustDelegate(paint_cb_id, parent);
}

// ---- QPainter primitives ----
void painter_draw_line(QPainter *p, int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
    p->drawLine(x1, y1, x2, y2);
}
void painter_draw_text_at(QPainter *p, int32_t x, int32_t y, rust::Str text) {
    p->drawText(x, y, from_rust_str(text));
}

void painter_save(QPainter *p) { p->save(); }
void painter_restore(QPainter *p) { p->restore(); }
void painter_set_pen_color(QPainter *p, QColor *color) { p->setPen(*color); }
void painter_set_font(QPainter *p, QFont *font) { p->setFont(*font); }
void painter_draw_text(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, int32_t flags,
                       rust::Str text) {
    p->drawText(QRect(x, y, w, h), flags, from_rust_str(text));
}
void painter_draw_pixmap(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QPixmap *pm) {
    p->drawPixmap(QRect(x, y, w, h), *pm);
}
void painter_draw_icon(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QIcon *icon) {
    icon->paint(p, QRect(x, y, w, h));
}
void painter_fill_rect(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QColor *color) {
    p->fillRect(QRect(x, y, w, h), *color);
}
void painter_set_clip_rect(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h) {
    p->setClipRect(x, y, w, h);
}
void painter_translate(QPainter *p, double x, double y) { p->translate(x, y); }
void painter_scale(QPainter *p, double sx, double sy) { p->scale(sx, sy); }
rust::String painter_elided_text(QPainter *p, rust::Str text, int32_t mode, int32_t width) {
    return to_rust_string(
        p->fontMetrics().elidedText(from_rust_str(text), static_cast<Qt::TextElideMode>(mode), width));
}

// ---- QModelIndex data access ----
rust::String index_data_string(QModelIndex *idx, int32_t role) {
    return to_rust_string(idx->data(role).toString());
}
bool index_data_bool(QModelIndex *idx, int32_t role) { return idx->data(role).toBool(); }
int64_t index_data_i64(QModelIndex *idx, int32_t role) {
    return idx->data(role).toLongLong();
}

// ---- QTimer ----
QTimer *timer_new(QObject *parent) { return new QTimer(parent); }
void timer_start(QTimer *t, int32_t msec) { t->start(msec); }
void timer_stop(QTimer *t) { t->stop(); }
void timer_single_shot(int32_t msec, size_t cb_id) {
    auto *app = QCoreApplication::instance();
    if (!app) {
        dtk_cb_dispose(cb_id); // no event loop to deliver the timer
        return;
    }
    // Own the timer explicitly: Qt cancels it when the application is destroyed,
    // and its destroyed signal also releases an unfired Rust callback.
    auto *timer = new QTimer(app);
    timer->setSingleShot(true);
    dispose_on_destroy(timer, cb_id);
    QObject::connect(timer, &QTimer::timeout, timer, [timer, cb_id] {
        dtk_cb0(cb_id);
        timer->deleteLater();
    });
    timer->start(msec);
}

// ---- DtkPaintWidget factory + clipboard + shortcuts ----
QWidget *paint_widget_new(size_t cb_id, QWidget *parent) {
    return new DtkPaintWidget(cb_id, parent);
}

void paint_widget_inject_key(QWidget *w, int32_t key, int32_t mods, rust::Str text) {
    QKeyEvent ev(QEvent::KeyPress, key, static_cast<Qt::KeyboardModifiers>(mods),
                 from_rust_str(text));
    QCoreApplication::sendEvent(w, &ev);
}

void clipboard_set_text(rust::Str text, int32_t mode) {
    QGuiApplication::clipboard()->setText(from_rust_str(text),
                                          mode == 1 ? QClipboard::Selection : QClipboard::Clipboard);
}

rust::String clipboard_text(int32_t mode) {
    return to_rust_string(QGuiApplication::clipboard()->text(
        mode == 1 ? QClipboard::Selection : QClipboard::Clipboard));
}

void shortcut_new(QWidget *parent, rust::Str key, size_t cb_id) {
    auto *sc = new QShortcut(QKeySequence(from_rust_str(key)), parent);
    dispose_on_destroy(sc, cb_id);
    QObject::connect(sc, &QShortcut::activated, sc, [cb_id] { dtk_cb0(cb_id); });
}

// ---- signal callbacks ----
bool relay_connect0(QObject *sender, rust::Str signal, size_t cb_id) {
    return DtkRelay::connect0(sender, std::string(signal).c_str(), cb_id);
}

bool relay_connect_i32(QObject *sender, rust::Str signal, size_t cb_id) {
    return DtkRelay::connectI32(sender, std::string(signal).c_str(), cb_id);
}

bool relay_connect_bool(QObject *sender, rust::Str signal, size_t cb_id) {
    return DtkRelay::connectBool(sender, std::string(signal).c_str(), cb_id);
}

bool relay_connect_i32_i32(QObject *sender, rust::Str signal, size_t cb_id) {
    return DtkRelay::connectI32I32(sender, std::string(signal).c_str(), cb_id);
}

void relay_disconnect(size_t cb_id) { DtkRelay::disconnectId(cb_id); }

void widget_set_cursor(QWidget *w, int32_t shape) {
    w->setCursor(static_cast<Qt::CursorShape>(shape));
}
void widget_unset_cursor(QWidget *w) { w->unsetCursor(); }
} // namespace dtkrs
