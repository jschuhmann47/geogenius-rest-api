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
    "long_description": "El Cabildo de Buenos Aires es un edificio histórico que fue sede del gobierno colonial y donde se llevaron a cabo importantes eventos durante la Revolución de Mayo de 1810, que marcaron el comienzo de la independencia de Argentina. Hoy, funciona como museo, exhibiendo colecciones relacionadas con la historia y la cultura del país.",
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
    "long_description": "Las Violetas es una cafetería histórica en el barrio de Almagro, conocida por su arquitectura Art Deco y su exquisita oferta de pasteles, tortas y café. Fundada en 1884, ha sido un punto de encuentro para generaciones de porteños, ofreciendo un ambiente clásico y acogedor. Es ideal tanto para disfrutar de un café por la tarde como para degustar su famosa pastelería.",
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
    "long_description": "El Parque Patricios es un gran parque urbano en el sur de Buenos Aires, que ofrece amplias áreas de césped, senderos para caminar y espacios para deportes y recreación. Además de su belleza natural, el parque alberga el Club Huracán, uno de los equipos de fútbol más populares de la ciudad. Es un lugar ideal para un paseo en familia o para disfrutar de una tarde al aire libre.",
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
  },
  {
    "id": "4",
    "name": "Café Tortoni",
    "description": "El café más antiguo de Buenos Aires, con una rica historia cultural.",
    "long_description": "El Café Tortoni, fundado en 1858, es uno de los cafés más emblemáticos de Buenos Aires, conocido por su arquitectura clásica y su conexión con la cultura argentina. Ha sido un punto de encuentro para artistas, escritores y músicos. Su ambiente tradicional y su historia lo convierten en un lugar imperdible para los amantes del café.",
    "address": "Av. de Mayo 825, Buenos Aires",
    "rating": 4.6,
    "images": [
      "https://aguiarbuenosaires.com/wp-content/uploads/2015/09/Cafe-Tortoni-Buenos-Aires.jpg"
    ],
    "coordinates": {
      "x": -34.611256,
      "y": -58.380106
    },
    "type": "cafe"
  },
  {
    "id": "5",
    "name": "Museo Nacional de Bellas Artes",
    "description": "Uno de los museos más importantes de Argentina, con una vasta colección de arte.",
    "long_description": "El Museo Nacional de Bellas Artes de Buenos Aires alberga una de las colecciones de arte más completas de Argentina, con obras que van desde el Renacimiento hasta el arte contemporáneo. Sus salas exponen pinturas, esculturas y objetos de artistas nacionales e internacionales, siendo un lugar clave para los amantes del arte.",
    "address": "Av. del Libertador 1473, Buenos Aires",
    "rating": 4.8,
    "images": [
      "https://www.lugaresturisticosdeargentina.com/wp-content/uploads/2017/02/El-Museo-Nacional-de-Bellas-Artes-1.jpg"
    ],
    "coordinates": {
      "x": -34.587197,
      "y": -58.395162
    },
    "type": "museum"
  },
  {
    "id": "6",
    "name": "Parque Tres de Febrero",
    "description": "Un amplio parque urbano en Buenos Aires, ideal para paseos y actividades al aire libre.",
    "long_description": "El Parque Tres de Febrero, también conocido como Parque de los Pringles, es un extenso parque ubicado en el barrio de Palermo. Con grandes lagos, senderos, jardines y una famosa rosaleda, es un lugar ideal para caminar, hacer deportes o simplemente relajarse en contacto con la naturaleza.",
    "address": "Av. del Libertador y Av. Sarmiento, Buenos Aires",
    "rating": 4.7,
    "images": [
      "https://turismo.buenosaires.gob.ar/sites/turismo/files/parque_tres_3_de_febrero_palermo_1200.jpg"
    ],
    "coordinates": {
      "x": -34.587512,
      "y": -58.437042
    },
    "type": "park"
  },
  {
    "id": "7",
    "name": "Café de los Angelitos",
    "description": "Un café tradicional de Buenos Aires, famoso por su historia y su tango.",
    "long_description": "El Café de los Angelitos es un emblemático café de Buenos Aires que ha sido un punto de encuentro de figuras de la cultura nacional. Inaugurado en 1890, fue escenario de numerosos eventos históricos y culturales, y hoy sigue siendo un lugar ideal para disfrutar de un café y ver espectáculos de tango.",
    "address": "Avenida Rivadavia 2100, Buenos Aires",
    "rating": 4.4,
    "images": [
      "https://media-cdn.tripadvisor.com/media/photo-s/0e/ac/2a/b6/facade-du-cafe-de-los.jpg"
    ],
    "coordinates": {
      "x": -34.612611,
      "y": -58.407814
    },
    "type": "cafe"
  },
  {
    "id": "8",
    "name": "Museo de Arte Moderno de Buenos Aires (MAMBA)",
    "description": "Un museo dedicado al arte moderno y contemporáneo.",
    "long_description": "El Museo de Arte Moderno de Buenos Aires es uno de los museos más importantes en su categoría, con una colección que abarca desde la primera mitad del siglo XX hasta la actualidad. Exhibe obras de artistas argentinos y extranjeros, y es un referente para los amantes del arte moderno y contemporáneo.",
    "address": "Av. San Juan 350, Buenos Aires",
    "rating": 4.7,
    "images": [
      "https://aguiarbuenosaires.com/wp-content/uploads/2019/11/DESTACADA-MAMBA.png"
    ],
    "coordinates": {
      "x": -34.625650,
      "y": -58.417412
    },
    "type": "museum"
  },
  {
    "id": "9",
    "name": "Parque Centenario",
    "description": "Un parque urbano en el corazón de Buenos Aires, perfecto para paseos y actividades.",
    "long_description": "El Parque Centenario es uno de los parques más grandes y populares de Buenos Aires, ubicado en el barrio de Caballito. Con amplias áreas verdes, un lago artificial y diversas esculturas, es un lugar ideal para hacer deporte, pasear en familia o disfrutar de un día al aire libre.",
    "address": "Av. Lillo y Av. Diaz Velez, Buenos Aires",
    "rating": 4.6,
    "images": [
      "https://turismo.buenosaires.gob.ar/sites/turismo/files/parque_centenario_caballito_1200.jpg"
    ],
    "coordinates": {
      "x": -34.609014,
      "y": -58.439437
    },
    "type": "park"
  },
  {
    "id": "10",
    "name": "Café San Juan",
    "description": "Un café tradicional conocido por su ambiente acogedor y su gastronomía.",
    "long_description": "Café San Juan es un tradicional café en el barrio de San Telmo, conocido por su ambiente cálido y su gastronomía que combina sabores clásicos con toques modernos. Es ideal para disfrutar de un buen café o un almuerzo mientras se disfruta de un paseo por uno de los barrios más característicos de Buenos Aires.",
    "address": "Calle San Juan 600, Buenos Aires",
    "rating": 4.5,
    "images": [
      "https://4.bp.blogspot.com/_EfPi4Px-w9w/S8-tZ_-LNZI/AAAAAAAAA0Y/9Z7RJqmqWpU/s1600/P1110995.JPG"
    ],
    "coordinates": {
      "x": -34.617446,
      "y": -58.375721
    },
    "type": "cafe"
  }
]
"#;
    // Return the raw JSON string with the correct content-type header
    Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Full::from(json_str))
        .unwrap()
}
