
import { useEffect, useRef } from 'react';
import { LineChart } from 'chartist';

import 'chartist/styles.css';
import './Graficos.css';

export default function Grafico({
    historico = [],
    cor = '#00e5ff',
    altura = 80,
    top = 170
}) {
    const container = useRef(null);
    const grafico = useRef(null);

    useEffect(() => {
        if (historico.length < 2) return;

        const dados = {
            labels: historico.map(() => ''),
            series: [historico]
        };

        const opcoes = {
            low: 0,
            high: 100,
            fullWidth: true,
            showPoint: false,
            showArea: false,
            showLine: true,
            lineSmooth: true,
            chartPadding: 0,

            axisX: {
                showGrid: false,
                showLabel: false,
                offset: 0
            },

            axisY: {
                showGrid: false,
                showLabel: false,
                offset: 0
            }
        };

        if (!grafico.current) {
            grafico.current = new LineChart(
                container.current,
                dados,
                opcoes
            );
        } else {
            grafico.current.update(dados, opcoes);
        }
    }, [historico]);

    useEffect(() => {
        return () => {
            grafico.current?.detach();
            grafico.current = null;
        };
    }, []);

    return (
        <div
            ref={container}
            className="grafico"
            style={{
                height: `${altura}px`,top: `${top}px`, '--grafico-cor': cor
            }}
        />
    );
}
