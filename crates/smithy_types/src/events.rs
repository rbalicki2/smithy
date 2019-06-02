// TODO custom_derive iter_variant_names
// or https://github.com/Lolirofle/enum_traits

/// An enum of events that a DOM element can potentially
/// handle.
///
/// These are included on dom elements as follows:
/// ```rs
/// smd!(
///   <some-dom-node ui_event={ui_event_handler} />
/// )
/// ```
///
/// e.g.
/// ```rs
/// smd!(
///   <div on_click={|e: web_sys::MouseEvent| { /* ... */ }} />
/// )
/// ```
pub enum UiEvent {
  // --Clipboard
  /// Usage:
  /// ```rs
  /// on_copy={|e: web_sys::ClipboardEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `clipboard-events` feature.
  #[cfg(feature = "clipboard-events")]
  OnCopy(web_sys::ClipboardEvent),
  /// Usage:
  /// ```rs
  /// on_cut={|e: web_sys::ClipboardEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `clipboard-events` feature.
  #[cfg(feature = "clipboard-events")]
  OnCut(web_sys::ClipboardEvent),
  /// Usage:
  /// ```rs
  /// on_paste={|e: web_sys::ClipboardEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `clipboard-events` feature.
  #[cfg(feature = "clipboard-events")]
  OnPaste(web_sys::ClipboardEvent),
  // --Composition
  // onCompositionEnd
  // onCompositionStart
  // onCompositionUpdate
  // --Keyboard
  /// Usage:
  /// ```rs
  /// on_key_down={|e: web_sys::KeyboardEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `keyboard-events` feature.
  #[cfg(feature = "keyboard-events")]
  OnKeyDown(web_sys::KeyboardEvent),
  /// Usage:
  /// ```rs
  /// on_key_press={|e: web_sys::KeyboardEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `keyboard-events` feature.
  #[cfg(feature = "keyboard-events")]
  OnKeyPress(web_sys::KeyboardEvent),
  /// Usage:
  /// ```rs
  /// on_key_up={|e: web_sys::KeyboardEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `keyboard-events` feature.
  #[cfg(feature = "keyboard-events")]
  OnKeyUp(web_sys::KeyboardEvent),
  // --Focus
  /// Usage:
  /// ```rs
  /// on_focus={|e: web_sys::FocusEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `focus-events` feature.
  #[cfg(feature = "focus-events")]
  OnFocus(web_sys::FocusEvent),
  /// Usage:
  /// ```rs
  /// on_blur={|e: web_sys::FocusEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `focus-events` feature.
  #[cfg(feature = "focus-events")]
  OnBlur(web_sys::FocusEvent),
  // --Form
  /// Usage:
  /// ```rs
  /// on_change={|e: web_sys::InputEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `input-events` feature.
  #[cfg(feature = "input-events")]
  OnChange(web_sys::InputEvent),
  /// Usage:
  /// ```rs
  /// on_input={|e: web_sys::InputEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `input-events` feature.
  #[cfg(feature = "input-events")]
  OnInput(web_sys::InputEvent),
  /// Usage:
  /// ```rs
  /// on_invalid={|e: web_sys::InputEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `input-events` feature.
  #[cfg(feature = "input-events")]
  OnInvalid(web_sys::InputEvent),
  /// Usage:
  /// ```rs
  /// on_submit={|e: web_sys::InputEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `input-events` feature.
  #[cfg(feature = "input-events")]
  OnSubmit(web_sys::InputEvent),
  // --Mouse
  /// Usage:
  /// ```rs
  /// on_click={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnClick(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_context_menu={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnContextMenu(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_dbl_cilck={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDblClick(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_drag={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDrag(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_drag_end={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDragEnd(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_drag_enter={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDragEnter(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_drag_exit={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDragExit(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_drag_leave={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDragLeave(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_drag_over={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDragOver(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_drag_start={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDragStart(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_drop={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnDrop(web_sys::MouseEvent),

  /// Usage:
  /// ```rs
  /// on_mouse_down={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnMouseDown(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_mouse_enter={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnMouseEnter(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_mouse_leave={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnMouseLeave(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_move_move={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnMouseMove(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_mouse_over={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnMouseOver(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_mouse_out={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnMouseOut(web_sys::MouseEvent),
  /// Usage:
  /// ```rs
  /// on_mouse_up={|e: web_sys::MouseEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `mouse-events` feature.
  #[cfg(feature = "mouse-events")]
  OnMouseUp(web_sys::MouseEvent),
  // --Pointer
  /// Usage:
  /// ```rs
  /// on_pointer_down={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnPointerDown(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_pointer_move={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnPointerMove(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_pointer_up={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnPointerUp(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_pointer_cancel={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnPointerCancel(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_got_pointer_capture={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnGotPointerCapture(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_lost_pointer_capture={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnLostPointerCapture(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_pointer_enter={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnPointerEnter(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_pointer_leave={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnPointerLeave(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_pointer_over={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnPointerOver(web_sys::PointerEvent),
  /// Usage:
  /// ```rs
  /// on_pointer_out={|e: web_sys::PointerEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `pointer-events` feature.
  #[cfg(feature = "pointer-events")]
  OnPointerOut(web_sys::PointerEvent),
  // --Selection
  /// Usage:
  /// ```rs
  /// on_select={|e: web_sys::UiEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `select-events` feature.
  #[cfg(feature = "select-events")]
  OnSelect(web_sys::UiEvent),
  // --Touch
  /// Usage:
  /// ```rs
  /// on_touch_cancel={|e: web_sys::TouchEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `touch-events` feature.
  #[cfg(feature = "touch-events")]
  OnTouchCancel(web_sys::TouchEvent),
  /// Usage:
  /// ```rs
  /// on_touch_end={|e: web_sys::TouchEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `touch-events` feature.
  #[cfg(feature = "touch-events")]
  OnTouchEnd(web_sys::TouchEvent),
  /// Usage:
  /// ```rs
  /// on_touch_move={|e: web_sys::TouchEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `touch-events` feature.
  #[cfg(feature = "touch-events")]
  OnTouchMove(web_sys::TouchEvent),
  /// Usage:
  /// ```rs
  /// on_touch_start={|e: web_sys::TouchEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `touch-events` feature.
  #[cfg(feature = "touch-events")]
  OnTouchStart(web_sys::TouchEvent),
  // --Scroll
  /// Usage:
  /// ```rs
  /// on_scroll={|e: web_sys::ScrollAreaEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `scroll-events` feature.
  #[cfg(feature = "scroll-events")]
  OnScroll(web_sys::ScrollAreaEvent),
  // --Wheel
  // onWheel
  // --Media
  // onAbort
  // onCanPlay
  // onCanPlayThrough
  // onDurationChange
  // onEmptied
  // onEncrypted
  // onEnded
  // onError
  // onLoadedData
  // onLoadedMetadata
  // onLoadStart
  // onPause
  // onPlay
  // onPlaying
  // onProgress
  // onRateChange
  // onSeeked
  // onSeeking
  // onStalled
  // onSuspend
  // onTimeUpdate
  // onVolumeChange
  // onWaiting
  // --Image
  /// Usage:
  /// ```rs
  /// on_load={|e: web_sys::UiEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `image-events` feature.
  #[cfg(feature = "image-events")]
  OnLoad(web_sys::UiEvent),
  /// Usage:
  /// ```rs
  /// on_error={|e: web_sys::UiEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `image-events` feature.
  #[cfg(feature = "image-events")]
  OnError(web_sys::UiEvent),
  // --Animation
  /// Usage:
  /// ```rs
  /// on_animation_start={|e: web_sys::AnimationEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `animation-events` feature.
  #[cfg(feature = "animation-events")]
  OnAnimationStart(web_sys::AnimationEvent),
  /// Usage:
  /// ```rs
  /// on_animation_end={|e: web_sys::AnimationEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `animation-events` feature.
  #[cfg(feature = "animation-events")]
  OnAnimationEnd(web_sys::AnimationEvent),
  /// Usage:
  /// ```rs
  /// on_animation_iteration={|e: web_sys::AnimationEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `animation-events` feature.
  #[cfg(feature = "animation-events")]
  OnAnimationIteration(web_sys::AnimationEvent),
  // --Transition
  /// Usage:
  /// ```rs
  /// on_transition_end={|e: web_sys::TransitionEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `transition-events` feature.
  #[cfg(feature = "transition-events")]
  OnTransitionEnd(web_sys::TransitionEvent),
  // --Other
  /// Usage:
  /// ```rs
  /// on_toggle={|e: web_sys::UiEvent| { /* ... */ }}
  /// ```
  ///
  /// Requires the `toggle-events` feature.
  #[cfg(feature = "toggle-events")]
  OnToggle(web_sys::UiEvent),
  // TODO figure out why cfg(test) does not work here
  /// An event used for testing.
  OnTest(bool),
}

/// An enum representing global events that can occur and that a smithy
/// app can potentially handle.
///
/// Window events are included as part of the `smd!` syntax as follows:
/// ```rs
/// smd!(
///   window_event={window_event_handler};
/// )
/// ```
///
/// e.g.
/// ```rs
/// smd!(
///   on_hash_change={|e: web_sys::HashChangeEvent| { /* ... */ }};
/// )
/// ```
pub enum WindowEvent {
  /// Usage:
  /// ```rs
  /// on_before_unload={|e: web_sys::BeforeUnloadEvent| { /* ... */ }};
  /// ```
  ///
  /// Requires the `before-unload-events` feature.
  #[cfg(feature = "before-unload-events")]
  OnBeforeUnload(web_sys::BeforeUnloadEvent),
  /// Usage:
  /// ```rs
  /// on_hash_change={|e: web_sys::HashChangeEvent| { /* ... */ }};
  /// ```
  ///
  /// Requires the `hash-change-events` feature.
  #[cfg(feature = "hash-change-events")]
  OnHashChange(web_sys::HashChangeEvent),
  /// Usage:
  /// ```rs
  /// on_pop_state={|e: web_sys::PopStateEvent| { /* ... */ }};
  /// ```
  ///
  /// Requires the `pop-state-events` feature.
  #[cfg(feature = "pop-state-events")]
  OnPopState(web_sys::PopStateEvent),
  /// Usage:
  /// ```rs
  /// on_unhandled_rejection={|e: web_sys::PromiseRejectionEvent| { /* ... */ }};
  /// ```
  ///
  /// Requires the `promise-rejection-events` feature.
  #[cfg(feature = "promise-rejection-events")]
  OnUnhandledRejection(web_sys::PromiseRejectionEvent),
}
