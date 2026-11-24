
use crate::rede::{interface_rede_atual, rede_mac, rede_ip};
use crate::disco::{disco_lista};
use crate::MONITOR;
use crate::INFO_RPANEL;
use std::fs;
use std::env;
use crate::{
    modelos::{DadosSobre, ListaDisco},
};

use sysinfo::System;

//Verifica idioma local sistema linux
fn idioma_sistema() -> String {
    let locale = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|variavel| {
            std::env::var(variavel)
                .ok()
                .filter(|valor| !valor.trim().is_empty())
        })
        .unwrap_or_else(|| "C".to_string());

    // Ex.: "pt_BR.UTF-8" vira "pt_BR".
    locale
        .split(|caractere| caractere == '.' || caractere == '@')
        .next()
        .unwrap_or("C")
        .to_string()
}

//Ler arquivo DMI:
fn ler_dmi(campo: &str) -> String {
    fs::read_to_string(format!("/sys/class/dmi/id/{campo}"))
        .map(|valor| valor.trim().to_string())
        .ok()
        .filter(|valor| !valor.is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn sistema_janelas() -> String {
    match env::var("XDG_SESSION_TYPE").as_deref() {
        Ok("wayland") => "Wayland".to_string(),
        Ok("x11") => "X11".to_string(),
        Ok(tipo) => tipo.to_string(),
        Err(_) => {
            if env::var_os("WAYLAND_DISPLAY").is_some() {
                "Wayland".to_string()
            } else if env::var_os("DISPLAY").is_some() {
                "X11".to_string()
            } else {
                "-".to_string()
            }
        }
    }
}



#[tauri::command]
pub(crate) fn info() -> DadosSobre{
    let mut dados = DadosSobre {
        nome_os: "".to_string(),
        versao_os: "".to_string(),
        kernel: "".to_string(),
        sistema_janelas: "".to_string(),
        nome_host: "".to_string(),
        idioma: "".to_string(),
        interface_rede: "".to_string(),
        mac: "".to_string(),
        ip: "".to_string(),
        fabricante_placa_mae: "".to_string(),
        modelo_placa_mae: "".to_string(),
        versao_firmware: "".to_string(),
        discos: ListaDisco {lista: Vec::new(),},
        sobre_rpanel: INFO_RPANEL.clone(),
    };
    

    dados.nome_os = System::name().unwrap_or_else(|| "".to_string());
    dados.versao_os = System::os_version().unwrap_or_else(|| "".to_string());
    dados.kernel = System::kernel_version().unwrap_or_else(|| "".to_string());
    dados.sistema_janelas=sistema_janelas();
    dados.nome_host = System::host_name().unwrap_or_else(|| "".to_string());
    dados.idioma = idioma_sistema();


    let interface = interface_rede_atual();


    dados.interface_rede = interface.clone().unwrap_or_else(|| "".to_string());

    {
        let monitor = MONITOR.get().unwrap().lock().unwrap();

        dados.mac = rede_mac(&monitor.redes, interface.clone());
        dados.ip = rede_ip(&monitor.redes, interface);
    }

    dados.fabricante_placa_mae=ler_dmi("board_vendor");
    dados.modelo_placa_mae=ler_dmi("board_name");
    dados.versao_firmware=ler_dmi("bios_version");
    
    let monitor = MONITOR.get().unwrap().lock().unwrap();
    dados.discos.lista = disco_lista(&monitor.discos);

    dados

}