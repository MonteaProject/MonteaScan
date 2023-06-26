"use client";
// import React from 'react';
// import { Bubble } from 'react-chartjs-2';
// import Chart from 'chart.js/auto';
// import StreamingPlugin from 'chartjs-plugin-streaming';
// Chart.register(StreamingPlugin);
// // import 'chartjs-adapter-date-fns';
// // import { ja } from 'date-fns/locale';
import React from 'react';
import {
  Chart as ChartJS,
  LinearScale,
  PointElement,
  Tooltip,
  Legend,
} from 'chart.js';
import { Bubble } from 'react-chartjs-2';
import { Box } from '../common/components';

ChartJS.register(LinearScale, PointElement, Tooltip, Legend);

export function BubbleChart() {
  const labels = ['Red'];
  return (
    <Box>
      <Bubble
        data={{
          labels: labels,
          datasets: [{
            label: 'First Dataset',
            data: [{
              x: 20,
              y: 30,
              r: 15
            }, {
              x: 40,
              y: 10,
              r: 10
            }],
            backgroundColor: 'rgb(255, 99, 132)'
          }]
        }}
      />
    </Box>
  )
}