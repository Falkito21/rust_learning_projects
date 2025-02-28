use reqwest::header;
use serde_json::{ Value, json};

/*permite que la funcion main pueda manejar asincronismo*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 

    /*configuramos los headers*/
    let mut headers = header::HeaderMap::new();

    headers.insert("Accept-Encoding", header::HeaderValue::from_static("application/gzip"));
    headers.insert("X-RapidAPI-Key", header::HeaderValue::from_static("39407343b7mshc77a9e6b3e554b5p1fcba0jsnf5fedc18fc0d"));
    headers.insert("X-RapidAPI-Host", header::HeaderValue::from_static("google-translate113.p.rapidapi.com"));


    /*creamos un cliente el cual se puede utilizar para muchas peticiones*/
    /*let cliente = reqwest::Client::new();*/
    let cliente = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let url_get = "https://google-translate113.p.rapidapi.com/api/v1/translator/support-languages";

    let r_get = cliente.get(url_get)
        .send()
        .await?;

    println!("Estatus de respuesta: ");
    println!("{}", r_get.status().as_u16());
    println!("--------------------------");

    let resp_json = r_get.json::<Value>().await?;

    for (index, respuesta) in resp_json.as_array().unwrap().iter().enumerate()  {
        if index == 5 {break;}
        println!("{}", respuesta["language"].as_str().unwrap());
    }

    let mut headers_post = header::HeaderMap::new();

    headers_post.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers_post.insert("X-RapidAPI-Key", header::HeaderValue::from_static("39407343b7mshc77a9e6b3e554b5p1fcba0jsnf5fedc18fc0d"));
    headers_post.insert("X-RapidAPI-Host", header::HeaderValue::from_static("google-translate113.p.rapidapi.com"));

    let body = json!({
        "from": "auto",
        "to": "vi",
        "protected_paths": ["extra.last_comment.author"],
        "common_protected_paths": ["image"],
        "json": {
            "title": "The Importance of Regular Exercise",
            "author": "John Doe",
            "rate": 4.2999,
            "extra": {
                "image": "hello.jpg",
                "comment_counts": 10,
                "last_comment": {
                    "author": "not be translated",
                    "short_text": "Hi thank for your post... We need more information"
                }
            }
        }
    });

    println!("----------------------------");
    println!("Realizamos peticion POST");

    let url_post = "https://google-translate113.p.rapidapi.com/api/v1/translator/json";

    let cliente_post = reqwest::Client::new()
        .post(url_post)
        .headers(headers_post)
        .json(&body)
        .send()
        .await?;


    println!("Estatus de respuesta: ");
    println!("{}", cliente_post.status().as_u16());
    println!("--------------------------");

    let resp_json = cliente_post.json::<Value>().await?;
    println!("Respuesta en formato JSON: ");
    println!("{:?}", resp_json);

    Ok(())
}
