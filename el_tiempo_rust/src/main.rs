use std::collections::HashMap;
use std::io;
use serde::Deserialize;
use colored::*;

// Estructura para deserializar la respuesta json de la API de openWeatherMap (https://openweathermap.org)
#[derive(Deserialize, Debug)]
struct RespuestaTiempo {
    weather: Vec<Tiempo>,
    main: Main,
    wind: Viento,
    name: String,
}

// Estructura para representar la descripción del tiempo
#[derive(Deserialize, Debug)]
struct Tiempo {
    description: String,
}

// Estructura para representar los parametros principales del tiempo
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Estructura para representar la informacion del viento
#[derive(Deserialize, Debug)]
struct Viento {
    speed: f64,
}

// Función para obtener la información del tiempo de la API de OpenWeatherMap
fn info_tiempo(ciudad: &str, codigo_pais: &str, llave_api: &str) -> Result<RespuestaTiempo, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        ciudad, codigo_pais, llave_api
    );

    let respuesta = reqwest::blocking::get(&url)?;
    let respuesta_json: RespuestaTiempo = respuesta.json::<RespuestaTiempo>()?;
    Ok(respuesta_json)
}

// Función para traducir descripciones del clima con HashMap
fn traducir_descripcion(ingles: &str) -> String {
    let mut traducciones = HashMap::new();
    traducciones.insert("clear sky", "cielo despejado");
    traducciones.insert("few clouds", "pocas nubes");
    traducciones.insert("scattered clouds", "nubes dispersas");
    traducciones.insert("broken clouds", "nubes rotas");
    traducciones.insert("overcast clouds", "cielo nublado");
    traducciones.insert("mist", "neblina");
    traducciones.insert("haze", "calina");
    traducciones.insert("smoke", "humo");
    traducciones.insert("sand", "arena");
    traducciones.insert("dust", "polvo");
    traducciones.insert("fog", "niebla");
    traducciones.insert("shower rain", "lluvia intensa");
    traducciones.insert("rain", "lluvia");
    traducciones.insert("thunderstorm", "tormenta eléctrica");
    traducciones.insert("snow", "nieve");
    traducciones.insert("squalls", "ráfagas de viento");
    traducciones.insert("tornado", "tornado");

    traducciones.get(ingles).unwrap_or(&ingles).to_string()
}

// Función para mostrar la información del tiempo
fn mostrar_info_tiempo(respuesta: &RespuestaTiempo) {
    let descripcion_en = &respuesta.weather[0].description;
    let descripcion_es = traducir_descripcion(descripcion_en);
    
    let temperatura: f64 = respuesta.main.temp;
    let humedad: f64 = respuesta.main.humidity;
    let presion_atm: f64 = respuesta.main.pressure;
    let velocidad: f64 = respuesta.wind.speed;
    let velocidad_kmh: f64 = velocidad * 3.6;

    let tiempo_texto: String = format!(
        "El Tiempo en {}: {} {}
        > Temperatura: {:.1}ºC
        > Humedad: {:.1}%
        > Presión atmosférica: {:.1} hPa
        > Velocidad del viento: {:.1} km/h",
        respuesta.name,
        descripcion_es,
        temp_emoji(temperatura),
        temperatura,
        humedad,
        presion_atm,
        velocidad_kmh
    );
// coloreando el texto del tiempo basado en las condiciones del tiempo
    let tiempo_coloreado = match descripcion_en.as_str() {
        "clear sky" => tiempo_texto.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => tiempo_texto.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => tiempo_texto.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => tiempo_texto.bright_cyan(),
        _ => tiempo_texto.normal(),
    };

    println!("{}", tiempo_coloreado);
}

// funcion emoji :)
fn temp_emoji(temperatura: f64) -> &'static str {
    if temperatura < 0.0 {
        "❄️"
    } else if temperatura < 10.0 {
        "☁️"
    } else if temperatura < 20.0 {
        "⛅"
    } else if temperatura < 30.0 {
        "🌤️"
    } else if temperatura < 35.0 {
        "☀️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "¡Bienvenide a la Estación Meteorológica!".bright_yellow());

    loop {
        // leyendo ciudad
        println!("{}", "Por favor introduzca el nombre de la ciudad:".bright_magenta());
        let mut ciudad = String::new();
        io::stdin().read_line(&mut ciudad).expect("Error de lectura del input :s");
        let ciudad = ciudad.trim();

        // fraga addition :p
        let ciudad_consulta = if ciudad.eq_ignore_ascii_case("La Farga de Bebié") || ciudad.eq_ignore_ascii_case("fraga bby"){
            "Ripoll"
        } else {
            ciudad
        };
        // leyendo pais
        println!("{}", "Por favor introduzca el código del país (por ejemplo 'PS' para Palestina o 'SY' para Siria):".bright_magenta());
        let mut codigo_pais = String::new();
        io::stdin().read_line(&mut codigo_pais).expect("Error leyendo el input :s");
        let codigo_pais = codigo_pais.trim();
        // llave api
        let llave_api = "b3962c53d2fbd64d04e7bd1e5fcf6ca4";

        // llamando a la funcion de info del tiempo
        match info_tiempo(ciudad_consulta, codigo_pais, llave_api) {
            Ok(mut respuesta) => {
                // que aparezca la fraga en pantalla xd
                if ciudad.eq_ignore_ascii_case("La Farga de Bebié") {
                    respuesta.name = "La Farga de Bebié".to_string();
                } else if ciudad.eq_ignore_ascii_case("fraga bby"){
                    respuesta.name = "fraga bby".to_string();
                }
                mostrar_info_tiempo(&respuesta);
            }
            Err(err) => eprintln!("Error: {}", err),
        }

        println!("{}", "¿Quieres ver el tiempo de otra ciudad? (si/no):".bright_magenta()); //para que el usuario pueda continuar o salir
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fallo al leer el input :s");
        let input = input.trim().to_lowercase();

        if input != "si" {
            println!("{}", "¡Gracias por usar el programa! :3".bright_magenta());
            break; //saliendo del loop si el usuario quiere salir del programa :p
        }
    }
}
