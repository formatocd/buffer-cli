use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::io::{self, Write};

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = prompt("Introduce tu API Key de Buffer: ");
    
    let client = Client::new();
    let api_url = "https://api.buffer.com";

    let org_query = json!({
        "query": "query GetOrganizations {\n  account {\n    organizations {\n      id\n      name\n      ownerEmail\n    }\n  }\n}"
    });

    let res_orgs: Value = client
        .post(api_url)
        .bearer_auth(&api_key)
        .json(&org_query)
        .send()
        .await?
        .json()
        .await?;

    let organizations = res_orgs["data"]["account"]["organizations"]
        .as_array()
        .ok_or("No se encontraron organizaciones o el formato de respuesta es incorrecto.")?;

    if organizations.is_empty() {
        println!("No se encontraron organizaciones en esta cuenta.");
        return Ok(());
    }

    let selected_org_id = if organizations.len() == 1 {
        organizations[0]["id"].as_str().unwrap().to_string()
    } else {
        println!("\nSe han encontrado varias organizaciones:");
        for (index, org) in organizations.iter().enumerate() {
            let name = org["name"].as_str().unwrap_or("Sin nombre");
            let id = org["id"].as_str().unwrap_or("Sin ID");
            println!("{}. {} (ID: {})", index + 1, name, id);
        }

        let mut choice = 0;
        while choice == 0 || choice > organizations.len() {
            let input = prompt(&format!("Indica el número de la organización que quieres usar (1-{}): ", organizations.len()));
            choice = input.parse::<usize>().unwrap_or(0);
        }

        organizations[choice - 1]["id"].as_str().unwrap().to_string()
    };

    println!("\nUsando la organización con ID: {}", selected_org_id);

    let channels_query = json!({
        "query": format!("query GetChannels {{\n  channels(input: {{\n    organizationId: \"{}\"\n  }}) {{\n    id\n    name\n    displayName\n    service\n    avatar\n    isQueuePaused\n  }}\n}}", selected_org_id)
    });

    let res_channels: Value = client
        .post(api_url)
        .bearer_auth(&api_key)
        .json(&channels_query)
        .send()
        .await?
        .json()
        .await?;

    let channels = res_channels["data"]["channels"]
        .as_array()
        .ok_or("No se encontraron canales o el formato de respuesta es incorrecto.")?;

    println!("\n--- Listado de Canales ---");
    if channels.is_empty() {
        println!("No hay canales asociados a esta organización.");
    } else {
        for channel in channels {
            let service = channel["service"].as_str().unwrap_or("Desconocido");
            let id = channel["id"].as_str().unwrap_or("Sin ID");
            println!("{}: {}", service, id);
        }
    }
    println!("--------------------------\n");

    Ok(())
}