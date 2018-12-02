use lazy_static::lazy_static;
use std::collections::HashMap;
// see attributes.rs for an explanation

lazy_static! {
  pub static ref EVENT_NAMES: HashMap<String, String> = {
    // Remember to update this as you add more!
    let mut x = HashMap::with_capacity(54);

    // TODO make sure this gets deleted or goes behind a #[cfg(test)]
    x.insert("OnTest".into(), "OnTest".into());
    // --Clipboard
    x.insert("on_copy".into(), "OnCopy".into());
    x.insert("on_cut".into(), "OnCut".into());
    x.insert("on_paste".into(), "OnPaste".into());
    // --Composition
    // onCompositionEnd
    // onCompositionStart
    // onCompositionUpdate
    // --Keyboard
    x.insert("on_key_down".into(), "OnKeyDown".into());
    x.insert("on_key_press".into(), "OnKeyPress".into());
    x.insert("on_key_up".into(), "OnKeyUp".into());
    // --Focus
    x.insert("on_focus".into(), "OnFocus".into());
    x.insert("on_blur".into(), "OnBlur".into());
    // --Form
    x.insert("on_change".into(), "OnChange".into());
    x.insert("on_input".into(), "OnInput".into());
    x.insert("on_invalid".into(), "OnInvalid".into());
    x.insert("on_submit".into(), "OnSubmit".into());
    // --Mouse
    x.insert("on_click".into(), "OnClick".into());
    x.insert("on_context_menu".into(), "OnContextMenu".into());
    x.insert("on_dbl_click".into(), "OnDblClick".into());

    x.insert("on_drag".into(), "OnDrag".into());
    x.insert("on_drag_end".into(), "OnDragEnd".into());
    x.insert("on_drag_enter".into(), "OnDragEnter".into());
    x.insert("on_drag_exit".into(), "OnDragExit".into());
    x.insert("on_drag_leave".into(), "OnDragLeave".into());
    x.insert("on_drag_over".into(), "OnDragOver".into());
    x.insert("on_drag_start".into(), "OnDragStart".into());
    x.insert("on_drop".into(), "OnDrop".into());

    x.insert("on_mouse_down".into(), "OnMouseDown".into());
    x.insert("on_mouse_enter".into(), "OnMouseEnter".into());
    x.insert("on_mouse_leave".into(), "OnMouseLeave".into());
    x.insert("on_mouse_move".into(), "OnMouseMove".into());
    x.insert("on_mouse_over".into(), "OnMouseOver".into());
    x.insert("on_mouse_out".into(), "OnMouseOut".into());
    x.insert("on_mouse_up".into(), "OnMouseUp".into());
    // --Pointer
    x.insert("on_pointer_down".into(), "OnPointerDown".into());
    x.insert("on_pointer_move".into(), "OnPointerMove".into());
    x.insert("on_pointer_up".into(), "OnPointerUp".into());
    x.insert("on_pointer_cancel".into(), "OnPointerCancel".into());
    x.insert(
      "on_got_pointer_capture".into(),
      "OnGotPointerCapture".into(),
    );
    x.insert(
      "on_lost_pointer_capture".into(),
      "OnLostPointerCapture".into(),
    );
    x.insert("on_pointer_enter".into(), "OnPointerEnter".into());
    x.insert("on_pointer_leave".into(), "OnPointerLeave".into());
    x.insert("on_pointer_over".into(), "OnPointerOver".into());
    x.insert("on_pointer_out".into(), "OnPointerOut".into());
    // --Selection
    x.insert("on_select".into(), "OnSelect".into());
    // --Touch
    x.insert("on_touch_cancel".into(), "OnTouchCancel".into());
    x.insert("on_touch_end".into(), "OnTouchEnd".into());
    x.insert("on_touch_move".into(), "OnTouchMove".into());
    x.insert("on_touch_start".into(), "OnTouchStart".into());
    x.insert("on_scroll".into(), "OnScroll".into());
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
    x.insert("on_load".into(), "OnLoad".into());
    x.insert("on_error".into(), "OnError".into());
    // --Animation
    x.insert("on_animation_start".into(), "OnAnimationStart".into());
    x.insert("on_animation_end".into(), "OnAnimationEnd".into());
    x.insert(
      "on_animation_iteration".into(),
      "OnAnimationIteration".into(),
    );
    // --Transition
    x.insert("on_transition_end".into(), "OnTransitionEnd".into());
    // --Other
    x.insert("on_toggle".into(), "OnToggle".into());
    x
  };
}
