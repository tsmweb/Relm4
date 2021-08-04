use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{impl_model, send, AppUpdate, RelmApp, RelmWidgets, Sender, WidgetPlus};

enum AppMsg {
    Increment,
}

#[tracker::track]
struct AppModel {
    width: u32,
    counter: u32,
}

impl_model!(AppModel, AppMsg);

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) {
        self.reset();
        // reset tracker value of the model
        //self.reset();
        // set_#member_name() will set a bit in the tracker variable of the model
        match msg {
            AppMsg::Increment => {
                self.update_counter(|cnt| *cnt += 1);
            }
        }
        //println!("counter: {}", self.counter);
    }
}

#[relm4_macros::widget]
impl RelmWidgets for AppWidgets {
    // specify generic types
    type Model = AppModel;

    view! {
      main_window = gtk::ApplicationWindow {
        set_default_height: 400,
        set_default_width: model.width as i32,
        set_child = Some(&gtk::Box) {
          set_orientation: gtk::Orientation::Vertical,
          set_margin_all: 10,
          set_spacing: 10,
          set_hexpand: true,
          append: label = &gtk::Label {
            set_label: track!(model.changed(AppModel::counter()),
                &format!("Counter is at: {}", model.counter)),
          },
          append: button = &gtk::Button::new() {
            set_label: watch!{ &format!("Clicked: {}", model.counter)},
            set_visible: true,
            connect_clicked => move |_btn| {
              send!(sender, AppMsg::Increment);
            },
          },
          append: _inv_label = &gtk::Label {
            set_label: "Green inverted text!",
            inline_css: b"transform: rotate(180deg); color: green;",
          }
        }
      }
    }
}

fn main() {
    let model = AppModel {
        width: 1000,
        counter: 0,
        tracker: 0,
    };
    let relm: RelmApp<AppWidgets> = RelmApp::new(model);
    relm.run();
}