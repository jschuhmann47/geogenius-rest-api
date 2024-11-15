use axum::{http::Response, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(return_json_string));

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn return_json_string() -> impl IntoResponse {
    let json_str = r#"[
        {
          "id": "1",
          "name": "Cabildo",
          "description": "Lugar histórico de Bs. As.",
          "long_description": "Este es un monumento que etc...",
          "address": "Bolivar 65",
          "rating": 4.7,
          "images": [
            "https://3.bp.blogspot.com/_ibvvg3ue1vE/TPaNMzFPOAI/AAAAAAAAAEk/2aWdti3-Xeg/s1600/IMG_3235.jpg"
          ],
          "coordinates": {
            "x": -34.608774370389014,
            "y": -58.37378778505391
          },
          "type": "museum"
        },
        {
          "id": "2",
          "name": "Las violetas",
          "description": "Café típico de Almagro",
          "long_description": "Este es un café que etc...",
          "address": "Av. Rivadavia 3899",
          "rating": 4.5,
          "images": [
            "https://www.edesur.com.ar/wordpress/wp-content/uploads/2022/05/Las-Violetas-TW-4-scaled.jpg"
          ],
          "coordinates": {
            "x": -34.61129366620751,
            "y": -58.42090720836726
          },
          "type": "cafe"
        },
        {
          "id": "3",
          "name": "Parque Patricios",
          "description": "Gran parque del barrio de Parque Patricios",
          "long_description": "Parque con club y lugares para comer...",
          "address": "Av. Caseros 3021",
          "rating": 4.5,
          "images": [
            "https://buenosairesconnect.com/wp-content/uploads/2021/08/Parque-Patricios-Buenos-Aires.jpg"
          ],
          "coordinates": {
            "x": -34.638192698002236,
            "y": -58.40729163420362
          },
          "type": "park"
        }
      ]"#;
    // Return the raw JSON string with the correct content-type header
    Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Full::from(json_str))
        .unwrap()
}
