use std::sync::OnceLock;
use std::{ path::Path, thread, time::Duration};
use std::process::Command;
use tauri::Emitter;
use crate::{modelos::{DadosSistemaRAM, DadosRam}};
use crate::MONITOR;




// Executa o udevadm e devolve as propriedades da memória como texto.
fn propriedades_memoria_udev() -> Option<String> {
    let saida = Command::new("udevadm")
        .args(["info", "-p", "/sys/devices/virtual/dmi/id"])
        .output()
        .ok()?;

    if !saida.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&saida.stdout).into_owned())
}



// Guarda os dados para que o udevadm rode apenas uma vez durante a execução do app.
static DADOS_SISTEMA: OnceLock<DadosSistemaRAM> = OnceLock::new();

fn carregar_dados_sistema_ram() -> DadosSistemaRAM {
    DADOS_SISTEMA.get_or_init(|| {
            // Começa com valores seguros caso o sistema não forneça os detalhes da RAM.
            let mut dados = DadosSistemaRAM {
                ddr: "Indisponível".to_string(),
                soquetes: "Indisponível".to_string(),
                instalados: "Indisponível".to_string(),
                velocidade: "Indisponível".to_string(),
            };

            let Some(propriedades) = propriedades_memoria_udev() else {
                return dados;
            };

            let mut pentes_instalados = 0;

            // Interpreta as propriedades no formato E: CHAVE=VALOR.
            for linha in propriedades.lines() {
                let Some(propriedade) = linha.strip_prefix("E: ") else {
                    continue;
                };
                let Some((chave, valor)) = propriedade.split_once('=') else {
                    continue;
                };

                if chave == "MEMORY_ARRAY_NUM_DEVICES" {
                    dados.soquetes = valor.to_string();
                }

                if chave.starts_with("MEMORY_DEVICE_")
                    && chave.ends_with("_SIZE")
                    && valor.parse::<u64>().unwrap_or(0) > 0
                {
                    pentes_instalados += 1;
                }

                if chave.starts_with("MEMORY_DEVICE_")
                    && chave.ends_with("_TYPE")
                    && dados.ddr == "Indisponível"
                    && valor != "Unknown"
                {
                    dados.ddr = valor.to_string();
                }

                if chave.starts_with("MEMORY_DEVICE_")
                    && chave.ends_with("_CONFIGURED_SPEED_MTS")
                    && dados.velocidade == "Indisponível"
                    && valor != "Unknown"
                {
                    dados.velocidade = format!("{valor} MT/s");
                }
            }

            dados.instalados = pentes_instalados.to_string();
            dados
        })
        .clone()
}

#[tauri::command]
pub(crate) fn ram_dados() -> DadosSistemaRAM {
    carregar_dados_sistema_ram()
}

pub(crate) fn ram_leitura(total: f64, usada: f64) -> DadosRam {
    DadosRam {
        total,
        usada,
        restante: total - usada,
        uso_percentual: if total > 0.0 {
            (usada / total * 100.0).round() as u8
        } else {
            0
        },
    }
}