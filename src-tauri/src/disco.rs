use std::{ path::Path, thread, time::Duration};

use sysinfo::{Components, DiskKind, Disks, Networks, System};
use tauri::Emitter;

use crate::{
    modelos::{DadosDisco, ListaDisco, Disco},
};

const GB: f64 = 1_073_741_824.0;
const MB: f64 = 1_048_576.0;

// Identifica o tipo do disco.
fn tipo_disco(disco: &sysinfo::Disk) -> &'static str {
    let nome = disco.name().to_string_lossy().to_ascii_lowercase();

    if nome.contains("nvme") {
        return "NVMe";
    }

    match disco.kind() {
        DiskKind::HDD => "HD",
        DiskKind::SSD => "SSD",
        DiskKind::Unknown(_) => "Indefinido",
    }
}


pub(crate) fn disco_os_data(discos: &Disks) -> DadosDisco {
    let mut dados_disco = DadosDisco {
        tipo: "Indefinido".to_string(),
        total: 0.0,
        usada: 0.0,
        restante: 0.0,
        uso_percentual: 0,
    };

    if let Some(disco) = discos.list().iter()
        .find(|disco| disco.mount_point() == Path::new("/"))
    {
        let total = disco.total_space();
        let disponivel = disco.available_space();
        let usado = total.saturating_sub(disponivel);

        dados_disco = DadosDisco {
            tipo: tipo_disco(disco).to_string(),
            total: total as f64,
            usada: usado as f64,
            restante: disponivel as f64,
            uso_percentual: if total > 0 {
                (usado as f64 / total as f64 * 100.0).round() as u8
            } else {
                0
            },
        };
    }

    dados_disco
}


pub(crate) fn disco_lista(discos: &Disks) -> Vec<Disco> {
    let mut lista = Vec::new();

    for disco in discos.list() {
        let tipo = tipo_disco(disco);
        let montagem = disco.mount_point().display().to_string();

        if tipo != "Indefinido" && montagem != "/boot/efi" {
            let total = disco.total_space();
            let disponivel = disco.available_space();
            let usado = total.saturating_sub(disponivel);

            lista.push(Disco {
                dispositivo: disco.name().to_string_lossy().to_string(),
                montagem,
                tipo: tipo.to_string(),
                total: total as f64,
                usada: usado as f64,
                restante: disponivel as f64,
                uso_percentual: if total > 0 {
                    (usado as f64 / total as f64 * 100.0).round() as u8
                } else {
                    0
                },
            });
        }
    }

    lista
}