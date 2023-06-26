"use client";
// // import React from 'react';
// import { Bar } from 'react-chartjs-2';
// import Chart from 'chart.js/auto';
// import StreamingPlugin from 'chartjs-plugin-streaming';
// Chart.register(StreamingPlugin);
// // import 'chartjs-adapter-date-fns';
// // import { ja } from 'date-fns/locale';
// import { Box } from '../common/components';

import React from 'react';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { Bar } from 'react-chartjs-2';
import { Box } from '../common/components';

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

export function StackedBarChart() {
  const labels = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];
  return (
    <Box>
      <Bar
        data={{
          labels,
          datasets: [
            {
              label: 'Dataset 1',
              data: labels.map(() => Math.floor(Math.random() * 198) - 99),
              backgroundColor: 'rgb(255, 99, 132)',
            },
            {
              label: 'Dataset 2',
              data: labels.map(() => Math.floor(Math.random() * 198) - 99),
              backgroundColor: 'rgb(75, 192, 192)',
            },
            {
              label: 'Dataset 3',
              data: labels.map(() => Math.floor(Math.random() * 198) - 99),
              backgroundColor: 'rgb(53, 162, 235)',
            },
          ]
        }}
        options={{
          plugins: {
            title: {
              display: true,
              text: 'Chart.js Bar Chart - Stacked',
            },
          },
          responsive: true,
          scales: {
            x: {
              stacked: true,
            },
            y: {
              stacked: true,
            },
          }
        }}
      />
    </Box>
  )
}