use std::thread;
use std::time::Duration;
use reqwest;

fn main() {
    let target_url = "https://www.example.com"; // URL do aplicativo ou sistema a ser verificado
    let sleep_duration = Duration::from_secs(60); // Intervalo de verificação em segundos

    loop {
        // Realizar uma verificação de ping no URL
        match reqwest::blocking::get(target_url) {
            Ok(response) => {
                if response.status().is_success() {
                    println!("O aplicativo ou sistema está online.");
                } else {
                    println!("ATENÇÃO: O aplicativo ou sistema está offline!");
                    // Aqui você pode adicionar lógica para enviar uma notificação, por exemplo, via e-mail, SMS, etc.
                }
            }
            Err(_) => {
                println!("ATENÇÃO: O aplicativo ou sistema está offline!");
                // Aqui você pode adicionar lógica para enviar uma notificação, por exemplo, via e-mail, SMS, etc.
            }
        }

        thread::sleep(sleep_duration);
    }
}
