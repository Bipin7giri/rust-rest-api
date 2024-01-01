use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /api => 200 OK with body {"name":"John","address":"Doe"}
    let api = warp::path("api")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"name": "John", "address": "Doe"})));

    // GET /me => 200 OK with body "hello"
    let me = warp::path("me")
        .and(warp::get())
        .map(|| warp::reply::html("<h1>Bipin</h1>"));

    // GET /portfolio => JSON response with portfolio details
    let getportfolio = warp::path("portfolio")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({
            "_id": {
              "$oid": "6591a2545713e65268fcfe17"
            },
            "name": "Bipin Giri",
            "title": "FullStack Developer",
            "contact": {
              "phone": "9847050632",
              "email": "bipingiri27@gmail.com",
              "location": "Kathmandu",
              "linkedin": "https://www.linkedin.com/in/bipin-giri-8005b2267/",
              "github": "https://github.com/Bipin7giri",
              "portfolio": "http://bipingiri77.com.np"
            },
            "summary": "Organized and dependable candidate successful at managing multiple priorities with a positive attitude. Willingness to take on added responsibilities to meet team goals. Hardworking and passionate job seeker with strong organizational skills eager to secure entry-level junior positon",
            "education": {
              "degree": "Bachelors in Computer Application",
              "college": "Kathford College of Engineering And Management",
              "location": "Kathmandu",
              "date": "Jan, 2020 - Present"
            },
            "workExperience": [
              {
                "title": "FullStack Developer",
                "company": "Phalano Job",
                "location": "Kathmandu",
                "dateRange": {
                  "start": "Sep, 2023",
                  "end": "Present"
                },
                "description": "Phalano Job is an innovative job hiring platform...",
                "technologies": [
                  "AWS API Gateway",
                  "Cognito",
                  "Lambda",
                  "S3",
                  "GraphQL",
                  "CloudWatch",
                  "EC2",
                  "CDK",
                  "DynamoDB",
                  "Next.js",
                  "TypeScript"
                ]
              },
              {
                "title": "FullStack Developer",
                "company": "Saral Lagani(Stock Management)",
                "location": "Kathmandu",
                "dateRange": {
                  "start": "Sep, 2022",
                  "end": "Sep, 2023"
                },
                "description": "As a fullstack developer, I had the opportunity to gain experience...",
                "technologies": [
                  "Express",
                  "JavaScript",
                  "Redis",
                  "PostgreSQL",
                  "VPS",
                  "React",
                  "Next.js",
                  "Redux",
                  "CICD",
                  "Swagger UI"
                ]
              }
            ],
            "trainingCertifications": [
              {
                "title": "MERN STACK",
                "provider": "Broadway Infosys",
                "year": "2022"
              }
            ],
            "projects": [
              {
                "title": "Saral Lagani",
                "description": "Saral Lagani is a versatile stock portfolio management system...",
                "link": "https://sarallagani.com"
              },
              {
                "title": "NodeJS CLI - API Endpoint Generator NPM Package (Open source)",
                "description": "NodeJS CLI - a powerful tool designed to simplify the process of creating API endpoints...",
                "link": "https://www.npmjs.com/package/express_boilerplate_cli?activeTab=readme#search"
              },
              {
                "title": "Salesberry",
                "description": "Discover an exceptional e-commerce platform developed using the LoopBack framework...",
                "link": "https://www.salesberry.com.np/"
              }
            ]
          })));

    // Combine the filters for all endpoints
    let routes = api.or(me).or(getportfolio);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
