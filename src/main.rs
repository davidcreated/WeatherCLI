use std::io;
use serde::Deserialize;
use colored::*;

// struct to deserializae the JSON response from the open weather map api


#[derive(Debug, Deserialize)]

struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}


// Struct to represent weather information
#[derive(Debug, Deserialize)]

struct Weather{
    description: String,
}


// struct to represent the main weather data

#[derive(Debug, Deserialize)]

struct Main{
    temp:f64,
    humidity: f64,
    pressure: f64,
}

// struct to rep wind rep

#[derive(Debug, Deserialize)]

struct Wind{
    speed: f64,

}

// Function to get the weather info from openWeatherMap API


fn get_weather_info(city: &str, country_code: &str, api_key: &str) ->
Result<WeatherResponse, reqwest::Error>{
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?.json::<WeatherResponse>()?;
    Ok(response)
}

// function to display weather info 

fn display_weather_info(response : &WeatherResponse){
     // Extract the weather information from the response

     let weather_description = &response.weather[0].description;
     let temperature: f64 = response.main.temp;
     let humidity: f64 = response.main.humidity;
     let pressure: f64 = response.main.pressure;
     let wind_speed: f64 = response.wind.speed;

     // formatting weather info into a string 

     let weather_text: String = format!(
         "Weather: {}\nTemperature: {:.1}°C\nHumidity: {:.1}%\nPressure: {:.1} hPa\nWind Speed: {:.1} m/s",
         weather_description, temperature, humidity, pressure, wind_speed



     );
     println!("{}", weather_text);


     /// function to get emoji based on temperature 
      fn get_temp_emoji(temprature : f64) -> &'static str{
          if temprature < 0.0 {
              "❄️"
          } else if temprature < 10.0 {
              "🧥"
          } else if temprature < 20.0 {
              "🌤️"
          } else if temprature < 30.0 {
              "☀️"
          } else {
              "🔥"
          }
      }
 }


 fn main (){
    println!("{}", "Welcome to the Weather CLI!".green().bold());
    loop{ 

        // city 
        println!("{}", "Please enter a city and country code (e.g. London, GB):".blue().bold());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read line");
    let city = city.trim();


    // country 
    println!("{}", "Please enter a country code (e.g. GB):".blue().bold());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read line");
        let country_code = country_code.trim();

        // api key 
        println!("{}", "Please enter your OpenWeatherMap API key:".blue().bold());
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key).expect("Failed to read line");
        let api_key = api_key.trim();

        // get weather info 
        match get_weather_info(city, country_code, api_key){
            Ok(response) => display_weather_info(&response),
            Err(e) => println!("Error: {}", e),
        }
    }

 }