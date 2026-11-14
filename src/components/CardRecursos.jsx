import './CardRecursos.css';
import ProgressBar from './ProgressBar';
import RedeRecursos from './RedeRecursos';
import { Cpu, Thermometer, CircleAlert } from 'lucide-react';

function CardRecursos({
  tipo = 0,
  ativado = true,
  icone: Icone = Cpu,
  titulo = "COMPONENTE DE SISTEMA",
  infocard = "Sem informação!",
  temperatura = null,
  download = 0,
  upload = 0,
  porcentagem = 0,
  label = "",
  info = "",
}) {
  let opacity=1;
  titulo = (titulo === null || titulo === "") ? "COMPONENTE DE SISTEMA" : titulo;
  infocard = (infocard === null || infocard.length === 0) ? "Sem informação!" : infocard;
  tipo = (tipo === null || tipo.length === 0) ? 0 : tipo;
  porcentagem = Number(porcentagem) || 0;
  const numero = Number(porcentagem);
  let valor = Number(porcentagem);
  if (valor >= 101){valor=100;}

  let cor = '#61dafb';
  if (valor >= 80) {
    cor = '#ff2222';
  } else if (valor >= 65) {
    cor = '#ff6822';
  } else if (valor >= 40) {
    cor = '#7842ff';
  } else{
    cor = '#61dafb';
  }
  if (!ativado || tipo === 0){
    opacity=0.5;
    cor = '#7d7d7ddb';
  }
  else{
    opacity=1;
  }

  return (
    <article className="cartao-recurso" style={{ '--cor': cor, 'opacity': opacity }}>
      <div className="cartao-recurso-categoria">
        <Icone className="cartao-recurso-icone" aria-hidden="true" size={17} strokeWidth={1.8} />
        <span>{titulo}</span>
      </div>

      {tipo === 0 ? <div className="alerta_sem_modelo"><CircleAlert size={50}/><h2>SEM MODELO DE DADOS!</h2></div>:""}
      {tipo === "GeigerBar" ? <ProgressBar porcentagem={valor} label={label} info={info} /> : ''}
      {tipo === "GraficoRede" ? <RedeRecursos recebimento={download} envio={upload} /> : ''}

      {temperatura ? <div className="cartao-recurso-rodape cartao-recurso-rodape-temperatura">
        <span className="cartao-recurso-rotulo-temperatura">
          <Thermometer aria-hidden="true" size={15} strokeWidth={1.8} />
          Temperatura
        </span>
        <strong>{temperatura ?? '—'}{temperatura != null ? ' °C' : ''}</strong>
      </div>:""}

      <div className="cartao-recurso-rodape cartao-recurso-rodape-detalhes">
        <span>Detalhes: </span>
        <strong>{infocard}</strong>
      </div>
    </article>
  );
}


export default CardRecursos;

