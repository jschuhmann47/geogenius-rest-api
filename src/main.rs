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
          "description": "Uno de los edificios más emblemáticos de la historia argentina.",
          "long_description": "El Cabildo de Buenos Aires es un edificio histórico que fue sede del gobierno colonial y donde se llevaron a cabo importantes eventos durante la Revolución de Mayo de 1810, que marcaron el comienzo de la independencia de Argentina. Hoy, funciona como museo, exhibiendo colecciones relacionadas con la historia y la cultura del país."
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
          "description": "Un tradicional café de Buenos Aires, famoso por su ambiente único y su repostería.",
          "long_description": "Las Violetas es una cafetería histórica en el barrio de Almagro, conocida por su arquitectura Art Deco y su exquisita oferta de pasteles, tortas y café. Fundada en 1884, ha sido un punto de encuentro para generaciones de porteños, ofreciendo un ambiente clásico y acogedor. Es ideal tanto para disfrutar de un café por la tarde como para degustar su famosa pastelería."
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
          "description": "Un gran espacio verde en el corazón del barrio de Parque Patricios.",
          "long_description": "El Parque Patricios es un gran parque urbano en el sur de Buenos Aires, que ofrece amplias áreas de césped, senderos para caminar y espacios para deportes y recreación. Además de su belleza natural, el parque alberga el Club Huracán, uno de los equipos de fútbol más populares de la ciudad. Es un lugar ideal para un paseo en familia o para disfrutar de una tarde al aire libre."
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
