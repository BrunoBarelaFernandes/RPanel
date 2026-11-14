use std::{thread, time::Duration};
use std::sync::{Arc, Mutex, mpsc};
use std::{process::Command, sync::OnceLock};
use chrono::format::parse;


pub(crate) fn pid(nome: &str) -> u32 {
    let Ok(saida) = Command::new("systemd-cgls")
        .args([&format!("--user-unit={nome}"), "--no-pager"])
        .output()
    else {
        return 0;
    };

    let processos = String::from_utf8_lossy(&saida.stdout);
    let mut pid_wrapper = 0;

    for linha in processos.lines() {
        let linha = linha.trim_start();

        if !(linha.starts_with('├')
            || linha.starts_with('└')
            || linha.starts_with('│'))
        {
            continue;
        }

        let Some(numero) = linha
            .trim_start_matches(|c: char| !c.is_ascii_digit())
            .split_whitespace()
            .next()
        else {
            continue;
        };

        let Ok(pid) = numero.parse::<u32>() else {
            continue;
        };

        let nome_processo = std::fs::read_to_string(format!("/proc/{pid}/comm"))
            .unwrap_or_default();
        let nome_processo = nome_processo.trim();

        if nome_processo == "snap-confine" || nome_processo == "snap-exec" {
            if pid_wrapper == 0 {
                pid_wrapper = pid;
            }
            continue;
        }

        return pid;
    }

    // Se só encontrou wrappers, mantém o PID disponível como fallback.
    pid_wrapper
}

pub(crate) fn icone(pid: u32) -> String {
    let Ok(cgroups) = std::fs::read_to_string(format!("/proc/{pid}/cgroup")) else {
        return String::new();
    };

    let unidade = cgroups
        .lines()
        .filter_map(|linha| linha.rsplit_once(':').map(|(_, caminho)| caminho))
        .flat_map(|caminho| caminho.split('/'))
        .find(|parte| parte.starts_with("snap.") && parte.ends_with(".scope"));

    let Some(unidade) = unidade else {
        return String::new();
    };

    let Some(identificador) = unidade
        .strip_prefix("snap.")
        .and_then(|nome| nome.strip_suffix(".scope"))
    else {
        return String::new();
    };

    let Some((snap, app_com_uuid)) = identificador.split_once('.') else {
        return String::new();
    };

    // Remove o UUID completo, que tem 36 caracteres e vários hífens.
    let app = app_com_uuid
        .char_indices()
        .filter(|(_, caractere)| *caractere == '-')
        .find_map(|(inicio, _)| {
            let uuid = &app_com_uuid[inicio + 1..];

            let formato_valido = uuid.len() == 36
                && uuid.chars().enumerate().all(|(i, caractere)| {
                    if [8, 13, 18, 23].contains(&i) {
                        caractere == '-'
                    } else {
                        caractere.is_ascii_hexdigit()
                    }
                });

            formato_valido.then_some(&app_com_uuid[..inicio])
        })
        .unwrap_or(app_com_uuid);

    let desktop = format!(
        "/var/lib/snapd/desktop/applications/{snap}_{app}.desktop"
    );

    let Ok(conteudo) = std::fs::read_to_string(desktop) else {
        return String::new();
    };

    let Some(icone) = conteudo
        .lines()
        .find_map(|linha| linha.strip_prefix("Icon="))
        .map(str::trim)
    else {
        return String::new();
    };

    let caminho = std::path::Path::new(icone);

    if caminho.is_file() {
        return std::fs::canonicalize(caminho)
            .unwrap_or_else(|_| caminho.to_path_buf())
            .to_string_lossy()
            .into_owned();
    }

    icon_finder::find_icon(icone, 128)
        .map(|caminho| caminho.to_string_lossy().into_owned())
        .unwrap_or_default()
}