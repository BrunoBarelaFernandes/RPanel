use std::{ path::Path, thread, time::Duration};
use std::process::Command;
use sysinfo::{Components, DiskKind, Disks, Networks, System};
use tauri::Emitter;
use reqwest;

use crate::{
    modelos::{DadosRede},
};


const GB: f64 = 1_073_741_824.0;
const MB: f64 = 1_048_576.0;

pub(crate) fn indicador_internet(app: tauri::AppHandle){ 
    
    loop {
        thread::sleep(Duration::from_secs(5));
        let resposta = reqwest::blocking::get("https://google.com");
        
        match resposta {
            Ok(resposta) => {
                //println!("Status: {}", resposta.status()); 
                let _ = app.emit("IndicadorInternet", true);   
            }

            Err(erro) => {
                //println!("Erro na conexão: {}", erro);
                let _ = app.emit("IndicadorInternet", false);
            }
        }
    }
}



// Descobre a interface que o sistema usa para alcançar a internet.
pub(crate) fn interface_rede_atual() -> Option<String> {
    let saida = Command::new("ip")
        .args(["route", "get", "1.1.1.1"])
        .output()
        .ok()?;

    if !saida.status.success() {
        return None;
    }

    let texto = String::from_utf8_lossy(&saida.stdout);
    let partes: Vec<&str> = texto.split_whitespace().collect();
    let indice_dev = partes.iter().position(|parte| *parte == "dev")?;
    partes.get(indice_dev + 1).map(|nome| nome.to_string())
}



//Pega MAC atual
pub(crate) fn rede_mac(interface_atual: Option<String>) -> String{

    let redes = Networks::new_with_refreshed_list();
    let mac = interface_atual
       .and_then(|nome| {
        redes.get(nome.as_str())
        .map(|dados| dados.mac_address().to_string())
    }).unwrap_or_else(|| "Indisponível".to_string());

    mac
}

//Pega IP atual
pub(crate) fn rede_ip(interface_atual: Option<String>) -> String{
    
    let redes = Networks::new_with_refreshed_list();
    let ip = interface_atual
        .and_then(|nome| {
            redes.get(nome.as_str()).and_then(|dados| {
                dados
                   .ip_networks()
                   .iter()
                   .find(|rede| rede.addr.is_ipv4())
                   .map(|rede| rede.addr.to_string())
                })
            }).unwrap_or_else(|| "Indisponível".to_string());

    ip
}


pub(crate) fn rede_rx_tx(interface_atual: Option<String>)->DadosRede{

    let mut redes = Networks::new_with_refreshed_list();
    redes.refresh(true);

    let mut rede_d = DadosRede {
        interface: "Desconhecido".to_string(),
        download: 0.0,
        upload: 0.0,
    };
    if let Some(nome) = interface_atual.as_deref() {
        if let Some(dados) = redes.get(nome) {
            let download = dados.received() as f64 / MB;
            let upload = dados.transmitted() as f64 / MB;

            rede_d.interface = nome.to_string();
            rede_d.download = download;
            rede_d.upload = upload;

                //println!("Interface: {nome}");
                //println!("Download: {download} MB");
                //println!("Upload: {upload} MB");
        }
    }
    rede_d
}
