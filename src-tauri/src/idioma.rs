use std::fs;
use serde;
use crate::{modelos::{Labels, Config}};
use tauri::Manager;


pub(crate) fn config_idioma(app: tauri::AppHandle)->String{
    let caminho = app.path()
    .resource_dir().expect("ERRO: Caminho config")
    .join("RPanelData")
    .join("config")
    .join("config.json");

    let dados=fs::read_to_string(&caminho).unwrap();

    let config: Config = serde_json::from_str(&dados).unwrap();

    config.idioma.to_string()
}


#[tauri::command]
pub(crate) fn idioma(app: tauri::AppHandle)->Labels{
    let mut idioma = config_idioma(app.clone());
    if idioma == "" || idioma == "indefinido" || idioma == "indefined"{
        idioma="ingles".to_string();
    }
    let arquivo_idioma = format!("{}.json", idioma);
    let caminho = app.path()
    .resource_dir().expect("ERRO: Caminho idiomas")
    .join("RPanelData")
    .join("idiomas")
    .join(arquivo_idioma);
    let caminho_padrao = app.path()
    .resource_dir().expect("ERRO: Caminho idiomas padrão")
    .join("RPanelData")
    .join("idiomas")
    .join("English.json");

    //println!("Caminho do idioma: {}", caminho.display());
    //println!("Arquivo existe: {}", caminho.exists());

    let dados=fs::read_to_string(&caminho).unwrap();
    let dados_padrao=fs::read_to_string(&caminho_padrao).unwrap();

    //let labels: Labels = serde_json::from_str(&dados).unwrap();

    let labels: Labels = match serde_json::from_str(&dados) {
    Ok(labels) => labels,
    Err(e) => {
        eprintln!("Erro ao interpretar JSON: {}", e);
        serde_json::from_str(&dados_padrao).unwrap()
    }
};
labels

}

#[tauri::command]
pub(crate) fn lista_idiomas(app: tauri::AppHandle)->Vec<String>{
    let caminho = app.path()
    .resource_dir().expect("ERRO: Caminho idiomas")
    .join("RPanelData")
    .join("idiomas");

    let dados = fs::read_dir(&caminho).unwrap();
    let mut idiomas: Vec<String> = Vec::new();


    for d in dados {
        let nome = d.unwrap().file_name().to_string_lossy().to_string();
        if nome.contains(".json") {
            idiomas.push(nome.split(".json").next().unwrap().to_string());
        }
    }
    
    

    idiomas

}

#[tauri::command]
pub(crate) fn mudar_idioma(app: tauri::AppHandle, idioma: String){
    let caminho = app.path()
    .resource_dir().expect("ERRO: Caminho idiomas")
    .join("RPanelData")
    .join("config")
    .join("config.json");

    let dados = fs::read_to_string(&caminho).unwrap();
    let mut config: Config = serde_json::from_str(&dados).unwrap();
    config.idioma=idioma;
    let c = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&caminho, c).unwrap();

}