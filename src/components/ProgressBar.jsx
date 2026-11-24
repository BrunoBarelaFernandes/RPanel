import './ProgressBar.css';


function ProgressBar({ porcentagem, label, info }) {
  let valor = Number(porcentagem);
  if (valor >= 101){valor=100;}

  let cor="#8ecfcb";
  let cor2="#61dafb";
  let cor3="#61dafb";

  if (valor >= 40 && valor < 65){
    cor = "#7842ff";
    cor2 = "#7842ff";
    cor3 = "#7842ff";
  }
  else if (valor >= 65 && valor < 80){
    cor = "#ff6822";
    cor2 = "#7842ff";
    cor3 = "#ff6822";
  }
    else if (valor >= 80){
    cor = "#ff2222";
    cor2 = "#7842ff";
    cor3 = "#ff2222";
  }
  else{
    cor = "#61dafb";
    cor2 = "#61dafb";
    cor3 = "#61dafb";
  }

  return (
    <section className="progress-bar">
      <div
        className="progress-bar-b"

        style={{ 
          '--angulo': `${valor * 1.8}deg`, 
          '--cor': cor,
          '--cor1': '#61dafb',
          '--cor2': cor2,
          '--cor3': cor3

      
      }}
      >
        <div className="progress-bar-dados">
          <strong>{valor}%</strong>
          <span>{label}</span>
        </div>

      </div>

      <p className="progress-bar-info">{info}</p>

    </section>
  );
}

export default ProgressBar;
