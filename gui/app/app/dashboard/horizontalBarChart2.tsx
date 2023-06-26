"use client";
// // import React from 'react';
// import { Bar } from 'react-chartjs-2';
// import Chart from 'chart.js/auto';
// import StreamingPlugin from 'chartjs-plugin-streaming';
// Chart.register(StreamingPlugin);
// // import 'chartjs-adapter-date-fns';
// // import { ja } from 'date-fns/locale';
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

export function HorizontalBarChart2() {
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
              borderColor: 'rgb(255, 99, 132)',
              backgroundColor: 'rgba(255, 99, 132, 0.5)',
            },
            {
              label: 'Dataset 2',
              data: labels.map(() => Math.floor(Math.random() * 198) - 99),
              borderColor: 'rgb(53, 162, 235)',
              backgroundColor: 'rgba(53, 162, 235, 0.5)',
            },
          ]
        }}
        options={{
          indexAxis: 'y' as const,
          elements: {
            bar: {
              borderWidth: 2,
            },
          },
          responsive: true,
          plugins: {
            legend: {
              position: 'right' as const,
            },
            title: {
              display: true,
              text: 'Chart.js Horizontal Bar Chart',
            },
          }
        }}
      />
    </Box>
  )
}