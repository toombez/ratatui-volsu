use qrcode::QrCode;
use ratatui::{DefaultTerminal, Frame};
use tui_qrcode::{Colors, QrCodeWidget};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let qr_code = QrCode
        ::new("https://ratatui.rs")
        .expect("failed to create QR code");
    let widget = QrCodeWidget
        ::new(qr_code)
        .colors(Colors::Inverted);
    frame.render_widget(widget, frame.area());
}
