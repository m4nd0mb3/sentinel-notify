
<p align="center">
  <a href="https://github.com/m4nd0mb3/sentinel-notify" target="_blank">
    <img alt="Sentinel Notify" width="" src="./src/assets/images/Notify-removebg-preview.png">
  </a>
</p>

<p align="center">
<a href="https://github.com/m4nd0mb3/sentinel-notify/releases">
    <img src="https://badgen.net/github/release/m4nd0mb3/sentinel-notify" alt="GitHub release">
  </a>
  <a href="https://github.com/m4nd0mb3/sentinel-notify/releases">
    <img src="https://img.shields.io/github/downloads/m4nd0mb3/sentinel-notify/total.svg" alt="GitHub downloads">
  </a>
  <a href="https://github.com/m4nd0mb3/sentinel-notify/blob/master/LICENSE">
    <img src="https://badgen.net/github/license/m4nd0mb3/sentinel-notify" alt="GitHub license">
  </a><br/>
  <a href='https://sentinel-notify.readthedocs.io/en/latest/?badge=latest'>
    <img src='https://readthedocs.org/projects/sentinel-notify/badge/?version=latest' alt='Documentation Status' />
  </a>   
  <a href="https://github.com/m4nd0mb3/sentinel-notify/issues">
    <img src="https://badgen.net/github/issues/m4nd0mb3/sentinel-notify" alt="GitHub issues">
  </a>
  <a href="https://github.com/m4nd0mb3">
    <img src="https://badgen.net/github/contributors/m4nd0mb3/sentinel-notify" alt="GitHub followers">
  </a>
  <a href="https://github.com/carboneio/carbone">
    <img src="https://badgen.net/github/forks/m4nd0mb3/sentinel-notify?icon=github" alt="github fork badge">
  </a>
</p>



<h1 align="center">Sentinel Notify</h1>

<p align="center">
  <strong>Simple Rust script to periodically check the availability of an application or system.</strong>
  <br>
  Sentinel Notify is a simple Rust script that periodically checks the availability of an application or system by pinging a specified URL. If the application or system is detected as offline, you can implement logic to send notifications.
</p>

<p align="center">
  <a href="#usage">Usage</a> •
  <a href="#customization">Customization</a> •
  <a href="#contributing">Contributing</a> •
  <a href="#license">License</a>
</p>

## Usage

1. Make sure you have Rust installed. You can download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone this repository:

```sh
git clone https://github.com/m4nd0mb3/sentinel-notify.git
cd sentinel-notify
```

3. Replace the contents of `src/main.rs` with the code provided in the [main.rs](src/main.rs) file.

4. Run the script:

```sh
cargo run
```

The script will periodically check the specified URL and print messages indicating whether the application or system is online or offline. You can modify the code to add your desired notification logic.

## Customization

Feel free to customize the `target_url` and `sleep_duration` variables to fit your requirements. You can also implement additional logic for sending notifications in case of an offline status.

## Contributing

Contributions are welcome! If you find any issues or want to improve the script, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
