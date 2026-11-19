
import { useState, useEffect } from 'react';
import './PopUp.css';
import Alert from './Alert';
import { CircleCheckBig } from 'lucide-react';


function PopUp({ aberto, nomeAplicativo, onConfirmar, onFechar}) {

    const [estado, setEstado] = useState('confirmacao');
    const [mensagemErro, setMensagemErro] = useState('');

    useEffect(() => {
        if (aberto) {
            setEstado('confirmacao');
            setMensagemErro('');
        }
    }, [aberto]);

    if (!aberto) return null;

    async function encerrarAplicativo() {
        setEstado('carregando');

        try {
            const resultado = await onConfirmar();

            if (resultado === false) {
                throw new Error('Não foi possível encerrar o aplicativo.');
            }

            setEstado('sucesso');
        } catch (erro) {
            setMensagemErro(
                erro?.message || 'Ocorreu um erro ao encerrar o aplicativo.'
            );

            setEstado('erro');
        }
    }

    function fechar() {
        if (estado === 'carregando') return;

        onFechar();
    }

    return (
        <div className="popup-janela">
            <div className="popup-container">

                {estado === 'confirmacao' && (
                    <>
                        <div className="popup-icone aviso"><span><Alert tamanho={50} /></span></div>

                        <h2>Encerrar aplicativo?</h2>

                        <p>
                            Deseja realmente encerrar o aplicativo
                            <strong> {nomeAplicativo}</strong>?
                        </p>

                        <p className="popup-detalhe">
                            Essa ação encerrará o aplicativo e poderá
                            causar a perda de dados não salvos.
                        </p>

                        <div className="popup-botoes">
                            <button
                                className="popup-btn cancelar"
                                onClick={fechar}
                            >
                                Cancelar
                            </button>

                            <button
                                className="popup-btn encerrar"
                                onClick={encerrarAplicativo}
                            >
                                Encerrar
                            </button>
                        </div>
                    </>
                )}

                {estado === 'carregando' && (
                    <div className="popup-carregando">
                        <div className="popup-spinner"></div>

                        <h2>Encerrando aplicativo</h2>

                        <p>
                            Aguarde enquanto o RPanel encerra
                            <strong> {nomeAplicativo}</strong>...
                        </p>
                    </div>
                )}
                {estado === 'sucesso' && (
                    <>
                        <div className="popup-icone sucesso"><CircleCheckBig size={50} /></div>

                        <h2>Aplicativo encerrado</h2>

                        <p>
                            O aplicativo
                            <strong> {nomeAplicativo}</strong> foi
                            encerrado com sucesso.
                        </p>

                        <div className="popup-botoes">
                            <button
                                className="popup-btn confirmar"
                                onClick={fechar}
                            >
                                Fechar
                            </button>
                        </div>
                    </>
                )}

                {estado === 'erro' && (
                    <>
                        <div className="popup-icone erro"><span><Alert tamanho={50} /></span></div>

                        <h2>Falha ao encerrar</h2>

                        <p>
                            Não foi possível encerrar
                            <strong> {nomeAplicativo}</strong>
                        </p>

                        <p className="popup-detalhe">
                            {mensagemErro}
                        </p>

                        <div className="popup-botoes">
                            <button
                                className="popup-btn confirmar"
                                onClick={fechar}
                            >
                                Ok
                            </button>
                        </div>
                    </>
                )}

            </div>
        </div>
    );
}


export default PopUp;