import {
  CalendarDays,
  Clock3,
  Cpu,
  Gpu,
  HardDrive,
  MemoryStick,
  Microchip,
  Monitor,
  Network,
  CircuitBoard,
  FileSliders,
  Grid2x2,
  Languages,
  MonitorCog,
  RefreshCcwDot,
  Code,
  Calendar,
  Scale,

} from 'lucide-react';
import './Sobre.css';
import CardSecao from '../components/CardSecao';



function GithubIcon({ labels, size = 40, color = "currentColor", className = "" }) {
  return (
    <svg 
      xmlns="http://www.w3.org/2000/svg" 
      width={size} 
      height={size} 
      viewBox="0 0 24 24" 
      fill={color}
      className={className}
    >
      <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
    </svg>
  );
}



function formatarMemoria(valor) {
  if (valor === null || valor === undefined || valor === '') return '—';

  const bytes = Number(valor);
  if (!Number.isFinite(bytes) || bytes < 0) return '—';

  const MB = 1_000_000;
  const GB = 1_000_000_000;

  const formatar = (numero) =>
    numero.toLocaleString('pt-BR', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    });

  if (bytes >= GB) {
    return `${formatar(bytes / GB)} GB`;
  }

  return `${formatar(bytes / MB)} MB`;
}


function formatarMemoria2(valor) {
  if (valor === null || valor === undefined || valor === '') return '—';

  const bytes = Number(valor);
  if (!Number.isFinite(bytes) || bytes < 0) return '—';

  const MB = 1_000_000;
  const GB = 1_000_000_000;

  const formatar = (numero) =>
    numero.toLocaleString('pt-BR', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    });

  if (bytes >= GB) {
    return `${formatar(Math.floor(bytes / GB))} GB`;
  }

  return `${formatar(Math.floor(bytes / MB))} MB`;
}

function Sobre({labels, labelsAvisos, monitor, monitorGPU, dadosSistemaRAM, dadosSobre}) {


let dadosRAM = monitor.ram.usoPercentual
                ? `${dadosSistemaRAM.ddr} · ${dadosSistemaRAM.instalados}/${dadosSistemaRAM.soquetes} ${labels?.parteHardwareRamPentes} · ${dadosSistemaRAM.velocidade} · ${formatarMemoria2(monitor.ram.total)}`
                : labelsAvisos.indefinido

const dadosSistema = [
  { rotulo: labels?.parteSistemaOs, valor: dadosSobre.nomeOs+" "+dadosSobre.versaoOs, icone: Monitor },
  { rotulo: labels?.parteSistemaNomeHost, valor: dadosSobre.nomeHost, icone: MonitorCog },
  { rotulo: labels?.parteSistemaKernel, valor: dadosSobre.kernel, icone: Microchip },
  { rotulo: labels?.parteSistemaSistemaJanelas, valor: dadosSobre.sistemaJanelas, icone: Grid2x2 },
  { rotulo: labels?.parteSistemaIdioma, valor: dadosSobre.idioma, icone: Languages },
  { rotulo: labels?.parteSistemaTempoSistema, valor: monitor.tempoOs, icone: Clock3 },
  { rotulo: labels?.parteSistemaDataHora, valor: monitor.dataHora, icone: CalendarDays },
  { rotulo: labels?.parteSistemaEnderecoMac, valor: dadosSobre.mac, icone: Network },
  { rotulo: labels?.parteSistemaEnderecoIp, valor: dadosSobre.ip, icone: Network },
  { rotulo: labels?.parteSistemaInterfaceRede, valor: dadosSobre.interfaceRede, icone: Network },
  { rotulo: labels?.parteSistemaPlacaMae, valor: dadosSobre.fabricantePlacaMae != "-" ? dadosSobre.fabricantePlacaMae+" "+dadosSobre.modeloPlacaMae : labelsAvisos.indisponivel, icone: CircuitBoard },
  { rotulo: labels?.parteSistemaVersaoFirmware, valor: dadosSobre.versaoFirmware, icone: FileSliders },
];

const recursos = [
  { rotulo: labels?.parteHardwareCpu, valor: monitor.cpu.modelo, icone: Cpu },
  { rotulo: labels?.parteHardwareRam, valor: dadosRAM, icone: MemoryStick },
  { rotulo: labels?.parteHardwareDisco, valor: monitor.disco.usoPercentual ? monitor.disco.tipo+" · "+formatarMemoria(monitor.disco.total):labelsAvisos.indefinido, icone: HardDrive },
  { rotulo: labels?.parteHardwareVram, valor: monitorGPU.memoriaTotal ? labels?.parteHardwareVramFabricante+" "+monitorGPU.fabricante+" · "+labels?.parteHardwareVramTotal+" "+formatarMemoria2(monitorGPU.memoriaTotal)+" · "+labels?.parteHardwareVramUsada+" "+formatarMemoria(monitorGPU.memoriaUsada)+" · "+labels?.parteHardwareVramLivre+" "+formatarMemoria(monitorGPU.memoriaRestante) : labelsAvisos.semMemoriaVideo, icone: Microchip },
  { rotulo: labels?.parteHardwareGpu, valor: monitorGPU.usoPercentual || monitorGPU.modelo != labelsAvisos.desconhecido || monitorGPU.temperatura ? monitorGPU.modelo : labelsAvisos.semGpu, icone: Gpu },
];

// const discos = [
//   { nome: '/', tipo: 'SSD', total: '400 GB', usado: '200 GB', livre: '200 GB' },
//   { nome: 'Dados1', tipo: 'HD', total: '400 GB', usado: '250 GB', livre: '150 GB' },
//   { nome: 'Dados2', tipo: 'HD', total: '450 GB', usado: '350 GB', livre: '100 GB' },
// ];

const sobre = [
  { rotulo: labels?.parteSobreVersao, valor: dadosSobre.sobreRpanel.versao, icone: RefreshCcwDot },
  { rotulo: labels?.parteSobreDataVersao, valor: dadosSobre.sobreRpanel.dataVersao, icone: Calendar },
  { rotulo: labels?.parteSobreDev, valor: dadosSobre.sobreRpanel.dev, icone: Code },
  { rotulo: "GitHub", link: dadosSobre.sobreRpanel.enderecoLinkGitHub, valor: dadosSobre.sobreRpanel.nomeLinkGitHub, icone: GithubIcon },
  { rotulo: labels?.parteSobreLicenca, valor: dadosSobre.sobreRpanel.licenca, icone: Scale },

];

  return (
    <main className="pagina-sobre">


      <CardSecao 
       rotulo={labels?.parteSistemaTitulo}
       titulo={labels?.parteSistemaTitulo2}
       descricao={labels?.parteSistemaDescricao}
       icone={Monitor}
       display={1}
       dados={dadosSistema} 
        />

      <CardSecao 
       rotulo={labels?.parteHardwareTitulo}
       titulo={labels?.parteHardwareTitulo2}
       descricao={labels?.parteHardwareDescricao}
       icone={Cpu}
       display={2}
       dados={recursos} 
        />





      <section className="sobre-secao" aria-labelledby="sobre-discos-titulo">
        <div className="sobre-secao-cabecalho">
          <div>
            <span className="sobre-rotulo">{labels?.parteArmazenamentoTitulo}</span>
            <h2 id="sobre-discos-titulo">{labels?.parteArmazenamentoTitulo2}</h2>
            <p>{labels?.parteArmazenamentoDescricao}</p>
          </div>
          <HardDrive className="sobre-secao-icone" aria-hidden="true" size={20} />
        </div>

        <div className="sobre-tabela-rolagem">
          <table className="sobre-tabela-discos">
            <thead>
              <tr>
                <th>{labels?.parteArmazenamentoTituloTabela}</th>
                <th>{labels?.parteArmazenamentoTituloTabelaTipo}</th>
                <th>{labels?.parteArmazenamentoTituloTabelaTotal}</th>
                <th>{labels?.parteArmazenamentoTituloTabelaUsado}</th>
                <th>{labels?.parteArmazenamentoTituloTabelaLivre}</th>
              </tr>
            </thead>
            <tbody>
              {dadosSobre.discos.lista.map((disco, indice) => (
                <tr className={indice === 0 ? 'disco-principal' : ''} key={disco.dispositivo}>
                  <td>
                    <span className="disco-nome"><HardDrive size={15} />{disco.dispositivo}</span>
                    {indice === 0 && <span className="disco-sistema">{labels?.parteArmazenamentoMarcaSistemaTabela}</span>}
                  </td>
                  <td><span className="disco-tipo">{disco.tipo}</span></td>
                  <td>{formatarMemoria(disco.total)}</td>
                  <td>{formatarMemoria(disco.usada)}</td>
                  <td>{formatarMemoria(disco.restante)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>

      <CardSecao 
       rotulo={labels?.parteSobreTitulo}
       titulo={labels?.parteSobreTitulo2}
       descricao={labels?.parteSobreDescricao}
       icone={Cpu}
       display={1}
       dados={sobre} 
      />


    </main>
  );
}

export default Sobre;
