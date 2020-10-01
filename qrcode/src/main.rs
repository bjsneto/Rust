use qrcode::QrCode;
use image::Luma;
use std::io::stdin;

static PATH: &str = "C:/Projetos/rust/qrcode.png";

fn main() {
    println!("Digite o número do telefone:");
    let mut numero = String::new();
    stdin().read_line(&mut numero).ok().expect("Número inválido");
    let code = QrCode::new(format!("https://api.whatsapp.com/send?phone={}&text=Ol%C3%A1%2C%20bem-vindo", numero.trim())).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save(PATH).unwrap();    
}
