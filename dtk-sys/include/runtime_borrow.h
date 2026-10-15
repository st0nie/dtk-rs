// Runtime transactions and synchronous callback borrows, not QObject ownership.
#pragma once
#include <QObject>
#include <QEvent>
#include <memory>

namespace dtkrs {

// Rust holds this until returned native pointers have become guarded/owned views.
// It never constructs a C++/Rust exclusive reference from a shared QObject handle.
class NativeCallFrame {
public:
    NativeCallFrame();
    ~NativeCallFrame();
    int loop_level;
    bool registered;
};

// Only painter/return-value callbacks are synchronous. Safe QObject access and
// nested loops are forbidden while their borrowed native frame is on the stack.
class SynchronousBorrow {
public:
    SynchronousBorrow();
    ~SynchronousBorrow();
};

std::unique_ptr<NativeCallFrame> runtime_call_new(bool control);
bool runtime_call_allowed(bool control);
std::unique_ptr<NativeCallFrame> runtime_paint_call_new();
bool runtime_native_borrow_active();
bool runtime_can_dispatch_callbacks();
bool runtime_can_finish_call();
bool runtime_can_close(QObject *object);
void runtime_schedule_callbacks();
void runtime_borrow_begin_scope(QObject *application);
void runtime_borrow_end_scope();
// Interpose on main-thread DeferredDelete delivery in BOTH application classes.
// Clones preserve Qt's original loop/scope level and deleteLater de-bounce state.
bool runtime_hold_deferred_delete(QObject *receiver, QEvent *event);
bool runtime_is_callback_event(QObject *receiver, QEvent *event);

} // namespace dtkrs
