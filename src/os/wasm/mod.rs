#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::buffer_helper;
use crate::error::Error;
use crate::key_handler::KeyHandler;
use crate::mouse_handler;
use crate::InputCallback;
use crate::Result;
use crate::{CursorStyle, MouseButton, MouseMode};
use crate::{Key, KeyRepeat};
use crate::{MenuHandle, MenuItem, MenuItemHandle, UnixMenu, UnixMenuItem};
use crate::{Scale, WindowOptions};

use std::cmp;
use std::os::raw;

pub struct Window {
    is_open: bool,
    is_active: bool,
    mouse_pos: Option<(i32, i32)>,
    mouse_scroll: Option<(i32, i32)>,
    /// The state of the left, middle and right mouse buttons
    mouse_state: (bool, bool, bool),
    buffer_width: usize,
    buffer_height: usize,
    canvas_scale: usize,
    key_handler: KeyHandler,
    menu_counter: MenuHandle,
    menus: Vec<UnixMenu>,

    document: web_sys::Document,
    context: web_sys::CanvasRenderingContext2d,
}

impl Window {
    pub fn new(name: &str, width: usize, height: usize, opts: WindowOptions) -> Result<Window> {
        let canvas_scale = match opts.scale {
            Scale::X1 => 1,
            Scale::X2 => 2,
            Scale::X4 => 4,
            Scale::X8 => 8,
            Scale::X16 => 16,
            Scale::X32 => 32,
            Scale::FitScreen => unimplemented!(),
        };

        let canvas_width = width as u32 * canvas_scale as u32;
        let canvas_height = height as u32 * canvas_scale as u32;

        let document = web_sys::window().unwrap().document().unwrap();
        document.set_title(name);

        let canvas = document.create_element("canvas").unwrap();

        let body = document.body().unwrap();
        body.append_child(&canvas);

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Ok(Window {
            mouse_pos: None,
            mouse_scroll: None,
            mouse_state: (false, false, false),
            is_open: true,
            is_active: true,
            buffer_width: width,
            buffer_height: height,
            canvas_scale,
            key_handler: KeyHandler::new(),
            menu_counter: MenuHandle(0),
            menus: Vec::new(),
            document,
            context,
        })
    }

    pub fn set_title(&mut self, title: &str) {
        self.document.set_title(title);
    }

    pub fn get_window_handle(&self) -> *mut raw::c_void {
        0 as *mut raw::c_void
    }

    pub fn update_with_buffer_stride(
        &mut self,
        buffer: &[u32],
        width: usize,
        height: usize,
        stride: usize,
    ) -> Result<()> {
        self.process_events();
        self.key_handler.update();

        self.render_buffer(buffer);

        Ok(())
    }

    pub fn update(&mut self) {
        self.process_events();
        self.key_handler.update();
    }

    pub fn set_position(&mut self, x: isize, y: isize) {
        unimplemented!()
    }

    pub fn get_size(&self) -> (usize, usize) {
        unimplemented!()
    }

    pub fn get_scroll_wheel(&self) -> Option<(f32, f32)> {
        if let Some((scroll_x, scroll_y)) = self.mouse_scroll {
            Some((scroll_x as f32, scroll_y as f32))
        } else {
            None
        }
    }

    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.mouse_state.0,
            MouseButton::Middle => self.mouse_state.1,
            MouseButton::Right => self.mouse_state.2,
        }
    }

    pub fn get_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        if let Some((mouse_x, mouse_y)) = self.mouse_pos {
            mouse_handler::get_pos(
                mode,
                mouse_x as f32,
                mouse_y as f32,
                self.canvas_scale as f32,
                self.buffer_width as f32 * self.canvas_scale as f32,
                self.buffer_height as f32 * self.canvas_scale as f32,
            )
        } else {
            None
        }
    }

    pub fn get_unscaled_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        if let Some((mouse_x, mouse_y)) = self.mouse_pos {
            mouse_handler::get_pos(
                mode,
                mouse_x as f32,
                mouse_y as f32,
                1.0 as f32,
                self.buffer_width as f32 * self.canvas_scale as f32,
                self.buffer_height as f32 * self.canvas_scale as f32,
            )
        } else {
            None
        }
    }

    pub fn set_cursor_style(&mut self, _cursor: CursorStyle) {
        unimplemented!()
    }

    pub fn get_keys(&self) -> Option<Vec<Key>> {
        self.key_handler.get_keys()
    }

    pub fn get_keys_pressed(&self, repeat: KeyRepeat) -> Option<Vec<Key>> {
        self.key_handler.get_keys_pressed(repeat)
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.key_handler.is_key_down(key)
    }

    pub fn set_key_repeat_delay(&mut self, delay: f32) {
        self.key_handler.set_key_repeat_delay(delay)
    }

    pub fn set_key_repeat_rate(&mut self, rate: f32) {
        self.key_handler.set_key_repeat_rate(rate)
    }

    pub fn is_key_pressed(&self, key: Key, repeat: KeyRepeat) -> bool {
        self.key_handler.is_key_pressed(key, repeat)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.key_handler.is_key_released(key)
    }

    pub fn set_input_callback(&mut self, callback: Box<InputCallback>) {
        self.key_handler.set_input_callback(callback)
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn is_active(&mut self) -> bool {
        self.is_active
    }

    fn process_events(&mut self) {
        self.mouse_scroll = None;
    }

    /// Renders the given pixel data into the canvas
    fn render_buffer(&mut self, buffer: &[u32]) {
        unimplemented!()
    }

    fn next_menu_handle(&mut self) -> MenuHandle {
        let handle = self.menu_counter;
        self.menu_counter.0 += 1;
        handle
    }

    pub fn add_menu(&mut self, menu: &Menu) -> MenuHandle {
        let handle = self.next_menu_handle();
        let mut menu = menu.internal.clone();
        menu.handle = handle;
        self.menus.push(menu);
        handle
    }

    pub fn get_unix_menus(&self) -> Option<&Vec<UnixMenu>> {
        Some(&self.menus)
    }

    pub fn remove_menu(&mut self, handle: MenuHandle) {
        self.menus.retain(|ref menu| menu.handle != handle);
    }

    pub fn is_menu_pressed(&mut self) -> Option<usize> {
        None
    }

    pub fn set_rate(&mut self, time: Option<std::time::Duration>) {
        unimplemented!()
    }

    pub fn update_rate(&mut self) {
        unimplemented!()
    }

    pub fn set_background_color(&mut self, color: u32) {
        unimplemented!()
    }
}

unsafe impl raw_window_handle::HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        unimplemented!()
    }
}

pub struct Menu {
    pub internal: UnixMenu,
}

impl Menu {
    pub fn new(name: &str) -> Result<Menu> {
        Ok(Menu {
            internal: UnixMenu {
                handle: MenuHandle(0),
                item_counter: MenuItemHandle(0),
                name: name.to_owned(),
                items: Vec::new(),
            },
        })
    }

    pub fn add_sub_menu(&mut self, name: &str, sub_menu: &Menu) {
        let handle = self.next_item_handle();
        self.internal.items.push(UnixMenuItem {
            label: name.to_owned(),
            handle,
            sub_menu: Some(Box::new(sub_menu.internal.clone())),
            id: 0,
            enabled: true,
            key: Key::Unknown,
            modifier: 0,
        });
    }

    fn next_item_handle(&mut self) -> MenuItemHandle {
        let handle = self.internal.item_counter;
        self.internal.item_counter.0 += 1;
        handle
    }

    pub fn add_menu_item(&mut self, item: &MenuItem) -> MenuItemHandle {
        let item_handle = self.next_item_handle();
        self.internal.items.push(UnixMenuItem {
            sub_menu: None,
            handle: self.internal.item_counter,
            id: item.id,
            label: item.label.clone(),
            enabled: item.enabled,
            key: item.key,
            modifier: item.modifier,
        });
        item_handle
    }

    pub fn remove_item(&mut self, handle: &MenuItemHandle) {
        self.internal
            .items
            .retain(|ref item| item.handle.0 != handle.0);
    }
}
