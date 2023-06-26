"use client";
// import React from 'react';
// import Chart from 'chart.js/auto';
// import { Scatter } from 'react-chartjs-2';
// import StreamingPlugin from 'chartjs-plugin-streaming';
// Chart.register(StreamingPlugin);
// import 'chartjs-adapter-date-fns';
// import { ja } from 'date-fns/locale';
// import { Box } from '../common/components';

import React from 'react';
import {
  Chart as ChartJS,
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Legend,
} from 'chart.js';
import { Scatter } from 'react-chartjs-2';
import { Box } from '../common/components';

ChartJS.register(LinearScale, PointElement, LineElement, Tooltip, Legend);


export function ScatterChart() {
  return (
    <Box>
      <Scatter
        data={{
          datasets: [
            {
              label: 'A dataset',
              data: Array.from({ length: 100 }, () => ({
                x: Math.floor(Math.random() * 198) - 99,
                y: Math.floor(Math.random() * 198) - 99,
              })),
              backgroundColor: 'rgba(255, 99, 132, 1)',
            },
          ]
        }}
        options={{
          scales: {
            y: {
              beginAtZero: true,
            },
          }
        }}
      />
    </Box>
  )
}