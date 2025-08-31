use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ ApplicationWindow, EventControllerMotion };
use std::cell::{ Cell, RefCell };
use std::rc::Rc;

use crate::config::{ bar_edge, ANIM_MS, BAR_HEIGHT, PEEK };
use gtk4_layer_shell::LayerShell;

#[allow(dead_code)]
pub struct SlideHover {
    current_margin: Rc<Cell<i32>>,
    target_margin: Rc<Cell<i32>>,
    anim_source_id: Rc<RefCell<Option<glib::SourceId>>>,
    hidden_margin: i32,
}

impl SlideHover {
    pub fn attach(window: &ApplicationWindow, hover_surface: &impl IsA<gtk::Widget>) -> Self {
        let edge = bar_edge();
        let hidden_margin = -(BAR_HEIGHT - PEEK);
        window.set_margin(edge, hidden_margin);

        let current_margin = Rc::new(Cell::new(hidden_margin));
        let target_margin = Rc::new(Cell::new(hidden_margin));
        let anim_source_id: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));

        let animate = {
            let window = window.clone();
            let current_margin = current_margin.clone();
            let target_margin = target_margin.clone();
            let anim_source_id = anim_source_id.clone();

            move |to: i32| {
                target_margin.set(to);
                if anim_source_id.borrow().is_some() {
                    return;
                }
                let start = current_margin.get();
                let distance = (to - start).abs().max(1) as f32;
                let step = (distance / ((ANIM_MS as f32) / 16.0)).ceil().max(1.0) as i32;

                let id = glib::timeout_add_local(std::time::Duration::from_millis(16), {
                    let window = window.clone();
                    let current_margin = current_margin.clone();
                    let target_margin = target_margin.clone();
                    let anim_source_id = anim_source_id.clone();
                    let edge = edge;

                    move || {
                        let cur = current_margin.get();
                        let to = target_margin.get();

                        if cur == to {
                            *anim_source_id.borrow_mut() = None;
                            return glib::ControlFlow::Break;
                        }
                        let dir = if to > cur { 1 } else { -1 };
                        let mut next = cur + dir * step;
                        if (to - next).signum() != (to - cur).signum() {
                            next = to;
                        }

                        current_margin.set(next);
                        window.set_margin(edge, next);
                        glib::ControlFlow::Continue
                    }
                });
                *anim_source_id.borrow_mut() = Some(id);
            }
        };

        let motion = EventControllerMotion::new();
        {
            let animate_in = animate.clone();
            motion.connect_enter(move |_, _, _| animate_in(0));
        }
        {
            let animate_out = animate.clone();
            motion.connect_leave(move |_| animate_out(hidden_margin));
        }
        hover_surface.add_controller(motion);

        Self { current_margin, target_margin, anim_source_id, hidden_margin }
    }
}
