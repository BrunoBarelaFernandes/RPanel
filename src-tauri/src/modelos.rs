use serde::Serialize;
use sysinfo::{Components, Disks, Networks, System};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosSistemaRAM {
    pub(crate) ddr: String,
    pub(crate) soquetes: String,
    pub(crate) instalados: String,
    pub(crate) velocidade: String,
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosSobre {
    pub(crate) nome_os: String,
    pub(crate) versao_os: String,
    pub(crate) kernel: String,
    pub(crate) sistema_janelas: String,
    pub(crate) nome_host: String,
    pub(crate) idioma: String,
    pub(crate) interface_rede: String,
    pub(crate) mac: String,
    pub(crate) ip: String,
    pub(crate) fabricante_placa_mae: String,
    pub(crate) modelo_placa_mae: String,
    pub(crate) versao_firmware: String,
    pub(crate) discos: ListaDisco,
    pub(crate) sobre_rpanel: InfoRPanel,

}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosProcesso {
    pub(crate) icone: String,
    pub(crate) pid: u32,
    pub(crate) nome: String,
    pub(crate) ram: f64,
    pub(crate) cpu: f64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosMonitor {
    pub(crate) cpu: DadosCPU,
    pub(crate) ram: DadosRam,
    pub(crate) disco: DadosDisco,
    pub(crate) rede: DadosRede,
    pub(crate) tempo_os: String,
    pub(crate) data_hora: String,

}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosGPU {
    pub fabricante: String,
    pub modelo: String,
    pub uso_percentual: u8,
    pub uso_percentual_memoria: u8,
    pub temperatura: f64,
    pub memoria_total: f64,
    pub memoria_usada: f64,
    pub memoria_restante: f64,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosCPU {
    pub(crate) modelo: String,
    pub(crate) frequencia: String,
    pub(crate) uso_percentual: u8,
    pub(crate) temperatura: u8,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosRam {
    pub(crate) total: f64,
    pub(crate) usada: f64,
    pub(crate) restante: f64,
    pub(crate) uso_percentual: u8,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosDisco {
    pub(crate) tipo: String,
    pub(crate) total: f64,
    pub(crate) usada: f64,
    pub(crate) restante: f64,
    pub(crate) uso_percentual: u8,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosRede {
    pub(crate) interface: String,
    pub(crate) download: f64,
    pub(crate) upload: f64,
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Disco {
    pub(crate) dispositivo: String,
    pub(crate) montagem: String,
    pub(crate) tipo: String,
    pub(crate) total: f64,
    pub(crate) usada: f64,
    pub(crate) restante: f64,
    pub(crate) uso_percentual: u8,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListaDisco {
    pub(crate) lista: Vec<Disco>,

}


pub(crate) struct DadosSysInfo {
    pub(crate) sistema: System,
    pub(crate) componentes: Components,
    pub(crate) discos: Disks,
    pub(crate) redes: Networks,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InfoRPanel {
    pub(crate) versao: String,
    pub(crate) data_versao: String,
    pub(crate) dev: String,
    pub(crate) nome_link_git_hub: String,
    pub(crate) endereco_link_git_hub: String,
    pub(crate) licenca: String,

}