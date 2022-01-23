#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {

// CLASS: wxObject
wxObject *NewObject();
wxObject *NewObject(const wxObject & other);

// CLASS: wxEvtHandler
wxEvtHandler *NewEvtHandler();

// CLASS: wxWindow
wxWindow *NewWindow();
wxWindow *NewWindow(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, long style, const wxString & name);

// CLASS: wxControl
wxControl *NewControl(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, long style, const wxValidator & validator, const wxString & name);
wxControl *NewControl();

// CLASS: wxAnyButton
wxAnyButton *NewAnyButton();

// CLASS: wxButton
wxButton *NewButton();
wxButton *NewButton(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, long style, const wxValidator & validator, const wxString & name);

} // namespace wxrust

