use vgtk::lib::{gdk, glib, gtk::*};

pub trait WidgetExtHelpers {
    fn get_size_request(&self) -> (i32, i32);
    fn set_size_request(&self, size: (i32, i32));
    fn connect_button_pressed<F>(&self, f: F) -> glib::SignalHandlerId
    where
        F: Fn(&Self, &gdk::EventButton) + 'static;
}

impl<T> WidgetExtHelpers for T
where
    T: WidgetExt,
{
    fn get_size_request(&self) -> (i32, i32) {
        WidgetExt::get_size_request(self)
    }

    fn set_size_request(&self, (width, height): (i32, i32)) {
        WidgetExt::set_size_request(self, width, height)
    }

    fn connect_button_pressed<F>(&self, f: F) -> glib::SignalHandlerId
    where
        F: Fn(&Self, &gdk::EventButton) + 'static,
    {
        self.connect_button_press_event(move |s, e| {
            f(s, e);
            Inhibit(false)
        })
    }
}
