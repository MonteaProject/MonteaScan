"use client";
// // import React from 'react';
// import { Doughnut } from 'react-chartjs-2';
// import Chart from 'chart.js/auto';
// import StreamingPlugin from 'chartjs-plugin-streaming';
// Chart.register(StreamingPlugin);
// // import 'chartjs-adapter-date-fns';
// // import { ja } from 'date-fns/locale';
import React from 'react';
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from 'chart.js';
import { Doughnut } from 'react-chartjs-2';
import { Box } from '../common/components';

ChartJS.register(ArcElement, Tooltip, Legend);

export function DoughnutChart() {
  const labels = ['Red', 'Orange', 'Yellow', 'Blue'];
  return (
    <Box>
      <Doughnut
        data={{
          labels: labels,
          datasets: [
            {
              label: 'My First Dataset',
              data: [1, 1, 1, 1],
              backgroundColor: [
                '#F24C3D',
                '#F79327',
                '#FFD95A',
                '#36A2EB',
              ],
              hoverOffset: 4
            }
          ]
        }}
      />
    </Box>
  )
}