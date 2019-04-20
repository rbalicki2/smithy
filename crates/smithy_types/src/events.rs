#[cfg(feature = "animation-events")]
use web_sys::AnimationEvent;

#[cfg(feature = "clipboard-events")]
use web_sys::ClipboardEvent;

#[cfg(feature = "focus-events")]
use web_sys::FocusEvent;

#[cfg(feature = "keyboard-events")]
use web_sys::KeyboardEvent;

#[cfg(feature = "mouse-events")]
use web_sys::MouseEvent;

#[cfg(feature = "pointer-events")]
use web_sys::PointerEvent;

#[cfg(feature = "scroll-events")]
use web_sys::ScrollAreaEvent;

#[cfg(feature = "touch-events")]
use web_sys::TouchEvent;

#[cfg(feature = "transition-events")]
use web_sys::TransitionEvent;

#[cfg(feature = "input-events")]
use web_sys::InputEvent;

#[cfg(feature = "web-sys-ui-events")]
use web_sys::UiEvent as WebSysUiEvent;

use web_sys::{
  BeforeUnloadEvent,
  HashChangeEvent,
  PopStateEvent,
  PromiseRejectionEvent,
};

// TODO custom_derive iter_variant_names
// or https://github.com/Lolirofle/enum_traits

/// An enum of events that a DOM element can potentially
/// handle.
pub enum UiEvent {
  // TODO figure out why cfg(test) does not work here
  OnTest(bool),
  // --Clipboard
  #[cfg(feature = "clipboard-events")]
  OnCopy(ClipboardEvent),
  #[cfg(feature = "clipboard-events")]
  OnCut(ClipboardEvent),
  #[cfg(feature = "clipboard-events")]
  OnPaste(ClipboardEvent),
  // --Composition
  // onCompositionEnd
  // onCompositionStart
  // onCompositionUpdate
  // --Keyboard
  #[cfg(feature = "keyboard-events")]
  OnKeyDown(KeyboardEvent),
  #[cfg(feature = "keyboard-events")]
  OnKeyPress(KeyboardEvent),
  #[cfg(feature = "keyboard-events")]
  OnKeyUp(KeyboardEvent),
  // --Focus
  #[cfg(feature = "focus-events")]
  OnFocus(FocusEvent),
  #[cfg(feature = "focus-events")]
  OnBlur(FocusEvent),
  // --Form
  #[cfg(feature = "input-events")]
  OnChange(InputEvent),
  #[cfg(feature = "input-events")]
  OnInput(InputEvent),
  #[cfg(feature = "input-events")]
  OnInvalid(InputEvent),
  #[cfg(feature = "input-events")]
  OnSubmit(InputEvent),
  // --Mouse
  #[cfg(feature = "mouse-events")]
  OnClick(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnContextMenu(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDblClick(MouseEvent),

  #[cfg(feature = "mouse-events")]
  OnDrag(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragEnd(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragEnter(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragExit(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragLeave(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragOver(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDragStart(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnDrop(MouseEvent),

  #[cfg(feature = "mouse-events")]
  OnMouseDown(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseEnter(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseLeave(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseMove(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseOver(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseOut(MouseEvent),
  #[cfg(feature = "mouse-events")]
  OnMouseUp(MouseEvent),
  // --Pointer
  #[cfg(feature = "pointer-events")]
  OnPointerDown(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerMove(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerUp(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerCancel(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnGotPointerCapture(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnLostPointerCapture(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerEnter(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerLeave(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerOver(PointerEvent),
  #[cfg(feature = "pointer-events")]
  OnPointerOut(PointerEvent),
  // --Selection
  #[cfg(feature = "select-events")]
  OnSelect(WebSysUiEvent),
  // --Touch
  #[cfg(feature = "touch-events")]
  OnTouchCancel(TouchEvent),
  #[cfg(feature = "touch-events")]
  OnTouchEnd(TouchEvent),
  #[cfg(feature = "touch-events")]
  OnTouchMove(TouchEvent),
  #[cfg(feature = "touch-events")]
  OnTouchStart(TouchEvent),
  // --Scroll
  #[cfg(feature = "scroll-events")]
  OnScroll(ScrollAreaEvent),
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
  OnLoad(WebSysUiEvent),
  #[cfg(feature = "image-events")]
  OnError(WebSysUiEvent),
  // --Animation
  #[cfg(feature = "animation-events")]
  OnAnimationStart(AnimationEvent),
  #[cfg(feature = "animation-events")]
  OnAnimationEnd(AnimationEvent),
  #[cfg(feature = "animation-events")]
  OnAnimationIteration(AnimationEvent),
  // --Transition
  #[cfg(feature = "transition-events")]
  OnTransitionEnd(TransitionEvent),
  // --Other
  #[cfg(feature = "toggle-events")]
  OnToggle(WebSysUiEvent),
}

/// An enum representing global events that can occur and that a smithy
/// app can potentially handle.
pub enum WindowEvent {
  #[cfg(feature = "before-unload-events")]
  OnBeforeUnload(BeforeUnloadEvent),
  #[cfg(feature = "hash-change-events")]
  OnHashChange(HashChangeEvent),
  #[cfg(feature = "pop-state-events")]
  OnPopState(PopStateEvent),
  #[cfg(feature = "promise-rejection-events")]
  OnUnhandledRejection(PromiseRejectionEvent),
}
