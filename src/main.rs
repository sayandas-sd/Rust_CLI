use std::io;
use serde::Deserialize;
use colored::*;
use reqwest;


#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    name: String,
    wind: Wind,
}


#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    pressure: f64,
    humidity: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn weather_information(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    
    let response = reqwest::blocking::get(&url)?;
   
    let response_json: WeatherResponse = response.json()?;

   
    Ok(response_json)
}

fn weather_display(response: &WeatherResponse) {
    let description: &String = &response.weather[0].description;
    let humidity: f64 = response.main.humidity;
    let temperature: f64 = response.main.temp;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    let weather_text: String = format!(
        "Weather in {}: {} {}\n\
        -> Temperature: {:.1}°C\n\
        -> Pressure: {:.1} hPa\n\
        -> Wind Speed: {:.1} m/s\n\
        -> Humidity: {:.1}%",
        response.name,
        description,
        get_emoji(temperature),
        temperature,
        pressure,
        wind_speed,
        humidity,
    );

    let text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

   
    println!("{}", text_colored);
}

fn get_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅️"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to Rust Weather CLI".bright_yellow());

    loop {
        println!("{}", "Enter the name of the city: ".bright_green());
        let mut city = String::new();

        io::stdin().read_line(&mut city).expect("Write valid input!");
        let city: &str = city.trim();

        println!("{}", "Enter the country code (e.g., UK for United Kingdom):".bright_green());
        let mut country = String::new();
        io::stdin().read_line(&mut country).expect("Write valid input!");
        let country: &str = country.trim();

        // API key
        let api_key = "c2ca59e5fc6b4307a17a48597bfd87fb";
        match weather_information(city, country, api_key) {
            Ok(response) => {
                weather_display(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        println!("{}", "Do you want to search for another city? (y/n):".bright_green());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Write valid input!");
        let input = input.trim().to_lowercase();

        if input != "y" {
            println!("Thank you");
            break;
        }
    }
}
