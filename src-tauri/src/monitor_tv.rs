use drm::{
    control::{
        connector::{Handle as ConnectorHandle, State},
        Device as ControlDevice,
    },
};
use std::{
    fs::{self, File, OpenOptions},
    os::fd::{AsFd, BorrowedFd},
};

struct Card(File);

impl AsFd for Card {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl drm::Device for Card {}
impl ControlDevice for Card {}

#[derive(Debug)]
struct Monitor {
    nome: String,
    resolucao: String,
    frequencia: u32,
}

// Identifica o modelo do monitor através do EDID.
fn modelo_monitor( placa: &Card, conector: ConnectorHandle) -> Option<String> {
    let propriedades = placa.get_properties(conector).ok()?;

    for (id, valor) in propriedades.iter() {
        let info = placa.get_property(*id).ok()?;

        if info.name().to_bytes() != b"EDID" || *valor == 0 {
                continue;
        }

        let edid = placa.get_property_blob(*valor).ok()?;

        if edid.len() < 128 {
            continue;
        }

        // Procura o descritor de nome do monitor.
        for inicio in (54..126).step_by(18) {
            if edid[inicio..inicio + 5] == [0, 0, 0, 0xFC, 0] {
                let nome = String::from_utf8_lossy(
                    &edid[inicio + 5..inicio + 18]
                );

                let nome = nome
                    .trim_matches(|c: char| {
                        c == '\0' || c == '\n' || c == '\r' || c == ' '
                    })
                    .to_string();

                if !nome.is_empty() {
                    return Some(nome);
                }
            }
        }

        // Caso não exista nome, utiliza fabricante e produto.
        let codigo = u16::from_be_bytes([edid[8], edid[9]]);

        let fabricante: String = [
            ((codigo >> 10) & 0x1F) as u8,
            ((codigo >> 5) & 0x1F) as u8,
            (codigo & 0x1F) as u8,
        ]
        .iter()
        .map(|c| char::from(c + 64))
        .collect();

        let produto = u16::from_le_bytes([edid[10], edid[11]]);

        return Some(format!("{} - {:04X}", fabricante, produto));
    }

    None
}

// Descobre automaticamente os monitores conectados.

pub(crate) fn monitor_atual() -> String {
    let Ok(dispositivos) = fs::read_dir("/dev/dri") else {
        return String::new();
    };

    for dispositivo in dispositivos.flatten() {
        let caminho = dispositivo.path();

        let Some(nome_arquivo) = caminho.file_name()
            .and_then(|nome| nome.to_str())
        else {
            continue;
        };

        if !nome_arquivo
            .strip_prefix("card")
            .is_some_and(|numero| {
                !numero.is_empty()
                    && numero.chars().all(|c| c.is_ascii_digit())
            })
        {
            continue;
        }

        let Ok(arquivo) = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&caminho)
        else {
            continue;
        };

        let placa = Card(arquivo);

        let Ok(recursos) = placa.resource_handles() else {
            continue;
        };

        for conector_handle in recursos.connectors() {
            let Ok(conector) = placa.get_connector(
                *conector_handle,
                false,
            ) else {
                continue;
            };

            if conector.state() != State::Connected {
                continue;
            }

            let Some(encoder_handle) = conector.current_encoder() else {
                continue;
            };

            let Ok(encoder) = placa.get_encoder(encoder_handle) else {
                continue;
            };

            let Some(crtc_handle) = encoder.crtc() else {
                continue;
            };

            let Ok(crtc) = placa.get_crtc(crtc_handle) else {
                continue;
            };

            let Some(modo) = crtc.mode() else {
                continue;
            };

            let (largura, altura) = modo.size();

            let nome = modelo_monitor(
                &placa,
                *conector_handle,
            )
            .unwrap_or_else(|| {
                format!(
                    "{:?}-{}",
                    conector.interface(),
                    conector.interface_id()
                )
            });

            let resolucao = format!("{}x{}", largura, altura);
            let frequencia = modo.vrefresh();

            return format!(
                "{}, {}, {} Hz",
                nome,
                resolucao,
                frequencia
            );
        }
    }

    String::new()
}
