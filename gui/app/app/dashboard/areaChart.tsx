"use client";
// import React from 'react';
// import Chart from 'chart.js/auto';
// import { Line } from 'react-chartjs-2';
// import StreamingPlugin from 'chartjs-plugin-streaming';
// Chart.register(StreamingPlugin);
// import 'chartjs-adapter-date-fns';
import React from 'react';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Filler,
  Legend,
} from 'chart.js';
import { Line } from 'react-chartjs-2';
import { Box } from '../common/components';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Filler,
  Legend
);

export function AreaChart() {
  const labels = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];
  return (
    <Box>
      <Line
        data={{
          labels,
          datasets: [
            {
              fill: true,
              label: 'Dataset 1',
              data: labels.map(() => Math.floor(Math.random() * 101)),
              borderColor: 'rgb(255, 99, 132)',
              backgroundColor: 'rgba(255, 99, 132, 0.5)',
              yAxisID: 'y',
            },
            {
              fill: true,
              label: 'Dataset 2',
              data: labels.map(() => Math.floor(Math.random() * 101)),
              borderColor: 'rgb(53, 162, 235)',
              backgroundColor: 'rgba(53, 162, 235, 0.5)',
              yAxisID: 'y1',
            },
          ]
        }}
        options={{
          responsive: true,
          interaction: {
            mode: 'index' as const,
            intersect: false,
          },
          // stacked: false,
          plugins: {
            title: {
              display: true,
              text: 'Chart.js Line Chart - Multi Axis',
            },
          },
          scales: {
            y: {
              type: 'linear' as const,
              display: true,
              position: 'left' as const,
            },
            y1: {
              type: 'linear' as const,
              display: true,
              position: 'right' as const,
              grid: {
                drawOnChartArea: false,
              },
            },
          }
        }}
      />
    </Box>
  )
}