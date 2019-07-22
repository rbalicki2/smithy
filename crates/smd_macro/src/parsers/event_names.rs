use lazy_static::lazy_static;
use std::collections::HashMap;
// see attributes.rs for an explanation

lazy_static! {
  pub static ref UI_EVENT_NAMES: HashMap<String, (String, bool)> = {
    let mut event_names = HashMap::new();

    // TODO figure out why I can't wrap this in if (cfg![test])
    event_names.insert("on_test".into(), ("OnTest".into(), false));
    // --Clipboard
    // #[cfg(feature = "clipboard-events")]
    {
      event_names.insert("on_copy".into(), ("OnCopy".into(), false));
      event_names.insert("on_cut".into(), ("OnCut".into(), false));
      event_names.insert("on_paste".into(), (("OnPaste".into()), false));
    }
    // --Composition
    // onCompositionEnd
    // onCompositionStart
    // onCompositionUpdate
    // --Keyboard
    // #[cfg(feature = "keyboard-events")]
    {
      event_names.insert("on_key_down".into(), ("OnKeyDown".into(), false));
      event_names.insert("on_key_press".into(), ("OnKeyPress".into(), false));
      event_names.insert("on_key_up".into(), ("OnKeyUp".into(), false));
    }
    // --Focus
    // #[cfg(feature = "focus-events")]
    {
      event_names.insert("on_focus".into(), ("OnFocus".into(), false));
      event_names.insert("on_blur".into(), ("OnBlur".into(), false));
    }
    // --Form
    // #[cfg(feature = "input-events")]
    {
      event_names.insert("on_change".into(), ("OnChange".into(), false));
      event_names.insert("on_input".into(), ("OnInput".into(), false));
      event_names.insert("on_invalid".into(), ("OnInvalid".into(), false));
      event_names.insert("on_submit".into(), ("OnSubmit".into(), false));
    }
    // --Mouse
    // #[cfg(feature = "mouse-events")]
    {
      event_names.insert("on_click".into(), ("OnClick".into(), true));
      event_names.insert("on_context_menu".into(), ("OnContextMenu".into(), true));
      event_names.insert("on_dbl_click".into(), ("OnDblClick".into(), true));

      event_names.insert("on_drag".into(), ("OnDrag".into(), false));
      event_names.insert("on_drag_end".into(), ("OnDragEnd".into(), false));
      event_names.insert("on_drag_enter".into(), ("OnDragEnter".into(), false));
      event_names.insert("on_drag_exit".into(), ("OnDragExit".into(), false));
      event_names.insert("on_drag_leave".into(), ("OnDragLeave".into(), false));
      event_names.insert("on_drag_over".into(), ("OnDragOver".into(), true));
      event_names.insert("on_drag_start".into(), ("OnDragStart".into(), true));
      event_names.insert("on_drop".into(), ("OnDrop".into(), true));

      event_names.insert("on_mouse_down".into(), ("OnMouseDown".into(), true));
      event_names.insert("on_mouse_enter".into(), ("OnMouseEnter".into(), true));
      event_names.insert("on_mouse_leave".into(), ("OnMouseLeave".into(), true));
      event_names.insert("on_mouse_move".into(), ("OnMouseMove".into(), true));
      event_names.insert("on_mouse_over".into(), ("OnMouseOver".into(), true));
      event_names.insert("on_mouse_out".into(), ("OnMouseOut".into(), true));
      event_names.insert("on_mouse_up".into(), ("OnMouseUp".into(), true));
    }
    // --Pointer
    // #[cfg(feature = "pointer-events")]
    {
      event_names.insert("on_pointer_down".into(), ("OnPointerDown".into(), true));
      event_names.insert("on_pointer_move".into(), ("OnPointerMove".into(), true));
      event_names.insert("on_pointer_up".into(), ("OnPointerUp".into(), true));
      event_names.insert("on_pointer_cancel".into(), ("OnPointerCancel".into(), true));
      event_names.insert(
        "on_got_pointer_capture".into(),
        ("OnGotPointerCapture".into(), true),
      );
      event_names.insert(
        "on_lost_pointer_capture".into(),
        ("OnLostPointerCapture".into(), true),
      );
      event_names.insert("on_pointer_enter".into(), ("OnPointerEnter".into(), true));
      event_names.insert("on_pointer_leave".into(), ("OnPointerLeave".into(), true));
      event_names.insert("on_pointer_over".into(), ("OnPointerOver".into(), true));
      event_names.insert("on_pointer_out".into(), ("OnPointerOut".into(), true));
    }
    // --Selection
    // #[cfg(feature = "select-events")]
    {
      event_names.insert("on_select".into(), ("OnSelect".into(), false));
    }
    // --Touch
    // #[cfg(feature = "touch-events")]
    {
      event_names.insert("on_touch_cancel".into(), ("OnTouchCancel".into(), true));
      event_names.insert("on_touch_end".into(), ("OnTouchEnd".into(), true));
      event_names.insert("on_touch_move".into(), ("OnTouchMove".into(), true));
      event_names.insert("on_touch_start".into(), ("OnTouchStart".into(), true));
    }
    // #[cfg(feature = "scroll-events")]
    {
      event_names.insert("on_scroll".into(), ("OnScroll".into(),false));
    }
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
    // #[cfg(feature = "image-events")]
    {
      event_names.insert("on_load".into(), ("OnLoad".into(), false));
      event_names.insert("on_error".into(), ("OnError".into(), false));
    }
    // --Animation
    // #[cfg(feature = "animation-events")]
    {
      event_names.insert("on_animation_start".into(), ("OnAnimationStart".into(), false));
      event_names.insert("on_animation_end".into(), ("OnAnimationEnd".into(), false));
      event_names.insert(
        "on_animation_iteration".into(),
        ("OnAnimationIteration".into(), false),
      );
    }
    // --Transition
    // #[cfg(feature = "transition-events")]
    {
      event_names.insert("on_transition_end".into(), ("OnTransitionEnd".into(), false));
    }
    // --Other
    // #[cfg(feature = "toggle-events")]
    {
      event_names.insert("on_toggle".into(), ("OnToggle".into(), false));
    }
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

pub fn should_include_rest_param(opt: &Option<String>) -> bool {
  println!("\n\nopt {:?}", opt);
  opt
    .as_ref()
    .and_then(|provided_event_name| {
      UI_EVENT_NAMES
        .iter()
        .find_map(|(_key, (event_name, should_include_rest_param))| {
          if (provided_event_name != event_name) {
            return None;
          }
          return Some(*should_include_rest_param);
        })
    })
    .unwrap_or(false)
}
