import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Activity, Cpu, Gauge, HardDrive, Info, List, MemoryStick, Wifi, EthernetPort, Gpu, Microchip } from 'lucide-react';
import Recursos from './paginas/Recursos';
import Sobre from './paginas/Sobre';
import Processos from './paginas/Processos';
import StatusInternet from './components/StatusInternet';

import './App.css';



const menuItems = [
  { id: 'recursos', label: 'Recursos', icon: Gauge },
  { id: 'processos', label: 'Processos', icon: List },
  { id: 'sobre', label: 'Sobre', icon: Info },
];


function App() {
  const [pagina, setPagina] = useState('recursos');
  const [dadosSistemaRAM, setDadosSistemaRAM] = useState({
    ddr: "", soquetes: "0", instalados: "0", velocidade: "",
  });
  const [monitor, setMonitor] = useState({
    cpu: { usoPercentual: 0, temperatura: null },
    ram: { total: 0, usada: 0, restante: 0, usoPercentual: 0 },
    disco: { tipo: "Indefinido", total: 0, usada: 0, restante: 0, usoPercentual: 0 },
    rede: { interface: "Indefinido", download: 0, upload: 0 },
    tempoOs: "0", dataHora: "00/00/0000 - 00:00:00"
  });
  const [IndicadorInternet, setIndicadorInternet] = useState(true);
  const [monitorGPU, setMonitorGPU] = useState({
    fabricante: "Desconhecido", modelo: "Desconhecido", usoPercentual: 0, usoPercentualMemoria: 0, temperatura: null,
    memoriaTotal: 0, memoriaUsada: 0, memoriaRestante: 0
  });
  const [dadosSobre, setDadosSobre] = useState({
    nomeOs: "Desconhecido", versaoOs: "Desconhecido", kernel: "Desconhecido", sistemaJanelas: "Desconhecido", nomeHost: "Desconhecido",
    idioma: "Desconhecido", interfaceRede: "Desconhecido", mac: "Desconhecido", ip: "Desconhecido", fabricantePlacaMae: "Desconhecido",
    modeloPlacaMae: "Desconhecido", versaoFirmware: "Desconhecido", discos: { lista: [] }, sobreRpanel: { versao: "Desconhecido", 
      dataVersao: "Desconhecido", dev: "Desconhecido", nomeLinkGitHub: "Desconhecido", enderecoLinkGitHub: "Desconhecido", licenca: "Desconhecido" }
  });

  const [dadosProcessos, setDadosProcessos] = useState([]);



  useEffect(() => {
    const bloquearMenuPadrao = (event) => { event.preventDefault(); };
    document.addEventListener("contextmenu", bloquearMenuPadrao);
    return () => { document.removeEventListener("contextmenu", bloquearMenuPadrao); };
  }, []);


  useEffect(() => {
    let ativo = true;
    let cancelarMonitor;
    let cancelarIndicadorInternet;
    let cancelarMonitorGPU;
    let cancelarMonitorProcessos;

    async function registrarListeners() {
      const pararmonitor = await listen("DadosMonitor", (event) => {
        setMonitor(event.payload);
      });

      const pararIndicadorInternet = await listen("IndicadorInternet", (event) => {
        setIndicadorInternet(event.payload);
      });

      const pararMonitorGPU = await listen("DadosMonitorGPU", (event) => {
        setMonitorGPU(event.payload);
      });

      const pararMonitorProcessos = await listen("DadosMonitorProcessos", (event) => {
        setDadosProcessos(event.payload);
      });


      if (ativo) {
        cancelarMonitor = pararmonitor;//Compartilhamos a função de encerramento
        cancelarIndicadorInternet = pararIndicadorInternet;
        cancelarMonitorGPU = pararMonitorGPU;
        cancelarMonitorProcessos = pararMonitorProcessos;
      } else {
        pararmonitor();//Iniciamos a função de encerrramento
        pararIndicadorInternet();
        pararMonitorGPU();
        pararMonitorProcessos();
      }
    }

    registrarListeners();//Iniciamos o registrador

    return () => {//Return do useEffect
      ativo = false;//Botamos para false
      cancelarMonitor?.();//Verificamos se a variável tem uma função, se sim executa para encerrar
      cancelarIndicadorInternet?.();
      cancelarMonitorGPU?.();
      cancelarMonitorProcessos?.();
    };
  }, []);

  useEffect(() => {
    let ativo = true;

    invoke('solicitar', { sistema: "true", gpu: "true", processos: "false" });

    invoke('ram_dados')
      .then((dados) => {
        if (ativo) setDadosSistemaRAM(dados);
      })
      .catch((erro) => { console.error('Não foi possível carregar os detalhes da RAM:', erro); });

    invoke('info')
      .then((dados) => { if (ativo) setDadosSobre(dados); })
      .catch((erro) => { console.error('Não foi possível carregar os DadosSobre:', erro); });

    return () => {
      ativo = false;
    };
  }, []);

  useEffect(() => {
    if (pagina === "recursos") {
      invoke('solicitar', { sistema: "true", gpu: "true", processos: "false" });
    }

    if (pagina === "processos") {
      invoke('solicitar', { sistema: "false", gpu: "false", processos: "true" });
    }

    if (pagina === "sobre") {
      invoke('solicitar', { sistema: "true", gpu: "true", processos: "false" });
    }
  }, [pagina]);

  return (
    <main className="estrutura-aplicativo">
      <aside className="menu-lateral">
        <a className="marca" href="#recursos" onClick={() => setPagina('recursos')}>
          <img className="marca-simbolo" src="logo.png" />
        </a>

        <span className="menu-lateral-titulo">MENU</span>
        <nav className="menu-lateral-navegacao" aria-label="Menu principal">
          {menuItems.map((item) => (
            <button
              className={`item-menu ${pagina === item.id ? 'item-menu--ativo' : ''}`}
              key={item.id}
              type="button"
              onClick={() => setPagina(item.id)}
              aria-current={pagina === item.id ? 'page' : undefined}
            >
              <item.icon className="item-menu-icone" aria-hidden="true" size={18} strokeWidth={1.8} />
              <span>{item.label}</span>
              {pagina === item.id && <span className="item-menu-indicador" />}
            </button>
          ))}

        </nav>

        <div className="menu-lateral-rodape">
          <span className="menu-lateral-ponto-status" />
          <span>RPanel 1.0.0</span>
        </div>
      </aside>

      <section className="area-principal">
        <header className="cabecalho">
          <div className="cabecalho-titulo-container">
            <span className="rotulo-superior">RPanel / PAINEL</span>
            <h1>{menuItems.find((item) => item.id === pagina)?.label}</h1>
          </div>




          <StatusInternet conectado={IndicadorInternet} />






        </header>

        {pagina === 'recursos' && (<Recursos monitor={monitor} monitorGPU={monitorGPU} dadosSistemaRAM={dadosSistemaRAM} />)}
        {pagina === 'sobre' && <Sobre monitor={monitor} monitorGPU={monitorGPU} dadosSistemaRAM={dadosSistemaRAM} dadosSobre={dadosSobre} />}
        {pagina === 'processos' && <Processos processos={dadosProcessos} />}

      </section>
    </main>
  );
}

export default App;

