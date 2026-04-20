struct SettingsState {
    dark_mode: bool,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self { dark_mode: false }
    }
}

fn settings_view(state: &SettingsState) -> Element<'_, Message> {
    container(text("Settings Page"))
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .center_y(Fill)
        .padding(20)
        .style(|_theme: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::BLACK)),
            border: Border {
                width: 1.0,
                color: Color::WHITE,
                radius: 0.into(),
            },
            ..Default::default()
        })
        .into()
}
