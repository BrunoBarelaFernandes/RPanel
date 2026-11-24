import {Activity, Cpu, Gauge, HardDrive, Info, List, MemoryStick, Wifi, EthernetPort, Gpu, Microchip} from 'lucide-react';
import CardRecursos from '../components/CardRecursos';
import './Recursos.css';


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



function Recursos({labels, labelsAvisos, monitor, monitorGPU, dadosSistemaRAM, historicoCPU, historicoRAM, historicoGPU, historicoVRAM, historicoDownload, historicoUpload}){
return (
    <>
    <div className="conteudo-painel">
            <div className="cabecalho-secao">
              <div>
                <h2>{labels?.titulo}</h2>
                <p>{labels?.descricao}</p>
              </div>
              <span className="etiqueta-monitoramento"><span /> {labels?.descricao2}</span>
            </div>

            <div className="grade-recursos">

              

              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitor.cpu.usoPercentual ? true : false}
              icone={Cpu}
              titulo={labels?.cpuTitulo}
              infocard={monitor.cpu.modelo}
              historico={historicoCPU}
              temperatura={monitor.cpu.temperatura}
              labelTemperatura={labels?.cpuTemperatura}
              porcentagem={monitor.cpu.usoPercentual}
              label={labels?.cpuTitulo2}
              info={Number(monitor.cpu.frequencia) ? Number(monitor.cpu.frequencia).toLocaleString('pt-BR')+' MHz' : labelsAvisos?.indefinido} 
              labelDetalhes={labels?.cpuDetalhes}
              labels={labels} 
              labelsAvisos={labelsAvisos}
             />

              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitorGPU.usoPercentual || monitorGPU.modelo != labelsAvisos?.desconhecido || monitorGPU.temperatura ? true : false}
              icone={Gpu}
              titulo={labels?.gpuTitulo}
              infocard={monitorGPU.modelo}
              historico={historicoGPU}
              temperatura={monitorGPU.temperatura}
              labelTemperatura={labels?.gpuTemperatura}
              porcentagem={monitorGPU.usoPercentual}
              label={labels?.gpuTitulo2}
              info={monitorGPU.usoPercentual || monitorGPU.modelo != labelsAvisos?.desconhecido || monitorGPU.temperatura ? monitorGPU.fabricante : labelsAvisos?.semGpu}
              labelDetalhes={labels?.gpuDetalhes}
              labels={labels} 
              labelsAvisos={labelsAvisos}
             />

              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitor.ram.usoPercentual ? true : false}
              icone={MemoryStick}
              titulo={labels?.ramTitulo}
              infocard={monitor.ram.usoPercentual
                ? `${dadosSistemaRAM.ddr} · ${dadosSistemaRAM.instalados}/${dadosSistemaRAM.soquetes} ${labels?.ramDetalhes2} · ${dadosSistemaRAM.velocidade} · ${formatarMemoria2(monitor.ram.total)}`
                : labelsAvisos?.indefinido}
              historico={historicoRAM}
              porcentagem={monitor.ram.usoPercentual}
              label={labels?.ramTitulo2}
              info={monitor.ram.usoPercentual ? labels?.ramLabelUsada+" "+formatarMemoria(monitor.ram.usada)+" "+labels?.ramLabelLivre+" "+formatarMemoria(monitor.ram.restante):labelsAvisos?.indefinido} 
              labelDetalhes={labels?.ramDetalhes}
              labels={labels} 
              labelsAvisos={labelsAvisos}
              />



              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitorGPU.memoriaTotal ? true : false}
              icone={Microchip}
              titulo={labels?.vramTitulo}
              infocard={monitorGPU.memoriaTotal && monitorGPU.memoriaTotal != 0 ? formatarMemoria2(monitorGPU.memoriaTotal) : labelsAvisos?.indefinido}
              porcentagem={monitorGPU.usoPercentualMemoria}
              historico={historicoVRAM}
              temperatura={monitorGPU.memoriaTotal ? monitorGPU.temperatura:""}
              labelTemperatura={labels?.gpuTemperatura}
              label={labels?.vramTitulo2}
              info={monitorGPU.memoriaTotal ? labels?.vramLabelUsada+" "+formatarMemoria(monitorGPU.memoriaUsada)+" · "+labels?.vramLabelLivre+" "+formatarMemoria(monitorGPU.memoriaRestante) : labelsAvisos?.semMemoriaVideo} 
              labelDetalhes={labels?.vramDetalhes}
              labels={labels} 
              labelsAvisos={labelsAvisos}
              />

              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitor.disco.usoPercentual ? true : false}
              icone={HardDrive}
              titulo={labels?.discoTitulo}
              infocard={monitor.disco.usoPercentual ? monitor.disco.tipo+" · "+formatarMemoria2(monitor.disco.total):labelsAvisos?.indefinido}
              porcentagem={monitor.disco.usoPercentual}
              label={labels?.discoTitulo2}
              info={monitor.disco.usoPercentual ? labels?.discoLabelUsada+" "+formatarMemoria(monitor.disco.usada)+" · "+labels?.discoLabelLivre+" "+formatarMemoria(monitor.disco.restante):labelsAvisos?.indefinido}
              labelDetalhes={labels?.discoDetalhes}
              labels={labels} 
              labelsAvisos={labelsAvisos}
              />

              <CardRecursos 
              tipo="GraficoRede"
              ativado={monitor.rede.interface != labelsAvisos?.indefinido ? true : false}
              icone={EthernetPort}
              titulo={labels?.redeTitulo}
              download={monitor.rede.download}
              upload={monitor.rede.upload}
              historico={historicoDownload}
              historico2={historicoUpload}
              infocard={labels?.redeDetalhes2+' '+monitor.rede.interface}
              labelDetalhes={labels?.redeDetalhes}
              labels={labels}
              labelsAvisos={labelsAvisos}
              />

            </div>

          </div>
    </>
);
}

export default Recursos;