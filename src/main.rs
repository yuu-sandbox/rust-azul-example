use azul::{prelude::*, widgets::{label::Label, button::Button}};

struct Root {
    counter: usize,
}

fn update_counter(event: CallbackInfo<Root>) -> UpdateScreen {
    event.state.data.counter += 1;
    Redraw
}

impl Layout for Root {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("{}", self.counter)).dom();
        let button = Button::with_label("update counter").dom()
            .with_callback(On::MouseUp, update_counter);
        Dom::div()
            .with_child(label)
            .with_child(button)
    }
}

fn main() {
    let mut app = App::new(Root{counter: 0}, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();
}
