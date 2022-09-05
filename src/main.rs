use druid::widget::{
    Button,
    Flex,
    Label
};

use druid::{
    AppLauncher,
    LocalizedString,
    PlatformError,
    Widget,
    WidgetExt,
    WindowDesc
};

fn ui_builder() -> impl Widget<i32> {
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &i32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let btn_up = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(8.0);

    let btn_down = Button::new("decrement")
        .on_click(|_ctx, data, _env| *data -= 1)
        .padding(8.0);

    Flex::column().with_child(label).with_child(btn_up).with_child(btn_down)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .title("counter-app");
    let data = 0_i32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}
