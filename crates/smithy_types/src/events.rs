use web_sys::{
  AnimationEvent,
  BeforeUnloadEvent,
  ClipboardEvent,
  FocusEvent,
  HashChangeEvent,
  InputEvent,
  KeyboardEvent,
  MouseEvent,
  PointerEvent,
  PopStateEvent,
  PromiseRejectionEvent,
  ScrollAreaEvent,
  TouchEvent,
  TransitionEvent,
  UiEvent as WebSysUiEvent,
};

// TODO custom_derive iter_variant_names
// or https://github.com/Lolirofle/enum_traits

/// An enum of all Events a dom node can handle
pub enum UiEvent {
  // TODO figure out why cfg(test) does not work here
  OnTest(bool),
  // --Clipboard
  OnCopy(ClipboardEvent),
  OnCut(ClipboardEvent),
  OnPaste(ClipboardEvent),
  // --Composition
  // onCompositionEnd
  // onCompositionStart
  // onCompositionUpdate
  // --Keyboard
  OnKeyDown(KeyboardEvent),
  OnKeyPress(KeyboardEvent),
  OnKeyUp(KeyboardEvent),
  // --Focus
  OnFocus(FocusEvent),
  OnBlur(FocusEvent),
  // --Form
  OnChange(InputEvent),
  OnInput(InputEvent),
  OnInvalid(InputEvent),
  OnSubmit(InputEvent),
  // --Mouse
  OnClick(MouseEvent),
  OnContextMenu(MouseEvent),
  OnDblClick(MouseEvent),

  OnDrag(MouseEvent),
  OnDragEnd(MouseEvent),
  OnDragEnter(MouseEvent),
  OnDragExit(MouseEvent),
  OnDragLeave(MouseEvent),
  OnDragOver(MouseEvent),
  OnDragStart(MouseEvent),
  OnDrop(MouseEvent),

  OnMouseDown(MouseEvent),
  OnMouseEnter(MouseEvent),
  OnMouseLeave(MouseEvent),
  OnMouseMove(MouseEvent),
  OnMouseOver(MouseEvent),
  OnMouseOut(MouseEvent),
  OnMouseUp(MouseEvent),
  // --Pointer
  OnPointerDown(PointerEvent),
  OnPointerMove(PointerEvent),
  OnPointerUp(PointerEvent),
  OnPointerCancel(PointerEvent),
  OnGotPointerCapture(PointerEvent),
  OnLostPointerCapture(PointerEvent),
  OnPointerEnter(PointerEvent),
  OnPointerLeave(PointerEvent),
  OnPointerOver(PointerEvent),
  OnPointerOut(PointerEvent),
  // --Selection
  OnSelect(WebSysUiEvent),
  // --Touch
  OnTouchCancel(TouchEvent),
  OnTouchEnd(TouchEvent),
  OnTouchMove(TouchEvent),
  OnTouchStart(TouchEvent),
  // --Scroll
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
  OnLoad(WebSysUiEvent),
  OnError(WebSysUiEvent),
  // --Animation
  OnAnimationStart(AnimationEvent),
  OnAnimationEnd(AnimationEvent),
  OnAnimationIteration(AnimationEvent),
  // --Transition
  OnTransitionEnd(TransitionEvent),
  // --Other
  OnToggle(WebSysUiEvent),
}

pub enum WindowEvent {
  OnBeforeUnload(BeforeUnloadEvent),
  OnHashChange(HashChangeEvent),
  OnPopState(PopStateEvent),
  OnUnhandledRejection(PromiseRejectionEvent),
}
