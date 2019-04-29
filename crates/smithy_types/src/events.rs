// TODO custom_derive iter_variant_names
// or https://github.com/Lolirofle/enum_traits

/// An enum of events that a DOM element can potentially
/// handle.
pub enum UiEvent {
  // TODO figure out why cfg(test) does not work here
  OnTest(bool),
  // --Clipboard
  #[cfg(feature = "clipboard-events")]
  OnCopy(web_sys::ClipboardEvent),
  #[cfg(feature = "clipboard-events")]
  OnCut(web_sys::ClipboardEvent),
  #[cfg(feature = "clipboard-events")]
  OnPaste(web_sys::ClipboardEvent),
  // --Composition
  // onCompositionEnd
  // onCompositionStart
  // onCompositionUpdate
  // --Keyboard
  #[cfg(feature = "keyboard-events")]
  OnKeyDown(web_sys::KeyboardEvent),
  #[cfg(feature = "keyboard-events")]
  OnKeyPress(web_sys::KeyboardEvent),
  #[cfg(feature = "keyboard-events")]
  OnKeyUp(web_sys::KeyboardEvent),
  // --Focus
  #[cfg(feature = "focus-events")]
  OnFocus(web_sys::FocusEvent),
  #[cfg(feature = "focus-events")]
  OnBlur(web_sys::FocusEvent),
  // --Form
  #[cfg(feature = "input-events")]
  OnChange(web_sys::InputEvent),
  #[cfg(feature = "input-events")]
  OnInput(web_sys::InputEvent),
  #[cfg(feature = "input-events")]
  OnInvalid(web_sys::InputEvent),
  #[cfg(feature = "input-events")]
  OnSubmit(web_sys::InputEvent),
  // --Mouse
  #[cfg(feature = "mouse-events")]
  OnClick(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnContextMenu(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDblClick(web_sys::MouseEvent),

  #[cfg(feature = "mouse-events")]
  OnDrag(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragEnd(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragEnter(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragExit(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragLeave(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragOver(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragStart(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDrop(web_sys::MouseEvent),

  #[cfg(feature = "mouse-events")]
  OnMouseDown(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseEnter(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseLeave(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseMove(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseOver(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseOut(web_sys::MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseUp(web_sys::MouseEvent),
  // --Pointer
  #[cfg(feature = "pointer-events")]
  OnPointerDown(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerMove(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerUp(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerCancel(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnGotPointerCapture(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnLostPointerCapture(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerEnter(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerLeave(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerOver(web_sys::PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerOut(web_sys::PointerEvent),
  // --Selection
  #[cfg(feature = "select-events")]
  OnSelect(web_sys::WebSysUiEvent),
  // --Touch
  #[cfg(feature = "touch-events")]
  OnTouchCancel(web_sys::TouchEvent),
  #[cfg(feature = "touch-events")]
  OnTouchEnd(web_sys::TouchEvent),
  #[cfg(feature = "touch-events")]
  OnTouchMove(web_sys::TouchEvent),
  #[cfg(feature = "touch-events")]
  OnTouchStart(web_sys::TouchEvent),
  // --Scroll
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
  #[cfg(feature = "image-events")]
  OnLoad(web_sys::WebSysUiEvent),
  #[cfg(feature = "image-events")]
  OnError(web_sys::WebSysUiEvent),
  // --Animation
  #[cfg(feature = "animation-events")]
  OnAnimationStart(web_sys::AnimationEvent),
  #[cfg(feature = "animation-events")]
  OnAnimationEnd(web_sys::AnimationEvent),
  #[cfg(feature = "animation-events")]
  OnAnimationIteration(web_sys::AnimationEvent),
  // --Transition
  #[cfg(feature = "transition-events")]
  OnTransitionEnd(web_sys::TransitionEvent),
  // --Other
  #[cfg(feature = "toggle-events")]
  OnToggle(web_sys::WebSysUiEvent),
}

/// An enum representing global events that can occur and that a smithy
/// app can potentially handle.
pub enum WindowEvent {
  #[cfg(feature = "before-unload-events")]
  OnBeforeUnload(web_sys::BeforeUnloadEvent),
  #[cfg(feature = "hash-change-events")]
  OnHashChange(web_sys::HashChangeEvent),
  #[cfg(feature = "pop-state-events")]
  OnPopState(web_sys::PopStateEvent),
  #[cfg(feature = "promise-rejection-events")]
  OnUnhandledRejection(web_sys::PromiseRejectionEvent),
}
