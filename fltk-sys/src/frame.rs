/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut libc::c_void)>;
pub type custom_handler_callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: libc::c_int, arg2: *mut libc::c_void) -> libc::c_int,
>;
pub type custom_draw_callback =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_void)>;
extern "C" {
    pub fn Fl_Widget_new(
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        title: *const libc::c_char,
    ) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Widget_x(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_y(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_width(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_height(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_label(arg1: *mut Fl_Widget) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_Widget_set_label(arg1: *mut Fl_Widget, title: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Widget_redraw(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_show(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_hide(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_activate(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_deactivate(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_redraw_label(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_resize(
        arg1: *mut Fl_Widget,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Widget_tooltip(arg1: *mut Fl_Widget) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_Widget_set_tooltip(arg1: *mut Fl_Widget, txt: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Widget_get_type(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_type(arg1: *mut Fl_Widget, typ: libc::c_int);
}
extern "C" {
    pub fn Fl_Widget_color(arg1: *mut Fl_Widget) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_color(arg1: *mut Fl_Widget, color: libc::c_uint);
}
extern "C" {
    pub fn Fl_Widget_label_color(arg1: *mut Fl_Widget) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_label_color(arg1: *mut Fl_Widget, color: libc::c_uint);
}
extern "C" {
    pub fn Fl_Widget_label_font(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_font(arg1: *mut Fl_Widget, font: libc::c_int);
}
extern "C" {
    pub fn Fl_Widget_label_size(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_size(arg1: *mut Fl_Widget, sz: libc::c_int);
}
extern "C" {
    pub fn Fl_Widget_label_type(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_type(arg1: *mut Fl_Widget, typ: libc::c_int);
}
extern "C" {
    pub fn Fl_Widget_box(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_box(arg1: *mut Fl_Widget, typ: libc::c_int);
}
extern "C" {
    pub fn Fl_Widget_changed(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_changed(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_clear_changed(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_align(arg1: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_align(arg1: *mut Fl_Widget, typ: libc::c_int);
}
extern "C" {
    pub fn Fl_Widget_delete(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_set_image(arg1: *mut Fl_Widget, arg2: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Widget_set_handler(
        self_: *mut Fl_Widget,
        cb: custom_handler_callback,
        data: *mut libc::c_void,
    );
}
extern "C" {
    pub fn Fl_Widget_set_draw(
        self_: *mut Fl_Widget,
        cb: custom_draw_callback,
        data: *mut libc::c_void,
    );
}
extern "C" {
    pub fn Fl_Widget_set_trigger(arg1: *mut Fl_Widget, arg2: libc::c_int);
}
extern "C" {
    pub fn Fl_Widget_image(arg1: *const Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_parent(self_: *const Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_selection_color(arg1: *mut Fl_Widget) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_selection_color(arg1: *mut Fl_Widget, color: libc::c_uint);
}
extern "C" {
    pub fn Fl_Widget_do_callback(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_inside(self_: *const Fl_Widget, arg1: *mut libc::c_void) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_window(arg1: *const Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_top_window(arg1: *const Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_takes_events(arg1: *const Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_user_data(arg1: *const Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_take_focus(self_: *mut Fl_Widget) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_visible_focus(self_: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_clear_visible_focus(self_: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_visible_focus(self_: *mut Fl_Widget, v: libc::c_int);
}
extern "C" {
    pub fn Fl_Widget_has_visible_focus(self_: *mut Fl_Widget) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_user_data(arg1: *mut Fl_Widget, data: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Widget_draw_data(self_: *const Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_handle_data(self_: *const Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_set_draw_data(self_: *mut Fl_Widget, data: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Widget_set_handle_data(self_: *mut Fl_Widget, data: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Widget_damage(self_: *const Fl_Widget) -> libc::c_uchar;
}
extern "C" {
    pub fn Fl_Widget_set_damage(self_: *mut Fl_Widget, flag: libc::c_uchar);
}
extern "C" {
    pub fn Fl_Widget_clear_damage(self_: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_as_window(self_: *mut Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_as_group(self_: *mut Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_set_deimage(arg1: *mut Fl_Widget, arg2: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Widget_deimage(arg1: *const Fl_Widget) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Widget_set_callback(arg1: *mut Fl_Widget, arg2: Fl_Callback, arg3: *mut libc::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Box {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Box_new(
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        title: *const libc::c_char,
    ) -> *mut Fl_Box;
}
extern "C" {
    pub fn Fl_Box_x(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_y(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_width(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_height(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_label(arg1: *mut Fl_Box) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_Box_set_label(arg1: *mut Fl_Box, title: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Box_redraw(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_show(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_hide(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_activate(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_deactivate(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_redraw_label(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_resize(
        arg1: *mut Fl_Box,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_tooltip(arg1: *mut Fl_Box) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_Box_set_tooltip(arg1: *mut Fl_Box, txt: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Box_get_type(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_set_type(arg1: *mut Fl_Box, typ: libc::c_int);
}
extern "C" {
    pub fn Fl_Box_color(arg1: *mut Fl_Box) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_color(arg1: *mut Fl_Box, color: libc::c_uint);
}
extern "C" {
    pub fn Fl_Box_label_color(arg1: *mut Fl_Box) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_label_color(arg1: *mut Fl_Box, color: libc::c_uint);
}
extern "C" {
    pub fn Fl_Box_label_font(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_font(arg1: *mut Fl_Box, font: libc::c_int);
}
extern "C" {
    pub fn Fl_Box_label_size(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_size(arg1: *mut Fl_Box, sz: libc::c_int);
}
extern "C" {
    pub fn Fl_Box_label_type(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_type(arg1: *mut Fl_Box, typ: libc::c_int);
}
extern "C" {
    pub fn Fl_Box_box(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_set_box(arg1: *mut Fl_Box, typ: libc::c_int);
}
extern "C" {
    pub fn Fl_Box_changed(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_set_changed(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_clear_changed(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_align(arg1: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_set_align(arg1: *mut Fl_Box, typ: libc::c_int);
}
extern "C" {
    pub fn Fl_Box_delete(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_set_image(arg1: *mut Fl_Box, arg2: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Box_set_handler(
        self_: *mut Fl_Box,
        cb: custom_handler_callback,
        data: *mut libc::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_set_draw(self_: *mut Fl_Box, cb: custom_draw_callback, data: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Box_set_trigger(arg1: *mut Fl_Box, arg2: libc::c_int);
}
extern "C" {
    pub fn Fl_Box_image(arg1: *const Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_parent(self_: *const Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_selection_color(arg1: *mut Fl_Box) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_selection_color(arg1: *mut Fl_Box, color: libc::c_uint);
}
extern "C" {
    pub fn Fl_Box_do_callback(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_inside(self_: *const Fl_Box, arg1: *mut libc::c_void) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_window(arg1: *const Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_top_window(arg1: *const Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_takes_events(arg1: *const Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_user_data(arg1: *const Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_take_focus(self_: *mut Fl_Box) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Box_set_visible_focus(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_clear_visible_focus(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_visible_focus(self_: *mut Fl_Box, v: libc::c_int);
}
extern "C" {
    pub fn Fl_Box_has_visible_focus(self_: *mut Fl_Box) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_user_data(arg1: *mut Fl_Box, data: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Box_draw_data(self_: *const Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_handle_data(self_: *const Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_set_draw_data(self_: *mut Fl_Box, data: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Box_set_handle_data(self_: *mut Fl_Box, data: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Box_damage(self_: *const Fl_Box) -> libc::c_uchar;
}
extern "C" {
    pub fn Fl_Box_set_damage(self_: *mut Fl_Box, flag: libc::c_uchar);
}
extern "C" {
    pub fn Fl_Box_clear_damage(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_as_window(self_: *mut Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_as_group(self_: *mut Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_set_deimage(arg1: *mut Fl_Box, arg2: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_Box_deimage(arg1: *const Fl_Box) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_Box_set_callback(arg1: *mut Fl_Box, arg2: Fl_Callback, arg3: *mut libc::c_void);
}
