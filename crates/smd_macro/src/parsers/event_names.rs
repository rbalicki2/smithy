use lazy_static::lazy_static;
use std::collections::HashMap;
// see attributes.rs for an explanation

lazy_static! {
  pub static ref UI_EVENT_NAMES: HashMap<String, String> = {
    // Remember to update this as you add more!
    let mut event_names = HashMap::with_capacity(54);

    // TODO figure out why I can't wrap this in if (cfg![test])
    event_names.insert("on_test".into(), "OnTest".into());
    // --Clipboard
    event_names.insert("on_copy".into(), "OnCopy".into());
    event_names.insert("on_cut".into(), "OnCut".into());
    event_names.insert("on_paste".into(), "OnPaste".into());
    // --Composition
    // onCompositionEnd
    // onCompositionStart
    // onCompositionUpdate
    // --Keyboard
    event_names.insert("on_key_down".into(), "OnKeyDown".into());
    event_names.insert("on_key_press".into(), "OnKeyPress".into());
    event_names.insert("on_key_up".into(), "OnKeyUp".into());
    // --Focus
    event_names.insert("on_focus".into(), "OnFocus".into());
    event_names.insert("on_blur".into(), "OnBlur".into());
    // --Form
    event_names.insert("on_change".into(), "OnChange".into());
    event_names.insert("on_input".into(), "OnInput".into());
    event_names.insert("on_invalid".into(), "OnInvalid".into());
    event_names.insert("on_submit".into(), "OnSubmit".into());
    // --Mouse
    event_names.insert("on_click".into(), "OnClick".into());
    event_names.insert("on_context_menu".into(), "OnContextMenu".into());
    event_names.insert("on_dbl_click".into(), "OnDblClick".into());

    event_names.insert("on_drag".into(), "OnDrag".into());
    event_names.insert("on_drag_end".into(), "OnDragEnd".into());
    event_names.insert("on_drag_enter".into(), "OnDragEnter".into());
    event_names.insert("on_drag_exit".into(), "OnDragExit".into());
    event_names.insert("on_drag_leave".into(), "OnDragLeave".into());
    event_names.insert("on_drag_over".into(), "OnDragOver".into());
    event_names.insert("on_drag_start".into(), "OnDragStart".into());
    event_names.insert("on_drop".into(), "OnDrop".into());

    event_names.insert("on_mouse_down".into(), "OnMouseDown".into());
    event_names.insert("on_mouse_enter".into(), "OnMouseEnter".into());
    event_names.insert("on_mouse_leave".into(), "OnMouseLeave".into());
    event_names.insert("on_mouse_move".into(), "OnMouseMove".into());
    event_names.insert("on_mouse_over".into(), "OnMouseOver".into());
    event_names.insert("on_mouse_out".into(), "OnMouseOut".into());
    event_names.insert("on_mouse_up".into(), "OnMouseUp".into());
    // --Pointer
    event_names.insert("on_pointer_down".into(), "OnPointerDown".into());
    event_names.insert("on_pointer_move".into(), "OnPointerMove".into());
    event_names.insert("on_pointer_up".into(), "OnPointerUp".into());
    event_names.insert("on_pointer_cancel".into(), "OnPointerCancel".into());
    event_names.insert(
      "on_got_pointer_capture".into(),
      "OnGotPointerCapture".into(),
    );
    event_names.insert(
      "on_lost_pointer_capture".into(),
      "OnLostPointerCapture".into(),
    );
    event_names.insert("on_pointer_enter".into(), "OnPointerEnter".into());
    event_names.insert("on_pointer_leave".into(), "OnPointerLeave".into());
    event_names.insert("on_pointer_over".into(), "OnPointerOver".into());
    event_names.insert("on_pointer_out".into(), "OnPointerOut".into());
    // --Selection
    event_names.insert("on_select".into(), "OnSelect".into());
    // --Touch
    event_names.insert("on_touch_cancel".into(), "OnTouchCancel".into());
    event_names.insert("on_touch_end".into(), "OnTouchEnd".into());
    event_names.insert("on_touch_move".into(), "OnTouchMove".into());
    event_names.insert("on_touch_start".into(), "OnTouchStart".into());
    event_names.insert("on_scroll".into(), "OnScroll".into());
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
    event_names.insert("on_load".into(), "OnLoad".into());
    event_names.insert("on_error".into(), "OnError".into());
    // --Animation
    event_names.insert("on_animation_start".into(), "OnAnimationStart".into());
    event_names.insert("on_animation_end".into(), "OnAnimationEnd".into());
    event_names.insert(
      "on_animation_iteration".into(),
      "OnAnimationIteration".into(),
    );
    // --Transition
    event_names.insert("on_transition_end".into(), "OnTransitionEnd".into());
    // --Other
    event_names.insert("on_toggle".into(), "OnToggle".into());
    event_names
  };

  pub static ref WINDOW_EVENT_NAMES: HashMap<String, String> = {
    // Remember to update this as you add more!
    let mut event_names = HashMap::with_capacity(4);
    event_names.insert("on_before_unload".into(), "OnBeforeUnload".into());
    event_names.insert("on_hash_change".into(), "OnHashChange".into());
    event_names.insert("on_pop_state".into(), "OnPopState".into());
    event_names.insert("on_unhandled_rejection".into(), "OnUnhandledRejection".into());
    event_names
  };

  pub static ref LIFECYCLE_EVENT_NAMES: HashMap<String, String> = {
    let mut lifecycle_event_names = HashMap::with_capacity(1);
    lifecycle_event_names.insert("post_render".into(), "PostRender".into());
    lifecycle_event_names
  };
}
