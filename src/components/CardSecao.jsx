import { Cpu, Thermometer, CircleAlert } from 'lucide-react';
import './CardSecao.css';

function CardSecao({ rotulo, titulo, descricao, icone: Icone = Cpu, display, dados }) {
return (
        <section className="sobre-secao" aria-labelledby="sobre-sistema-titulo">
        <div className="sobre-secao-cabecalho">
          <div>
            <span className="sobre-rotulo">{rotulo}</span>
            <h2 id="sobre-sistema-titulo">{titulo}</h2>
            <p>{descricao}</p>
          </div>
          <Icone className="sobre-secao-icone" aria-hidden="true" size={20} />
        </div>

        <div className={"sobre-grade-"+display}>
          {display === 1 && dados.map(({ rotulo, valor, link, acao, icone: Icone }) => (

             <article className="dados-display-1" key={rotulo}>
              <span className="icone-dados-display-1" aria-hidden="true">
                <Icone size={16} strokeWidth={1.8} />
              </span>
              <div className="texto-dados-display-1">
                <span>{rotulo}</span>
                {valor && link == null && acao == null && <strong>{valor}</strong>}
                {link && acao == null && <strong><a style={{ color: "white" }} href={link} target="_blank" rel="noopener noreferrer">{valor}</a></strong>}
                {acao && link == null && <strong onClick={acao} style={{cursor:"pointer"}}>{valor}</strong>}
              </div>
            </article>

          ))}

         {display === 2 && dados.map(({ rotulo, valor, icone: Icone }) => (
            <article className="dados-display-2" key={rotulo}>
              <Icone className="icone-dados-display-2" aria-hidden="true" size={18} strokeWidth={1.8} />
              <span>{rotulo}</span>
              <strong>{valor}</strong>
            </article>
          ))}
        </div>
      </section>
);
}

export default CardSecao;
