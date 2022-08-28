use super::*;

// wxEditableListBox
pub trait EditableListBoxMethods: PanelMethods {
    // DTOR: fn ~wxEditableListBox()
    /// Creates the editable listbox for two-step construction.
    ///
    /// [See `wxEditableListBox::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_editable_list_box.html#a38b5b586e33716eb6431062ca9f685a0)
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxEditableListBox_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    /// Replaces current contents with given strings.
    ///
    /// [See `wxEditableListBox::SetStrings()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_editable_list_box.html#a6162b1d0e5c4b39896f6628de70fceee)
    fn set_strings<A: ArrayStringMethods>(&self, strings: &A) {
        unsafe {
            let strings = strings.as_ptr();
            ffi::wxEditableListBox_SetStrings(self.as_ptr(), strings)
        }
    }
    /// Returns in the given array the current contents of the control (the array will be erased before control's contents are appended).
    ///
    /// [See `wxEditableListBox::GetStrings()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_editable_list_box.html#ae6996b10e786261743eb83e8313357d3)
    fn get_strings<A: ArrayStringMethods>(&self, strings: &A) {
        unsafe {
            let strings = strings.as_ptr();
            ffi::wxEditableListBox_GetStrings(self.as_ptr(), strings)
        }
    }
}

// wxEraseEvent
pub trait EraseEventMethods: EventMethods {
    /// Returns the device context associated with the erase event to draw on.
    ///
    /// [See `wxEraseEvent::GetDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_erase_event.html#a44679163c23de75ecdbb2594add850cc)
    fn get_dc(&self) -> Option<DCIsOwned<false>> {
        unsafe { DC::option_from(ffi::wxEraseEvent_GetDC(self.as_ptr())) }
    }
}

// wxEventBlocker
pub trait EventBlockerMethods: EvtHandlerMethods {
    // DTOR: fn ~wxEventBlocker()
    // NOT_SUPPORTED: fn Block()
}
