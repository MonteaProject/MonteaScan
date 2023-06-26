"use client";
// import React from 'react';
// import { Chart as ChartJS } from 'chart.js/auto';
// import { Chart } from 'react-chartjs-2';
// // import StreamingPlugin from 'chartjs-plugin-streaming';
// // ChartJS.register(StreamingPlugin);
// import 'chartjs-adapter-date-fns';
// import { ja } from 'date-fns/locale';
// import { Box } from '../common/components';

import React from 'react';
import {
  Chart as ChartJS,
  LinearScale,
  CategoryScale,
  BarElement,
  PointElement,
  LineElement,
  Legend,
  Tooltip,
  LineController,
  BarController,
} from 'chart.js';
import { Chart } from 'react-chartjs-2';
import { Box } from '../common/components';

ChartJS.register(
  LinearScale,
  CategoryScale,
  BarElement,
  PointElement,
  LineElement,
  Legend,
  Tooltip,
  LineController,
  BarController
);

export function MultiTypeChart() {
  const labels = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];
  return (
    <Box>
      <Chart
        type='bar'
        data={{
          labels: labels,
          datasets: [
            {
              type: 'line' as const,
              label: 'Dataset 1',
              borderColor: 'rgb(255, 99, 132)',
              borderWidth: 2,
              fill: false,
              data: labels.map(() => Math.floor(Math.random() * 198) - 99),
            },
            {
              type: 'bar' as const,
              label: 'Dataset 2',
              backgroundColor: 'rgb(75, 192, 192)',
              borderColor: 'white',
              borderWidth: 2,
              data: labels.map(() => Math.floor(Math.random() * 198) - 99),
            },
            {
              type: 'bar' as const,
              label: 'Dataset 3',
              backgroundColor: 'rgb(53, 162, 235)',
              data: labels.map(() => Math.floor(Math.random() * 198) - 99),
            },
          ]
        }}
      />
    </Box>
  )
}