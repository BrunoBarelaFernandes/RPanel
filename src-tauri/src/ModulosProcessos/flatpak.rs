use std::{process::Command};


pub(crate) fn pid(nome: &str)->u32{
    let saida = Command::new("systemd-cgls")
        .args([&format!("--user-unit={}", nome), "--no-pager"])
        .output()
        .unwrap();

    let processos = String::from_utf8_lossy(&saida.stdout);

    for linha in processos.lines() {
        let linha = linha.trim();

        let conteudo = linha
            .trim_start_matches(|c: char| !c.is_ascii_digit());

        let mut partes = conteudo.split_whitespace();

        let Some(numero) = partes.next() else {
            continue;
        };

        let Some(comando) = partes.next() else {
            continue;
        };

        if matches!(
            comando.rsplit('/').next().unwrap_or(comando),
            "bwrap" | "xdg-dbus-proxy" | "cat" | "sh" | "bash"
        ) {
            continue;
        }

        if let Ok(pid) = numero.parse::<u32>() {
            return pid;
        }
    }
    0
}