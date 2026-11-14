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



function Recursos({monitor, monitorGPU, dadosSistemaRAM}){
return (
    <>
    <div className="conteudo-painel">
            <div className="cabecalho-secao">
              <div>
                <h2>Uso dos recursos</h2>
                <p>Acompanhe o estado atual do seu computador.</p>
              </div>
              <span className="etiqueta-monitoramento"><span /> MONITORAMENTO</span>
            </div>

            <div className="grade-recursos">

              

              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitor.cpu.usoPercentual ? true : false}
              icone={Cpu}
              titulo="PROCESSAMENTO"
              infocard={monitor.cpu.modelo}
              temperatura={monitor.cpu.temperatura}
              porcentagem={monitor.cpu.usoPercentual}
              label="CPU"
              info={Number(monitor.cpu.frequencia) ? Number(monitor.cpu.frequencia).toLocaleString('pt-BR')+' MHz' : "Indefinido"} 
              />

              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitorGPU.usoPercentual || monitorGPU.modelo != "Desconhecido" || monitorGPU.temperatura ? true : false}
              icone={Gpu}
              titulo="PLACA DE VÍDEO"
              infocard={monitorGPU.modelo}
              temperatura={monitorGPU.temperatura}
              porcentagem={monitorGPU.usoPercentual}
              label="GPU" 
              info={monitorGPU.usoPercentual || monitorGPU.modelo != "Desconhecido" || monitorGPU.temperatura ? monitorGPU.fabricante : "GPU indisponível!"}
              />

              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitor.ram.usoPercentual ? true : false}
              icone={MemoryStick}
              titulo="MEMORIA"
              infocard={monitor.ram.usoPercentual
                ? `${dadosSistemaRAM.ddr} · ${dadosSistemaRAM.instalados}/${dadosSistemaRAM.soquetes} pentes · ${dadosSistemaRAM.velocidade} · ${formatarMemoria2(monitor.ram.total)}`
                : 'Indefinido'}
              porcentagem={monitor.ram.usoPercentual}
              label="RAM"
              info={monitor.ram.usoPercentual ? "Usada: "+formatarMemoria(monitor.ram.usada)+" · Livre: "+formatarMemoria(monitor.ram.restante):"Indefinido"} 
              />



              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitorGPU.memoriaTotal ? true : false}
              icone={Microchip}
              titulo="MEMORIA DE VÍDEO"
              infocard={monitorGPU.memoriaTotal && monitorGPU.memoriaTotal != 0 ? formatarMemoria2(monitorGPU.memoriaTotal) : "Indefinido"}
              porcentagem={monitorGPU.usoPercentualMemoria}
              temperatura={monitorGPU.memoriaTotal ? monitorGPU.temperatura:""}
              label="VRAM"
              info={monitorGPU.memoriaTotal ? "Usada: "+formatarMemoria(monitorGPU.memoriaUsada)+" · Livre: "+formatarMemoria(monitorGPU.memoriaRestante) : "Memoria de vídeo indisponível!"} 
              />

              <CardRecursos 
              tipo="GeigerBar"
              ativado={monitor.disco.usoPercentual ? true : false}
              icone={HardDrive}
              titulo="DISCO"
              infocard={monitor.disco.usoPercentual ? monitor.disco.tipo+" · "+formatarMemoria2(monitor.disco.total):"Indefinido"}
              porcentagem={monitor.disco.usoPercentual}
              label="DISCO"
              info={monitor.disco.usoPercentual ? "Usada: "+formatarMemoria(monitor.disco.usada)+" · Livre: "+formatarMemoria(monitor.disco.restante):"Indefinido"} 
              />

              <CardRecursos 
              tipo="GraficoRede"
              ativado={monitor.rede.interface != "Indefinido" ? true : false}
              icone={EthernetPort}
              titulo="REDE"
              download={monitor.rede.download}
              upload={monitor.rede.upload}
              infocard={'Interface '+monitor.rede.interface}
              />

            </div>

          </div>
    </>
);
}

export default Recursos;