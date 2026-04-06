use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)] // automatocally generates code that allows struct to be converted into JSON
struct Document {
    id: u32,
    title: String,
    compliant: bool,
}

async fn root() -> &'static str {
    "API is running"
}

// GET "/"
/*
    JSON                                          <  Vec<Document                      >>
    wraps data so it becomes a HTTP JSON response < | a vector list of Document sructs >>
*/
async fn get_documents() -> Json<Vec<Document>> {
    let docs = vec![
        Document {
            id: 1,
            title: "Doc A".into(),
            compliant: true,
        },
        Document {
            id: 2,
            title: "Doc B".into(),
            compliant: false,
        },
        Document {
            id: 3,
            title: "Doc C".into(),
            compliant: true,
        },
    ];
    Json(docs)
}

// GET "/documents/compliant"
async fn get_compliant_documents() -> Json<Vec<Document>> {
    let docs = vec![
        Document {
            id: 1,
            title: "Doc A".into(),
            compliant: true,
        },
        Document {
            id: 2,
            title: "Doc B".into(),
            compliant: false,
        },
        Document {
            id: 3,
            title: "Doc C".into(),
            compliant: true,
        },
    ];
    /*
        -   .into_iter(): turns vector into an interator, processes each element
        -   .filter(|doc| doc.compliant): keeps documents where compliant=true
        -   .collect(): converts filtered results back into Vec<Document>
    */
    let filtered: Vec<Document> = docs.into_iter().filter(|doc| doc.compliant).collect();

    Json(filtered)
}

#[tokio::main] // starts tokio async runtime
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/documents", get(get_documents))
        .route("/documents/compliant", get(get_compliant_documents));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
